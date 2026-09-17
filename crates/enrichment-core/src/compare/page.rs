//! A total comparison order pinned to the exact snapshot pair and requested scopes.
use crate::search::CursorError;
use serde::{Deserialize, Serialize};

crate::native_struct! {
    #[derive(PartialOrd,Ord)]
    pub struct ComparisonKey {
        plan: u32 => crate::native_union::Rule::UnsignedRange {min:0,max:7},
        subject: String => crate::native_union::Rule::Text,
        key: String => crate::native_union::Rule::NonEmpty,
    }
}

/// Stable alternative order is `(value, source)` within an immutable changed key.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AlternativeCursor {
    pub key: ComparisonKey,
    pub before: bool,
    pub offset: usize,
    snapshots: String,
    selection: String,
    check: String,
}

impl AlternativeCursor {
    pub fn encode(
        key: ComparisonKey,
        before: bool,
        offset: usize,
        snapshots: &str,
        selection: &str,
    ) -> Result<String, serde_json::Error> {
        let mut cursor = Self {
            key,
            before,
            offset,
            snapshots: snapshots.into(),
            selection: selection.into(),
            check: String::new(),
        };
        cursor.check = cursor.checksum();
        Ok(format!(
            "alternatives2_{}",
            crate::search::hex(&serde_json::to_vec(&cursor)?)
        ))
    }
    fn checksum(&self) -> String {
        crate::canonical::digest_hex(&serde_json::json!([
            "comparison-alternatives/2",
            self.key,
            self.before,
            self.offset,
            self.snapshots,
            self.selection
        ]))
    }
    pub fn decode(text: &str, snapshots: &str, selection: &str) -> Result<Self, CursorError> {
        if text.len() > 32768 {
            return Err(CursorError::Malformed);
        }
        let bytes = text
            .strip_prefix("alternatives2_")
            .and_then(crate::search::unhex)
            .ok_or(CursorError::Malformed)?;
        let cursor: Self = serde_json::from_slice(&bytes).map_err(|_| CursorError::Malformed)?;
        // Offset zero is a valid first-page route for a side deferred by the byte budget.
        if cursor.check != cursor.checksum() || cursor.key.plan > 7 || cursor.key.key.is_empty() {
            return Err(CursorError::Malformed);
        }
        if cursor.snapshots != snapshots {
            return Err(CursorError::Mismatch {
                field: "snapshot pair",
            });
        }
        if cursor.selection != selection {
            return Err(CursorError::Mismatch {
                field: "comparison selection or limits",
            });
        }
        Ok(cursor)
    }
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
            contract: "comparison-keyset/2".into(),
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
            "comparison2_{}",
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
            text.strip_prefix("comparison2_")
                .ok_or(CursorError::Malformed)?,
        )
        .ok_or(CursorError::Malformed)?;
        let cursor: Self = serde_json::from_slice(&bytes).map_err(|_| CursorError::Malformed)?;
        if cursor.contract != "comparison-keyset/2"
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comparison_cursor_rejects_the_previous_generation() {
        let cursor = ComparisonCursor::new(
            "pair".into(),
            "selection".into(),
            1,
            ComparisonKey {
                plan: 0,
                subject: "pkg::Symbol".into(),
                key: "symbol".into(),
            },
        );
        let encoded = cursor.encode().expect("encode");
        assert!(ComparisonCursor::decode(&encoded, "pair", "selection").is_ok());
        assert!(
            ComparisonCursor::decode(
                &encoded.replacen("comparison2_", "comparison_", 1),
                "pair",
                "selection"
            )
            .is_err()
        );
    }
}
