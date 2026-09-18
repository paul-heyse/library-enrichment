//! Durable inspection producers and retained native result delivery (ADR-0026).
use super::{common, inspect, verify};
use crate::{execution::capsule, jobs, service::Service};
use enrichment_core::{
    canonical,
    evidence::{
        Artifact, ArtifactKind, SymbolHeader, catalog::PublishedJobKind, execution::*,
        relational::SubjectRef, snapshot::SnapshotMetadata,
    },
    identity::{Ecosystem, Environment},
    policy::ExecutionProfile,
    producer::ProducerRun,
    request::{InspectRequest, InspectionIntent, InspectionOptions},
    wire::{Envelope, ErrorCode, EvidenceClass, JobState},
};
use enrichment_store::{SnapshotReader, query::ExecutionSelection};
use std::{io, io::Read, path::Path, sync::atomic::Ordering};

pub struct Produced {
    pub environment: Environment,
    pub image: String,
    pub containment: String,
    pub producer: String,
    pub version: String,
    pub profile: ExecutionProfile,
    pub started_at: enrichment_core::native_time::ObservationTime,
    pub finished_at: enrichment_core::native_time::ObservationTime,
    pub facts: Vec<(SubjectRef, ExecutionPayload, EvidenceClass)>,
    pub inputs: Vec<Artifact>,
    pub lock: Vec<u8>,
    pub transcript: serde_json::Value,
}

pub use enrichment_core::native_semantics::methods;

/// Exact source/query selection is lowered before result hydration and alternatives bounds.
pub async fn retained_page(
    reader: &SnapshotReader,
    symbol: &SymbolHeader,
    options: Option<&InspectionOptions>,
    runtime: bool,
    limit: usize,
    after: Option<&str>,
) -> Result<enrichment_store::query::NativePage<ExecutionObservation>, enrichment_store::QueryError>
{
    retained_matching(reader, symbol, options, runtime, None, Some((limit, after))).await
}

/// Producer implementation identity is separate from the immutable container identity.
pub(super) fn producer_identity(runtime: bool) -> io::Result<(&'static str, String)> {
    let name = if runtime {
        "runtime-object"
    } else {
        "semantic-inspection"
    };
    let components = std::collections::BTreeMap::from([
        (
            "native-definition".into(),
            enrichment_store::runtime::DEFINITION_REVISION.into(),
        ),
        (
            "semantic-scope".into(),
            enrichment_store::semantic_scope::identity().map_err(io::Error::other)?,
        ),
    ]);
    let digest = enrichment_core::native_key::Key::ProducerImplementation
        .hex_digest(
            &enrichment_core::operation::identities::ProducerImplementation {
                producer: name.into(),
                components,
            },
        )
        .map_err(io::Error::other)?;
    Ok((name, format!("5+{digest}")))
}

async fn retained_matching(
    reader: &SnapshotReader,
    symbol: &SymbolHeader,
    options: Option<&InspectionOptions>,
    runtime: bool,
    qualification: Option<(&str, &str, &str, &str)>,
    page: Option<(usize, Option<&str>)>,
) -> Result<enrichment_store::query::NativePage<ExecutionObservation>, enrichment_store::QueryError>
{
    let default = InspectionOptions::default();
    let options = options.unwrap_or(&default);
    let is_runtime = runtime;
    let consumer = if !is_runtime {
        Some(
            enrichment_store::semantic_grants::consumer(
                reader.runtime(),
                symbol,
                reader.manifest().metadata.ecosystem,
                options,
            )
            .await
            .map_err(io::Error::other)?,
        )
    } else {
        None
    };
    let document_id = consumer.as_ref().map(|c| {
        enrichment_core::evidence::artifact_id_for(&canonical::sha256_hex(c.text.as_bytes()))
    });
    let selection = ExecutionSelection {
        symbol_id: Some(&symbol.symbol_id),
        document_id: document_id.as_deref(),
        kind: Some(if is_runtime {
            "runtime_object"
        } else {
            "semantic_query"
        }),
        methods: if is_runtime { &[] } else { &options.methods },
        position: consumer.as_ref().and_then(|c| c.position),
        runtime: if is_runtime {
            options.runtime.as_ref()
        } else {
            None
        },
        image: qualification.map(|q| q.0),
        containment: qualification.map(|q| q.1),
        producer: qualification.map(|q| (q.2, q.3)),
        ..Default::default()
    };
    if let Some((limit, after)) = page {
        reader.execution_page(selection, limit, after).await
    } else {
        Ok(enrichment_store::query::NativePage {
            items: reader.execution_selection(selection).await?,
            has_more: false,
            next_key: None,
        })
    }
}

/// Only explicit ExecuteOnMiss can discover a directly derived context. Retained reads
/// continue to mean exactly the supplied context, even if another environment is richer.
async fn retained_child(
    service: &Service,
    request: &InspectRequest,
    opened: &common::Opened,
    symbol: &SymbolHeader,
    options: &InspectionOptions,
) -> Result<Option<Envelope>, Box<Envelope>> {
    let catalog = std::sync::Arc::clone(&opened.reader.pinned().catalog);
    let Some(image) = (match opened.release.key.ecosystem {
        Ecosystem::Rust => service.config.execution.rust_image.as_deref(),
        Ecosystem::Python => service.config.execution.python_image.as_deref(),
    }) else {
        return Ok(None);
    };
    let children = enrichment_store::environment_plan::children(
        &service.repository.runtime,
        &catalog,
        &opened.context,
        &opened.environment,
        opened.release.key.ecosystem,
        image,
        &opened.reader.manifest().normalizer_version,
    )
    .await
    .map_err(|error| Box::new(common::operation_error(&error, "inspection_execution")))?;
    if children.is_empty() {
        return Ok(None);
    }
    let containment =
        match crate::execution::description::containment_identity(&service.config.execution) {
            Ok(id) => id,
            Err(_) => return Ok(None),
        };
    let (producer, version) = producer_identity(options.runtime.is_some())
        .map_err(|e| Box::new(common::operation_error(&e, "inspection_execution")))?;
    let mut candidates = Vec::new();
    for child in children {
        let child = common::open_context_at(
            service,
            std::sync::Arc::clone(&catalog),
            &child.context_id,
            None,
        )
        .await?;
        if !opened
            .reader
            .same_static_inputs(&child.reader)
            .await
            .map_err(|error| Box::new(common::query_error(&error)))?
        {
            continue;
        }
        let facts = retained_matching(
            &child.reader,
            symbol,
            Some(options),
            options.runtime.is_some(),
            Some((image, &containment, producer, &version)),
            None,
        )
        .await
        .map_err(|e| Box::new(common::query_error(&e)))?;
        if enrichment_store::inspection_execution_plan::sufficient(
            &service.repository.runtime,
            request,
            &facts.items,
        )
        .await
        .map_err(|error| Box::new(common::operation_error(&error, "inspection_reuse")))?
        {
            candidates.push(
                enrichment_store::inspection_execution_plan::RetainedCandidate {
                    context_id: child.context.context_id.clone(),
                    snapshot_id: child.snapshot_id.clone(),
                },
            );
        }
    }
    let selected = enrichment_store::inspection_execution_plan::retained_choice(
        &service.repository.runtime,
        request,
        &candidates,
    )
    .await
    .map_err(|error| Box::new(common::operation_error(&error, "inspection_reuse")))?;
    if !selected.actions.is_empty() {
        let mut result = crate::envelope::error(
            ErrorCode::UnsupportedFormat,
            "More than one exactly qualified derived context has retained observations.",
            "Select one of the explicit context requests in error.diagnostic.actions.",
            false,
        );
        result
            .error_mut()
            .expect("typed ambiguity")
            .diagnostic
            .actions = selected.actions;
        return Ok(Some(result));
    }
    let Some(selected) = selected.request else {
        return Ok(None);
    };
    Ok(Some(inspect::read(service, selected).await))
}

pub async fn submit(service: &Service, request: InspectRequest) -> Envelope {
    let options = request.execution.clone().unwrap_or_default();
    let initial = inspect::read(service, request.clone()).await;
    let enrichment_core::wire::data::ToolData::InspectSymbol(data) = initial.data.clone() else {
        return initial;
    };
    let Some(symbol) = data.symbol else {
        return initial;
    };
    if options.intent == InspectionIntent::ExecuteOnMiss {
        match enrichment_store::inspection_execution_plan::sufficient(
            &service.repository.runtime,
            &request,
            &data.execution_observations,
        )
        .await
        {
            Ok(true) => return initial,
            Ok(false) => {}
            Err(error) => return common::operation_error(&error, "inspection_reuse"),
        }
    }
    let opened = match common::open_context(
        service,
        &request.context_id,
        initial.snapshot_id.as_ref(),
    )
    .await
    {
        Ok(opened) => opened,
        Err(e) => return *e,
    };
    let admitted = match enrichment_store::inspection_execution_plan::admit_execution(
        &service.repository.runtime,
        &request,
        &symbol,
        opened.release.key.ecosystem,
        &opened.snapshot_id,
    )
    .await
    {
        Ok(admitted) => admitted,
        Err(error) => return common::operation_error(&error, "inspection_execution_scope"),
    };
    let request = admitted.request;
    let profile = admitted.profile;
    if options.intent == InspectionIntent::ExecuteOnMiss {
        match retained_child(service, &request, &opened, &symbol, &options).await {
            Ok(Some(result)) => return result,
            Ok(None) => {}
            Err(result) => return *result,
        }
    }
    let readiness =
        match crate::execution::readiness::assess(service, opened.release.key.ecosystem, profile)
            .await
        {
            Ok(readiness) => readiness,
            Err(error) => return common::operation_error(&error, "execution_policy"),
        };
    if !readiness.available {
        return crate::execution::readiness::refusal(&readiness);
    }
    let (record, token, new) = match service
        .jobs
        .submit(jobs::Arguments::Inspect {
            request: request.clone(),
        })
        .await
    {
        Ok(v) => v,
        Err(e) => return common::operation_error(&e, "inspection_execution"),
    };
    if new {
        let service = service.clone();
        let id = record.job_id;
        let jobs = std::sync::Arc::clone(&service.jobs);
        let runtime = service.repository.runtime.clone();
        if let Err(error) = jobs.spawn(&runtime, id, async move {
            let result = service
                .repository
                .runtime
                .job_operation(
                    id.to_string(),
                    service.operation_descriptor(&request.clone().into()),
                    std::time::Duration::from_secs(service.config.execution.deadline_seconds),
                    run(&service, &id, &opened, &symbol, &request),
                )
                .await;
            let (state, result) = match result {
                Ok(v) => v,
                Err(e) => (
                    if e.kind() == io::ErrorKind::Interrupted {
                        JobState::Cancelled
                    } else {
                        JobState::Failed
                    },
                    crate::envelope::error(
                        ErrorCode::VerificationFailed,
                        e.to_string(),
                        "Inspect retained evidence and retry explicitly after resolving the named prerequisite.",
                        false,
                    ),
                ),
            };
            if let Err(e) = service.jobs.finish(&id, state, result).await {
                eprintln!("inspection job {id}: terminal journal failed: {e}");
            }
        }) {
            return common::operation_error(&error, "native_effect_owner");
        }
    }
    match verify::wait(
        service,
        &record.job_id,
        service.config.limits.inline_wait_seconds.min(10),
    )
    .await
    {
        Ok(record) if jobs::terminal(record.state) => record
            .result
            .unwrap_or_else(|| denied("Terminal inspection lacks its result.")),
        Ok(record) => verify::pending(record.data(Some(token))),
        Err(e) => common::operation_error(&e, "inspection_execution"),
    }
}

fn denied(message: &str) -> Envelope {
    crate::envelope::error(
        ErrorCode::PolicyDenied,
        message,
        "Use retained inspection or configure and qualify the requested execution profile.",
        false,
    )
}

async fn run(
    service: &Service,
    job: &enrichment_core::identity::JobId,
    opened: &common::Opened,
    symbol: &SymbolHeader,
    request: &InspectRequest,
) -> io::Result<(JobState, Envelope)> {
    let cancel = service.jobs.cancellation(job)?;
    if !service.jobs.start(job).await? {
        return Err(io::Error::new(
            io::ErrorKind::Interrupted,
            "cancelled before inspection started",
        ));
    }
    let options = request
        .execution
        .as_ref()
        .ok_or_else(|| io::Error::other("execution options missing"))?;
    let produced = if options.runtime.is_some() {
        super::runtime_object::produce(service, opened, symbol, options, cancel.clone()).await?
    } else {
        super::semantics::produce(service, opened, symbol, options, cancel.clone()).await?
    };
    if cancel.load(Ordering::Acquire) {
        // Semantic producer settlement/discard and one-shot cleanup occurred before returning.
        return Err(io::Error::new(
            io::ErrorKind::Interrupted,
            "inspection cancelled before publication",
        ));
    }
    publish(service, job, request, opened, symbol, produced).await
}

pub fn store(service: &Service, bytes: &[u8], uri: &str, media: &str) -> io::Result<Artifact> {
    let retrieved_at =
        enrichment_core::native_time::AcquisitionTime::now().map_err(std::io::Error::other)?;
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
        .map(|b| b.acquired)
}

/// Only immutable input files copied into the capsule are read; output paths and symlinks
/// cannot turn a returned LSP URI into an arbitrary host read.
pub fn read_input(root: &Path, relative: &Path, limit: u64) -> io::Result<Vec<u8>> {
    if relative.as_os_str().is_empty() {
        return Err(io::Error::other("empty input path"));
    }
    let mut path = root.to_owned();
    for part in relative.components() {
        let std::path::Component::Normal(part) = part else {
            return Err(io::Error::other("input path is not relative"));
        };
        path.push(part);
        if std::fs::symlink_metadata(&path)?.file_type().is_symlink() {
            return Err(io::Error::other("input path contains a symlink"));
        }
    }
    let mut file = std::fs::File::open(path)?;
    let metadata = file.metadata()?;
    if !metadata.is_file() || metadata.len() > limit {
        return Err(io::Error::other("input file exceeds its read bound"));
    }
    let length = usize::try_from(metadata.len()).map_err(io::Error::other)?;
    let mut bytes = vec![0; length];
    file.read_exact(&mut bytes)?;
    if file.read(&mut [0u8])? != 0 {
        return Err(io::Error::other(
            "input changed after its bounded metadata read",
        ));
    }
    Ok(bytes)
}

/// Retain source archives and the exact dependency archive closure, independently of caches.
pub async fn capsule_inputs(
    service: &Service,
    opened: &common::Opened,
    prepared: &capsule::Capsule,
) -> io::Result<Vec<Artifact>> {
    let owned = service.clone();
    let source = opened
        .reader
        .source_artifact()
        .await
        .map_err(io::Error::other)?
        .ok_or_else(|| io::Error::other("selected snapshot lacks its source acquisition"))?;
    let selected = enrichment_store::capsule_input_plan::select(
        &service.repository.runtime,
        &prepared.prepared,
    )
    .await
    .map_err(io::Error::other)?;
    let owner = prepared.input_reader()?;
    let root = prepared.root.clone();
    service
        .repository
        .runtime
        .blocking(move || {
            let _reader = owner;
            capture_inputs(&owned, source, &root, &selected)
        })
        .await
        .map_err(io::Error::other)?
}

fn capture_inputs(
    service: &Service,
    source: Artifact,
    capsule_root: &Path,
    captures: &[enrichment_store::capsule_input_plan::Capture],
) -> io::Result<Vec<Artifact>> {
    let mut artifacts = vec![source];
    let pool = service
        .repository
        .runtime
        .session()
        .runtime_env()
        .memory_pool
        .clone();
    for capture in captures {
        let reservation =
            datafusion::execution::memory_pool::MemoryConsumer::new("consumer-input-capture")
                .register(&pool);
        reservation
            .try_grow(usize::try_from(capture.bytes).map_err(io::Error::other)?)
            .map_err(io::Error::other)?;
        let bytes = read_input(capsule_root, Path::new(&capture.path), capture.bytes)?;
        if bytes.len() as u64 != capture.bytes || canonical::sha256_hex(&bytes) != capture.sha256 {
            return Err(io::Error::other(
                "consumer input differs from the admitted prepared inventory",
            ));
        }
        artifacts.push(store(
            service,
            &bytes,
            &capture.source_uri,
            &capture.media_type,
        )?);
    }
    Ok(artifacts)
}

async fn publish(
    service: &Service,
    job: &enrichment_core::identity::JobId,
    request: &InspectRequest,
    opened: &common::Opened,
    symbol: &SymbolHeader,
    produced: Produced,
) -> io::Result<(JobState, Envelope)> {
    if crate::execution::description::containment_identity(&service.config.execution)?
        != produced.containment
    {
        return Err(io::Error::other("containment changed during inspection"));
    }
    let mut artifacts = produced.inputs;
    let lock = store(
        service,
        &produced.lock,
        &format!("consumer://lock/{}", canonical::sha256_hex(&produced.lock)),
        "text/plain",
    )?;
    artifacts.push(lock.clone());
    use enrichment_store::producer_run_plan::{Attempt, ReceiptInput, Role};
    let mut inputs: Vec<_> = artifacts
        .iter()
        .cloned()
        .map(|artifact| ReceiptInput {
            role: Role::Input,
            artifact,
        })
        .collect();
    inputs.push(ReceiptInput {
        role: Role::Named {
            name: "dependency-lock".into(),
        },
        artifact: lock,
    });
    let mut results = Vec::new();
    let mut ids = Vec::new();
    let summary = enrichment_store::inspection_execution_plan::summarize(
        &service.repository.runtime,
        request,
        produced
            .facts
            .iter()
            .map(|(_, payload, _)| payload.clone())
            .collect(),
    )
    .await
    .map_err(io::Error::other)?;
    for (subject, payload, class) in produced.facts {
        let bytes = payload.canonical_bytes().map_err(io::Error::other)?;
        let artifact = store(
            service,
            &bytes,
            "producer-result://inspection/3",
            "application/vnd.library-enrichment.canonical-arrow",
        )?;
        inputs.push(ReceiptInput {
            role: Role::Result,
            artifact: artifact.clone(),
        });
        ids.push(artifact.artifact_id.clone());
        artifacts.push(artifact.clone());
        results.push(enrichment_store::execution_fact_plan::Observed {
            subject,
            payload,
            evidence_class: class,
            artifact,
        });
    }
    let receipt = store(
        service,
        &serde_json::to_vec(&serde_json::json!({
            "job_id":job,"request":request,"image":produced.image,"containment":produced.containment,
            "results":ids,"transcript":produced.transcript,
        }))?,
        &format!("service://jobs/{job}/inspection-attempt"),
        "application/json",
    )?;
    artifacts.push(receipt.clone());
    let inputs =
        enrichment_store::producer_run_plan::receipt_inputs(&service.repository.runtime, &inputs)
            .await
            .map_err(io::Error::other)?;
    let run = enrichment_store::producer_run_plan::compose(
        &service.repository.runtime,
        Attempt {
            attempt_id: enrichment_core::identity::AttemptId::new(),
            producer: produced.producer,
            producer_version: produced.version,
            config_digest: enrichment_core::native_key::Key::InspectionConfiguration
                .hex_digest(
                    &enrichment_core::operation::identities::InspectionConfiguration {
                        release_id: opened.release.release_id.clone(),
                        environment: produced.environment.clone(),
                        image: produced.image.clone(),
                        containment: produced.containment.clone(),
                    },
                )
                .map_err(io::Error::other)?,
            profile: produced.profile,
            started_at: produced.started_at,
            finished_at: produced.finished_at,
            outcome: summary.run_outcome,
            gaps: summary.gaps,
            log: Some(receipt.artifact_id),
        },
        &inputs,
    )
    .await
    .map_err(io::Error::other)?;
    let facts = enrichment_store::execution_fact_plan::lower(
        &service.repository.runtime,
        &enrichment_store::execution_fact_plan::Producer {
            environment_id: produced.environment.environment_id.clone(),
            image_id: produced.image.clone(),
            containment_identity: produced.containment.clone(),
            run: run.clone(),
        },
        &results,
    )
    .await
    .map_err(io::Error::other)?;
    let publication_facts = facts.clone();
    let publication_artifacts = artifacts.clone();
    let context = if produced.environment.environment_id == opened.environment.environment_id {
        opened.context.clone()
    } else {
        let context = opened
            .context
            .derived_with(produced.environment.environment_id.clone());
        service
            .repository
            .derive_environment(
                opened.reader.pinned(),
                context.clone(),
                produced.environment.clone(),
            )
            .await
            .map_err(io::Error::other)?;
        context
    };
    let parent = opened.reader.manifest();
    // Rendering and journal-size admission precede the catalog commit. No fallible result
    // read after commit can relabel a published success as a failed producer attempt.
    let result = render(
        &service.repository.runtime,
        request,
        symbol.clone(),
        facts,
        run.clone(),
        artifacts,
    )
    .await?;
    crate::delivery::size(&result, crate::delivery::MAX_RESULT_BYTES)?;
    let result = enrichment_core::operation::results::ResultRecord::from_envelope(&result)
        .map_err(io::Error::other)?;
    let manifest = service
        .repository
        .publish_execution(
            SnapshotMetadata {
                context: context.clone(),
                release: opened.release.clone(),
                environment: produced.environment,
                symbol_package: parent.symbol_package.clone(),
                crate_name: parent.crate_name.clone(),
                crate_version: parent.crate_version.clone(),
                normalizer_version: parent.normalizer_version.clone(),
                observed_configuration: parent.observed_configuration.clone(),
                producer_items: parent.producer_items,
            },
            publication_facts,
            run.clone(),
            publication_artifacts,
            enrichment_store::repository::JobCompletion {
                publication_fence: service.jobs.publication_fence(job)?,
                job_id: *job,
                kind: PublishedJobKind::Inspect,

                attempt_id: run.attempt_id,
                result_artifact_ids: ids.clone(),
                result,
            },
        )
        .await
        .map_err(io::Error::other)?;
    let result = manifest
        .result
        .ok_or_else(|| io::Error::other("published execution lacks native result"))?;
    Ok((
        manifest
            .state
            .ok_or_else(|| io::Error::other("published execution lacks native state"))?,
        result,
    ))
}

async fn render(
    runtime: &enrichment_store::runtime::QueryRuntime,
    request: &InspectRequest,
    symbol: SymbolHeader,
    facts: Vec<ExecutionObservation>,
    run: ProducerRun,
    artifacts: Vec<Artifact>,
) -> io::Result<Envelope> {
    let selected = enrichment_store::inspection_execution_plan::render(
        runtime, request, symbol, facts, run, artifacts,
    )
    .await
    .map_err(io::Error::other)?;
    let result = crate::envelope::Research {
        summary: selected.summary,
        data: common::payload(&selected.data),
        context_id: None,
        snapshot_id: None,
        coverage: selected.coverage,
        freshness: crate::envelope::unverified_freshness(),
        evidence: Vec::new(),
        artifacts: selected.artifacts,
    };
    Ok(if selected.state == JobState::Succeeded {
        result.ok()
    } else {
        result.partial()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use enrichment_core::producer::RunOutcome;
    use enrichment_core::{
        config::Config,
        evidence::{
            SymbolKind,
            path::PublicPath,
            relational::{Definition, PublicBinding},
        },
        identity::{Context, Release, ReleaseKey, ResearchMode},
    };
    use enrichment_core::{
        evidence::relational::{FactSource, Locator},
        wire::SourceVersionMatch,
    };

    #[tokio::test]
    async fn retained_reads_and_committed_inspection_recovery_require_no_execution_policy() {
        retained_fixture(false, false).await;
    }

    #[tokio::test]
    async fn exact_derived_inspection_reuses_retained_observations() {
        retained_fixture(true, false).await;
    }

    #[tokio::test]
    async fn committed_large_inspection_survives_restart_without_reexecution() {
        retained_fixture(false, true).await;
    }

    async fn retained_fixture(derive: bool, large_delivery: bool) {
        let directory = tempfile::tempdir().unwrap();
        let paths = enrichment_store::StatePaths::explicit(
            directory.path().join("cache"),
            directory.path().join("data"),
        );
        let mut config = Config::default();
        config.policy.enabled_profiles = vec!["static".into()];
        let helper = directory.path().join("identity-helper");
        std::fs::write(
            &helper,
            b"nonexecuted identity input for native retained-read unit test",
        )
        .unwrap();
        config.execution.executor_path = Some(helper.clone());
        config.execution.broker_path = Some(helper);
        config.execution.python_image = Some(format!("sha256:{}", "a".repeat(64)));
        let containment =
            crate::execution::description::containment_identity(&config.execution).unwrap();
        let service = Service::open(config.clone(), paths.clone()).unwrap();
        let release = Release::new(ReleaseKey {
            ecosystem: Ecosystem::Python,
            registry: "pypi.org".into(),
            package: "fixture".into(),
            version: "1.0".into(),
            artifact_digest: None,
        });
        let environment = Environment::resolved(
            format!(
                "python-3.14.7;{}",
                config.execution.python_image.as_ref().unwrap()
            ),
            "linux-x86_64".into(),
            vec![],
            None,
            canonical::sha256_hex(b"lock"),
        );
        let parent_environment =
            Environment::python(Some("3.14".into()), Some("linux".into()), Some(vec![]));
        let parent_context = Context::new(
            release.release_id.clone(),
            parent_environment.environment_id.clone(),
            ResearchMode::Project,
        );
        let context = if derive {
            parent_context.derived_with(environment.environment_id.clone())
        } else {
            Context::new(
                release.release_id.clone(),
                environment.environment_id.clone(),
                ResearchMode::Project,
            )
        };
        let metadata = SnapshotMetadata {
            context: context.clone(),
            release,
            environment: environment.clone(),
            symbol_package: "python:fixture".into(),
            crate_name: "fixture".into(),
            crate_version: Some("1.0".into()),
            normalizer_version: "test/1".into(),
            observed_configuration: None,
            producer_items: 0,
        };
        let definition = Definition {
            definition_id: SymbolHeader::definition_id_for(
                "python:fixture",
                "fixture.f",
                SymbolKind::Function,
                None,
            ),
            definition_path: "fixture.f".into(),
            kind: SymbolKind::Function,
            defined_in_package: "python:fixture".into(),
            qualifier: None,
        };
        let path = PublicPath::parse(Ecosystem::Python, "fixture.f").unwrap();
        let binding = PublicBinding {
            symbol_id: PublicBinding::id_for("python:fixture", &path, SymbolKind::Function, None),
            definition_id: definition.definition_id.clone(),
            path,
            name: "f".into(),
            is_reexport: false,
            qualifier: None,
        };
        let mut base_metadata = metadata.clone();
        if derive {
            base_metadata.context = parent_context;
            base_metadata.environment = parent_environment;
        }
        let session = service.repository.runtime.session();
        let mut plans = enrichment_store::admission::Relation::ALL
            .into_iter()
            .map(|relation| {
                let batch = match relation {
                    enrichment_store::admission::Relation::Definitions => {
                        enrichment_store::projection::definitions(std::slice::from_ref(&definition))
                            .unwrap()
                    }
                    enrichment_store::admission::Relation::Symbols => {
                        enrichment_store::projection::bindings(std::slice::from_ref(&binding))
                            .unwrap()
                    }
                    _ => arrow::record_batch::RecordBatch::new_empty(relation.schema().unwrap()),
                };
                (relation, session.read_batch(batch).unwrap())
            })
            .collect();
        let base = service
            .repository
            .publish_native(
                base_metadata.clone(),
                std::mem::take(&mut plans),
                vec![],
                None,
                None,
            )
            .await
            .unwrap();
        if derive {
            let catalog = service.repository.catalog.pin().await.unwrap();
            let parent = service
                .repository
                .open_snapshot(catalog, &base.snapshot_id)
                .await
                .unwrap();
            service
                .repository
                .derive_environment(&parent, context.clone(), environment.clone())
                .await
                .unwrap();
        }
        let options = InspectionOptions {
            intent: InspectionIntent::ExecuteOnMiss,
            profile: Some(ExecutionProfile::Build),
            methods: vec![SemanticMethod::Hover],
            ..Default::default()
        };
        let request = InspectRequest {
            selection: enrichment_core::wire::ResearchSelection::Explicit {
                aspects: vec![enrichment_core::wire::research::AspectSelection {
                    aspect: enrichment_core::wire::InspectionAspect::Semantics,
                    cursor: None,
                    max_items: 32,
                    max_characters: None,
                }],
            },
            context_id: base_metadata.context.context_id.clone(),
            snapshot_id: Some(base.snapshot_id.clone()),
            symbol_path: "fixture.f".into(),
            execution: Some(options),
            definition_id: None,
            max_bytes: None,
        };
        let (record, _, _) = service
            .jobs
            .submit(jobs::Arguments::Inspect {
                request: request.clone(),
            })
            .await
            .unwrap();
        service.jobs.start(&record.job_id).await.unwrap();
        let document = store(
            &service,
            b"import fixture\nfixture.f\n",
            "consumer://fixture",
            "text/plain",
        )
        .unwrap();
        let lock = store(&service, b"lock", "consumer://lock", "text/plain").unwrap();
        let payload = ExecutionPayload::SemanticQuery(SemanticQuery {
            method: SemanticMethod::Hover,
            document_artifact_id: document.artifact_id.clone(),
            position: Some(Utf8Position { line: 1, byte: 8 }),
            anchor_symbol_id: Some(binding.symbol_id),
            server: "test-ty".into(),
            outcome: ExecutionOutcome::Results,
            hover: Some("def f(value: int) -> int".into()),
            locations: Vec::new(),
            diagnostics: Vec::new(),
            limitations: Vec::new(),
        });
        let result = store(
            &service,
            &payload.canonical_bytes().unwrap(),
            "producer-result://fixture",
            "application/json",
        )
        .unwrap();
        let receipt = store(
            &service,
            &serde_json::to_vec(&serde_json::json!({"job_id":record.job_id,"request":request}))
                .unwrap(),
            "attempt://fixture",
            "application/json",
        )
        .unwrap();
        let run = ProducerRun {
            attempt_id: enrichment_core::identity::AttemptId::new(),
            producer: producer_identity(false).unwrap().0.into(),
            producer_version: producer_identity(false).unwrap().1,
            config_digest: "fixture".into(),
            inputs: [
                ("document".into(), document.sha256.clone()),
                ("lock".into(), lock.sha256.clone()),
                ("result".into(), result.sha256.clone()),
            ]
            .into(),
            profile: ExecutionProfile::Build,
            started_at: enrichment_core::native_time::ObservationTime::try_from(
                "2026-09-14T00:00:00.000000Z".to_owned(),
            )
            .unwrap(),
            finished_at: enrichment_core::native_time::ObservationTime::try_from(
                "2026-09-14T00:00:01.000000Z".to_owned(),
            )
            .unwrap(),
            outcome: RunOutcome::Succeeded,
            gaps: Vec::new(),
            log: Some(receipt.artifact_id.clone()),
        };
        let fact = ExecutionObservation::new(
            SubjectRef::Document {
                artifact_id: document.artifact_id.clone(),
                heading: "consumer semantic query".into(),
            },
            environment.environment_id.clone(),
            format!("sha256:{}", "a".repeat(64)),
            containment,
            payload,
            FactSource {
                producer_binding_id: run.semantic_binding_id(),
                extractor: run.producer.clone(),
                extractor_version: run.producer_version.clone(),
                artifact_id: result.artifact_id.clone(),
                source_uri: Some(result.source_uri.clone()),
                source_version_match: SourceVersionMatch::Exact,
                locator: Locator::Artifact,
                evidence_class: EvidenceClass::TypecheckerObserved,
            },
        )
        .unwrap();
        let fixture_reader = SnapshotReader::open(
            &service.repository,
            service.repository.catalog.pin().await.unwrap(),
            &base.snapshot_id,
        )
        .await
        .unwrap();
        let fixture_symbol = fixture_reader
            .symbols_at(&request.symbol_path, request.definition_id.as_deref())
            .await
            .unwrap()
            .pop()
            .unwrap();
        let mut fixture_reply = render(
            &service.repository.runtime,
            &request,
            fixture_symbol,
            vec![fact.clone()],
            run.clone(),
            vec![result.clone(), receipt.clone()],
        )
        .await
        .unwrap();
        if large_delivery {
            fixture_reply
                .coverage
                .limitations
                .push("🌎\"\n".repeat(180_000));
        }
        let native_result =
            enrichment_core::operation::results::ResultRecord::from_envelope(&fixture_reply)
                .unwrap();
        let publication_artifacts = vec![document, lock, result.clone(), receipt];
        let manifest = service
            .repository
            .publish_execution(
                metadata,
                vec![fact.clone()],
                run.clone(),
                publication_artifacts,
                enrichment_store::repository::JobCompletion {
                    publication_fence: service.jobs.publication_fence(&record.job_id).unwrap(),
                    job_id: record.job_id,
                    kind: PublishedJobKind::Inspect,

                    attempt_id: run.attempt_id,
                    result_artifact_ids: vec![result.artifact_id],
                    result: native_result,
                },
            )
            .await
            .unwrap();
        // Publication and terminal state share the control commit even before worker acknowledgement.
        assert_eq!(
            service.jobs.get(&record.job_id).await.unwrap().state,
            JobState::Succeeded
        );
        let expected = verify::recover(&service.repository, &service.blobs, &record)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(expected.0, JobState::Succeeded);
        drop(service);
        let service = Service::open(config, paths).unwrap();
        let terminal = service.jobs.get(&record.job_id).await.unwrap();
        assert_eq!(terminal.state, JobState::Succeeded);
        assert_eq!(terminal.result.unwrap().data, expected.1.data);
        let mut read = request;
        if !derive {
            read.snapshot_id = Some(manifest.snapshot_id.clone());
        }
        let answer = inspect::inspect(&service, read.clone()).await;
        assert_eq!(
            answer.context_id.as_ref(),
            Some(&context.context_id),
            "{answer:?}"
        );
        let enrichment_core::wire::data::ToolData::InspectSymbol(data) = answer.data else {
            panic!("native inspection payload")
        };
        assert_eq!(data.execution_observations, vec![fact.clone()]);
        assert_eq!(
            service.jobs.counts().await.unwrap(),
            (0, 0),
            "execute_on_miss reuses a sound result even though execution is disabled"
        );
        read.execution.as_mut().unwrap().intent = InspectionIntent::Retained;
        let answer = inspect::inspect(&service, read).await;
        let enrichment_core::wire::data::ToolData::InspectSymbol(data) = answer.data else {
            panic!("native inspection payload")
        };
        assert_eq!(
            data.execution_observations,
            if derive { vec![] } else { vec![fact] },
            "a retained parent read never discovers child evidence implicitly"
        );
        assert_eq!(service.lsp.metrics().started, 0);
    }
}
