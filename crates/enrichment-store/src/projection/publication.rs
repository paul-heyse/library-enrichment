//! Generated native publication metadata in the atomic control transaction.
use arrow::{
    datatypes::{Schema, SchemaRef},
    error::ArrowError,
    record_batch::RecordBatch,
};
use enrichment_core::{
    evidence::{catalog::SnapshotEntry, snapshot::SnapshotDescriptor},
    native_union::NativeStruct,
};
use std::sync::Arc;

pub(crate) fn descriptor_schema() -> SchemaRef {
    Arc::new(Schema::new(SnapshotDescriptor::fields()))
}
pub fn encode(rows: &[SnapshotEntry]) -> Result<RecordBatch, ArrowError> {
    for row in rows {
        row.validate().map_err(super::cells::invalid)?;
    }
    SnapshotEntry::batch(rows)
}
pub fn decode(batch: &RecordBatch) -> Result<Vec<SnapshotEntry>, ArrowError> {
    if batch.num_rows() > 1024 {
        return Err(super::cells::invalid("publication batch exceeds bound"));
    }
    let rows = super::cells::RowSet::batch(batch)?;
    (0..batch.num_rows())
        .map(|index| {
            let row = <SnapshotEntry as NativeStruct>::decode(rows.row(index))?;
            row.validate().map_err(super::cells::invalid)?;
            Ok(row)
        })
        .collect()
}
