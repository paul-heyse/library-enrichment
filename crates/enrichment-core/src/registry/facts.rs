//! Mechanical sparse-index decoding. Release policy is evaluated by native plans.
use super::{IndexEntry, RegistryError};
use arrow::{
    datatypes::{Schema, SchemaRef},
    record_batch::RecordBatch,
};
use std::sync::Arc;

use crate::native_union::{NativeStruct, Rule};

crate::native_struct! {
    pub struct Fact {
        source_line: u64 => Rule::Coordinate(crate::native_union::Unit::LineOneBased),
        release: IndexEntry => Rule::Text,
    }
}

pub fn schema() -> SchemaRef {
    Arc::new(Schema::new(Fact::fields()))
}

/// Decode only input syntax/defaults; do not select, sort or derive semantic rows here.
/// # Errors
/// Malformed records retain their physical line number; Arrow conversion failures fail closed.
pub fn decode(text: &str, batch_rows: usize) -> Result<Vec<RecordBatch>, RegistryError> {
    if batch_rows == 0 || batch_rows > 1_000_000 {
        return Err(RegistryError::ArrowFacts("invalid fact batch bound".into()));
    }
    let contract = schema();
    let mut batches = Vec::new();
    let mut rows = Vec::with_capacity(batch_rows.min(1024));
    for (index, line) in text.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        rows.push(Fact {
            source_line: (index + 1) as u64,
            release: serde_json::from_str(line).map_err(|e| RegistryError::MalformedIndexLine {
                line: index + 1,
                message: e.to_string(),
            })?,
        });
        if rows.len() == batch_rows {
            batches.push(Fact::batch(&rows).map_err(|e| RegistryError::ArrowFacts(e.to_string()))?);
            rows.clear();
        }
    }
    if !rows.is_empty() {
        batches.push(Fact::batch(&rows).map_err(|e| RegistryError::ArrowFacts(e.to_string()))?);
    }
    if batches.is_empty() {
        batches.push(RecordBatch::new_empty(contract));
    }
    Ok(batches)
}
