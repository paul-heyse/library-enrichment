//! The declared observation owns its Arrow shape and mechanical codecs.
use super::cells::{RowSet, invalid};
use crate::{evidence::execution::ExecutionObservation, native_union::NativeStruct};
use arrow::{error::ArrowError, record_batch::RecordBatch};

pub fn encode(rows: &[ExecutionObservation]) -> Result<RecordBatch, ArrowError> {
    for row in rows {
        row.validate().map_err(invalid)?;
    }
    fields(rows)
}

pub(crate) fn fields(rows: &[ExecutionObservation]) -> Result<RecordBatch, ArrowError> {
    ExecutionObservation::batch(rows)
}

/// Decode a bounded batch and recompute every execution fact identity.
pub fn decode(batch: &RecordBatch) -> Result<Vec<ExecutionObservation>, ArrowError> {
    let rows = RowSet::batch(batch)?;
    (0..batch.num_rows())
        .map(|i| {
            let value = ExecutionObservation::decode(rows.row(i))?;
            value.validate().map_err(invalid)?;
            Ok(value)
        })
        .collect()
}
