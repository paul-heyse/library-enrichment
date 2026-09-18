//! Explicit verification submission and scoped probe evidence.
use super::common;
use crate::{
    envelope,
    execution::{Runner, capsule},
    jobs, metrics,
    service::Service,
};
use enrichment_core::{
    canonical,
    evidence::ArtifactKind,
    execution::{
        JobAction, JobData, JobRequest, ProbeMode, ProcessEnd, VerificationCapture,
        VerificationData, VerifyRequest,
    },
    identity::Ecosystem,
    wire::{Coverage, Envelope, ErrorCode, JobHandle, JobState, Outcome},
};
use std::{sync::Arc, time::Duration};

/// Record a verification the service refused before any probe existed, and return the refusal.
///
/// Every one of these is `Unresolved` by the rule [`metrics::ProbeOutcome`] states: the service
/// could not run the probe, which says nothing about the caller's code. Counting them matters,
/// because without it an operator on an unqualified host reads
/// `verification: {succeeded: 0, failed: 0, unresolved: 0}` — indistinguishable from "nobody
/// asked", which is the exact question the counter exists to answer.
fn refused(service: &Service, envelope: Envelope) -> Envelope {
    service
        .metrics
        .record_probe(metrics::ProbeOutcome::Unresolved);
    envelope
}

pub async fn verify(service: &Service, mut request: VerifyRequest) -> Envelope {
    if let Err(error) = enrichment_store::request_admission::research(
        &service.repository.runtime,
        &request.clone().into(),
        service.config.limits.verification_input_bytes,
    )
    .await
    {
        return refused(
            service,
            common::operation_error(&error, "verification_request_admission"),
        );
    }
    let opened = match common::open_context(
        service,
        &request.context_id,
        request.snapshot_id.as_ref(),
    )
    .await
    {
        Ok(v) => v,
        Err(e) => return *e,
    };
    let readiness = match crate::execution::readiness::assess(
        service,
        opened.release.key.ecosystem,
        request.profile,
    )
    .await
    {
        Ok(readiness) => readiness,
        Err(error) => return common::operation_error(&error, "execution_policy"),
    };
    if !readiness.available {
        return refused(service, crate::execution::readiness::refusal(&readiness));
    }
    let image = readiness
        .image_id
        .as_ref()
        .expect("ready route has an image");
    if (opened.release.key.ecosystem == Ecosystem::Rust && request.mode == ProbeMode::Typecheck)
        || (opened.release.key.ecosystem == Ecosystem::Python && request.mode == ProbeMode::Compile)
    {
        return refused(
            service,
            envelope::error(
                ErrorCode::UnsupportedCapability,
                "The probe mode does not match the language.",
                "Use compile for Rust, typecheck for Python, or runtime with explicit runtime policy.",
                false,
            ),
        );
    }
    request.snapshot_id = Some(opened.snapshot_id.clone());
    let (record, token, new) = match service.jobs.submit(request.clone()).await {
        Ok(v) => v,
        Err(e) => return common::operation_error(&e, "verification_job"),
    };
    if new {
        let owned = service.clone();
        let id = record.job_id;
        let image = image.clone();
        let jobs = std::sync::Arc::clone(&owned.jobs);
        let runtime = owned.repository.runtime.clone();
        if let Err(error) = jobs.spawn(&runtime, id, async move {
            owned
                .repository
                .runtime
                .job_operation(
                    id.to_string(),
                    owned.operation_descriptor(&request.clone().into()),
                    Duration::from_secs(owned.config.execution.deadline_seconds),
                    async {
                        let cancel = match owned.jobs.cancellation(&id) {
                            Ok(v) => v,
                            Err(e) => {
                                eprintln!("job {id}: {e}");
                                return;
                            }
                        };
                        let lease = match owned.lsp.execution_lease(&owned.execution, &cancel).await
                        {
                            Ok(Some(lease)) => lease,
                            Ok(None) => {
                                let result = envelope::error(
                                    ErrorCode::VerificationFailed,
                                    "Cancelled while waiting for execution capacity.",
                                    "Submit a new verification if needed.",
                                    false,
                                );
                                if let Err(error) =
                                    owned.jobs.finish(&id, JobState::Cancelled, result).await
                                {
                                    eprintln!("job {id}: failed to persist cancellation: {error}");
                                }
                                return;
                            }
                            Err(error) => {
                                let result = envelope::error(
                                    ErrorCode::PolicyDenied,
                                    error.to_string(),
                                    "Resolve execution cleanup or restart the daemon.",
                                    false,
                                );
                                if let Err(error) =
                                    owned.jobs.finish(&id, JobState::Failed, result).await
                                {
                                    eprintln!(
                                        "job {id}: failed to persist admission failure: {error}"
                                    );
                                }
                                return;
                            }
                        };
                        let (state, result) = match owned.jobs.start(&id).await {
                            Ok(true) => {
                                execute(&owned, &id, &image, &request, &opened, cancel, lease).await
                            }
                            Ok(false) => (
                                JobState::Cancelled,
                                envelope::error(
                                    ErrorCode::VerificationFailed,
                                    "Cancelled before execution started.",
                                    "Submit a new verification if needed.",
                                    false,
                                ),
                            ),
                            Err(e) => (
                                JobState::Failed,
                                common::operation_error(&e, "verification_job"),
                            ),
                        };
                        if let Err(e) = owned.jobs.finish(&id, state, result).await {
                            eprintln!("job {id}: failed to persist terminal result: {e}");
                        }
                    },
                )
                .await;
        }) {
            return common::operation_error(&error, "native_effect_owner");
        }
    }
    let latest = wait(
        service,
        &record.job_id,
        service.config.limits.inline_wait_seconds.min(10),
    )
    .await;
    match latest {
        Ok(record) if jobs::terminal(record.state) => record.result.unwrap_or_else(|| {
            envelope::error(
                ErrorCode::VerificationFailed,
                "Terminal job lacks a result.",
                "Inspect the daemon journal.",
                false,
            )
        }),
        Ok(record) => pending(record.data(Some(token))),
        Err(e) => common::operation_error(&e, "verification_job"),
    }
}

async fn execute(
    service: &Service,
    job: &enrichment_core::identity::JobId,
    image: &str,
    request: &VerifyRequest,
    opened: &common::Opened,
    cancel: Arc<std::sync::atomic::AtomicBool>,
    lease: Arc<crate::execution::cleanup::Lease>,
) -> (JobState, Envelope) {
    let result = execute_inner(service, job, image, request, opened, cancel, lease.clone()).await;
    // Keep the journal running/cancel_requested while removal retries. In particular, a
    // Cancelled process observation is not proof that the container stopped.
    if let Err(error) = service.execution.wait_for_cleanup(&lease).await {
        service
            .metrics
            .record_probe(metrics::ProbeOutcome::Unresolved);
        let mut failed = envelope::error(
            ErrorCode::PolicyDenied,
            error.to_string(),
            "Inspect the owned container and retained workspace, restore cleanup, then restart the daemon; execution remains quarantined.",
            false,
        );
        match result {
            Ok(produced) => {
                failed.data = produced.result.data;
                failed.artifacts = produced.result.artifacts;
            }
            Err(capsule::PreparationError::Process(observation, stage)) => {
                failed
                    .error_mut()
                    .expect("typed cleanup error")
                    .diagnostic
                    .stage = stage.clone();
                let raw = serde_json::to_vec(&observation).expect("process observation");
                match store(
                    service,
                    &raw,
                    &format!("service://jobs/{job}/failed-stage"),
                    "application/json",
                ) {
                    Ok(artifact) => {
                        failed
                            .error_mut()
                            .expect("typed cleanup error")
                            .diagnostic
                            .actions
                            .push(enrichment_core::wire::RecoveryAction::ReadArtifact {
                                artifact_id: artifact.artifact_id.clone(),
                                section: None,
                                cursor: None,
                            });
                        if let Some(handle) = common::handle_for(&artifact, stage) {
                            failed.artifacts.push(handle);
                        }
                    }
                    Err(error) => failed
                        .coverage
                        .limitations
                        .push(format!("Could not retain failure artifact: {error}")),
                }
            }
            Err(_) => {}
        }
        return (JobState::Failed, failed);
    }
    // This is the one funnel every probe leaves through, so it is where §14.3's verification
    // outcomes are counted. The classification is the point: a probe that RAN and failed is an
    // observation about the caller's code, while one the service could not run -- an
    // unqualified image, a denied profile, an unresolvable environment -- is an observation
    // about the service. Merging them would attribute our limitations to their library.
    match result {
        Ok(produced) => {
            let v = match publish_completed(service, image, opened, &produced).await {
                Ok(published) => published,
                Err(error) => {
                    let mut failed = envelope::error(
                        ErrorCode::VerificationFailed,
                        format!("The probe settled but its result was not published: {error}"),
                        "Inspect the durable job and retained attempt artifacts before explicitly retrying.",
                        false,
                    );
                    failed.data = produced.result.data;
                    failed.artifacts = produced.result.artifacts;
                    return (JobState::Failed, failed);
                }
            };
            service.metrics.record_probe(match v.0 {
                JobState::Succeeded | JobState::Partial => metrics::ProbeOutcome::Succeeded,
                JobState::Failed => metrics::ProbeOutcome::Failed,
                // Cancelled work reached no verdict at all; counting it either way would
                // invent one.
                _ => return v,
            });
            v
        }
        Err(error) => {
            let (state, code, message, observation) = match error {
                capsule::PreparationError::Cancelled => (JobState::Cancelled, ErrorCode::VerificationFailed, "Cancelled during bounded dependency acquisition; no execution process remains.".into(), None),
                capsule::PreparationError::Environment(message) => (
                    JobState::Failed,
                    ErrorCode::EnvironmentUnresolved,
                    message,
                    None,
                ),
                capsule::PreparationError::Policy(message) => {
                    (JobState::Failed, ErrorCode::PolicyDenied, message, None)
                }
                capsule::PreparationError::Process(observation, stage) => {
                    let state = if observation.end == ProcessEnd::Cancelled {
                        JobState::Cancelled
                    } else {
                        JobState::Failed
                    };
                    let code = if observation.exit_code == Some(125) {
                        ErrorCode::PolicyDenied
                    } else if observation.end != ProcessEnd::Exited {
                        ErrorCode::VerificationFailed
                    } else {
                        ErrorCode::EnvironmentUnresolved
                    };
                    (
                        state,
                        code,
                        format!(
                            "{stage} did not succeed; retained process evidence explains the outcome."
                        ),
                        Some(observation),
                    )
                }
            };
            // Every error in this branch is a `PreparationError`: the capsule, the dependency
            // closure, or the broker. The caller's snippet never ran, so none of them is an
            // observation about their code -- including a preparation-stage timeout, which an
            // earlier version counted as `Failed` because its error code happened to be
            // `VerificationFailed`. Cancelled work reached no verdict and is counted neither way.
            if state != JobState::Cancelled {
                service
                    .metrics
                    .record_probe(metrics::ProbeOutcome::Unresolved);
            }
            let mut result = envelope::error(
                code,
                message,
                "Inspect the retained process evidence and exact environment prerequisites; no host fallback is allowed.",
                false,
            );
            if let Some(observation) = observation {
                let raw = serde_json::to_vec(&observation).expect("process observation");
                match store(
                    service,
                    &raw,
                    &format!("service://jobs/{job}/failed-stage"),
                    "application/json",
                ) {
                    Ok(artifact) => {
                        let diagnostic = &mut result
                            .error_mut()
                            .expect("typed preparation error")
                            .diagnostic;
                        diagnostic.stage = "execution_preparation".into();
                        diagnostic.actions.push(
                            enrichment_core::wire::RecoveryAction::ReadArtifact {
                                artifact_id: artifact.artifact_id.clone(),
                                section: None,
                                cursor: None,
                            },
                        );
                        if let Some(handle) = common::handle_for(
                            &artifact,
                            "Failed preparation process evidence".into(),
                        ) {
                            result.artifacts.push(handle);
                        }
                    }
                    Err(e) => result
                        .coverage
                        .limitations
                        .push(format!("Could not retain failure artifact: {e}")),
                }
            }
            (state, result)
        }
    }
}
struct ProducedProbe {
    result: Envelope,
    capture: VerificationCapture,
}

async fn execute_inner(
    service: &Service,
    job: &enrichment_core::identity::JobId,
    image: &str,
    request: &VerifyRequest,
    opened: &common::Opened,
    cancel: Arc<std::sync::atomic::AtomicBool>,
    lease: Arc<crate::execution::cleanup::Lease>,
) -> Result<ProducedProbe, capsule::PreparationError> {
    let runner = Runner::new(
        &service.config.execution,
        &service.paths.cache_root,
        std::sync::Arc::clone(&service.execution),
        service.ownership.clone(),
    )
    .map_err(|e| capsule::PreparationError::from_runner(&e))?
    .using_lease(lease);
    let containment_identity = runner.containment_identity().map_err(|e| e.to_string())?;
    let snippet = store(
        service,
        request.snippet.as_bytes(),
        &format!(
            "consumer://snippet/{}",
            canonical::sha256_hex(request.snippet.as_bytes())
        ),
        "text/plain",
    )?;
    let mut capsule = capsule::prepare(
        service,
        opened,
        &runner,
        image,
        &job.to_string(),
        cancel.clone(),
    )
    .await?;
    let input_artifacts = super::inspect_execution::capsule_inputs(service, opened, &capsule)
        .await
        .map_err(|e| e.to_string())?;
    let lock = store(
        service,
        capsule.prepared.lock.as_bytes(),
        &format!(
            "consumer://lock/{}",
            canonical::sha256_hex(capsule.prepared.lock.as_bytes())
        ),
        "text/plain",
    )?;
    let derived_context = opened
        .context
        .derived_with(capsule.prepared.environment.environment_id.clone());
    let consumer_path = match opened.release.key.ecosystem {
        Ecosystem::Python => "consumer.py",
        Ecosystem::Rust => "src/main.rs",
    };
    capsule
        .write_input(consumer_path, &request.snippet)
        .map_err(|error| error.to_string())?;
    let invocation = enrichment_core::execution::producer::Invocation::Probe {
        ecosystem: opened.release.key.ecosystem,
        mode: request.mode,
    };
    let observation = runner
        .for_capsule(&capsule)
        .run(image, &capsule.root, &invocation, cancel)
        .await
        .map_err(|e| capsule::PreparationError::from_runner(&e))?;
    capsule.observations.push(observation);
    let capture = VerificationCapture {
        job_id: *job,
        request: request.clone(),
        source_release: opened.release.clone(),
        source_snapshot: opened.snapshot_id.clone(),
        environment: capsule.prepared.environment.clone(),
        containment_identity,
        observations: capsule.observations.clone(),
        input_artifacts,
    };
    let lowered = enrichment_store::probe_plan::lower(&service.repository.runtime, &capture)
        .await
        .map_err(|error| error.to_string())?;
    let raw = serde_json::to_vec(&capture).map_err(|e| e.to_string())?;
    let result_artifact = store(
        service,
        &raw,
        &format!("service://jobs/{job}/verification-result"),
        "application/json",
    )?;
    let data = VerificationData {
        evidence_class: lowered.evidence_class,
        producer_runs: Vec::new(),
        source_context_id: opened.context.context_id.clone(),
        source_snapshot_id: opened.snapshot_id.clone(),
        derived_context: Some(derived_context),
        derived_snapshot_id: None,
        environment: Some(capsule.prepared.environment.clone()),
        mode: request.mode,
        profile: request.profile,
        snippet_origin: "agent".into(),
        test_intent: request.test_intent.clone(),
        snippet_artifact_id: snippet.artifact_id.clone(),
        lock_artifact_id: Some(lock.artifact_id.clone()),
        result_artifact_id: result_artifact.artifact_id.clone(),
        observations: capsule.observations.clone(),
        limitations: lowered.limitations.clone(),
    };
    let mut result = probe_envelope(&lowered, data);
    result.context_id = Some(opened.context.context_id.clone());
    result.snapshot_id = Some(opened.snapshot_id.clone());
    result.artifacts = [&snippet, &lock, &result_artifact]
        .into_iter()
        .filter_map(|artifact| {
            common::handle_for(artifact, "Verification input or process evidence".into())
        })
        .collect();
    Ok(ProducedProbe { result, capture })
}
fn probe_envelope(
    selected: &enrichment_store::probe_plan::LoweredProbe,
    data: VerificationData,
) -> Envelope {
    Envelope::new(
        enrichment_core::wire::EnvelopeBody {
            request_id: envelope::new_request_id(),
            summary: selected.summary.clone(),
            context_id: None,
            snapshot_id: None,
            data: data.into(),
            coverage: selected.coverage.clone(),
            freshness: envelope::unverified_freshness(),
            evidence: Vec::new(),
            artifacts: Vec::new(),
            delivery: Default::default(),
        },
        selected.outcome.clone(),
    )
}

/// Called only after the supervisor confirms all one-shot execution has settled.
async fn publish_completed(
    service: &Service,
    image: &str,
    opened: &common::Opened,
    produced: &ProducedProbe,
) -> Result<(JobState, Envelope), String> {
    let capture = &produced.capture;
    let job = &capture.job_id;
    let request = &capture.request;
    let lowered = enrichment_store::probe_plan::settled(&service.repository.runtime, capture)
        .await
        .map_err(|error| error.to_string())?;
    let mut result = produced.result.clone();
    use enrichment_core::evidence::{relational::SubjectRef, snapshot::SnapshotMetadata};
    let enrichment_core::wire::data::ToolData::VerifyUsage(data) = result.data.clone() else {
        return Err("verification publication requires its native payload".into());
    };
    let mut data = *data;
    service
        .repository
        .catalog
        .retain_artifacts(
            &service.repository.runtime,
            &result
                .artifacts
                .iter()
                .map(|handle| handle.receipt.clone())
                .collect::<Vec<_>>(),
        )
        .await
        .map_err(|e| e.to_string())?;
    let pin = service
        .repository
        .catalog
        .pin()
        .await
        .map_err(|e| e.to_string())?;
    let find = async |id: &str| {
        pin.artifact(&service.repository.runtime, id)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| format!("missing producer artifact {id}"))
    };
    let snippet = find(&data.snippet_artifact_id).await?;
    let lock = find(
        data.lock_artifact_id
            .as_deref()
            .ok_or("missing resolved lock")?,
    )
    .await?;
    let receipt = find(&data.result_artifact_id).await?;
    let containment = capture.containment_identity.as_str();
    let input_artifacts = capture.input_artifacts.clone();
    let last = data
        .observations
        .last()
        .ok_or("probe has no process observation")?;
    let payload = lowered.payload.clone();
    let bytes = payload.canonical_bytes()?;
    let canonical_result = store(
        service,
        &bytes,
        "producer-result://usage-probe/3",
        "application/vnd.library-enrichment.canonical-arrow",
    )?;
    let environment = data
        .environment
        .clone()
        .ok_or("probe lacks resolved environment")?;
    use enrichment_store::producer_run_plan::{Attempt, ReceiptInput, Role};
    let mut inputs: Vec<_> = input_artifacts
        .iter()
        .cloned()
        .map(|artifact| ReceiptInput {
            role: Role::Dependency,
            artifact,
        })
        .collect();
    inputs.extend(
        [
            ("snippet", snippet.clone()),
            ("dependency-lock", lock.clone()),
            ("result", canonical_result.clone()),
        ]
        .map(|(name, artifact)| ReceiptInput {
            role: Role::Named { name: name.into() },
            artifact,
        }),
    );
    let inputs =
        enrichment_store::producer_run_plan::receipt_inputs(&service.repository.runtime, &inputs)
            .await
            .map_err(|error| error.to_string())?;
    let run = enrichment_store::producer_run_plan::compose(
        &service.repository.runtime,
        Attempt {
            attempt_id: enrichment_core::identity::AttemptId::new(),
            producer: "consumer-probe".into(),
            producer_version: "2".into(),
            config_digest: enrichment_core::native_key::Key::VerificationConfiguration
                .hex_digest(
                    &enrichment_core::operation::identities::VerificationConfiguration {
                        release_id: opened.release.release_id.clone(),
                        environment: environment.clone(),
                        mode: request.mode,
                        image: image.into(),
                        containment: containment.into(),
                    },
                )
                .map_err(|e| e.to_string())?,
            profile: request.profile,
            started_at: last.started_at,
            finished_at: last.finished_at,
            outcome: lowered.run_outcome,
            gaps: vec![],
            log: Some(receipt.artifact_id.clone()),
        },
        &inputs,
    )
    .await
    .map_err(|error| error.to_string())?;
    let facts = enrichment_store::execution_fact_plan::lower(
        &service.repository.runtime,
        &enrichment_store::execution_fact_plan::Producer {
            environment_id: environment.environment_id.clone(),
            image_id: image.into(),
            containment_identity: containment.into(),
            run: run.clone(),
        },
        &[enrichment_store::execution_fact_plan::Observed {
            subject: SubjectRef::Document {
                artifact_id: snippet.artifact_id.clone(),
                heading: "agent consumer snippet".into(),
            },
            payload,
            evidence_class: data.evidence_class,
            artifact: canonical_result.clone(),
        }],
    )
    .await
    .map_err(|error| error.to_string())?;
    let mut publication_artifacts = input_artifacts;
    publication_artifacts.extend([
        snippet.clone(),
        lock.clone(),
        canonical_result.clone(),
        receipt.clone(),
    ]);
    let context = if environment.environment_id == opened.environment.environment_id {
        opened.context.clone()
    } else {
        let derived = opened
            .context
            .derived_with(environment.environment_id.clone());
        service
            .repository
            .derive_environment(opened.reader.pinned(), derived.clone(), environment.clone())
            .await
            .map_err(|e| e.to_string())?;
        derived
    };
    let parent = opened.reader.manifest();
    let metadata = SnapshotMetadata {
        context: context.clone(),
        release: opened.release.clone(),
        environment,
        symbol_package: parent.symbol_package.clone(),
        crate_name: parent.crate_name.clone(),
        crate_version: parent.crate_version.clone(),
        normalizer_version: parent.normalizer_version.clone(),
        observed_configuration: parent.observed_configuration.clone(),
        producer_items: parent.producer_items,
    };
    data.producer_runs = vec![run.clone()];
    data.derived_context = Some(context.clone());
    data.derived_snapshot_id = None;
    data.result_artifact_id = canonical_result.artifact_id.clone();
    data.limitations = lowered.limitations.clone();
    let handles: Vec<_> = [&snippet, &lock, &canonical_result, &receipt]
        .into_iter()
        .filter_map(|a| {
            common::handle_for(
                a,
                "Retained verification input, result or attempt log".into(),
            )
        })
        .collect();
    crate::delivery::size(&data, crate::delivery::MAX_RESULT_BYTES).map_err(|e| e.to_string())?;
    let context_id = result.context_id.clone();
    let snapshot_id = result.snapshot_id.clone();
    result = probe_envelope(&lowered, data);
    result.context_id = context_id;
    result.snapshot_id = snapshot_id;
    result.artifacts = handles;
    let result = enrichment_core::operation::results::ResultRecord::from_envelope(&result)?;
    let manifest = service
        .repository
        .publish_execution(
            metadata,
            facts,
            run.clone(),
            publication_artifacts,
            enrichment_store::repository::JobCompletion {
                publication_fence: service
                    .jobs
                    .publication_fence(job)
                    .map_err(|error| error.to_string())?,
                job_id: *job,
                kind: enrichment_core::evidence::catalog::PublishedJobKind::Verify,

                attempt_id: run.attempt_id,
                result_artifact_ids: vec![canonical_result.artifact_id.clone()],
                result,
            },
        )
        .await
        .map_err(|e| e.to_string())?;
    Ok((
        manifest
            .state
            .ok_or("published verification lacks native state")?,
        manifest
            .result
            .ok_or("published verification lacks native result")?,
    ))
}

fn store(
    service: &Service,
    bytes: &[u8],
    uri: &str,
    media: &str,
) -> Result<enrichment_core::evidence::Artifact, String> {
    let retrieved_at =
        enrichment_core::native_time::AcquisitionTime::now().map_err(|error| error.to_string())?;
    service
        .blobs
        .put(bytes, |_| {
            enrichment_core::evidence::Artifact::describe(
                bytes,
                ArtifactKind::Other,
                media,
                uri,
                retrieved_at,
            )
        })
        // This call's record, not the stored one: these URIs are job-scoped
        // (`service://jobs/{job}/...`), so identical bytes from two jobs would otherwise cite the
        // first job as the source of the second job's evidence.
        .map(|b| b.acquired)
        .map_err(|e| e.to_string())
}

/// Recover a committed result after catalog publication but before terminal journal delivery.
/// This path is read-only and never admits a container or consults current execution policy.
pub async fn recover(
    repository: &enrichment_store::repository::EvidenceRepository,
    blobs: &enrichment_store::BlobStore,
    record: &jobs::JobRecord,
) -> std::io::Result<Option<(JobState, Envelope)>> {
    if matches!(record.specification, jobs::Arguments::Compare { .. }) {
        return super::compare_job::recover(repository, blobs, record).await;
    }
    let Some(recovered) =
        enrichment_store::job_recovery::recover(repository, blobs, &record.snapshot)
            .await
            .map_err(std::io::Error::other)?
    else {
        return Ok(None);
    };
    let result = crate::delivery::recover_job(
        blobs,
        &repository.runtime,
        &recovered.catalog,
        &recovered.publication,
    )
    .await?;
    Ok(Some((recovered.publication.state, result)))
}

pub async fn control(service: &Service, request: JobRequest) -> Envelope {
    let result = match request.action {
        JobAction::Cancel => match request.interest_token.as_ref() {
            Some(token) => service.jobs.cancel(&request.job_id, token).await,
            None => {
                return envelope::error(
                    ErrorCode::PolicyDenied,
                    "Cancellation requires your interest token.",
                    "Pass the interest_token returned by your verification submission.",
                    false,
                );
            }
        },
        JobAction::Wait => {
            wait(
                service,
                &request.job_id,
                request
                    .wait_seconds
                    .min(service.config.limits.max_job_wait_seconds)
                    .min(30),
            )
            .await
        }
        JobAction::Status => service.jobs.get(&request.job_id).await,
    };
    match result {
        Ok(record) => {
            let data = record.data(request.interest_token).into();
            envelope::ok(
                "Job state read from the durable journal; inspect result for the probe outcome.",
                data,
                Coverage {
                    details: None,
                    assessments: Vec::new(),
                    scope: "one durable job".into(),
                    indexed: Default::default(),
                    missing: Default::default(),
                    limitations: vec![],
                },
            )
        }
        Err(e) => common::job_error(e, &request.job_id),
    }
}
pub(super) async fn wait(
    service: &Service,
    id: &enrichment_core::identity::JobId,
    seconds: u64,
) -> std::io::Result<jobs::JobRecord> {
    let deadline = tokio::time::Instant::now() + Duration::from_secs(seconds);
    loop {
        let record = service.jobs.get(id).await?;
        if jobs::terminal(record.state) || tokio::time::Instant::now() >= deadline {
            return Ok(record);
        }
        tokio::time::sleep(Duration::from_millis(25)).await;
    }
}
pub(super) fn pending(data: JobData) -> Envelope {
    let job = JobHandle {
        job_id: data.job_id,
        state: data.state,
        stage: data.stage.clone(),
        poll_after_ms: 250,
    };
    let body = enrichment_core::wire::EnvelopeBody {
        request_id: envelope::new_request_id(),
        summary: "Research work is pending; use job_control to wait or cancel your interest."
            .into(),
        context_id: None,
        snapshot_id: None,
        data: data.into(),
        coverage: Coverage {
            details: None,
            assessments: Vec::new(),
            scope: "queued or running execution; no completion claim".into(),
            indexed: Default::default(),
            missing: Default::default(),
            limitations: vec![],
        },
        freshness: envelope::unverified_freshness(),
        evidence: vec![],
        artifacts: vec![],
        delivery: enrichment_core::wire::DeliveryDescriptor::default(),
    };
    Envelope::new(body, Outcome::Pending { job })
}
