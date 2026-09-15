//! Schema-neutral Parquet fixture/inspection utilities. Production evidence uses bounded dataset publication and admission.
use arrow::record_batch::RecordBatch;
use parquet::{
    arrow::ArrowWriter,
    basic::{Compression, ZstdLevel},
    file::properties::WriterProperties,
};
use std::{fs::File, io, path::Path};

/// Write one batch to a Parquet file with the key-value metadata every table carries.
///
/// # Errors
///
/// Fails on I/O or encoding error.
pub fn write_parquet(
    path: &Path,
    batch: &RecordBatch,
    metadata: &[(&str, &str)],
) -> io::Result<u64> {
    let props = WriterProperties::builder()
        .set_compression(Compression::ZSTD(
            ZstdLevel::try_new(3).map_err(|e| io::Error::other(e.to_string()))?,
        ))
        .set_key_value_metadata(Some(
            metadata
                .iter()
                .map(|(k, v)| {
                    parquet::file::metadata::KeyValue::new((*k).to_owned(), (*v).to_owned())
                })
                .collect(),
        ))
        .build();
    let file = File::create(path)?;
    let mut writer = ArrowWriter::try_new(file, batch.schema(), Some(props))
        .map_err(|e| io::Error::other(e.to_string()))?;
    writer
        .write(batch)
        .map_err(|e| io::Error::other(e.to_string()))?;
    writer
        .close()
        .map_err(|e| io::Error::other(e.to_string()))?;
    Ok(batch.num_rows() as u64)
}
