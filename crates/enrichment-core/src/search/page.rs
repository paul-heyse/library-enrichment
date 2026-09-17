//! Complete deterministic search ordering and a query/snapshot-bound keyset cursor.

use super::CursorError;
use crate::{
    identity::{ContextId, SnapshotId},
    native_key::Key,
    native_union::Rule,
};
crate::native_struct! {
    pub struct SearchScope {
        context_id: ContextId => Rule::Text,
        snapshot_id: SnapshotId => Rule::Text,
    }
}
crate::native_struct! {
    pub struct SearchKey {
        score: u32 => Rule::Text,
        hit_order: u32 => Rule::UnsignedRange { min: 0, max: 1 },
        subject: String => Rule::Text,
        candidate_id: String => Rule::NonEmpty,
    }
}
crate::native_struct! {
    pub struct SearchCursor {
        scope: SearchScope => Rule::Text,
        query_digest: String => Rule::NonEmpty,
        contract: String => Rule::NonEmpty,
        returned_before: u64 => Rule::Text,
        after: SearchKey => Rule::Text,
        check: String => Rule::NonEmpty,
    }
}
crate::native_struct! {
    pub(crate) struct SearchCursorBinding {
        scope: SearchScope => Rule::Text,
        query_digest: String => Rule::NonEmpty,
        contract: String => Rule::NonEmpty,
        returned_before: u64 => Rule::Text,
        after: SearchKey => Rule::Text,
    }
}

impl SearchCursor {
    fn contract() -> String {
        format!(
            "search-keyset/10;lexical/3;{}",
            crate::request::Operation::Search.contract_id()
        )
    }
    #[must_use]
    pub fn new(
        scope: SearchScope,
        query_digest: String,
        returned_before: u64,
        after: SearchKey,
    ) -> Self {
        let mut value = Self {
            scope,
            query_digest,
            contract: Self::contract(),
            returned_before,
            after,
            check: String::new(),
        };
        value.check = value.checksum();
        value
    }

    fn checksum(&self) -> String {
        Key::SearchCursor
            .record(&SearchCursorBinding {
                scope: self.scope.clone(),
                query_digest: self.query_digest.clone(),
                contract: self.contract.clone(),
                returned_before: self.returned_before,
                after: self.after.clone(),
            })
            .expect("declared native search cursor identity")
    }

    /// # Errors
    /// A failed serialization is not an empty cursor.
    pub fn encode(&self) -> Result<String, serde_json::Error> {
        Ok(format!(
            "search10_{}",
            super::hex(&serde_json::to_vec(self)?)
        ))
    }

    /// # Errors
    /// Malformed, corrupted, cross-query or cross-snapshot cursors are rejected.
    pub fn decode(text: &str, scope: &SearchScope, digest: &str) -> Result<Self, CursorError> {
        if text.len() > 32768 {
            return Err(CursorError::Malformed);
        }
        let bytes = super::unhex(
            text.strip_prefix("search10_")
                .ok_or(CursorError::Malformed)?,
        )
        .ok_or(CursorError::Malformed)?;
        let value: Self = serde_json::from_slice(&bytes).map_err(|_| CursorError::Malformed)?;
        if value.after.hit_order > 1
            || value.after.candidate_id.is_empty()
            || value.contract != Self::contract()
            || value.check != value.checksum()
        {
            return Err(CursorError::Malformed);
        }
        if &value.scope != scope {
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
        let scope = SearchScope {
            context_id: crate::identity::ContextId::try_from(format!("ctx_{}", "a".repeat(64)))
                .unwrap(),
            snapshot_id: crate::identity::SnapshotId::try_from(format!("snap_{}", "a".repeat(64)))
                .unwrap(),
        };
        let other = SearchScope {
            snapshot_id: crate::identity::SnapshotId::try_from(format!("snap_{}", "b".repeat(64)))
                .unwrap(),
            ..scope.clone()
        };
        let value = SearchCursor::new(scope.clone(), "query-a".into(), 2, key.clone());
        let encoded = value.encode().expect("encode");
        assert_eq!(
            SearchCursor::decode(&encoded, &scope, "query-a")
                .expect("decode")
                .after,
            key
        );
        assert!(SearchCursor::decode(&encoded, &other, "query-a").is_err());
        assert!(SearchCursor::decode(&encoded, &scope, "query-b").is_err());
        let legacy = encoded.replacen("search10_", "search_", 1);
        assert!(SearchCursor::decode(&legacy, &scope, "query-a").is_err());
        let mut corrupt = value;
        corrupt.after.subject = "pkg::Y".into();
        assert!(
            SearchCursor::decode(&corrupt.encode().expect("encode"), &scope, "query-a").is_err()
        );
        assert!(SearchCursor::decode(&"search10_00".repeat(4000), &scope, "query-a").is_err());
    }
}
