//! The daemon's accept loop and method dispatch.
//!
//! The daemon is the single writer and job owner (blueprint §2.1). It must serve several agents
//! at once without duplicating work, and it outlives the adapters that connect to it: an
//! adapter disconnecting never cancels a job another caller still needs (§2.2).

use std::path::Path;
use std::sync::Arc;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{UnixListener, UnixStream};
use tokio::sync::Notify;

use enrichment_core::producer::rustdoc;

use crate::paths::DaemonPaths;
use crate::rpc::{self, Request, Response, RpcError, codes};
use crate::status;

/// Serve requests until `shutdown` resolves.
///
/// # Errors
///
/// Fails if the socket cannot be bound -- most often because another daemon already holds it.
pub async fn serve(
    paths: &DaemonPaths,
    max_message_bytes: usize,
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
    let result = accept_loop(&listener, max_message_bytes, shutdown, Arc::clone(&stop)).await;

    // Leaving a live socket behind would make the next `start` look like a running daemon.
    let _ = std::fs::remove_file(&paths.socket);
    result
}

async fn accept_loop(
    listener: &UnixListener,
    max_message_bytes: usize,
    shutdown: impl std::future::Future<Output = ()> + Send,
    stop: Arc<Notify>,
) -> std::io::Result<()> {
    tokio::pin!(shutdown);
    loop {
        tokio::select! {
            () = &mut shutdown => return Ok(()),
            () = stop.notified() => return Ok(()),
            accepted = listener.accept() => {
                let (stream, _) = accepted?;
                // One task per connection: a slow adapter must not block the others, because
                // this daemon is shared across every agent on the machine.
                let stop = Arc::clone(&stop);
                tokio::spawn(async move {
                    if let Err(err) = handle_connection(stream, max_message_bytes, stop).await {
                        eprintln!("library-enrichmentd: connection ended: {err}");
                    }
                });
            }
        }
    }
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
    max_message_bytes: usize,
    stop: Arc<Notify>,
) -> std::io::Result<()> {
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

        let dispatched = dispatch(&frame);
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
#[must_use]
pub fn dispatch(frame: &str) -> Dispatched {
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

    // A notification is a request with no id; it is still dispatched, but answered with nothing.
    let is_notification = request.id.is_none();
    let mut shutdown = false;
    let response = match request.method.as_str() {
        "daemon.shutdown" => {
            shutdown = true;
            Response::ok(request.id, serde_json::json!({ "stopping": true }))
        }
        // Returns a COMPLETE wire envelope, built here. Identity, coverage and freshness are
        // evidence-model assertions and belong to the core (§1.1); the adapter forwards this
        // rather than composing its own, so there is one author of those fields.
        "service.status" => {
            let filter = request.params.get("component").and_then(|c| c.as_str());
            Response::ok(
                request.id,
                serde_json::to_value(status::status_envelope(filter))
                    .expect("an Envelope always serializes"),
            )
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
        unknown => Response::err(
            request.id,
            RpcError::new(
                codes::METHOD_NOT_FOUND,
                format!("unknown method `{unknown}`"),
                "UNSUPPORTED_CAPABILITY",
                "Call service.status to see which components this build implements.",
            ),
        ),
    };

    Dispatched {
        response: (!is_notification).then_some(response),
        shutdown,
    }
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
    use super::*;

    fn dispatch_str(frame: &str) -> Response {
        dispatch(frame)
            .response
            .expect("this frame expects a response")
    }

    #[test]
    fn service_status_is_answered_without_a_store() {
        let response = dispatch_str(r#"{"jsonrpc":"2.0","id":1,"method":"service.status"}"#);
        assert!(response.error.is_none());
        let envelope = response.result.expect("a status envelope");
        assert_eq!(envelope["status"], "ok");
        assert_eq!(
            envelope["data"]["health"]["cache_ready"],
            serde_json::json!(false)
        );
    }

    #[test]
    fn the_status_envelope_is_built_in_the_core() {
        // The fields the adapter used to author. If these ever come back empty, identity and
        // coverage have drifted back across the boundary §1.1 draws.
        let response = dispatch_str(r#"{"jsonrpc":"2.0","id":1,"method":"service.status"}"#);
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
        assert_eq!(envelope["schema_version"], "1.0");
    }

    #[test]
    fn an_unmatched_component_filter_is_partial_with_the_gap_named() {
        // An empty `ok` would be indistinguishable from "no such component exists", which is a
        // much stronger claim than "the filter matched nothing".
        let frame = r#"{"jsonrpc":"2.0","id":1,"method":"service.status",
                        "params":{"component":"no-such-producer"}}"#;
        let envelope = dispatch_str(frame).result.expect("an envelope");

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

    #[test]
    fn a_matched_component_filter_narrows_the_report() {
        let frame = r#"{"jsonrpc":"2.0","id":1,"method":"service.status",
                        "params":{"component":"griffe"}}"#;
        let envelope = dispatch_str(frame).result.expect("an envelope");

        assert_eq!(envelope["status"], "ok");
        let producers = envelope["data"]["producers"].as_array().expect("producers");
        assert_eq!(producers.len(), 1);
        assert_eq!(producers[0]["name"], "griffe");
    }

    #[test]
    fn an_unknown_method_is_a_typed_error() {
        let response = dispatch_str(r#"{"jsonrpc":"2.0","id":1,"method":"no.such.method"}"#);
        let error = response.error.expect("an error");
        assert_eq!(error.code, codes::METHOD_NOT_FOUND);
        assert_eq!(error.data.expect("data")["code"], "UNSUPPORTED_CAPABILITY");
    }

    #[test]
    fn malformed_json_is_a_typed_error_not_a_panic() {
        let error = dispatch_str("{not json").error.expect("an error");
        assert_eq!(error.code, codes::PARSE_ERROR);
        assert_eq!(error.data.expect("data")["code"], "UNSUPPORTED_FORMAT");
    }

    #[test]
    fn a_wrong_protocol_version_is_rejected() {
        let response = dispatch_str(r#"{"jsonrpc":"1.0","id":1,"method":"service.status"}"#);
        let error = response.error.expect("an error");
        assert_eq!(error.code, codes::INVALID_REQUEST);
    }

    #[test]
    fn an_unsupported_producer_format_is_a_typed_error_over_rpc() {
        // Gate R04 at the transport, not just in the library: the typed refusal has to survive
        // the crossing, or a caller sees a generic failure instead of a code it can branch on.
        let frame = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "producer.probe_format",
            "params": { "artifact": r#"{"format_version":53,"index":{}}"# },
        })
        .to_string();

        let error = dispatch_str(&frame).error.expect("an error");
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

    #[test]
    fn a_supported_producer_format_is_accepted_over_rpc() {
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

        let response = dispatch_str(&frame);
        assert!(response.error.is_none(), "{:?}", response.error);
        assert_eq!(response.result.expect("a probe")["supported"], true);
    }

    #[test]
    fn a_notification_gets_no_response() {
        assert!(
            dispatch(r#"{"jsonrpc":"2.0","method":"daemon.ping"}"#)
                .response
                .is_none()
        );
    }

    #[test]
    fn shutdown_is_acknowledged_before_the_daemon_stops() {
        let dispatched = dispatch(r#"{"jsonrpc":"2.0","id":1,"method":"daemon.shutdown"}"#);
        assert!(dispatched.shutdown, "the daemon must stop");
        assert!(
            dispatched.response.is_some(),
            "the caller must be told it is stopping"
        );
    }

    #[test]
    fn a_response_frame_is_exactly_one_line() {
        let response = dispatch_str(r#"{"jsonrpc":"2.0","id":1,"method":"service.status"}"#);
        let frame = rpc::write_frame(&response);
        assert_eq!(frame.matches('\n').count(), 1);
        assert!(frame.ends_with('\n'));
    }
}
