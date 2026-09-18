//! A total comparison order pinned to the exact snapshot pair and requested scopes.
use crate::search::CursorError;
use crate::{identity::SnapshotId, native_key::Key, native_union::Rule};

crate::native_struct! {
    pub struct SnapshotPair { before: SnapshotId => Rule::Text, after: SnapshotId => Rule::Text }
}

crate::native_struct! {
    #[derive(PartialOrd,Ord)]
    pub struct ComparisonKey {
        plan: u32 => crate::native_union::Rule::UnsignedRange {min:0,max:7},
        subject: String => crate::native_union::Rule::Text,
        key: String => crate::native_union::Rule::NonEmpty,
    }
}

crate::native_struct! {
    /// Stable alternative order is `(value, source)` within an immutable changed key.
    pub struct AlternativeCursor {
        key: ComparisonKey => Rule::Text,
        before: bool => Rule::Text,
        offset: usize => Rule::Text,
        snapshots: SnapshotPair => Rule::Text,
        selection: String => Rule::NonEmpty,
        contract: String => Rule::NonEmpty,
        check: String => Rule::NonEmpty,
    }
}

impl AlternativeCursor {
    pub fn encode(
        key: ComparisonKey,
        before: bool,
        offset: usize,
        snapshots: &SnapshotPair,
        selection: &str,
    ) -> Result<String, serde_json::Error> {
        let mut cursor = Self {
            key,
            before,
            offset,
            snapshots: snapshots.clone(),
            selection: selection.into(),
            contract: crate::request::Operation::Compare.contract_id().into(),
            check: String::new(),
        };
        cursor.check = cursor.checksum();
        crate::native_cursor::Kind::Alternatives.encode(&cursor)
    }
    fn checksum(&self) -> String {
        Key::AlternativeCursor
            .record(self)
            .expect("declared native alternative cursor identity")
    }
    pub fn decode(
        text: &str,
        snapshots: &SnapshotPair,
        selection: &str,
    ) -> Result<Self, CursorError> {
        let cursor: Self = crate::native_cursor::Kind::Alternatives.decode(text)?;
        // Offset zero is a valid first-page route for a side deferred by the byte budget.
        if cursor.contract != crate::request::Operation::Compare.contract_id()
            || cursor.check != cursor.checksum()
            || cursor.key.plan > 7
            || cursor.key.key.is_empty()
        {
            return Err(CursorError::Malformed);
        }
        if &cursor.snapshots != snapshots {
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

crate::native_struct! {
    pub struct ComparisonCursor {
        snapshots: SnapshotPair => Rule::Text,
        query_digest: String => Rule::NonEmpty,
        returned_before: u64 => Rule::Text,
        after: ComparisonKey => Rule::Text,
        contract: String => Rule::NonEmpty,
        check: String => Rule::NonEmpty,
    }
}

impl ComparisonCursor {
    #[must_use]
    pub fn new(
        snapshots: SnapshotPair,
        query_digest: String,
        returned_before: u64,
        after: ComparisonKey,
    ) -> Self {
        let mut cursor = Self {
            snapshots,
            query_digest,
            returned_before,
            after,
            contract: crate::request::Operation::Compare.contract_id().into(),
            check: String::new(),
        };
        cursor.check = cursor.checksum();
        cursor
    }
    fn checksum(&self) -> String {
        Key::ComparisonCursor
            .record(self)
            .expect("declared native comparison cursor identity")
    }
    /// # Errors
    /// Serialization failures remain explicit.
    pub fn encode(&self) -> Result<String, serde_json::Error> {
        crate::native_cursor::Kind::Comparison.encode(self)
    }
    /// # Errors
    /// Reject corrupted, malformed, cross-snapshot and cross-query cursors.
    pub fn decode(text: &str, snapshots: &SnapshotPair, digest: &str) -> Result<Self, CursorError> {
        let cursor: Self = crate::native_cursor::Kind::Comparison.decode(text)?;
        if cursor.contract != crate::request::Operation::Compare.contract_id()
            || cursor.check != cursor.checksum()
            || cursor.after.key.is_empty()
            || cursor.after.plan > 7
        {
            return Err(CursorError::Malformed);
        }
        if &cursor.snapshots != snapshots {
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
        let pair = SnapshotPair {
            before: SnapshotId::try_from(format!("snap_{}", "a".repeat(64))).unwrap(),
            after: SnapshotId::try_from(format!("snap_{}", "b".repeat(64))).unwrap(),
        };
        let cursor = ComparisonCursor::new(
            pair.clone(),
            "selection".into(),
            1,
            ComparisonKey {
                plan: 0,
                subject: "pkg::Symbol".into(),
                key: "symbol".into(),
            },
        );
        let encoded = cursor.encode().expect("encode");
        assert!(ComparisonCursor::decode(&encoded, &pair, "selection").is_ok());
        assert!(
            ComparisonCursor::decode(
                &encoded.replacen("comparison10_", "comparison_", 1),
                &pair,
                "selection"
            )
            .is_err()
        );
    }
}
