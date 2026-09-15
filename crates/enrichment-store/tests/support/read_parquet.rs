//! Raw fixture read for deliberate corruption tests.
use arrow::record_batch::RecordBatch;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use std::{fs::File, io, path::Path};

/// Read every batch of a Parquet file, concatenated.
///
/// # Errors
///
/// Fails on I/O or decoding error.
pub fn read_parquet(path: &Path) -> io::Result<Vec<RecordBatch>> {
    let file = File::open(path)?;
    let reader = ParquetRecordBatchReaderBuilder::try_new(file)
        .map_err(|e| io::Error::other(e.to_string()))?
        .build()
        .map_err(|e| io::Error::other(e.to_string()))?;
    reader
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| io::Error::other(e.to_string()))
}
