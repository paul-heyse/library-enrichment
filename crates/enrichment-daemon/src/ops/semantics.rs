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
        document::{self, Consumer},
    },
    service::Service,
};
use enrichment_core::{
    canonical,
    evidence::{Artifact, SymbolHeader, execution::*, relational::SubjectRef},
    identity::{Ecosystem, Release},
    policy::ExecutionProfile,
    request::InspectionOptions,
    wire::EvidenceClass,
};
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    io,
    path::Path,
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
    options: &InspectionOptions,
    cancel: Arc<AtomicBool>,
) -> io::Result<Produced> {
    let consumer =
        Consumer::new(symbol, opened.release.key.ecosystem, options).map_err(io::Error::other)?;
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
    let selected_methods = inspect_execution::methods(options);
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
            let served = runner.serve(&start_image, &prepared.root, &server.argv()).await?;
            Session::initialize(server, served, START_DEADLINE, start_cancel, Scope { environment: prepared.environment.clone(), lock: prepared.lock.clone(), inputs }).await
        }.await;
        if startup.is_err() { service.execution.wait_for_cleanup(&lease).await?; }
        startup
    }, move |session| Box::pin(async move {
        query(&query_service, &query_release, session, Query { consumer, symbol_id, structural_scope, methods: selected_methods, containment, cancel: query_cancel }).await
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
    consumer: Consumer,
    symbol_id: String,
    structural_scope: bool,
    methods: Vec<SemanticMethod>,
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
    let consumer = query.consumer;
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
    session
        .open(
            consumer.uri,
            if release.key.ecosystem == Ecosystem::Rust {
                "rust"
            } else {
                "python"
            },
            &consumer.text,
        )
        .await?;
    session.await_readiness(Duration::from_secs(15)).await?;
    let mut facts = Vec::new();
    let mut transcript = Vec::new();
    for method in query.methods {
        if query.cancel.load(Ordering::Acquire) {
            return Err(io::Error::new(
                io::ErrorKind::Interrupted,
                "semantic inspection cancelled",
            ));
        }
        let (name, capability) = match method {
            SemanticMethod::Hover => ("textDocument/hover", "hoverProvider"),
            SemanticMethod::Definition => ("textDocument/definition", "definitionProvider"),
            SemanticMethod::Implementation => {
                ("textDocument/implementation", "implementationProvider")
            }
            SemanticMethod::References => ("textDocument/references", "referencesProvider"),
            SemanticMethod::Diagnostics => ("textDocument/diagnostic", "diagnosticProvider"),
        };
        let position = if method == SemanticMethod::Diagnostics {
            None
        } else {
            consumer.position
        };
        let mut params = json!({"textDocument":{"uri":consumer.uri}});
        if let Some(position) = position {
            let (line, character) =
                document::protocol_position(&consumer.text, position, &session.position_encoding)
                    .map_err(io::Error::other)?;
            params["position"] = json!({"line":line,"character":character});
        }
        if method == SemanticMethod::References {
            params["context"] = json!({"includeDeclaration":true});
        }
        let answer = if method == SemanticMethod::Diagnostics {
            session.diagnostic(consumer.uri, QUERY_DEADLINE).await
        } else if session.advertises(capability) {
            session.request(name, params.clone(), QUERY_DEADLINE).await
        } else {
            Ok(json!({"kind":"unsupported"}))
        };
        let mut q = SemanticQuery {
            method,
            document_artifact_id: document.artifact_id.clone(),
            position,
            anchor_symbol_id: Some(query.symbol_id.clone()),
            server: session.server_version.clone(),
            outcome: ExecutionOutcome::Empty,
            hover: None,
            locations: Vec::new(),
            diagnostics: Vec::new(),
            limitations: Vec::new(),
        };
        match answer {
            Err(e) if e.kind() == io::ErrorKind::InvalidData => {
                q.outcome = ExecutionOutcome::Unresolved;
                q.limitations
                    .push(e.to_string().chars().take(1000).collect());
                transcript.push(json!({"method":name,"params":params,"error":e.to_string()}));
            }
            Err(e) => return Err(e),
            Ok(answer) => {
                if serde_json::to_vec(&answer)?.len() > 256 * 1024 {
                    return Err(io::Error::other(
                        "semantic response exceeds the retained record byte budget",
                    ));
                }
                transcript.push(json!({"method":name,"params":params,"result":answer}));
                if answer["kind"] == "unsupported" {
                    q.outcome = ExecutionOutcome::Unsupported;
                    q.limitations.push(format!("{} did not advertise {name}; no matching version-qualified diagnostic push was available when applicable.", session.server_version));
                } else if let Err(error) = normalize_answer(
                    service,
                    release,
                    session,
                    (&consumer, &document),
                    &mut inputs,
                    &answer,
                    &mut q,
                ) {
                    q.outcome = if q.hover.is_some()
                        || !q.locations.is_empty()
                        || !q.diagnostics.is_empty()
                    {
                        ExecutionOutcome::Incomplete
                    } else {
                        ExecutionOutcome::Unresolved
                    };
                    q.limitations.push(error.to_string());
                }
            }
        }
        if let Some(gap) = session.indexing_gap() {
            if matches!(
                q.outcome,
                ExecutionOutcome::Results | ExecutionOutcome::Empty
            ) {
                q.outcome = ExecutionOutcome::Incomplete;
            }
            q.limitations.push(gap);
        }
        if method == SemanticMethod::References {
            q.limitations.push("References cover the opened isolated consumer and the server's selected workspace; external projects were not searched.".into());
        }
        if session.server == lsp::settings::Server::RustAnalyzer {
            q.limitations.push("rust-analyzer runs with build scripts and procedural macros disabled; generated declarations may be unresolved.".into());
        }
        if method == SemanticMethod::Implementation
            && session.server == lsp::settings::Server::Ty
            && query.structural_scope
        {
            if matches!(
                q.outcome,
                ExecutionOutcome::Results | ExecutionOutcome::Empty
            ) {
                q.outcome = ExecutionOutcome::Incomplete;
            }
            q.limitations.push(
                "This query does not establish exhaustive structural protocol implementors.".into(),
            );
        }
        let class = if release.key.ecosystem == Ecosystem::Rust {
            EvidenceClass::CompilerDerived
        } else {
            EvidenceClass::TypecheckerObserved
        };
        facts.push((
            SubjectRef::Document {
                artifact_id: document.artifact_id.clone(),
                heading: "consumer semantic query".into(),
            },
            ExecutionPayload::SemanticQuery(q),
            class,
        ));
    }
    let raw = json!({"position_encoding":session.position_encoding,"document_version":session.document_version(), "capsule":session.inputs_root(), "server_log":session.diagnostics(),"queries":transcript});
    if serde_json::to_vec(&raw)?.len() > 1536 * 1024 {
        return Err(io::Error::other(
            "inspection transcript exceeds its byte budget",
        ));
    }
    Ok(Produced {
        environment: session.environment.clone(),
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
        lock: session.lock.clone(),
        transcript: raw,
    })
}

fn normalize_answer(
    service: &Service,
    release: &Release,
    session: &Session,
    source: (&Consumer, &Artifact),
    inputs: &mut BTreeMap<String, Artifact>,
    answer: &Value,
    query: &mut SemanticQuery,
) -> io::Result<()> {
    let (consumer, document) = source;
    if answer.is_null() {
        query.outcome = ExecutionOutcome::Unresolved;
        query.limitations.push("The server returned null; target resolution or method availability was not established by that response.".into());
        return Ok(());
    }
    match query.method {
        SemanticMethod::Hover => {
            if !answer.is_null() {
                let contents = answer
                    .get("contents")
                    .ok_or_else(|| io::Error::other("hover result lacks contents"))?;
                fn markup(value: &Value) -> io::Result<String> {
                    match value {
                        Value::String(s) => Ok(s.clone()),
                        Value::Array(values) => values
                            .iter()
                            .map(markup)
                            .collect::<Result<Vec<_>, _>>()
                            .map(|v| v.join("\n")),
                        Value::Object(_) => value
                            .get("value")
                            .and_then(Value::as_str)
                            .map(str::to_owned)
                            .ok_or_else(|| io::Error::other("invalid hover markup")),
                        _ => Err(io::Error::other("invalid hover content")),
                    }
                }
                let text = markup(contents)?;
                if text.len() > 32 * 1024 {
                    return Err(io::Error::other(
                        "hover text exceeds its retained byte budget",
                    ));
                }
                if !text.is_empty() {
                    query.hover = Some(text);
                }
            }
        }
        SemanticMethod::Diagnostics => {
            if answer["kind"] != "full" {
                return Err(io::Error::other(
                    "diagnostic result is not a resolved full report",
                ));
            }
            let items = answer["items"]
                .as_array()
                .ok_or_else(|| io::Error::other("diagnostic items missing"))?;
            if items.len() > 256 {
                return Err(io::Error::other("diagnostic result exceeds 256 items"));
            }
            for item in items {
                let range =
                    document::range(&consumer.text, &item["range"], &session.position_encoding)
                        .map_err(io::Error::other)?;
                let severity = item
                    .get("severity")
                    .map(|v| {
                        v.as_u64()
                            .and_then(|n| u32::try_from(n).ok())
                            .filter(|n| (1..=4).contains(n))
                            .ok_or_else(|| io::Error::other("invalid diagnostic severity"))
                    })
                    .transpose()?;
                let code = match item.get("code") {
                    None | Some(Value::Null) => None,
                    Some(Value::String(s)) => Some(s.clone()),
                    Some(Value::Number(n)) => Some(n.to_string()),
                    _ => return Err(io::Error::other("invalid diagnostic code")),
                };
                let source = item
                    .get("source")
                    .map(|v| {
                        v.as_str()
                            .map(str::to_owned)
                            .ok_or_else(|| io::Error::other("invalid diagnostic source"))
                    })
                    .transpose()?;
                let message = item["message"]
                    .as_str()
                    .ok_or_else(|| io::Error::other("diagnostic message missing"))?
                    .to_owned();
                query.diagnostics.push(ExecutionDiagnostic {
                    range,
                    severity,
                    code,
                    source,
                    message,
                });
            }
        }
        SemanticMethod::Definition
        | SemanticMethod::Implementation
        | SemanticMethod::References => {
            let locations = lsp::client::Location::all_from(answer)?;
            if locations.len() > 256 {
                return Err(io::Error::other("location result exceeds 256 items"));
            }
            for location in locations {
                query.locations.push(target(
                    service, release, session, consumer, document, inputs, &location,
                )?);
            }
        }
    }
    query.outcome =
        if query.hover.is_some() || !query.locations.is_empty() || !query.diagnostics.is_empty() {
            ExecutionOutcome::Results
        } else {
            ExecutionOutcome::Empty
        };
    Ok(())
}

fn target(
    service: &Service,
    release: &Release,
    session: &Session,
    consumer: &Consumer,
    document: &Artifact,
    inputs: &mut BTreeMap<String, Artifact>,
    location: &lsp::client::Location,
) -> io::Result<ExecutionTarget> {
    let range = |text: &str| -> Result<Utf8Range, String> {
        let range = Utf8Range {
            start: document::byte_position(
                text,
                location.line,
                location.character,
                &session.position_encoding,
            )?,
            end: document::byte_position(
                text,
                location.end_line,
                location.end_character,
                &session.position_encoding,
            )?,
        };
        range.validate()?;
        Ok(range)
    };
    if location.uri == consumer.uri {
        return Ok(ExecutionTarget::Artifact {
            artifact_id: document.artifact_id.clone(),
            range: range(&consumer.text).map_err(io::Error::other)?,
        });
    }
    let url = url::Url::parse(&location.uri).map_err(io::Error::other)?;
    let path = url
        .to_file_path()
        .map_err(|()| io::Error::other("server location is not a local file URI"))?;
    if let Ok(relative) = path.strip_prefix("/capsule") {
        match inspect_execution::read_input(session.inputs_root(), relative, 1024 * 1024) {
            Ok(bytes) => {
                if inputs.len() >= 256
                    || inputs
                        .values()
                        .filter(|a| a.media_type.starts_with("text/"))
                        .map(|a| a.size_bytes)
                        .sum::<u64>()
                        + bytes.len() as u64
                        > 16 * 1024 * 1024
                {
                    return Err(io::Error::other(
                        "retained source document closure exceeds its bound",
                    ));
                }
                let text = std::str::from_utf8(&bytes).map_err(io::Error::other)?;
                let range = range(text).map_err(io::Error::other)?;
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
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => return Err(error),
        }
    }
    let relative = path
        .strip_prefix(Path::new("/"))
        .map_err(io::Error::other)?
        .to_string_lossy()
        .to_string();
    let target = ExecutionTarget::External {
        scope: format!("release={};image={};server={}", release.release_id, session.image_id(), session.server_version),
        path: relative, limitation: "The server location is outside retained source documents; its range is not asserted as an artifact span.".into(),
    };
    target.validate().map_err(io::Error::other)?;
    Ok(target)
}
