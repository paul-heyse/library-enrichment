//! Bounded newline-delimited JSON-RPC 2.0 (blueprint §2.1).
//!
//! > "For v1, use bounded newline-delimited JSON-RPC 2.0, with embedded newlines JSON-escaped
//! > and an explicit message size limit. Large payloads travel by content-addressed artifact
//! > handles, not inline RPC messages. Do not confuse this internal protocol with MCP or LSP
//! > framing."
//!
//! So: one JSON-RPC 2.0 object per line. **Not** MCP's `Content-Length` header framing and
//! **not** LSP's -- a third thing. `serde_json` escapes embedded newlines by construction, so
//! `\n` is unambiguously a frame terminator.
//!
//! The size limit is not invented here: `config/service.example.toml` is frozen and already
//! specifies `limits.rpc_message_bytes = 1048576`. An over-long line is rejected without being
//! buffered -- a peer must not be able to exhaust daemon memory by withholding a newline.

use serde::{Deserialize, Serialize};
use tokio::io::{AsyncBufRead, AsyncBufReadExt};

/// The documented default from `config/service.example.toml` (`limits.rpc_message_bytes`).
///
/// The operative value comes from configuration -- `Config::limits.rpc_message_bytes`, which
/// `library-enrichmentd start` passes to [`crate::server::serve`]. This constant is the fallback
/// that `enrichment_core::config::Limits::default()` also carries, and
/// `config::tests::defaults_match_the_frozen_example` pins both to the frozen file.
pub const DEFAULT_MAX_MESSAGE_BYTES: usize = 1_048_576;

/// The JSON-RPC version string this protocol speaks.
pub const JSONRPC_VERSION: &str = "2.0";

/// A request frame.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Request {
    /// Always `"2.0"`.
    pub jsonrpc: String,
    /// The method name, e.g. `service.status`.
    pub method: String,
    /// Method parameters. Absent is equivalent to `{}`.
    #[serde(default)]
    pub params: serde_json::Value,
    /// Correlation id. `None` makes this a notification, which gets no response.
    #[serde(default)]
    pub id: Option<serde_json::Value>,
}

/// A response frame. Exactly one of `result`/`error` is present.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response {
    /// Always `"2.0"`.
    pub jsonrpc: String,
    /// The correlated request id, or `null` when the request could not be parsed.
    pub id: Option<serde_json::Value>,
    /// Present on success.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    /// Present on failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<RpcError>,
}

impl Response {
    /// A successful response.
    #[must_use]
    pub fn ok(id: Option<serde_json::Value>, result: serde_json::Value) -> Self {
        Self {
            jsonrpc: JSONRPC_VERSION.to_owned(),
            id,
            result: Some(result),
            error: None,
        }
    }

    /// A failed response.
    #[must_use]
    pub fn err(id: Option<serde_json::Value>, error: RpcError) -> Self {
        Self {
            jsonrpc: JSONRPC_VERSION.to_owned(),
            id,
            result: None,
            error: Some(error),
        }
    }
}

/// A JSON-RPC error object, carrying this service's own error code alongside the transport one.
///
/// `code` is the JSON-RPC number a generic client understands; `data.code` is the stable
/// service code from the frozen thirteen, which is what the adapter maps into the wire
/// envelope's `error.code`. Keeping both means the transport stays ordinary JSON-RPC while the
/// evidence contract stays the authority on what went wrong.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RpcError {
    /// Standard JSON-RPC error code.
    pub code: i32,
    /// Short description.
    pub message: String,
    /// `{"code": "<service error code>", "next_action": "..."}`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// Standard JSON-RPC 2.0 codes, plus the one application code this protocol adds.
pub mod codes {
    /// Invalid JSON was received.
    pub const PARSE_ERROR: i32 = -32700;
    /// The JSON is not a valid request object.
    pub const INVALID_REQUEST: i32 = -32600;
    /// The method does not exist.
    pub const METHOD_NOT_FOUND: i32 = -32601;
    /// Invalid method parameters.
    pub const INVALID_PARAMS: i32 = -32602;
    /// Internal daemon error.
    pub const INTERNAL_ERROR: i32 = -32603;
    /// Application code: the frame exceeded `rpc_message_bytes`.
    pub const MESSAGE_TOO_LARGE: i32 = -32000;
}

impl RpcError {
    /// Build an error carrying a stable service code and a concrete next action.
    ///
    /// Blueprint §7.2: retryability and a next action belong in the error record, not just a
    /// code. The `service_code` values come from the frozen thirteen.
    #[must_use]
    pub fn new(code: i32, message: impl Into<String>, service_code: &str, next: &str) -> Self {
        Self {
            code,
            message: message.into(),
            data: Some(serde_json::json!({ "code": service_code, "next_action": next })),
        }
    }
}

/// What went wrong while reading a frame.
#[derive(Debug, thiserror::Error)]
pub enum FrameError {
    /// The peer sent a line longer than the configured limit.
    #[error("message exceeds the {limit}-byte limit from limits.rpc_message_bytes")]
    TooLarge {
        /// The configured limit that was exceeded.
        limit: usize,
    },
    /// The underlying socket failed.
    #[error("rpc transport read failed: {0}")]
    Io(#[from] std::io::Error),
}

/// Read one newline-delimited frame, refusing to buffer more than `limit` bytes.
///
/// Returns `Ok(None)` at a clean end of stream.
///
/// This deliberately does not use `read_line`, which would grow its buffer without bound: a
/// peer that opens the socket and streams bytes with no newline would otherwise be an
/// out-of-memory vector against the daemon that owns every other agent's jobs.
pub async fn read_frame<R>(reader: &mut R, limit: usize) -> Result<Option<String>, FrameError>
where
    R: AsyncBufRead + Unpin,
{
    let mut buf = Vec::new();
    loop {
        let available = reader.fill_buf().await?;
        if available.is_empty() {
            return if buf.is_empty() {
                Ok(None)
            } else {
                // A final frame without a trailing newline is still a frame.
                Ok(Some(decode(buf)?))
            };
        }

        if let Some(index) = available.iter().position(|byte| *byte == b'\n') {
            if buf.len() + index > limit {
                // Consume the offending frame so the connection can carry on with the next one
                // rather than re-reading the same oversized bytes forever.
                reader.consume(index + 1);
                return Err(FrameError::TooLarge { limit });
            }
            buf.extend_from_slice(&available[..index]);
            reader.consume(index + 1);
            return Ok(Some(decode(buf)?));
        }

        let taken = available.len();
        if buf.len() + taken > limit {
            reader.consume(taken);
            return Err(FrameError::TooLarge { limit });
        }
        buf.extend_from_slice(available);
        reader.consume(taken);
    }
}

fn decode(buf: Vec<u8>) -> Result<String, FrameError> {
    String::from_utf8(buf).map_err(|err| {
        FrameError::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            err.to_string(),
        ))
    })
}

/// Serialize a response as one frame, newline included.
///
/// # Panics
///
/// Never in practice: `Response` is a plain data type with no map keys that can fail to
/// serialize.
#[must_use]
pub fn write_frame(response: &Response) -> String {
    let mut line = serde_json::to_string(response).expect("a Response always serializes");
    line.push('\n');
    line
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn read(input: &str, limit: usize) -> Result<Option<String>, FrameError> {
        let mut reader = tokio::io::BufReader::new(input.as_bytes());
        read_frame(&mut reader, limit).await
    }

    #[tokio::test]
    async fn a_frame_ends_at_the_newline() {
        let got = read("{\"a\":1}\n{\"b\":2}\n", 1024).await.expect("reads");
        assert_eq!(got.as_deref(), Some("{\"a\":1}"));
    }

    #[tokio::test]
    async fn an_empty_stream_is_a_clean_end() {
        assert_eq!(read("", 1024).await.expect("reads"), None);
    }

    #[tokio::test]
    async fn an_oversized_frame_is_rejected_rather_than_buffered() {
        let huge = format!("{}\n", "x".repeat(4096));
        let err = read(&huge, 64).await.expect_err("must be rejected");
        assert!(matches!(err, FrameError::TooLarge { limit: 64 }));
    }

    #[tokio::test]
    async fn an_unterminated_oversized_stream_is_rejected() {
        // No newline at all: the guard must fire on accumulated length, not on frame end.
        let huge = "x".repeat(4096);
        let err = read(&huge, 64).await.expect_err("must be rejected");
        assert!(matches!(err, FrameError::TooLarge { limit: 64 }));
    }

    #[test]
    fn embedded_newlines_are_escaped_so_the_delimiter_stays_unambiguous() {
        let response = Response::ok(
            Some(serde_json::json!(1)),
            serde_json::json!({ "summary": "line one\nline two" }),
        );
        let frame = write_frame(&response);
        assert_eq!(frame.matches('\n').count(), 1, "only the terminator");
        assert!(frame.contains("\\n"), "the payload newline is escaped");
    }
}
