//! Byte-content digests and deterministic JSON encoding for external artifacts.
//! Semantic identities are defined in native_key and native_identity, never in this module.

use serde_json::{Map, Value};
use sha2::{Digest, Sha256};

/// Sort every object's keys, recursively. Arrays keep their order.
#[must_use]
pub fn canonicalize(value: Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut entries: Vec<(String, Value)> = map
                .into_iter()
                .map(|(key, inner)| (key, canonicalize(inner)))
                .collect();
            entries.sort_by(|a, b| a.0.cmp(&b.0));
            let mut sorted = Map::new();
            for (key, inner) in entries {
                sorted.insert(key, inner);
            }
            Value::Object(sorted)
        }
        Value::Array(items) => Value::Array(items.into_iter().map(canonicalize).collect()),
        other => other,
    }
}

/// The compact canonical rendering of a value.
#[must_use]
pub fn to_canonical_string(value: &Value) -> String {
    let canonical = canonicalize(value.clone());
    // A `Value` always serializes; the only failure mode is a non-string map key, which
    // `serde_json::Value` cannot represent.
    canonical.to_string()
}

/// Lower-case hexadecimal SHA-256 of raw bytes.
#[must_use]
pub fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    hex(digest.as_slice())
}

/// Hash a bounded byte stream without allocating the complete file.
///
/// # Errors
/// Fails on I/O error or when the stream exceeds `limit` bytes.
pub fn sha256_reader(mut source: impl std::io::Read, limit: u64) -> std::io::Result<(String, u64)> {
    let mut digest = Sha256::new();
    let mut size = 0u64;
    let mut buffer = [0u8; 65536];
    loop {
        let count = source.read(&mut buffer)?;
        if count == 0 {
            return Ok((hex(digest.finalize().as_slice()), size));
        }
        size = size
            .checked_add(count as u64)
            .filter(|size| *size <= limit)
            .ok_or_else(|| std::io::Error::other("content exceeds the byte bound"))?;
        digest.update(&buffer[..count]);
    }
}

/// Deserialize exactly the bounded bytes whose digest is checked, without reopening the
/// pathname or allocating a complete byte copy. No value escapes on a digest/length mismatch.
/// # Errors
/// Invalid JSON, changed content, extra bytes and I/O failure are explicit errors.
pub fn verified_json<T: serde::de::DeserializeOwned>(
    source: impl std::io::Read,
    expected_digest: &str,
    expected_bytes: u64,
) -> std::io::Result<T> {
    verified_read(source, expected_digest, expected_bytes, |reader| {
        serde_json::from_reader(reader).map_err(Into::into)
    })
}

/// Consume exactly a content-addressed stream; injected transport framing need not be hashed.
/// # Errors
/// The consumer must exhaust the stream; incomplete reads and identity mismatches are refused.
pub fn verified_read<T>(
    source: impl std::io::Read,
    expected_digest: &str,
    expected_bytes: u64,
    consume: impl FnOnce(&mut dyn std::io::Read) -> std::io::Result<T>,
) -> std::io::Result<T> {
    struct Reader<R> {
        source: R,
        digest: Sha256,
        bytes: u64,
        expected: u64,
    }
    impl<R: std::io::Read> std::io::Read for Reader<R> {
        fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
            let n = self.source.read(buffer)?;
            self.bytes = self
                .bytes
                .checked_add(n as u64)
                .filter(|n| *n <= self.expected)
                .ok_or_else(|| std::io::Error::other("JSON content exceeds its declared bound"))?;
            self.digest.update(&buffer[..n]);
            Ok(n)
        }
    }
    let mut reader = Reader {
        source,
        digest: Sha256::new(),
        bytes: 0,
        expected: expected_bytes,
    };
    let mut buffered = std::io::BufReader::with_capacity(64 * 1024, &mut reader);
    let value = consume(&mut buffered)?;
    if std::io::BufRead::fill_buf(&mut buffered)?.is_empty() {
        drop(buffered);
    } else {
        return Err(std::io::Error::other(
            "verified consumer did not exhaust its input",
        ));
    }
    if reader.bytes != expected_bytes || hex(reader.digest.finalize().as_slice()) != expected_digest
    {
        return Err(std::io::Error::other(
            "JSON content differs from retained identity",
        ));
    }
    Ok(value)
}

/// Count serialized bytes without allocating a JSON value or byte buffer.
/// # Errors
/// The counter stops at the bound; serialization errors stay explicit.
pub fn serialized_size(value: &impl serde::Serialize, limit: usize) -> std::io::Result<usize> {
    struct Counter {
        bytes: usize,
        limit: usize,
    }
    impl std::io::Write for Counter {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            self.bytes = self
                .bytes
                .checked_add(bytes.len())
                .filter(|n| *n <= self.limit)
                .ok_or_else(|| {
                    std::io::Error::new(
                        std::io::ErrorKind::OutOfMemory,
                        "serialized value exceeds byte bound",
                    )
                })?;
            Ok(bytes.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let mut counter = Counter { bytes: 0, limit };
    serde_json::to_writer(&mut counter, value)?;
    Ok(counter.bytes)
}

fn hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(DIGITS[usize::from(byte >> 4)] as char);
        out.push(DIGITS[usize::from(byte & 0x0f)] as char);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn key_order_does_not_change_the_rendering() {
        let a = json!({"b": 1, "a": {"d": [1, 2], "c": null}});
        let b = json!({"a": {"c": null, "d": [1, 2]}, "b": 1});
        assert_eq!(to_canonical_string(&a), to_canonical_string(&b));
        assert_eq!(
            to_canonical_string(&a),
            r#"{"a":{"c":null,"d":[1,2]},"b":1}"#
        );
    }

    #[test]
    fn sha256_matches_the_known_vector() {
        // SHA-256("") from FIPS 180-4.
        assert_eq!(
            sha256_hex(b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(
            sha256_hex(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }
}
