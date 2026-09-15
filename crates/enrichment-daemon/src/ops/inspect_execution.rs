//! Durable inspection producers and retained native result delivery (ADR-0026).
use super::{common, inspect, verify};
use crate::{
    execution::{Runner, capsule},
    jobs,
    service::Service,
};
use enrichment_core::{
    canonical, clock,
    evidence::{
        Artifact, ArtifactKind, Symbol,
        catalog::{JobPublication, PublishedJobKind},
        execution::*,
        ingest,
        relational::{FactSource, Locator, SubjectRef},
        snapshot::SnapshotMetadata,
    },
    identity::{Ecosystem, Environment},
    policy::ExecutionProfile,
    producer::{ProducerRun, RunOutcome},
    request::{InspectRequest, InspectionIntent, InspectionOptions},
    wire::{
        Coverage, Envelope, ErrorCode, EvidenceClass, JobState, SourceVersionMatch,
        data::InspectData,
    },
};
use enrichment_store::{SnapshotReader, query::ExecutionSelection};
use std::{collections::BTreeMap, io, io::Read, path::Path, sync::atomic::Ordering};

pub struct Produced {
    pub environment: Environment,
    pub image: String,
    pub containment: String,
    pub producer: String,
    pub version: String,
    pub profile: ExecutionProfile,
    pub started_at: String,
    pub finished_at: String,
    pub facts: Vec<(SubjectRef, ExecutionPayload, EvidenceClass)>,
    pub inputs: Vec<Artifact>,
    pub lock: Vec<u8>,
    pub transcript: serde_json::Value,
}

pub fn methods(options: &InspectionOptions) -> Vec<SemanticMethod> {
    if options.methods.is_empty() {
        vec![
            SemanticMethod::Hover,
            SemanticMethod::Definition,
            SemanticMethod::References,
            SemanticMethod::Diagnostics,
        ]
    } else {
        options.methods.clone()
    }
}

/// Exact source/query selection is lowered before result hydration and alternatives bounds.
pub async fn retained(
    reader: &SnapshotReader,
    symbol: &Symbol,
    options: Option<&InspectionOptions>,
    runtime: bool,
) -> Result<Vec<ExecutionObservation>, enrichment_store::QueryError> {
    retained_matching(reader, symbol, options, runtime, None).await
}

/// Producer implementation identity is separate from the immutable container identity.
pub(super) fn producer_identity(runtime: bool) -> (&'static str, String) {
    let (name, source) = if runtime {
        (
            "runtime-object",
            concat!(
                include_str!("runtime_object.rs"),
                include_str!("../execution/runtime_object.py")
            ),
        )
    } else {
        (
            "semantic-inspection",
            concat!(
                include_str!("semantics.rs"),
                include_str!("../lsp/document.rs"),
                include_str!("../lsp/client.rs"),
                include_str!("../lsp/diagnostics.rs"),
                include_str!("../lsp/notifications.rs"),
                include_str!("../lsp/framing.rs"),
                include_str!("../lsp/settings.rs"),
                include_str!("../lsp/mod.rs")
            ),
        )
    };
    (
        name,
        format!(
            "3+{}",
            canonical::digest_hex(&serde_json::json!([
                source,
                include_str!("inspect_execution.rs"),
                include_str!("source_documents.rs"),
                include_str!("source_tree.rs"),
                include_str!("../execution/capsule.rs"),
                include_str!("../execution/python_closure.rs"),
                include_str!("../execution/inventory.rs"),
                include_str!("../../../enrichment-core/src/evidence/execution.rs"),
                include_str!("../../../enrichment-core/src/evidence/text.rs"),
                include_str!("../../../enrichment-core/src/canonical.rs"),
                include_str!("../../../../Cargo.lock"),
            ]))
        ),
    )
}

async fn retained_matching(
    reader: &SnapshotReader,
    symbol: &Symbol,
    options: Option<&InspectionOptions>,
    runtime: bool,
    qualification: Option<(&str, &str, &str, &str)>,
) -> Result<Vec<ExecutionObservation>, enrichment_store::QueryError> {
    let default = InspectionOptions::default();
    let options = options.unwrap_or(&default);
    let is_runtime = runtime || options.runtime.is_some();
    let consumer = if !is_runtime {
        Some(
            crate::lsp::document::Consumer::new(
                symbol,
                reader.manifest().metadata.ecosystem,
                options,
            )
            .map_err(io::Error::other)?,
        )
    } else {
        None
    };
    let document_id = consumer.as_ref().map(|c| {
        enrichment_core::evidence::artifact_id_for(&canonical::sha256_hex(c.text.as_bytes()))
    });
    reader
        .execution_selection(ExecutionSelection {
            symbol_id: Some(&symbol.symbol_id),
            document_id: document_id.as_deref(),
            kind: Some(if is_runtime {
                "runtime_object"
            } else {
                "semantic_query"
            }),
            methods: &options.methods,
            position: consumer.as_ref().and_then(|c| c.position),
            runtime: options.runtime.as_ref(),
            image: qualification.map(|q| q.0),
            containment: qualification.map(|q| q.1),
            producer: qualification.map(|q| (q.2, q.3)),
            ..Default::default()
        })
        .await
}

fn sufficient(facts: &[ExecutionObservation], options: &InspectionOptions) -> bool {
    if options.runtime.is_some() {
        facts.iter().any(|o| matches!(&o.payload, ExecutionPayload::RuntimeObject(r) if matches!(r.outcome, ExecutionOutcome::Results | ExecutionOutcome::Empty | ExecutionOutcome::Unsupported)))
    } else {
        methods(options).iter().all(|method| facts.iter().any(|o| matches!(&o.payload, ExecutionPayload::SemanticQuery(q) if q.method == *method && matches!(q.outcome, ExecutionOutcome::Results | ExecutionOutcome::Empty | ExecutionOutcome::Unsupported))))
    }
}

/// Only explicit ExecuteOnMiss can discover a directly derived context. Retained reads
/// continue to mean exactly the supplied context, even if another environment is richer.
async fn retained_child(
    service: &Service,
    request: &InspectRequest,
    opened: &common::Opened,
    symbol: &Symbol,
    options: &InspectionOptions,
) -> Result<Option<Envelope>, Box<Envelope>> {
    let catalog = std::sync::Arc::clone(&opened.reader.pinned().catalog);
    let children = catalog
        .children(&service.repository.runtime, &opened.context)
        .await
        .map_err(|e| Box::new(common::store_error(&e)))?;
    if children.is_empty() {
        return Ok(None);
    }
    let Some(image) = (match opened.release.key.ecosystem {
        Ecosystem::Rust => service.config.execution.rust_image.as_deref(),
        Ecosystem::Python => service.config.execution.python_image.as_deref(),
    }) else {
        return Ok(None);
    };
    let containment =
        match crate::execution::description::containment_identity(&service.config.execution) {
            Ok(id) => id,
            Err(_) => return Ok(None),
        };
    let (producer, version) = producer_identity(options.runtime.is_some());
    let inputs = opened
        .reader
        .static_inputs()
        .await
        .map_err(|e| Box::new(common::query_error(&e)))?;
    let mut candidates = Vec::new();
    for child in children {
        let child = common::open_context_at(
            service,
            std::sync::Arc::clone(&catalog),
            child.context_id.as_str(),
            None,
        )
        .await?;
        let declared = &opened.environment;
        let matches_environment = if opened.release.key.ecosystem == Ecosystem::Python {
            capsule::python_toolchain_accepted(declared.toolchain.as_deref(), image)
                && capsule::python_target_accepted(declared.target.as_deref())
                && child.environment.toolchain.as_deref()
                    == Some(format!("python-3.14.7;{image}").as_str())
                && child.environment.target.as_deref() == Some("linux-x86_64")
        } else {
            declared
                .toolchain
                .as_ref()
                .is_none_or(|v| child.environment.toolchain.as_ref() == Some(v))
                && declared
                    .target
                    .as_ref()
                    .is_none_or(|v| child.environment.target.as_ref() == Some(v))
        };
        if child.reader.manifest().normalizer_version != opened.reader.manifest().normalizer_version
            || !matches_environment
            || (declared.features_known && declared.features != child.environment.features)
            || declared
                .default_features
                .is_some_and(|v| child.environment.default_features != Some(v))
            || child
                .reader
                .static_inputs()
                .await
                .map_err(|e| Box::new(common::query_error(&e)))?
                != inputs
        {
            continue;
        }
        let facts = retained_matching(
            &child.reader,
            symbol,
            Some(options),
            options.runtime.is_some(),
            Some((image, &containment, producer, &version)),
        )
        .await
        .map_err(|e| Box::new(common::query_error(&e)))?;
        if sufficient(&facts, options) {
            candidates.push((
                child.context.context_id.to_string(),
                child.snapshot_id.to_string(),
            ));
        }
    }
    if candidates.len() > 1 {
        let mut result = crate::envelope::error(
            ErrorCode::UnsupportedFormat,
            "More than one exactly qualified derived context has retained observations.",
            "Select a context from data.candidates and repeat inspection with that explicit context.",
            false,
        );
        result.data = common::to_object(
            &serde_json::json!({"candidates": candidates.into_iter().map(|(context_id,snapshot_id)| serde_json::json!({"context_id":context_id,"snapshot_id":snapshot_id})).collect::<Vec<_>>()}),
        );
        return Ok(Some(result));
    }
    let Some((context_id, snapshot_id)) = candidates.pop() else {
        return Ok(None);
    };
    let mut selected = request.clone();
    selected.context_id = context_id;
    selected.snapshot_id = Some(snapshot_id);
    if let Some(execution) = &mut selected.execution {
        execution.intent = InspectionIntent::Retained;
    }
    Ok(Some(inspect::read(service, selected).await))
}

pub async fn submit(service: &Service, mut request: InspectRequest) -> Envelope {
    let options = request.execution.clone().unwrap_or_default();
    let initial = inspect::read(service, request.clone()).await;
    let Ok(data) =
        serde_json::from_value::<InspectData>(serde_json::Value::Object(initial.data.clone()))
    else {
        return initial;
    };
    let Some(symbol) = data.symbol else {
        return initial;
    };
    if options.intent == InspectionIntent::ExecuteOnMiss
        && sufficient(&data.execution_observations, &options)
    {
        return initial;
    }
    let opened =
        match common::open_context(service, &request.context_id, initial.snapshot_id.as_deref())
            .await
        {
            Ok(opened) => opened,
            Err(e) => return *e,
        };
    if options.runtime.is_some() && opened.release.key.ecosystem != Ecosystem::Python {
        return denied("Runtime object inspection is a Python producer.");
    }
    if let Some(selection) = &options.runtime {
        let selected_path = std::iter::once(selection.module.as_str())
            .chain(selection.attributes.iter().map(String::as_str))
            .collect::<Vec<_>>()
            .join(".");
        if selected_path != symbol.path {
            return denied(
                "The runtime import and attribute selection must name the selected public binding exactly.",
            );
        }
    }
    let profile = if options.runtime.is_some() {
        ExecutionProfile::Runtime
    } else {
        ExecutionProfile::Build
    };
    if options.intent == InspectionIntent::ExecuteOnMiss {
        match retained_child(service, &request, &opened, &symbol, &options).await {
            Ok(Some(result)) => return result,
            Ok(None) => {}
            Err(result) => return *result,
        }
    }
    if options.profile != Some(profile) || !profile.is_enabled(&service.config) {
        return denied("The required explicit inspection profile is not enabled by the operator.");
    }
    let qualification = crate::execution::admission::qualification(
        &service.config.execution,
        &service.paths.cache_root,
    );
    if !qualification.is_qualified() {
        return denied(&qualification.detail());
    }
    if let crate::execution::cleanup::Admission::Quarantined { detail, .. } =
        service.execution.admission()
    {
        return denied(&detail);
    }
    let image = match opened.release.key.ecosystem {
        Ecosystem::Rust => service.config.execution.rust_image.as_ref(),
        Ecosystem::Python => service.config.execution.python_image.as_ref(),
    };
    let Some(image) = image.filter(|s| Runner::valid_image(s)).cloned() else {
        return denied("An admitted immutable producer image is required.");
    };
    request.snapshot_id = Some(opened.snapshot_id.to_string());
    request.symbol_path = symbol.path.clone();
    request.definition_id = Some(symbol.definition_id.clone());
    let mut normalized = request.clone();
    normalized.max_bytes = None;
    if let Some(options) = &mut normalized.execution {
        options.intent = InspectionIntent::Retained;
    }
    let containment =
        match crate::execution::description::containment_identity(&service.config.execution) {
            Ok(v) => v,
            Err(e) => return common::store_error(&e),
        };
    let key = canonical::digest_hex(&serde_json::json!([
        "inspection/2",
        normalized,
        image,
        containment
    ]));
    let (record, token, new) = match service
        .jobs
        .submit(key, jobs::JobSpec::Inspect(request.clone()))
    {
        Ok(v) => v,
        Err(e) => return common::store_error(&e),
    };
    if new {
        let service = service.clone();
        let id = record.job_id.clone();
        tokio::spawn(async move {
            let result = run(&service, &id, &opened, &symbol, &request).await;
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
            if let Err(e) = service.jobs.finish(&id, state, result) {
                eprintln!("inspection job {id}: terminal journal failed: {e}");
            }
        });
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
        Err(e) => common::store_error(&e),
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
    job: &str,
    opened: &common::Opened,
    symbol: &Symbol,
    request: &InspectRequest,
) -> io::Result<(JobState, Envelope)> {
    let cancel = service.jobs.cancellation(job)?;
    if !service.jobs.start(job)? {
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
    service
        .blobs
        .put(bytes, |_| {
            enrichment_store::blob::describe_local(
                bytes,
                ArtifactKind::Other,
                media,
                uri,
                &clock::now_rfc3339(),
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
    let file = std::fs::File::open(path)?;
    let metadata = file.metadata()?;
    if !metadata.is_file() || metadata.len() > limit {
        return Err(io::Error::other("input file exceeds its read bound"));
    }
    let mut bytes = Vec::new();
    file.take(limit + 1).read_to_end(&mut bytes)?;
    if bytes.len() as u64 > limit {
        return Err(io::Error::other("input grew beyond its read bound"));
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
    let digest = opened
        .release
        .key
        .artifact_digest
        .clone()
        .ok_or_else(|| io::Error::other("source digest missing"))?;
    let ecosystem = opened.release.key.ecosystem;
    let root = prepared.root.clone();
    tokio::task::spawn_blocking(move || capture_inputs(&owned, &digest, ecosystem, &root))
        .await
        .map_err(io::Error::other)?
}

fn dependency_paths(root: &Path) -> io::Result<Vec<std::path::PathBuf>> {
    let mut directories = vec![std::path::PathBuf::new()];
    let mut files = Vec::new();
    let mut entries = 0usize;
    while let Some(directory) = directories.pop() {
        if directory.components().count() > 32 {
            return Err(io::Error::other("dependency path depth exceeds 32"));
        }
        for entry in std::fs::read_dir(root.join(&directory))? {
            entries += 1;
            if entries > 8192 {
                return Err(io::Error::other(
                    "dependency inventory exceeds 8192 entries",
                ));
            }
            let entry = entry?;
            let path = directory.join(entry.file_name());
            let kind = entry.file_type()?;
            if kind.is_dir() {
                directories.push(path);
            } else if kind.is_file() {
                if matches!(
                    path.extension().and_then(|s| s.to_str()),
                    Some("whl" | "crate")
                ) {
                    files.push(path);
                    if files.len() > 4096 {
                        return Err(io::Error::other("dependency archive count exceeds 4096"));
                    }
                }
            } else {
                return Err(io::Error::other(
                    "dependency inventory contains a link or non-file",
                ));
            }
        }
    }
    files.sort();
    Ok(files)
}

fn capture_inputs(
    service: &Service,
    digest: &str,
    ecosystem: Ecosystem,
    capsule_root: &Path,
) -> io::Result<Vec<Artifact>> {
    let source = service
        .blobs
        .artifact(digest)?
        .ok_or_else(|| io::Error::other("source acquisition missing"))?;
    let mut artifacts = BTreeMap::from([(source.sha256.clone(), source)]);
    let directory = match ecosystem {
        Ecosystem::Python => "wheelhouse",
        Ecosystem::Rust => "cargo-home/registry/cache",
    };
    let root = capsule_root.join(directory);
    let mut total = 0u64;
    if root.is_dir() {
        for path in dependency_paths(&root)? {
            let bytes = read_input(&root, &path, 256 * 1024 * 1024)?;
            total = total
                .checked_add(bytes.len() as u64)
                .filter(|n| *n <= 512 * 1024 * 1024)
                .ok_or_else(|| io::Error::other("dependency closure exceeds byte budget"))?;
            let digest = canonical::sha256_hex(&bytes);
            if let std::collections::btree_map::Entry::Vacant(entry) =
                artifacts.entry(digest.clone())
            {
                let artifact = store(
                    service,
                    &bytes,
                    &format!("consumer-dependency://archive/{digest}"),
                    "application/octet-stream",
                )?;
                entry.insert(artifact);
            }
        }
    }
    if capsule_root.join("Cargo.toml").is_file() {
        let bytes = read_input(capsule_root, Path::new("Cargo.toml"), 1024 * 1024)?;
        let a = store(
            service,
            &bytes,
            "consumer://manifest/cargo",
            "application/toml",
        )?;
        artifacts.insert(a.sha256.clone(), a);
    }
    Ok(artifacts.into_values().collect())
}

async fn publish(
    service: &Service,
    job: &str,
    request: &InspectRequest,
    opened: &common::Opened,
    symbol: &Symbol,
    produced: Produced,
) -> io::Result<(JobState, Envelope)> {
    if crate::execution::description::containment_identity(&service.config.execution)?
        != produced.containment
    {
        return Err(io::Error::other("containment changed during inspection"));
    }
    let mut artifacts: BTreeMap<_, _> = produced
        .inputs
        .into_iter()
        .map(|a| (a.sha256.clone(), a))
        .collect();
    let lock = store(
        service,
        &produced.lock,
        &format!("consumer://lock/{}", canonical::sha256_hex(&produced.lock)),
        "text/plain",
    )?;
    artifacts.insert(lock.sha256.clone(), lock.clone());
    let mut inputs: BTreeMap<_, _> = artifacts
        .values()
        .map(|a| (format!("input:{}", a.artifact_id), a.sha256.clone()))
        .collect();
    inputs.insert("dependency-lock".into(), lock.sha256.clone());
    let mut results = Vec::new();
    let mut ids = Vec::new();
    let mut complete = true;
    let mut gaps = Vec::new();
    for (subject, payload, class) in produced.facts {
        let bytes = payload.canonical_bytes().map_err(io::Error::other)?;
        let artifact = store(
            service,
            &bytes,
            "producer-result://inspection/2",
            "application/json",
        )?;
        match &payload {
            ExecutionPayload::SemanticQuery(q) => {
                complete &= matches!(
                    q.outcome,
                    ExecutionOutcome::Results | ExecutionOutcome::Empty
                )
            }
            ExecutionPayload::RuntimeObject(q) => {
                complete &= matches!(
                    q.outcome,
                    ExecutionOutcome::Results | ExecutionOutcome::Empty
                )
            }
            ExecutionPayload::UsageProbe(_) => {
                return Err(io::Error::other("wrong producer payload"));
            }
        }
        let (kind, outcome, detail) = match &payload {
            ExecutionPayload::SemanticQuery(q) => (
                enrichment_core::evidence::EvidenceKind::SemanticQueries,
                q.outcome,
                format!("{}: {}", q.method.as_str(), q.limitations.join(" ")),
            ),
            ExecutionPayload::RuntimeObject(q) => (
                enrichment_core::evidence::EvidenceKind::RuntimeApi,
                q.outcome,
                q.limitations.join(" "),
            ),
            ExecutionPayload::UsageProbe(_) => {
                return Err(io::Error::other("wrong inspection payload"));
            }
        };
        if !matches!(outcome, ExecutionOutcome::Results | ExecutionOutcome::Empty) {
            gaps.push(enrichment_core::evidence::Gap {
                kind,
                reason: match outcome {
                    ExecutionOutcome::Failed => {
                        enrichment_core::evidence::GapReason::ExtractionFailed
                    }
                    ExecutionOutcome::Unresolved => {
                        enrichment_core::evidence::GapReason::UpstreamUnavailable
                    }
                    _ => enrichment_core::evidence::GapReason::NotAttempted,
                },
                detail,
                planned_fallback: None,
            });
        }
        inputs.insert(
            format!("result:{}", artifact.artifact_id),
            artifact.sha256.clone(),
        );
        ids.push(artifact.artifact_id.clone());
        artifacts.insert(artifact.sha256.clone(), artifact.clone());
        results.push((subject, payload, class, artifact));
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
    artifacts.insert(receipt.sha256.clone(), receipt.clone());
    let run = ProducerRun {
        attempt_id: job.into(),
        producer: produced.producer,
        producer_version: produced.version,
        config_digest: canonical::digest_hex(&serde_json::json!([
            "inspection/2",
            opened.release.release_id,
            produced.environment,
            produced.image,
            produced.containment
        ])),
        inputs,
        profile: produced.profile,
        started_at: produced.started_at,
        finished_at: produced.finished_at,
        outcome: if complete {
            RunOutcome::Succeeded
        } else {
            RunOutcome::Partial
        },
        gaps,
        log: Some(receipt.artifact_id),
    };
    let facts = results
        .into_iter()
        .map(|(subject, payload, class, artifact)| {
            ExecutionObservation::new(
                subject,
                produced.environment.environment_id.to_string(),
                produced.image.clone(),
                produced.containment.clone(),
                payload,
                FactSource {
                    producer_binding_id: run.semantic_binding_id(),
                    extractor: run.producer.clone(),
                    extractor_version: run.producer_version.clone(),
                    artifact_id: artifact.artifact_id,
                    source_uri: Some(artifact.source_uri),
                    source_version_match: SourceVersionMatch::Exact,
                    locator: Locator::Artifact,
                    evidence_class: class,
                },
            )
        })
        .collect::<Result<Vec<_>, _>>()
        .map_err(io::Error::other)?;
    let evidence = ingest::normalize_execution_set(
        facts.clone(),
        run.clone(),
        artifacts.values().cloned().collect(),
    )
    .map_err(io::Error::other)?;
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
    let state = if complete {
        JobState::Succeeded
    } else {
        JobState::Partial
    };
    // Rendering and journal-size admission precede the catalog commit. No fallible result
    // read after commit can relabel a published success as a failed producer attempt.
    let result = render(
        request,
        symbol.clone(),
        facts,
        run.clone(),
        artifacts
            .into_values()
            .filter(|a| ids.contains(&a.artifact_id) || run.log.as_ref() == Some(&a.artifact_id))
            .collect(),
        state,
    )?;
    crate::delivery::size(&result, crate::delivery::MAX_RESULT_BYTES)?;
    let (prepare_delivery, delivery) =
        crate::delivery::prepare_job(service.blobs.clone(), move |manifest| {
            let mut result = result.clone();
            result.context_id = Some(manifest.context_id.to_string());
            result.snapshot_id = Some(manifest.snapshot_id.to_string());
            Ok(result)
        });
    let manifest = service
        .repository
        .publish_job(
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
            evidence,
            enrichment_store::repository::JobCompletion {
                job_id: job.into(),
                kind: PublishedJobKind::Inspect,
                state,
                attempt_id: run.attempt_id.clone(),
                result_artifact_ids: ids.clone(),
                prepare_delivery,
            },
        )
        .await
        .map_err(io::Error::other)?;
    Ok((state, delivery.get(manifest.snapshot_id.as_str())?))
}

async fn deliver(
    repository: &enrichment_store::repository::EvidenceRepository,
    blobs: &enrichment_store::BlobStore,
    publication: &JobPublication,
    request: &InspectRequest,
) -> io::Result<Envelope> {
    let catalog = repository.catalog.pin().await?;
    let reader = SnapshotReader::open(repository, catalog, &publication.snapshot_id)
        .await
        .map_err(io::Error::other)?;
    let attempt = reader
        .attempt(&publication.attempt_id)
        .await
        .map_err(io::Error::other)?;
    let log = attempt
        .run
        .log
        .clone()
        .ok_or_else(|| io::Error::other("inspection attempt log missing"))?;
    let artifact = attempt
        .artifacts
        .iter()
        .find(|a| a.artifact_id == log)
        .ok_or_else(|| io::Error::other("inspection log outside attempt"))?;
    if artifact.size_bytes > 2 * 1024 * 1024 {
        return Err(io::Error::other("inspection log exceeds replay budget"));
    }
    let receipt: serde_json::Value = blobs.read_json(artifact, 2 * 1024 * 1024)?;
    if receipt["job_id"].as_str() != Some(&publication.job_id)
        || receipt["request"] != serde_json::to_value(request)?
    {
        return Err(io::Error::other(
            "inspection receipt differs from durable job specification",
        ));
    }
    for id in &publication.result_artifact_ids {
        if !attempt.artifacts.iter().any(|a| {
            &a.artifact_id == id
                && attempt
                    .run
                    .inputs
                    .values()
                    .any(|digest| digest == &a.sha256)
        }) {
            return Err(io::Error::other(
                "inspection result outside exact attempt closure",
            ));
        }
    }
    repository
        .validate_delivery(publication)
        .await
        .map_err(io::Error::other)?;
    crate::delivery::recover_job(blobs, publication)
}

fn render(
    request: &InspectRequest,
    mut symbol: Symbol,
    mut facts: Vec<ExecutionObservation>,
    run: ProducerRun,
    mut artifacts: Vec<Artifact>,
    state: JobState,
) -> io::Result<Envelope> {
    facts.sort_by(|a, b| a.observation_id.cmp(&b.observation_id));
    artifacts.sort_by(|a, b| a.artifact_id.cmp(&b.artifact_id));
    symbol.docs = None;
    let mut limitations = Vec::new();
    for fact in &facts {
        match &fact.payload {
            ExecutionPayload::SemanticQuery(q) => limitations.extend(q.limitations.clone()),
            ExecutionPayload::RuntimeObject(q) => limitations.extend(q.limitations.clone()),
            ExecutionPayload::UsageProbe(_) => {
                return Err(io::Error::other("unexpected probe in inspection result"));
            }
        }
    }
    let runtime = request
        .execution
        .as_ref()
        .is_some_and(|o| o.runtime.is_some());
    let data = InspectData {
        symbol: Some(symbol),
        docs_truncated: false,
        also_at: Vec::new(),
        candidates: Vec::new(),
        aspects: vec![if runtime { "runtime" } else { "semantics" }.into()],
        availability: None,
        relationships: Vec::new(),
        observations: Vec::new(),
        fragments: Vec::new(),
        source: None,
        execution_observations: facts,
        producer_runs: vec![run],
    };
    let mut result = crate::envelope::Research {
        summary: format!(
            "Retained {} inspection of `{}` in its resolved environment.",
            if runtime { "runtime" } else { "semantic" },
            request.symbol_path
        ),
        data: common::to_object(&data),
        context_id: None,
        snapshot_id: None,
        coverage: Coverage {
            scope: "selected consumer queries; no complete-library execution claim".into(),
            indexed: [if runtime {
                "runtime_api"
            } else {
                "semantic_queries"
            }
            .into()]
            .into(),
            missing: if state == JobState::Succeeded {
                Default::default()
            } else {
                [if runtime {
                    "runtime_api"
                } else {
                    "semantic_queries"
                }
                .into()]
                .into()
            },
            limitations,
        },
        freshness: crate::envelope::unverified_freshness(),
        evidence: Vec::new(),
        artifacts: Vec::new(),
    };
    result.artifacts = artifacts
        .iter()
        .filter_map(|a| {
            common::handle_for(a, "Retained execution result or actual attempt log".into())
        })
        .collect();
    Ok(if state == JobState::Succeeded {
        result.ok()
    } else {
        result.partial()
    })
}

pub async fn recover(
    repository: &enrichment_store::repository::EvidenceRepository,
    blobs: &enrichment_store::BlobStore,
    record: &jobs::JobRecord,
) -> io::Result<Option<(JobState, Envelope)>> {
    let jobs::JobSpec::Inspect(request) = &record.specification else {
        return Err(io::Error::other("wrong inspection journal variant"));
    };
    let catalog = repository.catalog.pin().await?;
    let Some(publication) = catalog
        .job_publication(&repository.runtime, &record.job_id)
        .await?
    else {
        return Ok(None);
    };
    if publication.kind != PublishedJobKind::Inspect {
        return Err(io::Error::other("published job kind mismatch"));
    }
    Ok(Some((
        publication.state,
        deliver(repository, blobs, &publication, request).await?,
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use enrichment_core::{
        config::Config,
        evidence::{
            SymbolKind,
            path::PublicPath,
            relational::{Definition, PublicBinding},
        },
        identity::{Context, Release, ReleaseKey, ResearchMode},
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
            definition_id: Symbol::definition_id_for(
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
        let base = service
            .repository
            .publish(
                base_metadata.clone(),
                ingest::EvidenceBatch {
                    definitions: vec![definition],
                    symbols: vec![binding.clone()],
                    ..Default::default()
                },
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
            context_id: base_metadata.context.context_id.to_string(),
            snapshot_id: Some(base.snapshot_id.to_string()),
            symbol_path: "fixture.f".into(),
            execution: Some(options),
            ..Default::default()
        };
        let (record, _, _) = service
            .jobs
            .submit(
                "retained-fixture".into(),
                jobs::JobSpec::Inspect(request.clone()),
            )
            .unwrap();
        service.jobs.start(&record.job_id).unwrap();
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
            attempt_id: record.job_id.clone(),
            producer: producer_identity(false).0.into(),
            producer_version: producer_identity(false).1,
            config_digest: "fixture".into(),
            inputs: [
                ("document".into(), document.sha256.clone()),
                ("lock".into(), lock.sha256.clone()),
                ("result".into(), result.sha256.clone()),
            ]
            .into(),
            profile: ExecutionProfile::Build,
            started_at: "2026-09-14T00:00:00Z".into(),
            finished_at: "2026-09-14T00:00:01Z".into(),
            outcome: RunOutcome::Succeeded,
            gaps: Vec::new(),
            log: Some(receipt.artifact_id.clone()),
        };
        let fact = ExecutionObservation::new(
            SubjectRef::Document {
                artifact_id: document.artifact_id.clone(),
                heading: "consumer semantic query".into(),
            },
            environment.environment_id.to_string(),
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
            .symbols_at(
                &request.symbol_path,
                request.definition_id.as_deref(),
                false,
            )
            .await
            .unwrap()
            .pop()
            .unwrap();
        let mut fixture_reply = render(
            &request,
            fixture_symbol,
            vec![fact.clone()],
            run.clone(),
            vec![result.clone(), receipt.clone()],
            JobState::Succeeded,
        )
        .unwrap();
        if large_delivery {
            fixture_reply
                .coverage
                .limitations
                .push("🌎\"\n".repeat(180_000));
        }
        let (prepare_delivery, _delivery) =
            crate::delivery::prepare_job(service.blobs.clone(), move |manifest| {
                let mut reply = fixture_reply.clone();
                reply.context_id = Some(manifest.context_id.to_string());
                reply.snapshot_id = Some(manifest.snapshot_id.to_string());
                Ok(reply)
            });
        let evidence = ingest::normalize_execution(
            fact.clone(),
            run.clone(),
            vec![document, lock, result.clone(), receipt],
        )
        .unwrap();
        let manifest = service
            .repository
            .publish_job(
                metadata,
                evidence,
                enrichment_store::repository::JobCompletion {
                    job_id: record.job_id.clone(),
                    kind: PublishedJobKind::Inspect,
                    state: JobState::Succeeded,
                    attempt_id: run.attempt_id,
                    result_artifact_ids: vec![result.artifact_id],
                    prepare_delivery,
                },
            )
            .await
            .unwrap();
        // Simulate a process disappearing after catalog commit, before terminal journal write.
        assert_eq!(
            service.jobs.get(&record.job_id).unwrap().state,
            JobState::Running
        );
        let expected = recover(&service.repository, &service.blobs, &record)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(expected.0, JobState::Succeeded);
        drop(service);
        let service = Service::open(config, paths).unwrap();
        let terminal = service.jobs.get(&record.job_id).unwrap();
        assert_eq!(terminal.state, JobState::Succeeded);
        assert_eq!(terminal.result.unwrap().data, expected.1.data);
        let mut read = request;
        if !derive {
            read.snapshot_id = Some(manifest.snapshot_id.to_string());
        }
        let answer = inspect::inspect(&service, read.clone()).await;
        assert_eq!(
            answer.context_id.as_deref(),
            Some(context.context_id.as_str()),
            "{answer:?}"
        );
        let data: InspectData =
            serde_json::from_value(serde_json::Value::Object(answer.data)).unwrap();
        assert_eq!(data.execution_observations, vec![fact.clone()]);
        assert_eq!(
            service.jobs.counts(),
            (0, 0),
            "execute_on_miss reuses a sound result even though execution is disabled"
        );
        read.execution.as_mut().unwrap().intent = InspectionIntent::Retained;
        let answer = inspect::inspect(&service, read).await;
        let data: InspectData =
            serde_json::from_value(serde_json::Value::Object(answer.data)).unwrap();
        assert_eq!(
            data.execution_observations,
            if derive { vec![] } else { vec![fact] },
            "a retained parent read never discovers child evidence implicitly"
        );
        assert_eq!(service.lsp.metrics().started, 0);
    }
}
