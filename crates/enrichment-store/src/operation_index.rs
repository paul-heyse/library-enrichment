//! Once-only native key/score computation shared by count and page in one operation.
//!
//! DataFusion's spill writer owns the temporary bytes and quota; its streaming provider
//! owns scans. This is not a persisted cache: the private provider is never registered in
//! the daemon template, and its last reader drops the spill and captured snapshot leases.
use std::{
    fmt,
    io::{self, Write},
    sync::Arc,
};

use arrow::{
    buffer::Buffer,
    datatypes::SchemaRef,
    ipc::{reader::StreamDecoder, writer::StreamWriter},
};
use datafusion::{
    catalog::{TableProvider, streaming::StreamingTable},
    dataframe::DataFrame,
    error::Result,
    execution::{
        TaskContext,
        disk_manager::DiskManager,
        memory_pool::MemoryConsumer,
        spill_file::{SpillFile, SpillWriter},
    },
    physical_plan::{
        SendableRecordBatchStream, stream::RecordBatchStreamAdapter, streaming::PartitionStream,
    },
};
use futures::TryStreamExt;

use crate::{
    preparation::QueryFamily,
    runtime::{OperationContext, QueryRuntime},
};

/// DataFusion 55.1's OS spill writer returns an untyped io::ErrorKind::Other on quota
/// exhaustion. Preserve a typed witness where the public counters prove the limit before
/// writing. The native writer still owns atomic quota enforcement under concurrent writes;
/// an ambiguous native I/O failure remains I/O, never a classification guessed from text.
struct IndexWriter {
    inner: Box<dyn SpillWriter>,
    disk: Arc<DiskManager>,
}
impl Write for IndexWriter {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let observed = self
            .disk
            .used_disk_space()
            .saturating_add(bytes.len() as u64);
        let allowed = self.disk.max_temp_directory_size();
        if observed > allowed {
            return Err(io::Error::other(
                datafusion::error::DataFusionError::External(Box::new(
                    crate::runtime::BudgetFailure {
                        rule: "operation index spill bytes".into(),
                        cause: enrichment_core::wire::DiagnosticCause::Capacity,
                        observed: Some(observed),
                        allowed: Some(allowed),
                        operation_id: crate::runtime::operation_id(),
                    },
                )),
            ));
        }
        self.inner.write(bytes)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}
impl SpillWriter for IndexWriter {
    fn finish(&mut self) -> Result<()> {
        self.inner.finish()
    }
}

struct IndexPartition {
    schema: SchemaRef,
    file: Arc<dyn SpillFile>,
    max_batch_bytes: usize,
    operation: OperationContext,
    history: crate::query_diagnostics::History,
}

impl fmt::Debug for IndexPartition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OperationIndex")
            .field("max_batch_bytes", &self.max_batch_bytes)
            .finish_non_exhaustive()
    }
}

impl PartitionStream for IndexPartition {
    fn schema(&self) -> &SchemaRef {
        &self.schema
    }

    fn execute(&self, ctx: Arc<TaskContext>) -> SendableRecordBatchStream {
        self.history.index_read();
        self.operation.index(None);
        let schema = Arc::clone(self.schema());
        let result = (|| {
            // One unbuffered IPC batch plus decode workspace. Charge before opening the
            // reader, and keep both reservation and operation leases until stream drop.
            let reservation =
                MemoryConsumer::new("operation_index_reader").register(ctx.memory_pool());
            // The pinned native spill reader uses a 128 KiB byte buffer independently of
            // this index's batch size; account for it even when batches are tiny.
            reservation.try_grow(
                self.max_batch_bytes
                    .saturating_mul(2)
                    .saturating_add(128 * 1024),
            )?;
            let input = self.file.read_stream()?;
            let file = Arc::clone(&self.file);
            let operation = self.operation.clone();
            let state = (
                input,
                StreamDecoder::new(),
                Buffer::from(&[]),
                file,
                reservation,
                operation,
            );
            let input = futures::stream::try_unfold(state, |mut state| async move {
                loop {
                    if !state.2.is_empty()
                        && let Some(batch) = state.1.decode(&mut state.2)?
                    {
                        return Ok(Some((batch, state)));
                    }
                    match state.0.try_next().await? {
                        Some(bytes) => state.2 = Buffer::from(bytes),
                        None => {
                            state.1.finish()?;
                            return Ok(None);
                        }
                    }
                }
            });
            Ok(
                Box::pin(RecordBatchStreamAdapter::new(Arc::clone(&schema), input))
                    as SendableRecordBatchStream,
            )
        })();
        result.unwrap_or_else(|error| {
            Box::pin(RecordBatchStreamAdapter::new(
                schema,
                futures::stream::once(async move { Err(error) }),
            ))
        })
    }
}

/// Materialize only a declared operation index, not evidence payloads or an arbitrary table.
/// Upstream plans, all scan batches, writer work and readers retain the operation deadline.
pub(crate) async fn materialize(
    runtime: &QueryRuntime,
    frame: DataFrame,
    family: QueryFamily,
) -> Result<Arc<dyn TableProvider>> {
    if !matches!(
        family,
        QueryFamily::SearchIndex
            | QueryFamily::ComparisonKeys
            | QueryFamily::OverviewChildren
            | QueryFamily::OverviewNamespaces
    ) {
        return Err(datafusion::error::DataFusionError::Internal(
            "only a declared key/score index may be retained for operation reuse".into(),
        ));
    }
    // Declare exact native output fields across Parquet string-view adaptation before IPC.
    let session = runtime.session();
    let frame = session.read_table(Arc::new(crate::provider::DerivedRelation::new(
        frame.into_view(),
        "operation_index",
    )))?;
    let schema = Arc::new(frame.schema().as_arrow().clone());
    let env = session.runtime_env();
    let file = env
        .disk_manager
        .create_tmp_file("operation key/score index")?;
    let output = Arc::clone(&file);
    let fields = Arc::clone(&schema);
    let disk = Arc::clone(&env.disk_manager);
    let writer = runtime
        .blocking(move || -> Result<_> {
            Ok(StreamWriter::try_new(
                IndexWriter {
                    inner: output.open_writer()?,
                    disk,
                },
                &fields,
            )?)
        })
        .await??;
    let pool = Arc::clone(&env.memory_pool);
    let (writer, max_batch_bytes) = runtime
        .fold_blocking(
            frame,
            family,
            usize::MAX,
            (writer, 0usize),
            move |(mut writer, maximum), batch| {
                let size = batch.get_array_memory_size();
                let reservation = MemoryConsumer::new("operation_index_writer").register(&pool);
                reservation.try_grow(size.saturating_mul(2))?;
                writer.write(batch)?;
                Ok((writer, maximum.max(size)))
            },
        )
        .await?;
    runtime
        .blocking(move || -> Result<()> { writer.into_inner()?.finish() })
        .await??;
    let bytes = file.size().ok_or_else(|| {
        datafusion::error::DataFusionError::Internal(
            "operation index needs an accounted local spill file".into(),
        )
    })?;
    runtime.index_history().materialized_index(bytes);
    crate::runtime::capture_operation().index(Some(bytes));
    let partition = IndexPartition {
        schema: Arc::clone(&schema),
        file,
        max_batch_bytes,
        operation: crate::runtime::capture_operation(),
        history: runtime.index_history(),
    };
    Ok(Arc::new(StreamingTable::try_new(
        schema,
        vec![Arc::new(partition)],
    )?))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::QueryLimits;

    async fn input(runtime: &QueryRuntime) -> DataFrame {
        runtime.session().sql("SELECT CAST(0 AS BIGINT UNSIGNED) AS plan, 'α' AS key, 'one' AS label UNION ALL SELECT CAST(0 AS BIGINT UNSIGNED), 'β', NULL UNION ALL SELECT CAST(0 AS BIGINT UNSIGNED), 'α', 'one'").await.unwrap()
    }

    #[tokio::test]
    async fn index_replays_exact_native_rows_and_drops_quota_after_last_reader() {
        let dir = tempfile::tempdir().unwrap();
        let runtime = QueryRuntime::new(dir.path(), QueryLimits::default()).unwrap();
        let env = runtime.session().runtime_env();
        let index = materialize(&runtime, input(&runtime).await, QueryFamily::ComparisonKeys)
            .await
            .unwrap();
        assert!(env.disk_manager.used_disk_space() > 0);
        let session = runtime.session();
        let frame = session.read_table(Arc::clone(&index)).unwrap();
        session.register_table("keys", frame.into_view()).unwrap();
        let result = runtime.execute(session.sql("SELECT key, label, count(*) AS count FROM keys GROUP BY key, label ORDER BY key").await.unwrap()).await.unwrap();
        let text = arrow::util::pretty::pretty_format_batches(&result.batches)
            .unwrap()
            .to_string();
        assert!(text.contains("α   | one   | 2"), "{text}");
        assert!(text.contains("β   |       | 1"), "{text}");
        let frame = session
            .table("keys")
            .await
            .unwrap()
            .limit(0, Some(1))
            .unwrap();
        drop(session);
        drop(index);
        assert!(
            env.disk_manager.used_disk_space() > 0,
            "selected reader keeps file alive"
        );
        assert_eq!(runtime.execute(frame).await.unwrap().rows, 1);
        assert_eq!(env.disk_manager.used_disk_space(), 0);
        assert_eq!(env.memory_pool.reserved(), 0);
    }

    #[tokio::test]
    async fn empty_and_failed_index_release_native_spill_reservations() {
        for spill_bytes in [128, 1024 * 1024] {
            let dir = tempfile::tempdir().unwrap();
            let runtime = QueryRuntime::new(
                dir.path(),
                QueryLimits {
                    spill_bytes,
                    ..Default::default()
                },
            )
            .unwrap();
            let env = runtime.session().runtime_env();
            let input = input(&runtime)
                .await
                .filter(datafusion::prelude::lit(false))
                .unwrap();
            let result = materialize(&runtime, input, QueryFamily::ComparisonKeys).await;
            if spill_bytes == 128 {
                let error = result.expect_err("IPC header must respect the native disk quota");
                assert_eq!(
                    crate::QueryError::from(error).diagnostic().cause,
                    enrichment_core::wire::DiagnosticCause::Capacity
                );
            } else {
                let index = result.unwrap();
                assert_eq!(
                    runtime
                        .execute(runtime.session().read_table(index).unwrap())
                        .await
                        .unwrap()
                        .rows,
                    0
                );
            }
            assert_eq!(env.disk_manager.used_disk_space(), 0);
            assert_eq!(env.memory_pool.reserved(), 0);
        }
    }
}
