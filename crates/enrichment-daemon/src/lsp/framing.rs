//! LSP's `Content-Length` framing, over a container's stdin and stdout.
//!
//! Deliberately not the daemon's own transport. `crate::rpc` speaks newline-delimited JSON-RPC
//! because that is what ADR 0006 chose for our socket; a language server speaks header-framed
//! JSON-RPC because that is what the protocol specifies. Keeping the two in separate modules
//! means neither can quietly start accepting the other's frames.
//!
//! Every read is bounded. A server that announces a gigabyte, or never finishes a header, gets
//! an error rather than this process's memory.

use std::io;

use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};

/// The largest message this client will read. Generous for a diagnostics reply, far short of
/// anything that would threaten the daemon.
pub const MAX_MESSAGE_BYTES: usize = 16 * 1024 * 1024;
/// The most header lines to read before giving up on a malformed stream.
const MAX_HEADER_LINES: usize = 32;
const MAX_HEADER_LINE_BYTES: usize = 8192;

/// Write one framed JSON-RPC message.
///
/// # Errors
///
/// Fails if the message cannot be serialized or the pipe is closed.
pub async fn write(
    sink: &mut (impl AsyncWriteExt + Unpin),
    message: &serde_json::Value,
) -> io::Result<()> {
    let body = serde_json::to_vec(message)?;
    if body.len() > MAX_MESSAGE_BYTES {
        return Err(io::Error::other(
            "language-server message exceeds the write bound",
        ));
    }
    sink.write_all(format!("Content-Length: {}\r\n\r\n", body.len()).as_bytes())
        .await?;
    sink.write_all(&body).await?;
    sink.flush().await
}

/// Read one framed JSON-RPC message.
///
/// # Errors
///
/// Fails on a closed pipe, a missing or malformed `Content-Length`, a body over
/// [`MAX_MESSAGE_BYTES`], or a body that is not JSON.
pub async fn read(
    source: &mut BufReader<impl AsyncReadExt + Unpin>,
) -> io::Result<serde_json::Value> {
    let mut length: Option<usize> = None;
    for _ in 0..MAX_HEADER_LINES {
        let mut bytes = Vec::new();
        let mut limited = (&mut *source).take(MAX_HEADER_LINE_BYTES as u64 + 1);
        if limited.read_until(b'\n', &mut bytes).await? == 0 {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "the language server closed its output before sending a complete header",
            ));
        }
        if bytes.len() > MAX_HEADER_LINE_BYTES {
            return Err(io::Error::other(
                "language-server header line exceeds the byte bound",
            ));
        }
        let line = std::str::from_utf8(&bytes).map_err(io::Error::other)?;
        let line = line.trim_end_matches(['\r', '\n']);
        if line.is_empty() {
            let Some(length) = length else {
                return Err(io::Error::other(
                    "a language server frame arrived without a Content-Length header",
                ));
            };
            if length > MAX_MESSAGE_BYTES {
                return Err(io::Error::other(format!(
                    "the language server announced a {length}-byte message, over the \
                     {MAX_MESSAGE_BYTES}-byte bound"
                )));
            }
            let mut body = vec![0u8; length];
            source.read_exact(&mut body).await?;
            return serde_json::from_slice(&body).map_err(io::Error::other);
        }
        if let Some(value) = line.strip_prefix("Content-Length:") {
            if length.is_some() {
                return Err(io::Error::other("duplicate Content-Length header"));
            }
            length = Some(
                value
                    .trim()
                    .parse()
                    .map_err(|_| io::Error::other("malformed Content-Length"))?,
            );
        }
        // Every other header, `Content-Type` included, is ignored: the protocol defines only
        // these two and we have no use for the second.
    }
    Err(io::Error::other(
        "a language server frame had more headers than this client will read",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn a_written_frame_reads_back_identically() {
        let mut buffer = Vec::new();
        let message = serde_json::json!({"jsonrpc":"2.0","id":1,"method":"initialize"});
        write(&mut buffer, &message).await.expect("write");
        assert!(buffer.starts_with(b"Content-Length: "));
        let mut source = BufReader::new(std::io::Cursor::new(buffer));
        assert_eq!(read(&mut source).await.expect("read"), message);
    }

    #[tokio::test]
    async fn a_frame_without_a_length_is_refused_rather_than_guessed() {
        let mut source =
            BufReader::new(std::io::Cursor::new(b"Content-Type: x\r\n\r\n{}".to_vec()));
        let err = read(&mut source).await.expect_err("no length");
        assert!(err.to_string().contains("Content-Length"), "{err}");
    }

    #[tokio::test]
    async fn an_oversized_announcement_is_refused_before_allocating() {
        let frame = format!("Content-Length: {}\r\n\r\n", MAX_MESSAGE_BYTES + 1);
        let mut source = BufReader::new(std::io::Cursor::new(frame.into_bytes()));
        let err = read(&mut source).await.expect_err("too large");
        assert!(err.to_string().contains("over the"), "{err}");
    }

    #[tokio::test]
    async fn an_unterminated_header_is_bounded_without_waiting_for_a_newline() {
        let (source, mut server) = tokio::io::duplex(MAX_HEADER_LINE_BYTES + 2);
        server
            .write_all(&vec![b'x'; MAX_HEADER_LINE_BYTES + 1])
            .await
            .expect("server header");
        let mut source = BufReader::new(source);
        let error = tokio::time::timeout(std::time::Duration::from_secs(1), read(&mut source))
            .await
            .expect("reader rejects before server closes or writes newline")
            .expect_err("oversized header");
        assert!(error.to_string().contains("header line"));
    }

    #[tokio::test]
    async fn a_truncated_stream_is_an_error_not_a_hang() {
        let mut source = BufReader::new(std::io::Cursor::new(b"Content-Length: 10\r\n".to_vec()));
        assert!(read(&mut source).await.is_err());
    }

    #[tokio::test]
    async fn unicode_survives_the_round_trip_because_the_length_is_in_bytes() {
        // The length header counts bytes, not characters. Getting this wrong desynchronises the
        // stream at the first non-ASCII docstring, which is exactly the kind of bug that only
        // shows up against a real library.
        let mut buffer = Vec::new();
        let message = serde_json::json!({"text": "Grüße 🌍 from the capsule"});
        write(&mut buffer, &message).await.expect("write");
        let mut source = BufReader::new(std::io::Cursor::new(buffer));
        assert_eq!(read(&mut source).await.expect("read"), message);
    }
}
