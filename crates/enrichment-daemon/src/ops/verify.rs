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
        JobAction, JobData, JobRequest, ProbeMode, ProcessEnd, VerificationData, VerifyRequest,
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
    if let Err(message) = request.validate_shape(&service.config) {
        return refused(
            service,
            envelope::error(
                ErrorCode::PolicyDenied,
                message,
                "Choose the matching compile/typecheck/runtime mode and build/runtime profile.",
                false,
            ),
        );
    }
    let opened =
        match common::open_context(service, &request.context_id, request.snapshot_id.as_deref())
            .await
        {
            Ok(v) => v,
            Err(e) => return *e,
        };
    let readiness =
        crate::execution::readiness::assess(service, opened.release.key.ecosystem, request.profile);
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
    // Admission is refused while an owned container's absence is unconfirmed. Queueing behind
    // an unwatched boundary would let repeated cleanup failures exceed the worker bound.
    if let crate::execution::cleanup::Admission::Quarantined { detail, .. } =
        service.execution.admission()
    {
        return refused(
            service,
            envelope::error(
                ErrorCode::PolicyDenied,
                "New isolated execution is quarantined while owned container cleanup is unresolved.",
                detail,
                true,
            ),
        );
    }
    request.snapshot_id = Some(opened.snapshot_id.as_str().into());
    let mut key_request = request.clone();
    key_request.max_bytes = None;
    let key = canonical::digest_hex(
        &serde_json::json!({"producer":"verification-1", "request":key_request, "image":image, "environment":opened.environment, "execution":format!("{:?}", service.config.execution)}),
    );
    let (record, token, new) = match service.jobs.submit(key, request.clone()) {
        Ok(v) => v,
        Err(e) => return common::operation_error(&e, "verification_job"),
    };
    if new {
        let owned = service.clone();
        let id = record.job_id.clone();
        let image = image.clone();
        tokio::spawn(async move {
            owned
                .repository
                .runtime
                .job_operation(
                    id.clone(),
                    owned.operation_descriptor("usage.verify", &request),
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
                                    owned.jobs.finish(&id, JobState::Cancelled, result)
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
                                if let Err(error) = owned.jobs.finish(&id, JobState::Failed, result)
                                {
                                    eprintln!(
                                        "job {id}: failed to persist admission failure: {error}"
                                    );
                                }
                                return;
                            }
                        };
                        let (state, result) = match owned.jobs.start(&id) {
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
                        if let Err(e) = owned.jobs.finish(&id, state, result) {
                            eprintln!("job {id}: failed to persist terminal result: {e}");
                        }
                    },
                )
                .await;
        });
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
    job: &str,
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
            Ok((_, evidence)) => {
                failed.data = evidence.data;
                failed.artifacts = evidence.artifacts;
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
        Ok(mut v) => {
            match publish_completed(service, job, image, request, opened, v.0, v.1.clone()).await {
                Ok(published) => v = published,
                Err(error) => {
                    let mut failed = envelope::error(
                        ErrorCode::VerificationFailed,
                        format!("The probe settled but its result was not published: {error}"),
                        "Inspect the durable job and retained attempt artifacts before explicitly retrying.",
                        false,
                    );
                    failed.data = v.1.data;
                    failed.artifacts = v.1.artifacts;
                    return (JobState::Failed, failed);
                }
            }
            v.1.coverage.limitations.push(
                "Owned container absence was confirmed before the job became terminal; process observations retain the result of each initial cleanup attempt.".into(),
            );
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
async fn execute_inner(
    service: &Service,
    job: &str,
    image: &str,
    request: &VerifyRequest,
    opened: &common::Opened,
    cancel: Arc<std::sync::atomic::AtomicBool>,
    lease: Arc<crate::execution::cleanup::Lease>,
) -> Result<(JobState, Envelope), capsule::PreparationError> {
    let runner = Runner::new(
        &service.config.execution,
        &service.paths.cache_root,
        std::sync::Arc::clone(&service.execution),
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
    let mut capsule =
        capsule::prepare(service, opened, &runner, image, job, cancel.clone()).await?;
    let input_artifacts = super::inspect_execution::capsule_inputs(service, opened, &capsule)
        .await
        .map_err(|e| e.to_string())?;
    let lock = store(
        service,
        &capsule.lock,
        &format!("consumer://lock/{}", canonical::sha256_hex(&capsule.lock)),
        "text/plain",
    )?;
    let derived_context = opened
        .context
        .derived_with(capsule.environment.environment_id.clone());
    let args = match opened.release.key.ecosystem {
        Ecosystem::Python => {
            capsule
                .write_input("consumer.py", &request.snippet)
                .map_err(|e| e.to_string())?;
            if request.mode == ProbeMode::Typecheck {
                capsule::strings(&[
                    "/opt/producers/bin/ty",
                    "check",
                    "--project=/capsule",
                    "--python=/usr/local/bin/python3",
                    "--extra-search-path=/capsule/python",
                    "--python-version=3.14",
                    "--python-platform=linux",
                    "--output-format=concise",
                    "--color=never",
                    "--no-progress",
                    "--config-file=/capsule/probe-config/ty.toml",
                    "/capsule/consumer.py",
                ])
            } else {
                // Fixed bootstrap sets only the isolated dependency path, then executes the stored caller snippet.
                capsule::strings(&[
                    "/usr/local/bin/python3",
                    "-I",
                    "-S",
                    "-c",
                    "import sys,runpy; sys.path.insert(0,'/capsule/python'); runpy.run_path('/capsule/consumer.py',run_name='__main__')",
                ])
            }
        }
        Ecosystem::Rust => {
            capsule
                .write_input("src/main.rs", &request.snippet)
                .map_err(|e| e.to_string())?;
            capsule::strings(&[
                "/usr/local/cargo/bin/cargo",
                "+1.98.1",
                if request.mode == ProbeMode::Runtime {
                    "run"
                } else {
                    "check"
                },
                "--frozen",
                "--manifest-path=/capsule/Cargo.toml",
                "--target=x86_64-unknown-linux-gnu",
            ])
        }
    };
    let observation = runner
        .run(image, &capsule.root, &args, cancel)
        .await
        .map_err(|e| capsule::PreparationError::from_runner(&e))?;
    let success = observation.end == ProcessEnd::Exited && observation.exit_code == Some(0);
    // An unconfirmed removal is a boundary fact, not a probe fact. The probe evidence stays
    // whole; the result stops short of `ok` and says which assurance is missing.
    let cleanup_confirmed = observation.cleanup_confirmed;
    let state = if observation.end == ProcessEnd::Cancelled {
        JobState::Cancelled
    } else if !cleanup_confirmed && success {
        JobState::Partial
    } else if success {
        JobState::Succeeded
    } else {
        JobState::Failed
    };
    capsule.observations.push(observation);
    let mut limitations = vec!["Only this agent-supplied snippet was checked; no complete compatibility or assertion-coverage claim.".into(), "Synthetic consumer: project files, project lock, private configuration and native system dependencies were not imported.".into()];
    if !cleanup_confirmed {
        limitations.push(
            "Removal of the owned execution container was not confirmed; the cleanup supervisor \
             is still retrying and new execution admission may be quarantined."
                .into(),
        );
    }
    let raw = serde_json::to_vec(&serde_json::json!({"job_id":job,"request":request,"source_release":opened.release,"source_snapshot":opened.snapshot_id,"environment":capsule.environment,"containment_identity":containment_identity,"observations":capsule.observations,"limitations":limitations,"input_artifacts":input_artifacts})).map_err(|e| e.to_string())?;
    let result_artifact = store(
        service,
        &raw,
        &format!("service://jobs/{job}/verification-result"),
        "application/json",
    )?;
    let evidence_class = match request.mode {
        ProbeMode::Compile => enrichment_core::wire::EvidenceClass::CompilerDerived,
        ProbeMode::Typecheck => enrichment_core::wire::EvidenceClass::TypecheckerObserved,
        ProbeMode::Runtime => enrichment_core::wire::EvidenceClass::RuntimeObserved,
    };
    let data = VerificationData {
        evidence_class,
        producer_runs: Vec::new(),
        source_context_id: opened.context.context_id.as_str().into(),
        source_snapshot_id: opened.snapshot_id.as_str().into(),
        derived_context: Some(derived_context),
        derived_snapshot_id: None,
        environment: Some(capsule.environment.clone()),
        mode: request.mode,
        profile: request.profile,
        snippet_origin: "agent".into(),
        test_intent: request.test_intent.clone(),
        snippet_artifact_id: snippet.artifact_id.clone(),
        lock_artifact_id: Some(lock.artifact_id.clone()),
        result_artifact_id: result_artifact.artifact_id.clone(),
        observations: capsule.observations.clone(),
        limitations: limitations.clone(),
    };
    let payload = serde_json::to_value(data)
        .map_err(|e| e.to_string())?
        .as_object()
        .cloned()
        .ok_or("verification payload is not an object")?;
    let mut result = if success && cleanup_confirmed {
        envelope::ok(
            "The requested isolated consumer probe completed successfully within its recorded scope.",
            payload,
            Coverage {
                details: None,
                assessments: Vec::new(),
                scope: format!("{:?} of one supplied consumer snippet", request.mode),
                indexed: Default::default(),
                missing: Default::default(),
                limitations,
            },
        )
    } else if success {
        envelope::partial(
            "The probe completed, but removal of its execution container was not confirmed.",
            payload,
            Coverage {
                details: None,
                assessments: Vec::new(),
                scope: format!("{:?} of one supplied consumer snippet", request.mode),
                indexed: Default::default(),
                missing: ["confirmed execution container removal".to_owned()]
                    .into_iter()
                    .collect(),
                limitations,
            },
        )
    } else {
        let mut result = envelope::error(
            ErrorCode::VerificationFailed,
            "The isolated consumer probe did not succeed; read the recorded process outcome and logs.",
            "Correct the snippet or environment using the retained diagnostics, then submit a new probe.",
            false,
        );
        result.data = payload;
        result
    };
    result.context_id = Some(opened.context.context_id.as_str().into());
    result.snapshot_id = Some(opened.snapshot_id.as_str().into());
    result.artifacts = [&snippet, &lock, &result_artifact]
        .into_iter()
        .filter_map(|artifact| {
            common::handle_for(artifact, "Verification input or process evidence".into())
        })
        .collect();
    Ok((state, result))
}
/// Called only after the supervisor confirms all one-shot execution has settled.
async fn publish_completed(
    service: &Service,
    job: &str,
    image: &str,
    request: &VerifyRequest,
    opened: &common::Opened,
    state: JobState,
    mut result: Envelope,
) -> Result<(JobState, Envelope), String> {
    use enrichment_core::{
        evidence::{
            execution::{ExecutionObservation, ExecutionPayload, UsageProbe},
            ingest,
            relational::{FactSource, Locator, SubjectRef},
            snapshot::SnapshotMetadata,
        },
        producer::{ProducerRun, RunOutcome},
        wire::SourceVersionMatch,
    };
    let mut data: VerificationData =
        serde_json::from_value(serde_json::Value::Object(result.data.clone()))
            .map_err(|e| e.to_string())?;
    let find = |id: &str| {
        service
            .blobs
            .find(id)
            .map_err(|e| e.to_string())?
            .ok_or_else(|| format!("missing producer artifact {id}"))
    };
    let snippet = find(&data.snippet_artifact_id)?;
    let lock = find(
        data.lock_artifact_id
            .as_deref()
            .ok_or("missing resolved lock")?,
    )?;
    let receipt = find(&data.result_artifact_id)?;
    let raw: serde_json::Value = serde_json::from_slice(
        &service
            .blobs
            .read(&receipt.sha256)
            .map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    let containment = raw["containment_identity"]
        .as_str()
        .ok_or("attempt lacks containment identity")?;
    if crate::execution::description::containment_identity(&service.config.execution)
        .map_err(|e| e.to_string())?
        != containment
    {
        return Err("containment identity changed while the producer was running".into());
    }
    let input_artifacts: Vec<enrichment_core::evidence::Artifact> =
        serde_json::from_value(raw["input_artifacts"].clone()).map_err(|e| e.to_string())?;
    if input_artifacts.len() > 4098 {
        return Err("probe dependency input count exceeds its bound".into());
    }
    let last = data
        .observations
        .last()
        .ok_or("probe has no process observation")?;
    let payload = ExecutionPayload::UsageProbe(UsageProbe {
        mode: request.mode,
        snippet_artifact_id: snippet.artifact_id.clone(),
        end: last.end,
        exit_code: last.exit_code,
        stdout: last.stdout.clone(),
        stderr: last.stderr.clone(),
    });
    let bytes = payload.canonical_bytes()?;
    let canonical_result = store(
        service,
        &bytes,
        "producer-result://usage-probe/2",
        "application/json",
    )?;
    let environment = data
        .environment
        .clone()
        .ok_or("probe lacks resolved environment")?;
    let mut run = ProducerRun {
        attempt_id: job.into(),
        producer: "consumer-probe".into(),
        producer_version: "2".into(),
        config_digest: canonical::digest_hex(
            &serde_json::json!({"release":opened.release.release_id,
            "environment":environment,"mode":request.mode,"image":image,"containment":containment}),
        ),
        inputs: [
            ("snippet".into(), snippet.sha256.clone()),
            ("dependency-lock".into(), lock.sha256.clone()),
            ("result".into(), canonical_result.sha256.clone()),
        ]
        .into(),
        profile: request.profile,
        started_at: last.started_at.clone(),
        finished_at: last.finished_at.clone(),
        outcome: if last.end == ProcessEnd::Exited && last.exit_code == Some(0) {
            RunOutcome::Succeeded
        } else {
            RunOutcome::Failed
        },
        gaps: vec![],
        log: Some(receipt.artifact_id.clone()),
    };
    for artifact in &input_artifacts {
        run.inputs.insert(
            format!("dependency:{}", artifact.sha256),
            artifact.sha256.clone(),
        );
    }
    let fact = ExecutionObservation::new(
        SubjectRef::Document {
            artifact_id: snippet.artifact_id.clone(),
            heading: "agent consumer snippet".into(),
        },
        environment.environment_id.to_string(),
        image.into(),
        containment.into(),
        payload,
        FactSource {
            producer_binding_id: run.semantic_binding_id(),
            extractor: "consumer-probe".into(),
            extractor_version: "2".into(),
            artifact_id: canonical_result.artifact_id.clone(),
            source_uri: Some(canonical_result.source_uri.clone()),
            source_version_match: SourceVersionMatch::Exact,
            locator: Locator::Artifact,
            evidence_class: data.evidence_class,
        },
    )?;
    let mut acquisitions: std::collections::BTreeMap<_, _> = input_artifacts
        .into_iter()
        .map(|a| (a.sha256.clone(), a))
        .collect();
    for artifact in [
        snippet.clone(),
        lock.clone(),
        canonical_result.clone(),
        receipt.clone(),
    ] {
        acquisitions.insert(artifact.sha256.clone(), artifact);
    }
    let evidence =
        ingest::normalize_execution(fact, run.clone(), acquisitions.into_values().collect())?;
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
    // Eventual cleanup is now certain; a partial state caused solely by initial removal delay
    // can become successful. The actual initial ProcessObservation remains unchanged.
    let state = if state == JobState::Partial && run.outcome == RunOutcome::Succeeded {
        JobState::Succeeded
    } else {
        state
    };
    data.producer_runs = vec![run.clone()];
    data.derived_context = Some(context.clone());
    data.derived_snapshot_id = None;
    data.result_artifact_id = canonical_result.artifact_id.clone();
    data.limitations
        .retain(|s| !s.starts_with("Removal of the owned execution container"));
    data.limitations
        .push("All owned execution cleanup settled before this result was published.".into());
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
    result.data.clear();
    let (prepare_delivery, delivery) = crate::delivery::prepare_job(
        service.blobs.clone(),
        move |manifest, coverage| {
            let mut data = data.clone();
            data.derived_snapshot_id = Some(manifest.snapshot_id.to_string());
            let mut result = result.clone();
            let payload = common::to_object(&data);
            if state == JobState::Succeeded {
                result = envelope::ok(
                    "The isolated consumer probe succeeded; its scoped result is retained in the published snapshot.",
                    payload,
                    coverage.clone(),
                );
            } else {
                result.data = payload;
                result.coverage = coverage.clone();
            }
            result.coverage.limitations.extend(data.limitations);
            if !result.coverage.complete() {
                result = result.into_partial();
            }
            result.context_id = Some(context.context_id.to_string());
            result.snapshot_id = Some(manifest.snapshot_id.to_string());
            result.artifacts = handles.clone();
            Ok(result)
        },
    );
    let manifest = service
        .repository
        .publish_job(
            metadata,
            evidence,
            enrichment_store::repository::JobCompletion {
                job_id: job.into(),
                kind: enrichment_core::evidence::catalog::PublishedJobKind::Verify,
                state,
                attempt_id: run.attempt_id.clone(),
                result_artifact_ids: vec![canonical_result.artifact_id.clone()],
                prepare_delivery,
            },
        )
        .await
        .map_err(|e| e.to_string())?;
    Ok((
        state,
        delivery
            .get(manifest.snapshot_id.as_str())
            .map_err(|e| e.to_string())?,
    ))
}

fn store(
    service: &Service,
    bytes: &[u8],
    uri: &str,
    media: &str,
) -> Result<enrichment_core::evidence::Artifact, String> {
    service
        .blobs
        .put(bytes, |_| {
            enrichment_store::blob::describe_local(
                bytes,
                ArtifactKind::Other,
                media,
                uri,
                &enrichment_core::clock::now_rfc3339(),
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
    if matches!(record.specification, jobs::JobSpec::Compare(_)) {
        return super::compare_job::recover(repository, blobs, record).await;
    }
    if matches!(record.specification, jobs::JobSpec::Resolve(_)) {
        return super::resolve_job::recover(repository, blobs, record).await;
    }
    if matches!(record.specification, jobs::JobSpec::Inspect(_)) {
        return super::inspect_execution::recover(repository, blobs, record).await;
    }
    let fail = |e: String| std::io::Error::other(e);
    let catalog = repository
        .catalog
        .pin()
        .await
        .map_err(|e| fail(e.to_string()))?;
    let Some(publication) = catalog
        .job_publication(&repository.runtime, &record.job_id)
        .await
        .map_err(|e| fail(e.to_string()))?
    else {
        return Ok(None);
    };
    let jobs::JobSpec::Verify(request) = &record.specification else {
        return Err(fail("unsupported committed job recovery variant".into()));
    };
    if publication.kind != enrichment_core::evidence::catalog::PublishedJobKind::Verify {
        return Err(fail("published job kind differs from its journal".into()));
    }
    let (_context, environment) = catalog
        .context(&repository.runtime, &publication.context_id)
        .await
        .map_err(|e| fail(e.to_string()))?
        .ok_or_else(|| fail("published context missing".into()))?;
    let reader =
        enrichment_store::SnapshotReader::open(repository, catalog, &publication.snapshot_id)
            .await
            .map_err(|e| fail(e.to_string()))?;
    let attempt = reader
        .attempt(&publication.attempt_id)
        .await
        .map_err(|e| fail(e.to_string()))?;
    let log = attempt
        .artifacts
        .iter()
        .find(|a| Some(&a.artifact_id) == attempt.run.log.as_ref())
        .ok_or_else(|| fail("published probe lacks its actual log".into()))?;
    if log.size_bytes > 2 * 1024 * 1024 {
        return Err(fail(
            "probe receipt exceeds journal reconstruction budget".into(),
        ));
    }
    #[derive(serde::Deserialize)]
    struct Receipt {
        job_id: String,
        request: VerifyRequest,
        source_snapshot: String,
        environment: enrichment_core::identity::Environment,
        observations: Vec<enrichment_core::execution::ProcessObservation>,
    }
    let receipt: Receipt = blobs.read_json(log, 2 * 1024 * 1024)?;
    if receipt.job_id != record.job_id
        || receipt.request != *request
        || receipt.environment != environment
        || request.snapshot_id.as_ref() != Some(&receipt.source_snapshot)
    {
        return Err(fail(
            "published receipt disagrees with exact durable inputs".into(),
        ));
    }
    let input = |role: &str| -> std::io::Result<&enrichment_core::evidence::Artifact> {
        let digest = attempt
            .run
            .inputs
            .get(role)
            .ok_or_else(|| fail(format!("missing {role} input")))?;
        let mut found = attempt.artifacts.iter().filter(|a| &a.sha256 == digest);
        let artifact = found
            .next()
            .ok_or_else(|| fail(format!("missing {role} acquisition")))?;
        if found.next().is_some() {
            return Err(fail(format!("ambiguous {role} acquisition")));
        }
        Ok(artifact)
    };
    let snippet = input("snippet")?;
    input("dependency-lock")?;
    let result_artifact = input("result")?;
    if publication.result_artifact_ids != [result_artifact.artifact_id.clone()] {
        return Err(fail("publication result closure differs from probe".into()));
    }
    let last = receipt
        .observations
        .last()
        .ok_or_else(|| fail("probe receipt has no completed observation".into()))?;
    let observed = enrichment_core::evidence::execution::ExecutionPayload::UsageProbe(
        enrichment_core::evidence::execution::UsageProbe {
            mode: request.mode,
            snippet_artifact_id: snippet.artifact_id.clone(),
            end: last.end,
            exit_code: last.exit_code,
            stdout: last.stdout.clone(),
            stderr: last.stderr.clone(),
        },
    );
    if canonical::sha256_hex(&observed.canonical_bytes().map_err(fail)?) != result_artifact.sha256 {
        return Err(fail(
            "normalized probe differs from its retained raw observation".into(),
        ));
    }
    repository
        .validate_delivery(&publication)
        .await
        .map_err(|e| fail(e.to_string()))?;
    Ok(Some((
        publication.state,
        crate::delivery::recover_job(blobs, &publication)?,
    )))
}

pub async fn control(service: &Service, request: JobRequest) -> Envelope {
    let result = match request.action {
        JobAction::Cancel => match request.interest_token.as_deref() {
            Some(token) => service.jobs.cancel(&request.job_id, token),
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
        JobAction::Status => service.jobs.get(&request.job_id),
    };
    match result {
        Ok(record) => {
            let data = serde_json::to_value(record.data(request.interest_token))
                .expect("job data serializes")
                .as_object()
                .cloned()
                .expect("object");
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
    id: &str,
    seconds: u64,
) -> std::io::Result<jobs::JobRecord> {
    let deadline = tokio::time::Instant::now() + Duration::from_secs(seconds);
    loop {
        let record = service.jobs.get(id)?;
        if jobs::terminal(record.state) || tokio::time::Instant::now() >= deadline {
            return Ok(record);
        }
        tokio::time::sleep(Duration::from_millis(25)).await;
    }
}
pub(super) fn pending(data: JobData) -> Envelope {
    let job = JobHandle {
        job_id: data.job_id.clone(),
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
        data: serde_json::to_value(data)
            .expect("job data")
            .as_object()
            .cloned()
            .expect("object"),
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
