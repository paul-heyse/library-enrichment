//! Retain typed ty/rust-analyzer answers in an exact immutable consumer scope.
use super::{
    common,
    inspect_execution::{self, Produced},
};
use crate::{
    execution::{Runner, capsule},
    lsp::{
        self, SessionKey,
        client::{Scope, Session},
    },
    service::Service,
};
use enrichment_core::{
    canonical,
    evidence::{Artifact, SymbolHeader, execution::*, relational::SubjectRef},
    identity::{Ecosystem, Release},
    native_semantics::Consumer,
    policy::ExecutionProfile,
    request::InspectionOptions,
};
use serde_json::json;
use std::{
    collections::BTreeMap,
    io,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};
const START_DEADLINE: Duration = Duration::from_secs(45);
const QUERY_DEADLINE: Duration = Duration::from_secs(30);

pub async fn produce(
    service: &Service,
    opened: &common::Opened,
    symbol: &SymbolHeader,
    _options: &InspectionOptions,
    cancel: Arc<AtomicBool>,
) -> io::Result<Produced> {
    let prepared = enrichment_store::semantic_grants::prepare(
        enrichment_store::native_effect::authorize()
            .await
            .map_err(io::Error::other)?,
        &opened.reader,
        &symbol.symbol_id,
    )
    .await
    .map_err(io::Error::other)?;
    let image = match opened.release.key.ecosystem {
        Ecosystem::Rust => service.config.execution.rust_image.clone(),
        Ecosystem::Python => service.config.execution.python_image.clone(),
    }
    .ok_or_else(|| io::Error::other("producer image missing"))?;
    let runner = Runner::new(
        &service.config.execution,
        &service.paths.cache_root,
        service.execution.clone(),
        service.ownership.clone(),
    )?;
    let containment = runner.containment_identity()?;
    let key = SessionKey {
        server: lsp::server_for(opened.release.key.ecosystem).name(),
        image_id: image.clone(),
        capsule_digest: capsule::capsule_key(opened, &image, &containment),
    };
    let server = lsp::server_for(opened.release.key.ecosystem);
    let start_cancel = cancel.clone();
    let query_cancel = cancel.clone();
    let start_image = image.clone();
    let symbol_id = symbol.symbol_id.clone();
    // A direct base-free class is nominal. Other declarations may inherit a protocol
    // through an unresolved base, so their implementation results remain conservative.
    let structural_scope = opened.release.key.ecosystem == Ecosystem::Python
        && !opened
            .reader
            .is_nominal_python_class(&symbol.symbol_id)
            .await
            .map_err(io::Error::other)?;
    let query_service = service.clone();
    let query_release = opened.release.clone();
    service.lsp.with_session(key, cancel, &service.execution, move || async move {
        let lease = tokio::select! {
            biased;
            () = lsp::cancelled(&start_cancel) => return Err(io::Error::new(io::ErrorKind::Interrupted, "cancelled before capsule preparation")),
            lease = service.execution.lease() => lease?,
        };
        let runner = runner.using_lease(lease.clone());
        let startup = async {
            let prepared = capsule::prepare_retained(service, opened, &runner, &start_image, start_cancel.clone()).await.map_err(preparation_error)?;
            let inputs = inspect_execution::capsule_inputs(service, opened, &prepared).await?;
            if start_cancel.load(Ordering::Acquire) { return Err(io::Error::new(io::ErrorKind::Interrupted, "capsule preparation cancelled")); }
            let served = runner.for_capsule(&prepared).serve(&start_image, &prepared.root, &enrichment_core::execution::producer::Invocation::LanguageServer { ecosystem:opened.release.key.ecosystem }).await?;
            Session::initialize(server, served, START_DEADLINE, start_cancel, Scope { inputs }).await
        }.await;
        if startup.is_err() { service.execution.wait_for_cleanup(&lease).await?; }
        startup
    }, move |session| Box::pin(async move {
        query(&query_service, &query_release, session, Query { prepared, symbol_id, structural_scope, containment, cancel: query_cancel }).await
    })).await
}

pub fn preparation_error(error: capsule::PreparationError) -> io::Error {
    match error {
        capsule::PreparationError::Cancelled => {
            io::Error::new(io::ErrorKind::Interrupted, "capsule preparation cancelled")
        }
        capsule::PreparationError::Environment(detail)
        | capsule::PreparationError::Policy(detail) => io::Error::other(detail),
        capsule::PreparationError::Process(observation, stage) => io::Error::other(format!(
            "{stage}: {:?}, {:?}: {}",
            observation.end,
            observation.exit_code,
            observation.stderr.chars().take(400).collect::<String>()
        )),
    }
}

struct Query {
    prepared: enrichment_store::semantic_grants::Prepared,
    symbol_id: String,
    structural_scope: bool,
    containment: String,
    cancel: Arc<AtomicBool>,
}

async fn query(
    service: &Service,
    release: &Release,
    session: &mut Session,
    query: Query,
) -> io::Result<Produced> {
    let started_at =
        enrichment_core::native_time::ObservationTime::now().map_err(std::io::Error::other)?;
    let consumer = query.prepared.consumer().clone();
    let document = inspect_execution::store(
        service,
        consumer.text.as_bytes(),
        &format!(
            "consumer://document/{}",
            canonical::sha256_hex(consumer.text.as_bytes())
        ),
        "text/plain; charset=utf-8",
    )?;
    let mut inputs: BTreeMap<String, Artifact> = session
        .inputs
        .iter()
        .cloned()
        .map(|a| (a.sha256.clone(), a))
        .collect();
    inputs.insert(document.sha256.clone(), document.clone());
    session.open(query.prepared).await?;
    session.await_readiness(Duration::from_secs(15)).await?;
    let mut facts = Vec::new();
    let mut transcript = Vec::new();
    for method in consumer.methods.iter().copied() {
        if query.cancel.load(Ordering::Acquire) {
            return Err(io::Error::new(
                io::ErrorKind::Interrupted,
                "semantic inspection cancelled",
            ));
        }
        let name = method.protocol_method();
        let params = session.semantic_parameters(method)?;
        let answer = session.semantic_request(method, QUERY_DEADLINE).await;
        let response = match answer {
            Err(error) if error.kind() == io::ErrorKind::InvalidData => {
                transcript.push(json!({"method":name,"params":params,"error":error.to_string()}));
                enrichment_core::native_lsp::Decoded {
                    kind: enrichment_core::native_lsp::ResponseKind::Value,
                    hover: None,
                    locations: vec![],
                    diagnostics: vec![],
                    issue: Some(error.to_string().chars().take(1000).collect()),
                }
            }
            Err(error) => return Err(error),
            Ok(answer) => {
                let raw = serde_json::to_string(&answer)?;
                if raw.len() > 256 * 1024 {
                    return Err(io::Error::other(
                        "semantic response exceeds retained byte budget",
                    ));
                }
                let response = enrichment_store::semantic_response_plan::decode(
                    &service.repository.runtime,
                    &enrichment_core::native_lsp::Input {
                        method,
                        answer: raw,
                        document: consumer.text.clone(),
                        encoding: enrichment_core::native_semantics::PositionEncoding::parse(
                            &session.position_encoding,
                        )
                        .ok_or_else(|| {
                            io::Error::other("unnegotiated semantic response encoding")
                        })?,
                    },
                )
                .await
                .map_err(io::Error::other)?;
                transcript.push(json!({"method":name,"params":params,"result":answer}));
                response
            }
        };
        let mut locations = Vec::new();
        let mut capture_issue = None;
        for location in &response.locations {
            match target(
                service,
                release,
                session,
                &consumer,
                &document,
                &mut inputs,
                location,
            )
            .await
            {
                Ok(location) => locations.push(location),
                Err(error) => {
                    capture_issue = Some(error.to_string());
                    break;
                }
            }
        }
        let selected = enrichment_store::semantic_response_plan::lower(
            &service.repository.runtime,
            enrichment_store::semantic_response_plan::Capture {
                method,
                document_artifact_id: document.artifact_id.clone(),
                position: consumer.position,
                anchor_symbol_id: Some(query.symbol_id.clone()),
                server: session.server_version.clone(),
                response,
                locations,
                capture_issue,
                indexing_gap: session.indexing_gap(),
                ecosystem: release.key.ecosystem,
                structural_scope: query.structural_scope,
            },
        )
        .await
        .map_err(io::Error::other)?;
        facts.push((
            SubjectRef::Document {
                artifact_id: document.artifact_id.clone(),
                heading: "consumer semantic query".into(),
            },
            ExecutionPayload::SemanticQuery(selected.query),
            selected.evidence_class,
        ));
    }
    let raw = json!({"conversation_id":session.conversation_id(),"position_encoding":session.position_encoding,"document_version":session.document_version(), "capsule":session.inputs_root(), "server_log":session.diagnostics(),"queries":transcript});
    if serde_json::to_vec(&raw)?.len() > 1536 * 1024 {
        return Err(io::Error::other(
            "inspection transcript exceeds its byte budget",
        ));
    }
    Ok(Produced {
        environment: session.prepared()?.environment.clone(),
        image: session.image_id().into(),
        containment: query.containment,
        producer: "semantic-inspection".into(),
        version: inspect_execution::producer_identity(false)?.1,
        profile: ExecutionProfile::Build,
        started_at,
        finished_at: enrichment_core::native_time::ObservationTime::now()
            .map_err(std::io::Error::other)?,
        facts,
        inputs: inputs.into_values().collect(),
        lock: session.prepared()?.lock.as_bytes().to_vec(),
        transcript: raw,
    })
}

async fn target(
    service: &Service,
    release: &Release,
    session: &Session,
    consumer: &Consumer,
    document: &Artifact,
    inputs: &mut BTreeMap<String, Artifact>,
    location: &enrichment_core::native_lsp::Location,
) -> io::Result<ExecutionTarget> {
    use enrichment_store::semantic_source_plan::{self, RouteKind};
    let route = semantic_source_plan::route(
        &service.repository.runtime,
        &location.uri,
        &consumer.uri,
        &session.prepared()?.inventory,
    )
    .await
    .map_err(io::Error::other)?;
    if route.kind == RouteKind::Consumer {
        return Ok(ExecutionTarget::Artifact {
            artifact_id: document.artifact_id.clone(),
            range: enrichment_store::semantic_response_plan::range(
                &service.repository.runtime,
                &enrichment_core::native_lsp::RangeInput {
                    document: consumer.text.clone(),
                    range: location.range.clone(),
                },
            )
            .await
            .map_err(io::Error::other)?,
        });
    }
    if route.kind == RouteKind::Refused {
        return Err(io::Error::other(
            "server location exceeds the native source contract",
        ));
    }
    if route.kind == RouteKind::Installed {
        let relative = route
            .relative
            .as_deref()
            .ok_or_else(|| io::Error::other("installed source route has no path"))?;
        let root = session.inputs_root().to_owned();
        let path = std::path::PathBuf::from(relative);
        let bytes = service
            .repository
            .runtime
            .blocking(move || inspect_execution::read_input(&root, &path, 1024 * 1024))
            .await
            .map_err(io::Error::other)??;
        let text = std::str::from_utf8(&bytes).map_err(io::Error::other)?;
        semantic_source_plan::admit_document(
            &service.repository.runtime,
            &route,
            text,
            inputs.values().cloned().collect(),
        )
        .await
        .map_err(io::Error::other)?;
        let range = enrichment_store::semantic_response_plan::range(
            &service.repository.runtime,
            &enrichment_core::native_lsp::RangeInput {
                document: text.into(),
                range: location.range.clone(),
            },
        )
        .await
        .map_err(io::Error::other)?;
        let artifact = inspect_execution::store(
            service,
            &bytes,
            &format!(
                "consumer-source://document/{}",
                canonical::sha256_hex(&bytes)
            ),
            "text/plain; charset=utf-8",
        )?;
        inputs.insert(artifact.sha256.clone(), artifact.clone());
        return Ok(ExecutionTarget::Artifact {
            artifact_id: artifact.artifact_id,
            range,
        });
    }
    let relative = route
        .path
        .strip_prefix('/')
        .ok_or_else(|| io::Error::other("external source path is not absolute"))?
        .to_owned();
    let target = ExecutionTarget::External {
        scope: format!("release={};image={};server={}", release.release_id, session.image_id(), session.server_version),
        path: relative, limitation: "The server location is outside retained source documents; its range is not asserted as an artifact span.".into(),
    };
    target.validate().map_err(io::Error::other)?;
    Ok(target)
}
