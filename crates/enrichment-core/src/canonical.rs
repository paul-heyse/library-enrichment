//! Canonical JSON and content digests.
//!
//! Every content-derived identity in this service (blueprint §3.1, §6.3) is a SHA-256 over a
//! canonical rendering of its defining fields: object keys sorted recursively, arrays kept in
//! order, compact separators. Two facts that are equal produce the same bytes regardless of the
//! map type serde_json was compiled with -- which matters here because DataFusion turns on
//! `serde_json/preserve_order` for the whole workspace under feature unification, swapping
//! `serde_json::Map` from a `BTreeMap` to an insertion-ordered `IndexMap`. Sorting explicitly
//! is what keeps an identity stable across `cargo run -p` and `cargo nextest run --workspace`.
//!
//! Wall-clock timestamps and temporary paths are never part of a canonical value (§6.3): they
//! belong in provenance, so a consumer can tell "the same facts retrieved later" from "changed
//! evidence".

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
    serde_json::to_string(&canonical).unwrap_or_default()
}

/// Lower-case hexadecimal SHA-256 of raw bytes.
#[must_use]
pub fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    hex(digest.as_slice())
}

/// SHA-256 of a value's canonical rendering.
#[must_use]
pub fn digest_hex(value: &Value) -> String {
    sha256_hex(to_canonical_string(value).as_bytes())
}

/// The number of hex digits kept in a short content identity.
pub const SHORT_ID_HEX_DIGITS: usize = 16;

/// `<prefix>_<first 16 hex digits of the canonical digest>`.
///
/// Sixty-four bits is ample for identities that are always paired with the full record they
/// name, and short enough to read in a tool result.
#[must_use]
pub fn short_id(prefix: &str, value: &Value) -> String {
    let digest = digest_hex(value);
    format!("{prefix}_{}", &digest[..SHORT_ID_HEX_DIGITS])
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
    fn array_order_is_significant() {
        assert_ne!(digest_hex(&json!([1, 2])), digest_hex(&json!([2, 1])));
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

    #[test]
    fn a_short_id_carries_its_prefix_and_sixteen_hex_digits() {
        let id = short_id("rel", &json!({"x": 1}));
        assert!(id.starts_with("rel_"));
        assert_eq!(id.len(), 4 + SHORT_ID_HEX_DIGITS);
        assert!(id[4..].chars().all(|c| c.is_ascii_hexdigit()));
    }
}
