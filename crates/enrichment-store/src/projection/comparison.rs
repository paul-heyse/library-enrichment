//! Decode only bounded comparison indexes and selected source references.
use super::{
    cells::{RowSet, invalid},
    decode,
};
use arrow::{error::ArrowError, record_batch::RecordBatch};
use enrichment_core::{compare::page::ComparisonKey, evidence::relational::FactSource};

pub(crate) fn keys(batches: &[RecordBatch]) -> Result<Vec<ComparisonKey>, ArrowError> {
    let mut keys = Vec::new();
    for batch in batches {
        let rows = RowSet::batch(batch)?;
        for i in 0..batch.num_rows() {
            let row = rows.row(i);
            keys.push(ComparisonKey {
                plan: u32::try_from(row.number("plan")?).map_err(|e| invalid(e.to_string()))?,
                subject: row.text("label")?.into(),
                key: row.text("key")?.into(),
            });
        }
    }
    Ok(keys)
}

pub(crate) fn count(batches: &[RecordBatch]) -> Result<u64, ArrowError> {
    if batches.len() != 1 || batches[0].num_rows() != 1 {
        return Err(invalid("comparison count must have one row"));
    }
    RowSet::batch(&batches[0])?.row(0).number("count")
}

pub(crate) fn alternative_sources(
    batches: &[RecordBatch],
) -> Result<Vec<Option<FactSource>>, ArrowError> {
    let mut sources = Vec::new();
    for batch in batches {
        let rows = RowSet::batch(batch)?;
        for i in 0..batch.num_rows() {
            sources.push(
                rows.row(i)
                    .optional_struct("source")?
                    .map(decode::source)
                    .transpose()?,
            );
        }
    }
    Ok(sources)
}
