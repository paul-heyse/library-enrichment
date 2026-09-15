//! Continuations over an immutable relation's unique, totally ordered identity.
use super::CursorError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RowCursor {
    pub after: String,
    snapshot: String,
    selection: String,
    check: String,
}

impl RowCursor {
    pub fn encode(
        snapshot: &str,
        selection: &str,
        after: String,
    ) -> Result<String, serde_json::Error> {
        let mut value = Self {
            after,
            snapshot: snapshot.into(),
            selection: selection.into(),
            check: String::new(),
        };
        value.check = value.checksum();
        Ok(format!(
            "rows2_{}",
            super::hex(&serde_json::to_vec(&value)?)
        ))
    }
    fn checksum(&self) -> String {
        crate::canonical::digest_hex(&serde_json::json!([
            "research-rows/2",
            self.snapshot,
            self.selection,
            self.after
        ]))
    }
    pub fn decode(text: &str, snapshot: &str, selection: &str) -> Result<Self, CursorError> {
        if text.len() > 32768 {
            return Err(CursorError::Malformed);
        }
        let bytes = text
            .strip_prefix("rows2_")
            .and_then(super::unhex)
            .ok_or(CursorError::Malformed)?;
        let value: Self = serde_json::from_slice(&bytes).map_err(|_| CursorError::Malformed)?;
        if value.checksum() != value.check || value.after.is_empty() {
            return Err(CursorError::Malformed);
        }
        if value.snapshot != snapshot {
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
