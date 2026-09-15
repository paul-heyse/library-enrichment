//! One conversation with one language server.
//!
//! Owns framing, the initialize handshake, capability and position-encoding negotiation,
//! document versions, requests with cancellation, and shutdown. Blueprint §9.2 requires all of
//! it, and requires one thing more that is easy to lose: **an empty result, an unsupported
//! method, an unresolved dependency and incomplete indexing are four distinct outcomes.** They
//! are four variants of [`Outcome`] here, and nothing collapses them into an empty list.

use std::io;
use std::sync::{Arc, atomic::AtomicBool};
use std::time::Duration;

use serde_json::{Value, json};
use tokio::io::BufReader;
use tokio::process::{ChildStdin, ChildStdout};

use super::framing;
use super::settings::Server;
use crate::execution::ServedSession;

/// One mapped source location, as the server reported it.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Location {
    /// The document URI the server named.
    pub uri: String,
    /// Zero-based start line.
    pub line: u32,
    /// Start character, in the negotiated encoding.
    pub character: u32,
    /// Zero-based end line.
    pub end_line: u32,
    /// End character, in the negotiated encoding.
    pub end_character: u32,
}

impl Location {
    fn from_value(value: &Value) -> Option<Self> {
        // A server may answer with `Location`, `LocationLink` or a list of either. Both shapes
        // are read here rather than requiring one, because both were observed.
        let (uri, range) = if let Some(uri) = value.get("uri") {
            (uri, value.get("range")?)
        } else {
            (
                value.get("targetUri")?,
                value
                    .get("targetSelectionRange")
                    .or_else(|| value.get("targetRange"))?,
            )
        };
        Some(Self {
            uri: uri.as_str()?.to_owned(),
            line: u32::try_from(range["start"]["line"].as_u64()?).ok()?,
            character: u32::try_from(range["start"]["character"].as_u64()?).ok()?,
            end_line: u32::try_from(range["end"]["line"].as_u64()?).ok()?,
            end_character: u32::try_from(range["end"]["character"].as_u64()?).ok()?,
        })
    }

    /// Read every location out of a result value, whatever shape it arrived in.
    pub fn all_from(value: &Value) -> io::Result<Vec<Self>> {
        fn decode(value: &Value) -> io::Result<Location> {
            let location = Location::from_value(value).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "invalid LSP location")
            })?;
            if (location.end_line, location.end_character) < (location.line, location.character) {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "reversed LSP range",
                ));
            }
            Ok(location)
        }
        match value {
            Value::Array(items) => items.iter().map(decode).collect(),
            Value::Null => Ok(Vec::new()),
            single => Ok(vec![decode(single)?]),
        }
    }
}

/// Input closure used by every query in one immutable warm session.
pub struct Scope {
    pub environment: enrichment_core::identity::Environment,
    pub lock: Vec<u8>,
    pub inputs: Vec<enrichment_core::evidence::Artifact>,
}

/// A live, initialized language server session.
pub struct Session {
    session: ServedSession,
    stdout: BufReader<ChildStdout>,
    stdin: ChildStdin,
    next_id: i64,
    /// Monotonic per-document version, as the protocol requires.
    version: i32,
    /// What the server advertised at initialize.
    capabilities: Value,
    /// The encoding the server chose. Never assumed.
    pub position_encoding: String,
    /// Which server this is.
    pub server: Server,
    /// Its self-reported name and version, from `initialize`.
    pub server_version: String,
    /// The open document's URI, when one is open.
    open: Option<String>,
    notifications: super::notifications::Notifications,
    cancel: Arc<AtomicBool>,
    /// Immutable prepared scope, retained across warm queries.
    pub environment: enrichment_core::identity::Environment,
    pub lock: Vec<u8>,
    pub inputs: Vec<enrichment_core::evidence::Artifact>,
}

impl Session {
    pub fn inputs_root(&self) -> &std::path::Path {
        &self.session.inputs_root
    }
    /// Start a server in `served` and complete the initialize handshake.
    ///
    /// # Errors
    ///
    /// Fails on a framing error, a server that never answers `initialize` within the deadline,
    /// or a server that rejects our options. A failure carries the server's own stderr, which is
    /// where both of these servers explain themselves.
    pub async fn initialize(
        server: Server,
        mut served: ServedSession,
        deadline: Duration,
        cancel: Arc<AtomicBool>,
        scope: Scope,
    ) -> io::Result<Self> {
        let taken = || io::Error::other("this served session was already taken over");
        let stdin = served.stdin.take().ok_or_else(taken)?;
        let stdout = BufReader::new(served.stdout.take().ok_or_else(taken)?);
        let mut session = Self {
            session: served,
            stdout,
            stdin,
            next_id: 0,
            version: 0,
            capabilities: Value::Null,
            position_encoding: "utf-16".to_owned(),
            server,
            server_version: String::new(),
            open: None,
            notifications: Default::default(),
            cancel,
            environment: scope.environment,
            lock: scope.lock,
            inputs: scope.inputs,
        };

        let result = match session
            .request(
                "initialize",
                json!({
                    "processId": Value::Null,
                    "clientInfo": {"name": "library-enrichment", "version": env!("CARGO_PKG_VERSION")},
                    "rootUri": "file:///capsule",
                    "workspaceFolders": [{"uri": "file:///capsule", "name": "capsule"}],
                    "capabilities": super::settings::client_capabilities(),
                    "initializationOptions": server.initialization_options(),
                }),
                deadline,
            )
            .await
        {
            Ok(result) => result,
            Err(err) => {
                // Both servers explain themselves on stderr. Without it, "the server closed its
                // output" is a true sentence that helps nobody.
                let log = session.diagnostics();
                let kind = err.kind();
                session.stop().await?;
                return Err(io::Error::new(kind, format!(
                    "{} did not initialize: {err}. Server log: {}",
                    server.name(),
                    if log.is_empty() { "(silent)" } else { log.trim() }
                )));
            }
        };

        session.capabilities = result.get("capabilities").cloned().unwrap_or(Value::Null);
        // Respect the negotiated encoding rather than assuming byte offsets match UTF-16.
        if let Some(encoding) = session
            .capabilities
            .get("positionEncoding")
            .and_then(Value::as_str)
        {
            if !matches!(encoding, "utf-8" | "utf-16") {
                let message = format!("server selected an unoffered position encoding: {encoding}");
                session.stop().await?;
                return Err(io::Error::new(io::ErrorKind::InvalidData, message));
            }
            session.position_encoding = encoding.to_owned();
        }
        if let Some(info) = result.get("serverInfo") {
            session.server_version = format!(
                "{} {}",
                info.get("name")
                    .and_then(Value::as_str)
                    .unwrap_or(server.name()),
                info.get("version")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown")
            );
        } else {
            session.server_version = server.name().to_owned();
        }
        session.notify("initialized", json!({})).await?;
        Ok(session)
    }

    pub fn set_cancellation(&mut self, cancel: Arc<AtomicBool>) {
        self.cancel = cancel;
    }

    pub fn document_version(&self) -> i32 {
        self.version
    }

    /// The server's last 64 KiB of stderr, for a failure that needs explaining.
    #[must_use]
    pub fn diagnostics(&self) -> String {
        self.session
            .diagnostics
            .lock()
            .map(|buffer| String::from_utf8_lossy(&buffer).into_owned())
            .unwrap_or_default()
    }

    /// The image this session runs in.
    #[must_use]
    pub fn image_id(&self) -> &str {
        &self.session.image_id
    }

    /// Whether the server advertised a capability, by its `initialize` key.
    #[must_use]
    pub fn advertises(&self, capability: &str) -> bool {
        match self.capabilities.get(capability) {
            None | Some(Value::Null) | Some(Value::Bool(false)) => false,
            Some(_) => true,
        }
    }

    /// Open a document, replacing any previously open one.
    ///
    /// # Errors
    ///
    /// Fails if the notification cannot be written.
    pub async fn open(&mut self, uri: &str, language: &str, text: &str) -> io::Result<()> {
        if let Some(previous) = self.open.take() {
            self.notify(
                "textDocument/didClose",
                json!({"textDocument": {"uri": previous}}),
            )
            .await?;
        }
        self.version = self
            .version
            .checked_add(1)
            .ok_or_else(|| io::Error::other("document version exhausted; discard session"))?;
        self.notifications.diagnostics.open(uri, self.version);
        self.notify(
            "textDocument/didOpen",
            json!({"textDocument": {
                "uri": uri,
                "languageId": language,
                "version": self.version,
                "text": text,
            }}),
        )
        .await?;
        self.open = Some(uri.to_owned());
        Ok(())
    }

    /// The pinned analyzer explicitly reports when workspace loading/indexing has settled.
    /// ty initializes its workspaces before dispatch; no invented ty readiness event is used.
    pub async fn await_readiness(&mut self, deadline: Duration) -> io::Result<()> {
        if self.server == Server::RustAnalyzer && self.notifications.indexing_gap().is_some() {
            self.pump_until(deadline, |state| state.indexing_gap().is_none())
                .await?;
        }
        Ok(())
    }

    pub fn indexing_gap(&self) -> Option<String> {
        if self.server == Server::RustAnalyzer {
            self.notifications.indexing_gap()
        } else {
            None
        }
    }

    /// Waiting for notifications consumes no outstanding request; a timeout leaves an explicit
    /// missing observation. Cancellation discards the session through its manager.
    async fn pump_until(
        &mut self,
        deadline: Duration,
        done: impl Fn(&super::notifications::Notifications) -> bool,
    ) -> io::Result<()> {
        let conversation = async {
            while !done(&self.notifications) {
                let message = framing::read(&mut self.stdout).await?;
                self.notifications.accept(&message)?;
                if let (Some(_), Some(id)) = (message.get("method"), message.get("id")) {
                    framing::write(&mut self.stdin, &json!({"jsonrpc":"2.0","id":id,
                        "error":{"code":-32601,"message":"client does not advertise this request capability"}})).await?;
                }
            }
            Ok(())
        };
        tokio::select! {
            biased;
            () = super::cancelled(&self.cancel) => Err(io::Error::new(io::ErrorKind::Interrupted, "cancelled while awaiting document or indexing state")),
            result = tokio::time::timeout(deadline, conversation) => match result { Ok(result) => result, Err(_) => Ok(()) },
        }
    }

    /// Pull diagnostics for a document.
    ///
    /// # Errors
    ///
    /// Fails on a transport error.
    pub async fn diagnostic(&mut self, uri: &str, deadline: Duration) -> io::Result<Value> {
        if !self.advertises("diagnosticProvider") {
            self.pump_until(deadline.min(Duration::from_secs(2)), |state| {
                state.diagnostics.pushed(uri).is_some()
            })
            .await?;
            return Ok(self
                .notifications
                .diagnostics
                .pushed(uri)
                .unwrap_or_else(|| json!({"kind":"unsupported"})));
        }
        let mut params = json!({"textDocument": {"uri": uri}});
        if let Some(id) = self.notifications.diagnostics.previous_id() {
            params["previousResultId"] = json!(id);
        }
        let result = self
            .request("textDocument/diagnostic", params, deadline)
            .await?;
        self.notifications.diagnostics.pulled(uri, result)
    }

    /// Shut the server down politely, then remove its container.
    ///
    /// A server that ignores `shutdown` still loses its container: removal is the boundary, and
    /// the guard confirms absence either way.
    pub async fn stop(mut self) -> io::Result<()> {
        self.cancel = Arc::new(AtomicBool::new(false));
        let deadline = Duration::from_secs(2);
        let _ = self.request("shutdown", Value::Null, deadline).await;
        let _ = self.notify("exit", Value::Null).await;
        let _ = tokio::time::timeout(deadline, self.session.child.wait()).await;
        self.session.guard.settle().await
    }

    async fn notify(&mut self, method: &str, params: Value) -> io::Result<()> {
        tokio::time::timeout(
            Duration::from_secs(5),
            framing::write(
                &mut self.stdin,
                &json!({"jsonrpc": "2.0", "method": method, "params": params}),
            ),
        )
        .await
        .map_err(|_| {
            io::Error::new(
                io::ErrorKind::TimedOut,
                "language-server notification write timed out",
            )
        })?
    }

    /// Send a request and read until its answer arrives, within `deadline`.
    ///
    /// Server-to-client requests and notifications arrive interleaved with our answer. Requests
    /// get a minimal reply; document/version-qualified diagnostic notifications are retained.
    /// On timeout we send `$/cancelRequest`, because abandoning a request without cancelling it
    /// leaves the server working on an answer nobody will read.
    pub async fn request(
        &mut self,
        method: &str,
        params: Value,
        deadline: Duration,
    ) -> io::Result<Value> {
        self.next_id += 1;
        let id = self.next_id;
        exchange(
            &mut self.stdin,
            &mut self.stdout,
            &json!({"jsonrpc": "2.0", "id": id, "method": method, "params": params}),
            deadline,
            &mut self.notifications,
            &self.cancel,
        )
        .await
        .map_err(|error| {
            io::Error::new(
                error.kind(),
                format!(
                    "{} {method}: {error}. Server log: {}",
                    self.server.name(),
                    self.diagnostics()
                        .chars()
                        .rev()
                        .take(400)
                        .collect::<String>()
                ),
            )
        })
    }
}

/// Bound the entire exchange, including a blocked request write or client-response write.
/// A timed-out stream is discarded by the session manager; cancellation is only best effort.
async fn exchange(
    sink: &mut (impl tokio::io::AsyncWrite + Unpin),
    source: &mut BufReader<impl tokio::io::AsyncRead + Unpin>,
    request: &Value,
    deadline: Duration,
    notifications: &mut super::notifications::Notifications,
    cancel: &AtomicBool,
) -> io::Result<Value> {
    let id = request["id"].clone();
    let mut sent = false;
    let conversation = async {
        framing::write(sink, request).await?;
        sent = true;
        loop {
            let message = framing::read(source).await?;
            if message.get("method").is_some() {
                notifications.accept(&message)?;
                if let Some(other) = message.get("id") {
                    framing::write(sink, &json!({
                        "jsonrpc": "2.0", "id": other,
                        "error": {"code": -32601, "message": "this client implements no server-to-client requests"}
                    })).await?;
                }
            } else if message.get("id") == Some(&id) {
                if let Some(error) = message.get("error") {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("server refused request: {error}"),
                    ));
                }
                return Ok(message.get("result").cloned().unwrap_or(Value::Null));
            }
        }
    };
    let interrupted = tokio::select! {
        biased;
        () = super::cancelled(cancel) => Some(io::ErrorKind::Interrupted),
        result = tokio::time::timeout(deadline, conversation) => match result {
            Ok(result) => return result,
            Err(_) => Some(io::ErrorKind::TimedOut),
        },
    };
    match interrupted {
        None => unreachable!("every unsettled exchange has a reason"),
        Some(kind) => {
            if sent {
                // Never append another frame after a partially written request. Even a complete
                // request does not guarantee the server is still reading cancellation traffic.
                let _ = tokio::time::timeout(Duration::from_millis(100), framing::write(sink,
                    &json!({"jsonrpc": "2.0", "method": "$/cancelRequest", "params": {"id": id}}),
                )).await;
            }
            Err(io::Error::new(
                kind,
                "language-server exchange interrupted or timed out; session must be discarded",
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn a_nonreading_server_cannot_block_the_request_deadline() {
        let (mut sink, _nonreading_server) = tokio::io::duplex(8);
        let (source, _silent_server) = tokio::io::duplex(8);
        let mut source = BufReader::new(source);
        let result = tokio::time::timeout(
            Duration::from_secs(1),
            exchange(
                &mut sink,
                &mut source,
                &json!({"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {}}),
                Duration::from_millis(10),
                &mut Default::default(),
                &AtomicBool::new(false),
            ),
        )
        .await
        .expect("deadline includes the request write")
        .expect_err("server does not read");
        assert_eq!(result.kind(), io::ErrorKind::TimedOut);
    }

    #[tokio::test]
    async fn cancellation_and_server_reply_writes_are_also_bounded() {
        let (mut sink, mut server_reader) = tokio::io::duplex(8);
        let (source, mut server_writer) = tokio::io::duplex(2048);
        let mut source = BufReader::new(source);
        let server = tokio::spawn(async move {
            let mut reader = BufReader::new(&mut server_reader);
            let _request = framing::read(&mut reader)
                .await
                .expect("request is received");
            framing::write(
                &mut server_writer,
                &json!({
                    "jsonrpc": "2.0", "id": "server-question", "method": "unsupported",
                }),
            )
            .await
            .expect("server asks its question");
            // Keep both ends open while refusing to read the client's response or cancellation.
            std::future::pending::<()>().await;
        });
        let result = tokio::time::timeout(
            Duration::from_secs(1),
            exchange(
                &mut sink,
                &mut source,
                &json!({"jsonrpc": "2.0", "id": 1, "method": "shutdown"}),
                Duration::from_millis(20),
                &mut Default::default(),
                &AtomicBool::new(false),
            ),
        )
        .await
        .expect("deadline also bounds both follow-up writes")
        .expect_err("server stops reading");
        assert_eq!(result.kind(), io::ErrorKind::TimedOut);
        server.abort();
        let _ = server.await;
    }

    #[tokio::test]
    async fn an_explicit_cancellation_notifies_the_server_and_discards_the_exchange() {
        let (mut sink, reader) = tokio::io::duplex(2048);
        let (source, _writer) = tokio::io::duplex(2048);
        let mut source = BufReader::new(source);
        let cancel = Arc::new(AtomicBool::new(false));
        let trigger = cancel.clone();
        let server = tokio::spawn(async move {
            let mut reader = BufReader::new(reader);
            let request = framing::read(&mut reader).await.unwrap();
            trigger.store(true, std::sync::atomic::Ordering::Release);
            let cancellation = framing::read(&mut reader).await.unwrap();
            assert_eq!(cancellation["method"], "$/cancelRequest");
            assert_eq!(cancellation["params"]["id"], request["id"]);
        });
        let error = tokio::time::timeout(
            Duration::from_secs(1),
            exchange(
                &mut sink,
                &mut source,
                &json!({"jsonrpc":"2.0","id":7,"method":"textDocument/hover"}),
                Duration::from_secs(30),
                &mut Default::default(),
                &cancel,
            ),
        )
        .await
        .unwrap()
        .unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::Interrupted);
        server.await.unwrap();
    }

    #[test]
    fn a_location_link_and_a_plain_location_both_read() {
        let link = json!({
            "targetUri": "file:///capsule/consumer.py",
            "targetSelectionRange": {"start": {"line": 3, "character": 6},
                                     "end": {"line": 3, "character": 10}}
        });
        let plain = json!({
            "uri": "file:///capsule/consumer.py",
            "range": {"start": {"line": 3, "character": 6}, "end": {"line": 3, "character": 10}}
        });
        assert_eq!(
            Location::all_from(&link).unwrap(),
            Location::all_from(&plain).unwrap()
        );
        assert_eq!(Location::all_from(&link).unwrap()[0].line, 3);
    }

    #[test]
    fn an_array_a_single_value_and_null_are_three_different_answers() {
        let one = json!({"uri": "file:///x", "range": {"start": {"line": 0, "character": 0},
                                                        "end": {"line": 0, "character": 1}}});
        assert_eq!(
            Location::all_from(&json!([one.clone(), one.clone()]))
                .unwrap()
                .len(),
            2
        );
        assert_eq!(Location::all_from(&one).unwrap().len(), 1);
        assert!(Location::all_from(&Value::Null).unwrap().is_empty());
    }

    #[test]
    fn malformed_locations_are_never_dropped_or_wrapped_into_valid_coordinates() {
        let valid = json!({"uri":"file:///capsule/f.py", "range":{
            "start":{"line":0,"character":0}, "end":{"line":0,"character":1}}});
        assert!(Location::all_from(&json!([valid.clone(), {}])).is_err());
        let mut overflow = valid.clone();
        overflow["range"]["start"]["line"] = json!(u64::from(u32::MAX) + 1);
        assert!(Location::all_from(&overflow).is_err());
        let mut reversed = valid;
        reversed["range"]["start"]["character"] = json!(2);
        assert!(Location::all_from(&reversed).is_err());
    }
}
