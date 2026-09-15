//! Acceptance gate C19, RPC leg: a malformed or oversized request is rejected with a typed
//! error at the daemon's own boundary, not only at the MCP boundary above it.
//!
//! These drive a **real daemon over a real Unix socket** -- no mocked transport. Blueprint
//! §14.1 and `.claude/rules/no-mocks-in-acceptance-tiers.yml` both require that of the
//! integration tier: the point is to test the boundary, and a mocked boundary tests nothing.
//!
//! Every socket lives under a `tempfile` directory, so nothing here touches `$LIBENR_HOME` or
//! the real XDG paths. `just state-leak-check` proves that independently.

use std::sync::Arc;
use std::time::Duration;

use enrichment_core::config::Config;
use enrichment_daemon::paths::DaemonPaths;
use enrichment_daemon::rpc::DEFAULT_MAX_MESSAGE_BYTES;
use enrichment_daemon::server;
use enrichment_daemon::service::Service;
use enrichment_store::StatePaths;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;

/// A service over explicit roots under `dir`: the service never reads `LIBENR_*` itself.
fn open_service(dir: &std::path::Path, limit: usize) -> Arc<Service> {
    let mut config = Config::default();
    config.limits.rpc_message_bytes = limit;
    let state = StatePaths::explicit(dir.join("cache"), dir.join("data"));
    Arc::new(Service::open(config, state).expect("service opens"))
}

/// A daemon on a private socket, shut down when the guard is dropped.
struct TestDaemon {
    paths: DaemonPaths,
    stop: tokio::sync::oneshot::Sender<()>,
    handle: tokio::task::JoinHandle<std::io::Result<()>>,
    _dir: tempfile::TempDir,
}

impl TestDaemon {
    async fn start() -> Self {
        Self::start_with_limit(DEFAULT_MAX_MESSAGE_BYTES).await
    }

    async fn start_with_limit(limit: usize) -> Self {
        let dir = tempfile::tempdir().expect("a temp dir for the socket and state");
        let paths = DaemonPaths::under(dir.path().join("run"));
        let (stop, stopped) = tokio::sync::oneshot::channel();

        let service = open_service(dir.path(), limit);

        let serving = paths.clone();
        let handle = tokio::spawn(async move {
            server::serve(service, &serving, async {
                let _ = stopped.await;
            })
            .await
        });

        // Wait for the bind rather than sleeping a fixed interval.
        for _ in 0..200 {
            if UnixStream::connect(&paths.socket).await.is_ok() {
                return Self {
                    paths,
                    stop,
                    handle,
                    _dir: dir,
                };
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        panic!("daemon did not bind {} in time", paths.socket.display());
    }

    /// Send one frame and read one frame back.
    async fn exchange(&self, frame: &str) -> serde_json::Value {
        let stream = UnixStream::connect(&self.paths.socket)
            .await
            .expect("the daemon is listening");
        let (read_half, mut write_half) = stream.into_split();
        let mut reader = BufReader::new(read_half);

        write_half
            .write_all(format!("{frame}\n").as_bytes())
            .await
            .expect("write succeeds");
        write_half.flush().await.expect("flush succeeds");

        let mut line = String::new();
        reader.read_line(&mut line).await.expect("a response frame");
        assert!(line.ends_with('\n'), "every frame is newline-terminated");
        serde_json::from_str(&line).expect("the response is valid JSON")
    }

    async fn shutdown(self) {
        let _ = self.stop.send(());
        let _ = tokio::time::timeout(Duration::from_secs(5), self.handle).await;
    }
}

#[tokio::test]
async fn service_status_round_trips_over_a_real_socket() {
    let daemon = TestDaemon::start().await;
    let response = daemon
        .exchange(r#"{"jsonrpc":"2.0","id":1,"method":"service.status"}"#)
        .await;

    assert_eq!(response["jsonrpc"], "2.0");
    assert_eq!(response["id"], 1);
    assert!(response.get("error").is_none(), "status must succeed");

    // `result` is a complete wire envelope now, built in the core -- identity, coverage and
    // freshness included -- so the status payload sits under `data`.
    assert_eq!(response["result"]["status"], "ok");
    assert!(
        response["result"]["request_id"]
            .as_str()
            .is_some_and(|id| id.starts_with("req_")),
        "the core mints the request identity"
    );

    // The Phase 0 gate, still in force: absent components are reported as absent, truthfully.
    // With an open store the registry producer is the one that is genuinely available; every
    // other one still names why it is not.
    let producers = response["result"]["data"]["producers"]
        .as_array()
        .expect("producers are listed");
    assert!(!producers.is_empty());
    let mut available = Vec::new();
    for producer in producers {
        if producer["available"] == true {
            available.push(producer["name"].as_str().unwrap_or_default().to_owned());
        } else {
            assert!(
                !producer["detail"].as_str().unwrap_or_default().is_empty(),
                "an absent component must name why it is absent"
            );
        }
    }
    // Pinned rather than checked for membership, so a producer becoming available is a
    // deliberate edit here. The list grows as a phase lands: it was `["crates-io-registry"]`
    // at phase 0, and the phase-1 slice added `rustdoc-json`.
    available.sort();
    assert_eq!(
        available,
        vec![
            "crates-io-registry".to_owned(),
            "pypi-registry".to_owned(),
            "rustdoc-json".to_owned()
        ]
    );
    assert_eq!(response["result"]["data"]["health"]["cache_ready"], true);

    daemon.shutdown().await;
}

#[tokio::test]
async fn the_status_envelope_conforms_to_the_wire_contract() {
    // The daemon now emits envelopes, so what crosses the socket must satisfy the same
    // contract every other boundary enforces -- checked with the shared validator rather than
    // by eye.
    let daemon = TestDaemon::start().await;
    let response = daemon
        .exchange(r#"{"jsonrpc":"2.0","id":1,"method":"service.status"}"#)
        .await;

    let envelope = response["result"].to_string();
    let verdict = enrichment_daemon::validate::validate(&envelope);
    assert!(
        verdict.valid,
        "the daemon emitted a non-conforming envelope: {:?}",
        verdict.detail
    );

    daemon.shutdown().await;
}

#[tokio::test]
async fn malformed_json_is_a_typed_error_not_a_dropped_connection() {
    let daemon = TestDaemon::start().await;
    let response = daemon.exchange("{ this is not json").await;

    assert_eq!(response["error"]["data"]["code"], "UNSUPPORTED_FORMAT");
    assert!(
        !response["error"]["data"]["next_action"]
            .as_str()
            .unwrap_or_default()
            .is_empty(),
        "a typed error carries a concrete next action (blueprint §7.2)"
    );

    daemon.shutdown().await;
}

#[tokio::test]
async fn an_unknown_method_is_a_typed_error() {
    let daemon = TestDaemon::start().await;
    let response = daemon
        .exchange(r#"{"jsonrpc":"2.0","id":7,"method":"library.nonexistent"}"#)
        .await;

    assert_eq!(response["id"], 7, "the error correlates with the request");
    assert_eq!(response["error"]["data"]["code"], "UNSUPPORTED_CAPABILITY");

    daemon.shutdown().await;
}

#[tokio::test]
async fn a_wrong_protocol_version_is_rejected() {
    let daemon = TestDaemon::start().await;
    let response = daemon
        .exchange(r#"{"jsonrpc":"1.0","id":2,"method":"service.status"}"#)
        .await;

    assert_eq!(response["error"]["data"]["code"], "UNSUPPORTED_FORMAT");

    daemon.shutdown().await;
}

#[tokio::test]
async fn an_oversized_frame_is_rejected_without_being_buffered() {
    // Blueprint §2.1: "bounded newline-delimited JSON-RPC 2.0 ... with an explicit message size
    // limit. Large payloads travel by content-addressed artifact handles, not inline."
    let daemon = TestDaemon::start_with_limit(512).await;
    let padding = "x".repeat(4096);
    let frame = format!(
        r#"{{"jsonrpc":"2.0","id":3,"method":"service.status","params":{{"pad":"{padding}"}}}}"#
    );
    let response = daemon.exchange(&frame).await;

    assert_eq!(response["error"]["data"]["code"], "BUDGET_EXCEEDED");
    assert!(
        response["error"]["message"]
            .as_str()
            .unwrap_or_default()
            .contains("512"),
        "the error names the limit that was exceeded"
    );

    daemon.shutdown().await;
}

#[tokio::test]
async fn the_connection_survives_a_rejected_frame() {
    // A bad frame must not poison the stream: the oversized bytes are consumed, so the next
    // request on the same connection is answered normally.
    let daemon = TestDaemon::start_with_limit(512).await;
    let stream = UnixStream::connect(&daemon.paths.socket)
        .await
        .expect("the daemon is listening");
    let (read_half, mut write_half) = stream.into_split();
    let mut reader = BufReader::new(read_half);

    let padding = "x".repeat(4096);
    write_half
        .write_all(format!("{padding}\n").as_bytes())
        .await
        .expect("write succeeds");
    write_half
        .write_all(b"{\"jsonrpc\":\"2.0\",\"id\":9,\"method\":\"daemon.ping\"}\n")
        .await
        .expect("write succeeds");
    write_half.flush().await.expect("flush succeeds");

    let mut rejected = String::new();
    reader.read_line(&mut rejected).await.expect("a rejection");
    let rejected: serde_json::Value = serde_json::from_str(&rejected).expect("valid JSON");
    assert_eq!(rejected["error"]["data"]["code"], "BUDGET_EXCEEDED");

    let mut second = String::new();
    reader
        .read_line(&mut second)
        .await
        .expect("a second answer");
    let second: serde_json::Value = serde_json::from_str(&second).expect("valid JSON");
    assert_eq!(second["id"], 9, "the connection recovered");
    assert_eq!(second["result"]["pong"], true);

    daemon.shutdown().await;
}

#[tokio::test]
async fn a_second_daemon_refuses_to_steal_a_live_socket() {
    // Two daemons on one socket would break the single-writer invariant (blueprint §2.1).
    // Two independent guards enforce it and both are checked here, because they fail at
    // different moments and a caller sees different errors.
    let daemon = TestDaemon::start().await;

    // Same state roots: refused by the exclusive writer lock, before anything is served. This
    // is the earlier and stronger guard -- recovery must never race a live writer.
    let err = Service::open(
        Config::default(),
        StatePaths::explicit(
            daemon._dir.path().join("cache"),
            daemon._dir.path().join("data"),
        ),
    )
    .expect_err("a second writer over the same state roots must be refused");
    assert!(
        err.to_string()
            .contains("another writer owns this data root"),
        "{err}"
    );

    // State elsewhere, so the locks say nothing: the live socket must still refuse the bind.
    let elsewhere = tempfile::tempdir().expect("a temp dir");
    let second = open_service(elsewhere.path(), DEFAULT_MAX_MESSAGE_BYTES);
    let err = server::serve(second, &daemon.paths, std::future::pending())
        .await
        .expect_err("the second bind must fail");
    assert_eq!(err.kind(), std::io::ErrorKind::AddrInUse);

    daemon.shutdown().await;
}

#[tokio::test]
async fn a_stale_socket_from_a_dead_daemon_is_reclaimed() {
    let dir = tempfile::tempdir().expect("a temp dir");
    let paths = DaemonPaths::under(dir.path().to_path_buf());
    paths.ensure_dir().expect("the dir is creatable");

    // A leftover file with nothing listening: what a crash leaves behind.
    std::fs::write(&paths.socket, b"").expect("the stale file is writable");

    let (stop, stopped) = tokio::sync::oneshot::channel();
    let service = open_service(dir.path(), DEFAULT_MAX_MESSAGE_BYTES);
    let serving = paths.clone();
    let handle = tokio::spawn(async move {
        server::serve(service, &serving, async {
            let _ = stopped.await;
        })
        .await
    });

    let mut connected = false;
    for _ in 0..200 {
        if UnixStream::connect(&paths.socket).await.is_ok() {
            connected = true;
            break;
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    assert!(connected, "a stale socket file must not block startup");

    let _ = stop.send(());
    let _ = tokio::time::timeout(Duration::from_secs(5), handle).await;
}

#[tokio::test]
async fn shutdown_is_acknowledged_and_removes_the_socket() {
    let daemon = TestDaemon::start().await;
    let socket = daemon.paths.socket.clone();

    let response = daemon
        .exchange(r#"{"jsonrpc":"2.0","id":4,"method":"daemon.shutdown"}"#)
        .await;
    assert_eq!(response["result"]["stopping"], true);

    let _ = tokio::time::timeout(Duration::from_secs(5), daemon.handle).await;
    assert!(
        !socket.exists(),
        "a stopped daemon must not leave a socket that looks live to the next start"
    );
}
