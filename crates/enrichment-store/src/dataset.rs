//! Bounded Arrow/Parquet construction for a normalized evidence set.
//!
//! Only one batch is materialized at a time. JSON serialization below counts bytes without
//! retaining JSON; queried structures are always written as the native Arrow projections.

use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use arrow::record_batch::RecordBatch;
use enrichment_core::{canonical, evidence::ingest::EvidenceBatch};
use parquet::arrow::ArrowWriter;
use serde::Serialize;

use crate::admission::{EvidenceFile, Relation};

/// Conservative input/output limits independent of the query memory pool.
#[derive(Debug, Clone)]
pub struct WriteLimits {
    pub row_group_rows: usize,
    pub observation_bloom: bool,
    pub record_bytes: usize,
    pub batch_rows: usize,
    pub batch_bytes: usize,
    pub file_bytes: u64,
    pub table_rows: usize,
    pub row_groups: usize,
}

impl Default for WriteLimits {
    fn default() -> Self {
        Self {
            row_group_rows: enrichment_core::config::NativeQueryConfig::default().row_group_rows,
            observation_bloom: enrichment_core::config::NativeQueryConfig::default()
                .observation_bloom,
            record_bytes: 1024 * 1024,
            batch_rows: 1024,
            batch_bytes: 16 * 1024 * 1024,
            file_bytes: 256 * 1024 * 1024,
            table_rows: 1_000_000,
            row_groups: 1024,
        }
    }
}

pub(crate) fn validate_limits(limits: &WriteLimits) -> io::Result<()> {
    if limits.row_group_rows == 0
        || limits.record_bytes == 0
        || limits.batch_rows == 0
        || limits.batch_bytes < limits.record_bytes
        || limits.file_bytes == 0
        || limits.table_rows == 0
        || limits.row_groups == 0
    {
        return Err(io::Error::other("invalid evidence write budgets"));
    }
    Ok(())
}

pub(crate) struct BoundedFile {
    pub(crate) file: File,
    pub(crate) bytes: u64,
    pub(crate) limit: u64,
}

impl Write for BoundedFile {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let remaining = self.limit.saturating_sub(self.bytes);
        if bytes.len() as u64 > remaining {
            return Err(io::Error::other("Parquet file exceeds byte budget"));
        }
        let written = self.file.write(bytes)?;
        self.bytes += written as u64;
        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.file.flush()
    }
}
pub(crate) fn record_bytes(value: &impl Serialize, limit: usize) -> io::Result<usize> {
    canonical::serialized_size(value, limit)
}

/// Write exact files under an unpublished staging directory. Native encoders reject invalid
/// domains; the caller must still complete relational admission before publishing this directory.
///
/// # Errors
/// Invalid records, resource exhaustion or file errors leave an unpublished staging failure.
pub fn write(
    root: &Path,
    evidence: &EvidenceBatch,
    limits: &WriteLimits,
) -> io::Result<Vec<EvidenceFile>> {
    evidence.validate_bound().map_err(io::Error::other)?;
    let mut sink = crate::record_writer::RelationWriter::new(root, limits)?;
    evidence
        .clone()
        .drain_into(&mut sink)
        .map_err(io::Error::other)?;
    let files = sink.finish()?;
    File::open(root)?.sync_all()?;
    Ok(files)
}

/// Stream one native relation into a bounded blocking writer. Both the writer and its
/// physical inputs remain retained if the asynchronous caller is cancelled.
/// # Errors
/// Invalid schemas, resource exhaustion, cancellation and I/O abort unpublished staging.
pub async fn write_plan(
    root: &Path,
    relation: Relation,
    plan: datafusion::dataframe::DataFrame,
    runtime: &crate::runtime::QueryRuntime,
    limits: &WriteLimits,
    retention: std::sync::Arc<File>,
) -> datafusion::error::Result<EvidenceFile> {
    write_transformed_plan(root, relation, plan, runtime, limits, retention, |batch| {
        Ok(batch.clone())
    })
    .await
}

struct CancelWriter(std::sync::Arc<std::sync::atomic::AtomicBool>);
impl Drop for CancelWriter {
    fn drop(&mut self) {
        self.0.store(true, std::sync::atomic::Ordering::Release);
    }
}

/// Apply one bounded record kernel off the async executor before native Parquet writing.
/// # Errors
/// Transformation, cast, writer, cancellation or query errors abort the candidate.
pub async fn write_transformed_plan(
    root: &Path,
    relation: Relation,
    plan: datafusion::dataframe::DataFrame,
    runtime: &crate::runtime::QueryRuntime,
    limits: &WriteLimits,
    retention: std::sync::Arc<File>,
    transform: impl FnMut(&RecordBatch) -> datafusion::error::Result<RecordBatch> + Send + 'static,
) -> datafusion::error::Result<EvidenceFile> {
    use datafusion::error::DataFusionError;
    validate_limits(limits)?;
    let (send, receive) = tokio::sync::mpsc::channel(1);
    let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let _cancel = CancelWriter(cancelled.clone());
    let root = root.to_owned();
    let write_limits = limits.clone();
    let inputs = plan.clone();
    let writer = tokio::task::spawn_blocking(move || {
        let _retention = retention;
        let _inputs = inputs;
        write_stream(
            &root,
            relation,
            &write_limits,
            receive,
            cancelled,
            transform,
        )
    });
    let query = runtime
        .visit_async(plan, limits.table_rows, |batch| {
            let send = &send;
            async move {
                send.send(Some(batch)).await.map_err(|_| {
                    DataFusionError::Execution(
                        "native writer stopped before consuming the query".into(),
                    )
                })
            }
        })
        .await;
    if query.is_ok() {
        // Only an exhausted successful query may request finalization.
        let _ = send.send(None).await;
    }
    drop(send);
    let output = writer
        .await
        .map_err(|e| DataFusionError::External(Box::new(e)))?;
    match output {
        Err(error) => Err(error),
        Ok(file) => {
            query?;
            Ok(file)
        }
    }
}

/// Bound both the decoded row-group working set and footer growth independently
/// of the processing batch and row target. A failed writer is never published.
pub(crate) fn write_bounded(
    writer: &mut ArrowWriter<BoundedFile>,
    batch: &arrow::record_batch::RecordBatch,
    limits: &WriteLimits,
    group_bytes: &mut usize,
) -> io::Result<()> {
    let bytes = batch.get_array_memory_size();
    if bytes > limits.batch_bytes {
        return Err(io::Error::other("Arrow batch byte limit exceeded"));
    }
    if writer.in_progress_rows() > 0 && group_bytes.saturating_add(bytes) > limits.batch_bytes {
        writer.flush().map_err(io::Error::other)?;
        *group_bytes = 0;
    }
    let groups = writer.flushed_row_groups().len();
    writer.write(batch).map_err(io::Error::other)?;
    *group_bytes = if writer.flushed_row_groups().len() > groups {
        bytes
    } else {
        group_bytes.saturating_add(bytes)
    };
    if writer.memory_size() >= limits.batch_bytes {
        writer.flush().map_err(io::Error::other)?;
        *group_bytes = 0;
    }
    if writer.flushed_row_groups().len() + usize::from(writer.in_progress_rows() > 0)
        > limits.row_groups
    {
        return Err(io::Error::other("row-group metadata limit exceeded"));
    }
    Ok(())
}

fn write_stream(
    root: &Path,
    relation: Relation,
    limits: &WriteLimits,
    mut receive: tokio::sync::mpsc::Receiver<Option<RecordBatch>>,
    cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>,
    mut transform: impl FnMut(&RecordBatch) -> datafusion::error::Result<RecordBatch>,
) -> datafusion::error::Result<EvidenceFile> {
    use datafusion::error::DataFusionError;
    let schema = relation.schema()?;
    let path = root.join(format!("{}.parquet", relation.name()));
    let file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)?;
    let properties = crate::native_policy::writer_properties(
        limits.row_group_rows,
        crate::native_policy::bloom_key(relation, limits.observation_bloom),
    )?;
    let mut writer = ArrowWriter::try_new(
        BoundedFile {
            file,
            bytes: 0,
            limit: limits.file_bytes,
        },
        schema.clone(),
        Some(properties),
    )?;
    let mut rows = 0u64;
    let mut group_bytes = 0;
    loop {
        let message = receive.blocking_recv();
        if cancelled.load(std::sync::atomic::Ordering::Acquire) {
            return Err(DataFusionError::Execution(
                "native writing cancelled".into(),
            ));
        }
        let Some(message) = message else {
            return Err(DataFusionError::Execution(
                "native writing input interrupted".into(),
            ));
        };
        let Some(batch) = message else {
            break;
        };
        if batch.get_array_memory_size() > limits.batch_bytes {
            return Err(DataFusionError::ResourcesExhausted(
                "assembly input batch exceeds budget before transformation".into(),
            ));
        }
        for start in (0..batch.num_rows()).step_by(limits.batch_rows) {
            let batch =
                transform(&batch.slice(start, limits.batch_rows.min(batch.num_rows() - start)))?;
            if batch.get_array_memory_size() > limits.batch_bytes {
                return Err(DataFusionError::ResourcesExhausted(
                    "transformed assembly batch exceeds budget before cast".into(),
                ));
            }
            let columns = schema
                .fields()
                .iter()
                .map(|field| {
                    let column = batch.column_by_name(field.name()).ok_or_else(|| {
                        DataFusionError::Execution("assembly field missing".into())
                    })?;
                    arrow::compute::cast(column, field.data_type()).map_err(DataFusionError::from)
                })
                .collect::<datafusion::error::Result<Vec<_>>>()?;
            let canonical = RecordBatch::try_new(schema.clone(), columns)?;
            if canonical.get_array_memory_size() > limits.batch_bytes
                || writer.flushed_row_groups().len() >= limits.row_groups
            {
                return Err(DataFusionError::ResourcesExhausted(
                    "assembly batch or row-group budget exceeded".into(),
                ));
            }
            rows = rows
                .checked_add(canonical.num_rows() as u64)
                .filter(|n| *n <= limits.table_rows as u64)
                .ok_or_else(|| {
                    DataFusionError::ResourcesExhausted("assembly row budget exceeded".into())
                })?;
            write_bounded(&mut writer, &canonical, limits, &mut group_bytes)?;
        }
    }
    writer.finish()?;
    writer.inner().file.sync_all()?;
    let (sha256, bytes) = canonical::sha256_reader(File::open(&path)?, limits.file_bytes)?;
    Ok(EvidenceFile {
        relation,
        path,
        sha256,
        bytes,
        rows,
    })
}
