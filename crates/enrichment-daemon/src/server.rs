//! The daemon's accept loop and method dispatch.
//!
//! The daemon is the single writer and job owner (blueprint §2.1). It must serve several agents
//! at once without duplicating work, and it outlives the adapters that connect to it: an
//! adapter disconnecting never cancels a job another caller still needs (§2.2).
//!
//! Every handler receives the one shared [`Service`]; method names are dotted `noun.verb`
//! (ADR 0006) and deliberately not the MCP tool names, so one tool can become several core
//! calls without either side renaming anything.

use std::path::Path;
use std::sync::Arc;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{UnixListener, UnixStream};
use tokio::sync::Notify;

use enrichment_core::producer::rustdoc;
use enrichment_core::request::ResearchRequest;

use crate::ops;
use crate::paths::DaemonPaths;
use crate::rpc::{self, Request, Response, RpcError, codes};
use crate::service::Service;
use crate::status;

/// Serve requests until `shutdown` resolves.
///
/// # Errors
///
/// Fails if the socket cannot be bound -- most often because another daemon already holds it.
pub async fn serve(
    service: Arc<Service>,
    paths: &DaemonPaths,
    shutdown: impl std::future::Future<Output = ()> + Send,
) -> std::io::Result<()> {
    paths.ensure_dir()?;
    reclaim_stale_socket(&paths.socket)?;
    let listener = UnixListener::bind(&paths.socket)?;
    eprintln!(
        "library-enrichmentd: listening on {}",
        paths.socket.display()
    );

    // In-band shutdown: `daemon.shutdown` over the socket, so `library-enrichmentd stop` needs
    // no pidfile and no signal-sending privileges.
    let stop = Arc::new(Notify::new());
    // `Runner::serve` gives a warm session no container wall clock, on the stated grounds that
    // the session manager's idle policy bounds it. That is only true if something drives the
    // sweep, so this is that something. Without it the policy would be decorative and a single
    // inspection could leave a language server running until the daemon stopped.
    let sweeper = tokio::spawn({
        let service = Arc::clone(&service);
        async move {
            let interval = std::time::Duration::from_secs(
                service
                    .config
                    .execution
                    .lsp_idle_seconds
                    .clamp(10, 3600)
                    .min(60),
            );
            loop {
                tokio::time::sleep(interval).await;
                service.lsp.evict_idle().await;
            }
        }
    });
    let result = accept_loop(&listener, service.clone(), shutdown, Arc::clone(&stop)).await;
    sweeper.abort();
    let _ = sweeper.await;
    let stopped = service.shutdown().await;

    // Leaving a live socket behind would make the next `start` look like a running daemon.
    let _ = std::fs::remove_file(&paths.socket);
    result.and(stopped)
}

async fn accept_loop(
    listener: &UnixListener,
    service: Arc<Service>,
    shutdown: impl std::future::Future<Output = ()> + Send,
    stop: Arc<Notify>,
) -> std::io::Result<()> {
    tokio::pin!(shutdown);
    let mut connections = tokio::task::JoinSet::new();
    let result = loop {
        tokio::select! {
            () = &mut shutdown => break Ok(()),
            () = stop.notified() => break Ok(()),
            completed = connections.join_next(), if !connections.is_empty() => {
                if let Some(Err(error)) = completed {
                    eprintln!("library-enrichmentd: connection task ended: {error}");
                }
            }
            accepted = listener.accept() => {
                let (stream, _) = match accepted {
                    Ok(accepted) => accepted,
                    Err(error) => break Err(error),
                };
                // One task per connection: a slow adapter must not block the others, because
                // this daemon is shared across every agent on the machine.
                let stop = Arc::clone(&stop);
                let service = Arc::clone(&service);
                connections.spawn(async move {
                    if let Err(err) = handle_connection(stream, service, stop).await {
                        eprintln!("library-enrichmentd: connection ended: {err}");
                    }
                });
            }
        }
    };
    // Stop every transport root before service shutdown drains jobs/native descendants.
    // Connection cancellation does not certify physical exit of its blocking child work.
    connections.abort_all();
    while connections.join_next().await.is_some() {}
    result
}

/// A socket file left by a dead process is reclaimed; one a live daemon is listening on is not.
///
/// Connecting is the only reliable test -- a pidfile can be stale in the other direction.
fn reclaim_stale_socket(socket: &Path) -> std::io::Result<()> {
    if !socket.exists() {
        return Ok(());
    }
    match std::os::unix::net::UnixStream::connect(socket) {
        Ok(_) => Err(std::io::Error::new(
            std::io::ErrorKind::AddrInUse,
            format!(
                "a daemon is already listening on {}; run `library-enrichmentd stop` first",
                socket.display()
            ),
        )),
        Err(_) => {
            // Nothing is listening, so the file is a leftover from a crash.
            std::fs::remove_file(socket)
        }
    }
}

async fn handle_connection(
    stream: UnixStream,
    service: Arc<Service>,
    stop: Arc<Notify>,
) -> std::io::Result<()> {
    let max_message_bytes = service.config.limits.rpc_message_bytes;
    let (read_half, mut write_half) = stream.into_split();
    let mut reader = BufReader::new(read_half);

    loop {
        let frame = match rpc::read_frame(&mut reader, max_message_bytes).await {
            Ok(Some(frame)) => frame,
            Ok(None) => return Ok(()),
            Err(rpc::FrameError::TooLarge { limit }) => {
                // Bounded, and it says so: the caller learns the limit and that large payloads
                // travel as artifact handles, not inline (§2.1).
                let response = Response::err(
                    None,
                    RpcError::new(
                        codes::MESSAGE_TOO_LARGE,
                        format!("message exceeds the {limit}-byte limit"),
                        "BUDGET_EXCEEDED",
                        "Send large payloads as content-addressed artifact handles, not inline.",
                    ),
                );
                write_half
                    .write_all(rpc::write_frame(&response).as_bytes())
                    .await?;
                continue;
            }
            Err(rpc::FrameError::Io(err)) => return Err(err),
        };

        if frame.trim().is_empty() {
            continue;
        }

        let dispatched = dispatch(&service, &frame).await;
        if let Some(response) = dispatched.response {
            write_half
                .write_all(rpc::write_frame(&response).as_bytes())
                .await?;
        }
        if dispatched.shutdown {
            // Answer first, then stop: the caller needs its acknowledgement.
            write_half.flush().await?;
            stop.notify_waiters();
            return Ok(());
        }
    }
}

/// What routing one frame produced.
#[derive(Debug, Clone, PartialEq)]
pub struct Dispatched {
    /// The frame to write back, or `None` for a notification.
    pub response: Option<Response>,
    /// Whether the daemon should stop after answering.
    pub shutdown: bool,
}

impl Dispatched {
    fn reply(response: Response) -> Self {
        Self {
            response: Some(response),
            shutdown: false,
        }
    }
}

/// Route one frame.
pub async fn dispatch(service: &Service, frame: &str) -> Dispatched {
    let began = std::time::Instant::now();
    let request: Request = match serde_json::from_str(frame) {
        Ok(request) => request,
        Err(err) => {
            return Dispatched::reply(Response::err(
                None,
                RpcError::new(
                    codes::PARSE_ERROR,
                    format!("invalid request: {err}"),
                    "UNSUPPORTED_FORMAT",
                    "Send one JSON-RPC 2.0 object per line.",
                ),
            ));
        }
    };

    if request.jsonrpc != rpc::JSONRPC_VERSION {
        return Dispatched::reply(Response::err(
            request.id,
            RpcError::new(
                codes::INVALID_REQUEST,
                format!("unsupported jsonrpc version `{}`", request.jsonrpc),
                "UNSUPPORTED_FORMAT",
                "This transport speaks JSON-RPC 2.0.",
            ),
        ));
    }

    let id = request.id.clone();
    if let Err(error) = request.delivery.validate() {
        return Dispatched::reply(Response::err(
            id,
            RpcError::new(
                codes::INVALID_PARAMS,
                error.to_string(),
                "UNSUPPORTED_FORMAT",
                "Supply an admitted delivery framing profile.",
            ),
        ));
    }
    let delivery = request.delivery.clone();
    let parsed =
        ResearchRequest::from_rpc(&request.method, request.params.clone()).and_then(Result::ok);
    let requested = parsed.as_ref().and_then(ResearchRequest::requested_budget);
    let descriptor = parsed
        .as_ref()
        .map(|request| service.operation_descriptor(request))
        .unwrap_or_else(|| enrichment_core::telemetry::OperationDescriptor {
            method: request.method.clone(),
            // Administrative or invalid protocol input is exact transport provenance, never a
            // semantic research key. Preserve the bytes that actually arrived at the boundary.
            request_digest: enrichment_core::canonical::sha256_hex(frame.as_bytes()),
            policy_digest: enrichment_core::native_key::Key::OperationPolicy
                .record(&service.config)
                .expect("effective native configuration"),
        });
    let correlation = crate::envelope::new_request_id().to_string();
    let owned = service.clone();
    let runtime = service.repository.runtime.clone();
    let operation = correlation.clone();
    let task = runtime.spawn(async move {
        owned
            .repository
            .runtime
            .operation(
                operation,
                descriptor,
                Box::pin(dispatch_request(&owned, request, began)),
            )
            .await
    });
    match task
        .await
        .map_err(std::io::Error::other)
        .and_then(|result| result.map_err(std::io::Error::other))
    {
        Ok(result) => result,
        Err(error) => {
            let response = if let Some(id) = id {
                let mut result = ops::common::query_error(&error.into());
                result.request_id = enrichment_core::wire::RequestId::try_from(correlation)
                    .expect("admitted identity");
                Some(research_response(service, Some(id), result, requested, delivery).await)
            } else {
                None
            };
            Dispatched {
                response,
                shutdown: false,
            }
        }
    }
}

async fn dispatch_request(
    service: &Service,
    request: Request,
    began: std::time::Instant,
) -> Dispatched {
    // A notification is a request with no id; it is still dispatched, but answered with nothing.
    let is_notification = request.id.is_none();
    let mut shutdown = false;
    let response = match request.method.as_str() {
        "daemon.shutdown" => {
            shutdown = true;
            Response::ok(request.id, serde_json::json!({ "stopping": true }))
        }
        "daemon.ping" => Response::ok(request.id, serde_json::json!({ "pong": true })),
        // Gate R04, and the fourth clause of the blueprint's Phase-0 gate: an unsupported
        // producer format returns a TYPED error, never a best-effort parse.
        "producer.probe_format" => match request.params.get("artifact").and_then(|a| a.as_str()) {
            Some(artifact) => match rustdoc::probe_format(artifact) {
                Ok(probe) => Response::ok(
                    request.id,
                    serde_json::to_value(probe).expect("FormatProbe always serializes"),
                ),
                Err(err) => Response::err(
                    request.id,
                    RpcError::new(
                        codes::INVALID_PARAMS,
                        err.to_string(),
                        err.code(),
                        &err.next_action(),
                    ),
                ),
            },
            None => Response::err(
                request.id,
                RpcError::new(
                    codes::INVALID_PARAMS,
                    "producer.probe_format requires a string `artifact` parameter",
                    "UNSUPPORTED_FORMAT",
                    "Pass the producer artifact as {\"artifact\": \"<json text>\"}.",
                ),
            ),
        },
        // Gate C19's RPC leg. Deliberately the SAME `crate::validate::validate` the CLI calls,
        // so the two boundaries cannot disagree about which documents are well formed.
        "wire.validate" => match request.params.get("document").and_then(|d| d.as_str()) {
            Some(document) => Response::ok(
                request.id,
                serde_json::to_value(crate::validate::validate(document))
                    .expect("Validation always serializes"),
            ),
            None => Response::err(
                request.id,
                RpcError::new(
                    codes::INVALID_PARAMS,
                    "wire.validate requires a string `document` parameter",
                    "UNSUPPORTED_FORMAT",
                    "Pass the candidate envelope as {\"document\": \"<json text>\"}.",
                ),
            ),
        },
        unknown => match ResearchRequest::from_rpc(unknown, request.params) {
            Some(Ok(input)) => {
                let budget = input.requested_budget();
                let envelope = dispatch_research(service, input).await;
                research_response(
                    service,
                    request.id,
                    envelope,
                    budget,
                    request.delivery.clone(),
                )
                .await
            }
            Some(Err(error)) => invalid_params(
                request.id,
                unknown,
                &error,
                "the generated operation input contract",
            ),
            None => Response::err(
                request.id,
                RpcError::new(
                    codes::METHOD_NOT_FOUND,
                    format!("unknown method `{unknown}`"),
                    "UNSUPPORTED_CAPABILITY",
                    "Call service.status to see which components this build implements.",
                ),
            ),
        },
    };

    let status = response.observation.as_ref().map(|facts| facts.status);
    let has_gap = response
        .observation
        .as_ref()
        .is_some_and(|facts| facts.has_gap);

    // Measured even for a notification: the work happened, and a counter that quietly skipped
    // it would understate what this process did.
    let bytes = serde_json::to_vec(&response).map_or(0, |v| v.len());
    service.metrics.record_response(
        &request.method,
        status,
        has_gap,
        u64::try_from(began.elapsed().as_micros()).unwrap_or(u64::MAX),
        bytes as u64,
    );

    Dispatched {
        response: (!is_notification).then_some(response),
        shutdown,
    }
}

/// Routing is exhaustive over the generated operation declaration. Native admission runs
/// before every research tool/resource; the match only invokes its I/O-facing handler.
async fn dispatch_research(
    service: &Service,
    input: ResearchRequest,
) -> enrichment_core::wire::Envelope {
    if let Err(error) = enrichment_store::request_admission::research(
        &service.repository.runtime,
        &input,
        service.config.limits.verification_input_bytes,
    )
    .await
    {
        return ops::common::operation_error(&error, "research_request_admission");
    }
    match input {
        ResearchRequest::Resolve(request) => ops::resolve::resolve(service, request).await,
        ResearchRequest::Overview(request) => ops::overview::overview(service, request).await,
        ResearchRequest::Search(request) => ops::search::search(service, request).await,
        ResearchRequest::Inspect(request) => ops::inspect::inspect(service, request).await,
        ResearchRequest::Compare(request) => ops::compare::compare(service, request).await,
        ResearchRequest::Verify(request) => ops::verify::verify(service, request).await,
        ResearchRequest::ReadArtifact(request) => ops::artifact::read(service, request).await,
        ResearchRequest::Job(request) => ops::verify::control(service, request).await,
        ResearchRequest::ServiceStatus(request) => {
            status::status_envelope(service, request.component.as_deref()).await
        }
        ResearchRequest::SnapshotManifest(request) => {
            ops::manifest::manifest(service, request).await
        }
    }
}

/// The only native research-to-RPC projection. Keep the domain result typed through delivery
/// and diagnostics; serialize once when constructing the transport response.
async fn research_response(
    service: &Service,
    id: Option<serde_json::Value>,
    mut result: enrichment_core::wire::Envelope,
    requested: Option<usize>,
    profile: enrichment_core::mcp_delivery::DeliveryProfile,
) -> Response {
    if enrichment_store::runtime::operation_id().is_some() {
        result.request_id = crate::envelope::new_request_id();
    }
    if let Some(error) = result.error_mut() {
        if error.diagnostic.correlation_id.is_none() {
            error.diagnostic.correlation_id = enrichment_store::runtime::operation_id();
        }
        if error.diagnostic.rule.is_some() {
            service
                .repository
                .runtime
                .record_failure(error.diagnostic.clone());
        }
    }
    let bounded =
        ops::common::enforce_delivery_budget(service, result, requested, profile.clone()).await;
    let projected = match &profile {
        enrichment_core::mcp_delivery::DeliveryProfile::Envelope => serde_json::to_value(&bounded),
        enrichment_core::mcp_delivery::DeliveryProfile::McpStdio { era, .. } => {
            enrichment_core::mcp_delivery::project(&bounded, era)
                .map_err(serde_json::Error::io)
                .and_then(serde_json::to_value)
        }
        enrichment_core::mcp_delivery::DeliveryProfile::McpResourceStdio { era, uri, .. } => {
            enrichment_core::mcp_delivery::project_resource(&bounded, era, uri)
                .map_err(serde_json::Error::io)
                .and_then(serde_json::to_value)
        }
    };
    let mut response = Response::ok(id, projected.expect("bounded native research projection"));
    response.observation = Some(rpc::ResponseObservation {
        status: bounded.status(),
        has_gap: !bounded.coverage.missing.is_empty(),
    });
    if !matches!(
        profile,
        enrichment_core::mcp_delivery::DeliveryProfile::Envelope
    ) {
        response.delivery_bytes =
            Some(profile.measure(&bounded).expect("bounded measurement") as u64);
    }
    response
}

fn invalid_params(
    id: Option<serde_json::Value>,
    method: &str,
    err: &serde_json::Error,
    example: &str,
) -> Response {
    Response::err(
        id,
        RpcError::new(
            codes::INVALID_PARAMS,
            format!("{method} parameters are invalid: {err}"),
            "UNSUPPORTED_FORMAT",
            &format!("Pass at least {example}."),
        ),
    )
}

/// Ask a running daemon to stop, and wait for its acknowledgement.
///
/// # Errors
///
/// Fails if nothing is listening on the socket.
pub async fn request_shutdown(paths: &DaemonPaths) -> std::io::Result<()> {
    let stream = UnixStream::connect(&paths.socket).await?;
    let (read_half, mut write_half) = stream.into_split();
    let mut reader = BufReader::new(read_half);

    let request = serde_json::json!({
        "jsonrpc": rpc::JSONRPC_VERSION,
        "id": 1,
        "method": "daemon.shutdown",
    });
    write_half
        .write_all(format!("{request}\n").as_bytes())
        .await?;
    write_half.flush().await?;

    let mut line = String::new();
    reader.read_line(&mut line).await?;
    Ok(())
}

/// Ask a running daemon for its status.
///
/// # Errors
///
/// Fails if nothing is listening on the socket, or the exchange fails.
pub async fn request_status(paths: &DaemonPaths) -> std::io::Result<serde_json::Value> {
    let stream = UnixStream::connect(&paths.socket).await?;
    let (read_half, mut write_half) = stream.into_split();
    let mut reader = BufReader::new(read_half);

    let request = serde_json::json!({
        "jsonrpc": rpc::JSONRPC_VERSION,
        "id": 1,
        "method": "service.status",
        "params": {},
    });
    write_half
        .write_all(format!("{request}\n").as_bytes())
        .await?;
    write_half.flush().await?;

    let mut line = String::new();
    reader.read_line(&mut line).await?;
    serde_json::from_str(&line)
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err.to_string()))
}

#[cfg(test)]
mod tests {
    use enrichment_core::config::Config;
    use enrichment_store::StatePaths;

    use super::*;

    /// A service over a private temporary directory. Nothing here reads `LIBENR_*`.
    fn test_service() -> (tempfile::TempDir, Service) {
        let dir = tempfile::tempdir().expect("temp dir");
        let paths = StatePaths::explicit(dir.path().join("cache"), dir.path().join("data"));
        let service = Service::open(Config::default(), paths).expect("service opens");
        (dir, service)
    }

    async fn dispatch_str(frame: &str) -> Response {
        let (_dir, service) = test_service();
        dispatch(&service, frame)
            .await
            .response
            .expect("this frame expects a response")
    }

    #[tokio::test]
    async fn service_status_reports_the_store_as_ready_once_it_is_open() {
        let response = dispatch_str(r#"{"jsonrpc":"2.0","id":1,"method":"service.status"}"#).await;
        assert!(response.error.is_none());
        let envelope = response.result.expect("a status envelope");
        assert_eq!(envelope["status"], "ok");
        assert_eq!(
            envelope["data"]["health"]["cache_ready"],
            serde_json::json!(true)
        );
    }

    #[tokio::test]
    async fn the_status_envelope_is_built_in_the_core() {
        // The fields the adapter used to author. If these ever come back empty, identity and
        // coverage have drifted back across the boundary §1.1 draws.
        let response = dispatch_str(r#"{"jsonrpc":"2.0","id":1,"method":"service.status"}"#).await;
        let envelope = response.result.expect("a status envelope");

        assert!(
            envelope["request_id"]
                .as_str()
                .is_some_and(|id| id.starts_with("req_")),
            "the core mints the request identity"
        );
        assert!(
            !envelope["coverage"]["scope"]
                .as_str()
                .unwrap_or_default()
                .is_empty(),
            "the core states its own coverage"
        );
        assert_eq!(envelope["freshness"]["latest_verified"], false);
        assert_eq!(envelope["schema_version"], "3.0");
    }

    #[tokio::test]
    async fn an_unmatched_component_filter_is_partial_with_the_gap_named() {
        // An empty `ok` would be indistinguishable from "no such component exists", which is a
        // much stronger claim than "the filter matched nothing".
        let frame = r#"{"jsonrpc":"2.0","id":1,"method":"service.status",
                        "params":{"component":"no-such-producer"}}"#;
        let envelope = dispatch_str(frame).await.result.expect("an envelope");

        assert_eq!(envelope["status"], "partial");
        assert!(
            envelope["data"]["producers"]
                .as_array()
                .is_some_and(Vec::is_empty)
        );
        assert!(
            !envelope["coverage"]["missing"]
                .as_array()
                .unwrap_or(&Vec::new())
                .is_empty(),
            "the gap must be explicit"
        );
    }

    #[tokio::test]
    async fn a_matched_component_filter_narrows_the_report() {
        let frame = r#"{"jsonrpc":"2.0","id":1,"method":"service.status",
                        "params":{"component":"griffe"}}"#;
        let envelope = dispatch_str(frame).await.result.expect("an envelope");

        assert_eq!(envelope["status"], "ok");
        let producers = envelope["data"]["producers"].as_array().expect("producers");
        assert_eq!(producers.len(), 1);
        assert_eq!(producers[0]["name"], "griffe");
    }

    #[tokio::test]
    async fn an_unknown_method_is_a_typed_error() {
        let response = dispatch_str(r#"{"jsonrpc":"2.0","id":1,"method":"no.such.method"}"#).await;
        let error = response.error.expect("an error");
        assert_eq!(error.code, codes::METHOD_NOT_FOUND);
        assert_eq!(error.data.expect("data")["code"], "UNSUPPORTED_CAPABILITY");
    }

    #[tokio::test]
    async fn malformed_json_is_a_typed_error_not_a_panic() {
        let error = dispatch_str("{not json").await.error.expect("an error");
        assert_eq!(error.code, codes::PARSE_ERROR);
        assert_eq!(error.data.expect("data")["code"], "UNSUPPORTED_FORMAT");
    }

    #[tokio::test]
    async fn a_wrong_protocol_version_is_rejected() {
        let response = dispatch_str(r#"{"jsonrpc":"1.0","id":1,"method":"service.status"}"#).await;
        let error = response.error.expect("an error");
        assert_eq!(error.code, codes::INVALID_REQUEST);
    }

    #[tokio::test]
    async fn library_resolve_without_a_name_is_invalid_params() {
        let response =
            dispatch_str(r#"{"jsonrpc":"2.0","id":1,"method":"library.resolve","params":{}}"#)
                .await;
        // An empty object deserializes (every field defaults) and then fails validation in
        // the core, so the answer is an error ENVELOPE, not an RPC-level error.
        assert!(response.error.is_none(), "{:?}", response.error);
        let envelope = response.result.expect("an envelope");
        assert_eq!(envelope["status"], "error");
        assert_eq!(envelope["error"]["code"], "VERSION_NOT_FOUND");

        let response = dispatch_str(
            r#"{"jsonrpc":"2.0","id":1,"method":"library.resolve","params":{"name":"x","bogus":1}}"#,
        )
        .await;
        let error = response
            .error
            .expect("unknown field is an RPC parameter error");
        assert_eq!(error.code, codes::INVALID_PARAMS);
    }

    #[tokio::test]
    async fn library_resolve_offline_with_nothing_recorded_is_a_typed_error() {
        let frame = r#"{"jsonrpc":"2.0","id":1,"method":"library.resolve",
                        "params":{"name":"enr-fixture","version":"0.1.0","freshness":"offline"}}"#;
        let envelope = dispatch_str(frame).await.result.expect("an envelope");
        assert_eq!(envelope["status"], "error");
        assert_eq!(envelope["error"]["code"], "ARTIFACT_UNAVAILABLE");
        assert!(
            envelope["error"]["next_action"]
                .as_str()
                .is_some_and(|s| s.contains("freshness"))
        );
    }

    #[tokio::test]
    async fn an_unsupported_producer_format_is_a_typed_error_over_rpc() {
        // Gate R04 at the transport, not just in the library: the typed refusal has to survive
        // the crossing, or a caller sees a generic failure instead of a code it can branch on.
        let frame = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "producer.probe_format",
            "params": { "artifact": r#"{"format_version":53,"index":{}}"# },
        })
        .to_string();

        let error = dispatch_str(&frame).await.error.expect("an error");
        let data = error.data.expect("typed data");
        assert_eq!(data["code"], "UNSUPPORTED_FORMAT");
        assert!(
            !data["next_action"].as_str().unwrap_or_default().is_empty(),
            "a typed refusal carries a concrete next action"
        );
        assert!(
            error.message.contains("53"),
            "the refusal names the version it found: {}",
            error.message
        );
    }

    #[tokio::test]
    async fn a_supported_producer_format_is_accepted_over_rpc() {
        let artifact = format!(
            r#"{{"format_version":{},"index":{{}}}}"#,
            rustdoc::SUPPORTED_FORMAT_VERSIONS[0]
        );
        let frame = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "producer.probe_format",
            "params": { "artifact": artifact },
        })
        .to_string();

        let response = dispatch_str(&frame).await;
        assert!(response.error.is_none(), "{:?}", response.error);
        assert_eq!(response.result.expect("a probe")["supported"], true);
    }

    #[tokio::test]
    async fn a_notification_gets_no_response() {
        let (_dir, service) = test_service();
        assert!(
            dispatch(&service, r#"{"jsonrpc":"2.0","method":"daemon.ping"}"#)
                .await
                .response
                .is_none()
        );
    }

    #[tokio::test]
    async fn shutdown_is_acknowledged_before_the_daemon_stops() {
        let (_dir, service) = test_service();
        let dispatched = dispatch(
            &service,
            r#"{"jsonrpc":"2.0","id":1,"method":"daemon.shutdown"}"#,
        )
        .await;
        assert!(dispatched.shutdown, "the daemon must stop");
        assert!(
            dispatched.response.is_some(),
            "the caller must be told it is stopping"
        );
    }

    #[tokio::test]
    async fn a_response_frame_is_exactly_one_line() {
        let response = dispatch_str(r#"{"jsonrpc":"2.0","id":1,"method":"service.status"}"#).await;
        let frame = rpc::write_frame(&response);
        assert_eq!(frame.matches('\n').count(), 1);
        assert!(frame.ends_with('\n'));
    }
}
