//! Deterministic lexical search and pagination cursors (blueprint §7.1, §7.3).
//!
//! Exact qualified names outrank fuzzy matches: an exact path, then a path suffix, then a name
//! prefix, then tokens in the signature, then tokens in the documentation. Every hit carries
//! the factors that scored it, so a caller sees *why* something ranked where it did instead of
//! an unexplained number. No embeddings; nothing here needs a model.
//!
//! Near-duplicate re-exports are folded: hits are grouped by definition, the definition's own
//! path is preferred, and the other public paths are listed on the hit as `also_at`.
//!
//! A cursor binds the scope (snapshot), the query digest, the sort and the offset, plus a
//! checksum over all four. A cursor presented against a different query or snapshot is
//! refused with `INVALID_CURSOR` rather than producing an inconsistent page.

use crate::{native_key::Key, native_union::Rule};
pub mod page;
pub mod row_page;

/// The scoring table, in rank order. Exposed so a response can carry the legend.
pub const FACTORS: &[(&str, u32)] = &[
    ("exact_path", 1000),
    ("path_suffix", 800),
    ("name_exact", 700),
    ("name_prefix", 400),
    ("path_token", 200),
    ("signature_token", 120),
    ("summary_token", 80),
    ("docs_token", 40),
    ("subject_exact", 600),
    ("subject_token", 150),
    ("text_token", 50),
    ("definition_path", 5),
];
crate::native_struct! {
    /// Artifact window bound to immutable content and the exact requested selection.
    pub struct Cursor {
        scope: String => Rule::Reference(crate::native_union::Domain::Artifact),
        query_digest: String => Rule::NonEmpty,
        sort: String => Rule::NonEmpty,
        offset: u64 => Rule::Text,
        contract: String => Rule::NonEmpty,
        check: String => Rule::NonEmpty,
    }
}

/// Why a cursor was refused.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CursorError {
    /// Not a cursor this service issued.
    #[error("the cursor is not one this service issued")]
    Malformed,
    /// Issued for a different scope, query or sort.
    #[error("the cursor was issued for a different {field}")]
    Mismatch {
        /// Which binding differs.
        field: &'static str,
    },
}

impl Cursor {
    /// Mint a cursor.
    #[must_use]
    pub fn new(scope: &str, query_digest: &str, sort: &str, offset: u64) -> Self {
        Self {
            scope: scope.to_owned(),
            query_digest: query_digest.to_owned(),
            sort: sort.to_owned(),
            offset,
            contract: crate::request::Operation::ReadArtifact.contract_id().into(),
            check: Self::checksum(scope, query_digest, sort, offset),
        }
    }

    fn checksum(scope: &str, query_digest: &str, sort: &str, offset: u64) -> String {
        Key::ArtifactCursor
            .record(&Self {
                scope: scope.into(),
                query_digest: query_digest.into(),
                sort: sort.into(),
                offset,
                contract: crate::request::Operation::ReadArtifact.contract_id().into(),
                check: String::new(),
            })
            .expect("declared native artifact cursor identity")
    }

    /// The opaque wire form.
    /// # Errors
    /// Serialization failure cannot become an empty valid-looking cursor.
    pub fn encode(&self) -> Result<String, serde_json::Error> {
        Ok(format!("artifact10_{}", hex(&serde_json::to_vec(self)?)))
    }

    /// Parse and check a cursor against what the caller is paging now.
    ///
    /// # Errors
    ///
    /// See [`CursorError`].
    pub fn decode(
        text: &str,
        scope: &str,
        query_digest: &str,
        sort: &str,
    ) -> Result<Self, CursorError> {
        if text.len() > 32768 {
            return Err(CursorError::Malformed);
        }
        let body = text
            .strip_prefix("artifact10_")
            .ok_or(CursorError::Malformed)?;
        let bytes = unhex(body).ok_or(CursorError::Malformed)?;
        let cursor: Self = serde_json::from_slice(&bytes).map_err(|_| CursorError::Malformed)?;
        if cursor.contract != crate::request::Operation::ReadArtifact.contract_id()
            || cursor.check
                != Self::checksum(
                    &cursor.scope,
                    &cursor.query_digest,
                    &cursor.sort,
                    cursor.offset,
                )
        {
            return Err(CursorError::Malformed);
        }
        if cursor.scope != scope {
            return Err(CursorError::Mismatch {
                field: "snapshot or artifact",
            });
        }
        if cursor.query_digest != query_digest {
            return Err(CursorError::Mismatch {
                field: "query or filters",
            });
        }
        if cursor.sort != sort {
            return Err(CursorError::Mismatch { field: "sort" });
        }
        Ok(cursor)
    }
}

pub(crate) fn hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        out.push(DIGITS[usize::from(b >> 4)] as char);
        out.push(DIGITS[usize::from(b & 0x0f)] as char);
    }
    out
}

pub(crate) fn unhex(text: &str) -> Option<Vec<u8>> {
    if !text.len().is_multiple_of(2) {
        return None;
    }
    (0..text.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(text.get(i..i + 2)?, 16).ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cursors_bind_scope_query_and_sort() {
        let digest = "query-api";
        let cursor = Cursor::new("snap_x", &digest, "score", 12);
        let text = cursor.encode().expect("cursor serializes");
        assert!(text.starts_with("artifact10_"));
        assert_eq!(
            Cursor::decode(&text, "snap_x", &digest, "score")
                .expect("ok")
                .offset,
            12
        );
        assert!(matches!(
            Cursor::decode(&text, "snap_y", &digest, "score"),
            Err(CursorError::Mismatch {
                field: "snapshot or artifact"
            })
        ));
        let other = "query-docs";
        assert!(matches!(
            Cursor::decode(&text, "snap_x", &other, "score"),
            Err(CursorError::Mismatch {
                field: "query or filters"
            })
        ));
        assert!(matches!(
            Cursor::decode("artifact10_zz", "snap_x", &digest, "score"),
            Err(CursorError::Malformed)
        ));
        assert!(matches!(
            Cursor::decode("/etc/passwd", "snap_x", &digest, "score"),
            Err(CursorError::Malformed)
        ));
        // A tampered offset fails the checksum.
        assert!(
            Cursor::decode(
                &text.replacen("artifact10_", "cur_", 1),
                "snap_x",
                &digest,
                "score"
            )
            .is_err()
        );
        let mut tampered = cursor.clone();
        tampered.offset = 99;
        assert!(matches!(
            Cursor::decode(
                &tampered.encode().expect("cursor serializes"),
                "snap_x",
                &digest,
                "score"
            ),
            Err(CursorError::Malformed)
        ));
    }
}
pub mod spec;
