//! Continuations over an immutable relation's unique, totally ordered identity.
use super::CursorError;
use crate::{identity::SnapshotId, native_key::Key, native_union::Rule};

crate::native_struct! {
    pub struct RowCursor {
        after: String => Rule::NonEmpty,
        snapshot: SnapshotId => Rule::Text,
        selection: String => Rule::NonEmpty,
        contract: String => Rule::NonEmpty,
        check: String => Rule::NonEmpty,
    }
}
crate::native_struct! {
    pub(crate) struct RowCursorBinding {
        snapshot: SnapshotId => Rule::Text,
        selection: String => Rule::NonEmpty,
        contract: String => Rule::NonEmpty,
        after: String => Rule::NonEmpty,
    }
}

impl RowCursor {
    fn contract() -> String {
        format!(
            "{};{}",
            crate::request::Operation::Inspect.contract_id(),
            crate::request::Operation::Overview.contract_id()
        )
    }
    pub fn encode(
        snapshot: &SnapshotId,
        selection: &str,
        after: String,
    ) -> Result<String, serde_json::Error> {
        let mut value = Self {
            after,
            snapshot: snapshot.clone(),
            selection: selection.into(),
            contract: Self::contract(),
            check: String::new(),
        };
        value.check = value.checksum();
        Ok(format!(
            "rows10_{}",
            super::hex(&serde_json::to_vec(&value)?)
        ))
    }
    fn checksum(&self) -> String {
        Key::RowCursor
            .record(&RowCursorBinding {
                snapshot: self.snapshot.clone(),
                selection: self.selection.clone(),
                contract: self.contract.clone(),
                after: self.after.clone(),
            })
            .expect("declared native cursor identity")
    }
    pub fn decode(text: &str, snapshot: &SnapshotId, selection: &str) -> Result<Self, CursorError> {
        if text.len() > 32768 {
            return Err(CursorError::Malformed);
        }
        let bytes = text
            .strip_prefix("rows10_")
            .and_then(super::unhex)
            .ok_or(CursorError::Malformed)?;
        let value: Self = serde_json::from_slice(&bytes).map_err(|_| CursorError::Malformed)?;
        if value.contract != Self::contract()
            || value.checksum() != value.check
            || value.after.is_empty()
        {
            return Err(CursorError::Malformed);
        }
        if &value.snapshot != snapshot {
            return Err(CursorError::Mismatch {
                field: "selected snapshot",
            });
        }
        if value.selection != selection {
            return Err(CursorError::Mismatch {
                field: "aspect, subject or limits",
            });
        }
        Ok(value)
    }
}
