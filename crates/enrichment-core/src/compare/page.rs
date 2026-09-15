//! A total comparison order pinned to the exact snapshot pair and requested scopes.
use crate::search::CursorError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComparisonKey {
    pub plan: u32,
    pub subject: String,
    pub key: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComparisonCursor {
    pub snapshots: String,
    pub query_digest: String,
    pub returned_before: u64,
    pub after: ComparisonKey,
    contract: String,
    check: String,
}

impl ComparisonCursor {
    #[must_use]
    pub fn new(
        snapshots: String,
        query_digest: String,
        returned_before: u64,
        after: ComparisonKey,
    ) -> Self {
        let mut cursor = Self {
            snapshots,
            query_digest,
            returned_before,
            after,
            contract: "comparison-keyset/1".into(),
            check: String::new(),
        };
        cursor.check = cursor.checksum();
        cursor
    }
    fn checksum(&self) -> String {
        crate::canonical::digest_hex(&serde_json::json!([
            self.snapshots,
            self.query_digest,
            self.returned_before,
            self.after,
            self.contract
        ]))
    }
    /// # Errors
    /// Serialization failures remain explicit.
    pub fn encode(&self) -> Result<String, serde_json::Error> {
        Ok(format!(
            "comparison_{}",
            crate::search::hex(&serde_json::to_vec(self)?)
        ))
    }
    /// # Errors
    /// Reject corrupted, malformed, cross-snapshot and cross-query cursors.
    pub fn decode(text: &str, snapshots: &str, digest: &str) -> Result<Self, CursorError> {
        if text.len() > 32768 {
            return Err(CursorError::Malformed);
        }
        let bytes = crate::search::unhex(
            text.strip_prefix("comparison_")
                .ok_or(CursorError::Malformed)?,
        )
        .ok_or(CursorError::Malformed)?;
        let cursor: Self = serde_json::from_slice(&bytes).map_err(|_| CursorError::Malformed)?;
        if cursor.contract != "comparison-keyset/1"
            || cursor.check != cursor.checksum()
            || cursor.after.key.is_empty()
            || cursor.after.plan > 7
        {
            return Err(CursorError::Malformed);
        }
        if cursor.snapshots != snapshots {
            return Err(CursorError::Mismatch {
                field: "snapshot pair",
            });
        }
        if cursor.query_digest != digest {
            return Err(CursorError::Mismatch {
                field: "comparison scope or budget",
            });
        }
        Ok(cursor)
    }
}
