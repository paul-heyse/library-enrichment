//! Complete deterministic search ordering and a query/snapshot-bound keyset cursor.

use super::CursorError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SearchKey {
    pub score: u32,
    pub hit_order: u32,
    pub subject: String,
    pub candidate_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SearchCursor {
    pub scope: String,
    pub query_digest: String,
    pub contract: String,
    pub returned_before: u64,
    pub after: SearchKey,
    check: String,
}

impl SearchCursor {
    #[must_use]
    pub fn new(
        scope: String,
        query_digest: String,
        returned_before: u64,
        after: SearchKey,
    ) -> Self {
        let mut value = Self {
            scope,
            query_digest,
            contract: "search-keyset/2;lexical/2".into(),
            returned_before,
            after,
            check: String::new(),
        };
        value.check = value.checksum();
        value
    }

    fn checksum(&self) -> String {
        crate::canonical::digest_hex(&serde_json::json!([
            self.scope,
            self.query_digest,
            self.contract,
            self.returned_before,
            self.after,
        ]))
    }

    /// # Errors
    /// A failed serialization is not an empty cursor.
    pub fn encode(&self) -> Result<String, serde_json::Error> {
        Ok(format!(
            "search2_{}",
            super::hex(&serde_json::to_vec(self)?)
        ))
    }

    /// # Errors
    /// Malformed, corrupted, cross-query or cross-snapshot cursors are rejected.
    pub fn decode(text: &str, scope: &str, digest: &str) -> Result<Self, CursorError> {
        if text.len() > 32768 {
            return Err(CursorError::Malformed);
        }
        let bytes = super::unhex(
            text.strip_prefix("search2_")
                .ok_or(CursorError::Malformed)?,
        )
        .ok_or(CursorError::Malformed)?;
        let value: Self = serde_json::from_slice(&bytes).map_err(|_| CursorError::Malformed)?;
        if value.after.hit_order > 1
            || value.after.candidate_id.is_empty()
            || value.contract != "search-keyset/2;lexical/2"
            || value.check != value.checksum()
        {
            return Err(CursorError::Malformed);
        }
        if value.scope != scope {
            return Err(CursorError::Mismatch { field: "snapshot" });
        }
        if value.query_digest != digest {
            return Err(CursorError::Mismatch {
                field: "query or filters",
            });
        }
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keyset_cursor_binds_every_ordering_field_scope_and_contract() {
        let key = SearchKey {
            score: 805,
            hit_order: 0,
            subject: "pkg::X".into(),
            candidate_id: "obs_fixture/symbol_fixture".into(),
        };
        let value = SearchCursor::new("snapshot-a".into(), "query-a".into(), 2, key.clone());
        let encoded = value.encode().expect("encode");
        assert_eq!(
            SearchCursor::decode(&encoded, "snapshot-a", "query-a")
                .expect("decode")
                .after,
            key
        );
        assert!(SearchCursor::decode(&encoded, "snapshot-b", "query-a").is_err());
        assert!(SearchCursor::decode(&encoded, "snapshot-a", "query-b").is_err());
        let legacy = encoded.replacen("search2_", "search_", 1);
        assert!(SearchCursor::decode(&legacy, "snapshot-a", "query-a").is_err());
        let mut corrupt = value;
        corrupt.after.subject = "pkg::Y".into();
        assert!(
            SearchCursor::decode(&corrupt.encode().expect("encode"), "snapshot-a", "query-a")
                .is_err()
        );
        assert!(SearchCursor::decode(&"search2_00".repeat(4000), "snapshot-a", "query-a").is_err());
    }
}
