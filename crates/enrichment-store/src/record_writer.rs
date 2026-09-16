//! Incremental typed records to bounded Arrow IPC batches. No corpus of normalized payloads exists.
use crate::dataset::{BoundedFile, WriteLimits};
use arrow::ipc::writer::FileWriter;
use arrow::{error::ArrowError, record_batch::RecordBatch};
use enrichment_core::canonical;
use serde::Serialize;
use std::{fs::File, io, path::Path};

pub(crate) struct RelationBuffer<T> {
    name: String,
    path: std::path::PathBuf,
    writer: FileWriter<BoundedFile>,
    pending: Vec<T>,
    pending_bytes: usize,
    rows: usize,
    limits: WriteLimits,
    encode: fn(&[T]) -> Result<RecordBatch, ArrowError>,
    schema_nodes: usize,
}
impl<T: Serialize> RelationBuffer<T> {
    pub(crate) fn staging(
        root: &Path,
        name: &str,
        limits: &WriteLimits,
        encode: fn(&[T]) -> Result<RecordBatch, ArrowError>,
    ) -> io::Result<Self> {
        crate::dataset::validate_limits(limits)?;
        let path = root.join(format!("{name}.arrow"));
        let file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)?;
        let schema = encode(&[]).map_err(io::Error::other)?.schema();
        let schema_nodes = schema
            .fields()
            .iter()
            .map(|field| schema_nodes(field.data_type()))
            .sum();
        let writer = FileWriter::try_new(
            BoundedFile {
                file,
                bytes: 0,
                limit: limits.file_bytes,
            },
            &schema,
        )
        .map_err(io::Error::other)?;
        Ok(Self {
            name: name.into(),
            path,
            writer,
            pending: Vec::new(),
            pending_bytes: 0,
            rows: 0,
            limits: limits.clone(),
            encode,
            schema_nodes,
        })
    }
    fn construction_budget(&self, rows: usize, bytes: usize) -> usize {
        // Preflight space for nested offset/validity arrays, rounded small allocations and
        // derived scalar columns, as well as UTF-8 data. Final Arrow capacity is checked too.
        if rows == 0 {
            return 0;
        }
        bytes.saturating_mul(2).saturating_add(
            self.schema_nodes
                .saturating_mul(256usize.saturating_add(rows.saturating_mul(8))),
        )
    }
    pub(crate) fn push(&mut self, value: T) -> io::Result<()> {
        let bytes = canonical::serialized_size(&value, self.limits.record_bytes)?;
        if self.rows >= self.limits.table_rows {
            return Err(io::Error::other("relation row limit exceeded"));
        }
        if self.pending.len() >= self.limits.batch_rows
            || self.construction_budget(
                self.pending.len() + 1,
                self.pending_bytes.saturating_add(bytes),
            ) > self.limits.batch_bytes
        {
            self.flush()?;
        }
        if self.construction_budget(1, bytes) > self.limits.batch_bytes {
            return Err(io::Error::other("record cannot fit batch"));
        }
        self.rows += 1;
        self.pending_bytes += bytes;
        self.pending.push(value);
        Ok(())
    }
    fn flush(&mut self) -> io::Result<()> {
        if self.pending.is_empty() {
            return Ok(());
        }
        let rows = std::mem::take(&mut self.pending);
        let batch = (self.encode)(&rows).map_err(io::Error::other)?;
        if batch.get_array_memory_size() > self.limits.batch_bytes {
            return Err(io::Error::other(format!(
                "Arrow batch byte limit exceeded: relation={}, rows={}, serialized={}, arrow={}, limit={}",
                self.name,
                rows.len(),
                self.pending_bytes,
                batch.get_array_memory_size(),
                self.limits.batch_bytes
            )));
        }
        drop(rows);
        self.pending_bytes = 0;
        self.writer.write(&batch).map_err(io::Error::other)?;
        Ok(())
    }
    pub(crate) fn finish_staging(mut self) -> io::Result<(std::path::PathBuf, String, u64, u64)> {
        self.flush()?;
        self.writer.finish().map_err(io::Error::other)?;
        self.writer.get_ref().file.sync_all()?;
        let (sha256, bytes) =
            canonical::sha256_reader(File::open(&self.path)?, self.limits.file_bytes)?;
        Ok((self.path, sha256, bytes, self.rows as u64))
    }
}

fn schema_nodes(data_type: &arrow::datatypes::DataType) -> usize {
    use arrow::datatypes::DataType;
    match data_type {
        DataType::Struct(fields) => {
            1 + fields
                .iter()
                .map(|field| schema_nodes(field.data_type()))
                .sum::<usize>()
        }
        DataType::List(field) | DataType::LargeList(field) | DataType::FixedSizeList(field, _) => {
            1 + schema_nodes(field.data_type())
        }
        _ => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn encode(values: &[String]) -> Result<RecordBatch, ArrowError> {
        RecordBatch::try_from_iter([(
            "raw",
            std::sync::Arc::new(arrow::array::StringArray::from(values.to_vec()))
                as arrow::array::ArrayRef,
        )])
    }
    #[test]
    fn raw_fact_ipc_streams_under_bounds_and_rejects_excess() {
        let dir = tempfile::tempdir().unwrap();
        let limits = WriteLimits {
            batch_rows: 3,
            table_rows: 8,
            record_bytes: 128,
            batch_bytes: 4096,
            ..WriteLimits::default()
        };
        let mut writer = RelationBuffer::staging(dir.path(), "raw", &limits, encode).unwrap();
        for i in 0..8 {
            writer.push(format!("record {i}")).unwrap();
        }
        assert!(writer.push("excess row".into()).is_err());
        let (path, digest, bytes, rows) = writer.finish_staging().unwrap();
        assert_eq!(rows, 8);
        assert_eq!(
            canonical::sha256_reader(File::open(&path).unwrap(), limits.file_bytes).unwrap(),
            (digest, bytes)
        );
        let reader =
            arrow::ipc::reader::FileReader::try_new(File::open(path).unwrap(), None).unwrap();
        let counts = reader.map(|b| b.unwrap().num_rows()).collect::<Vec<_>>();
        assert_eq!(counts, [3, 3, 2]);
        let mut writer = RelationBuffer::staging(dir.path(), "oversize", &limits, encode).unwrap();
        assert!(writer.push("x".repeat(129)).is_err());
        let tiny = WriteLimits {
            file_bytes: 8,
            ..limits
        };
        assert!(RelationBuffer::staging(dir.path(), "file_bound", &tiny, encode).is_err());
    }
}
