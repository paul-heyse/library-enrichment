//! The kernel owns metric production; this boundary copies its typed facts into Arrow history.
use delta_kernel::metrics::{CommitFailureReason, MetricId, ScanType, TableType};
use delta_kernel::metrics::{MetricEvent, MetricsReporter, ReportGeneratorLayer};
use enrichment_core::telemetry::kernel::{
    CommitFailure, Context, Metric, Observation, ScanKind, TableKind,
};
use std::sync::Arc;
use tracing_subscriber::prelude::*;

tokio::task_local! { static METRIC_OWNER: Option<String>; }

struct Reporter(crate::telemetry_history::History);
impl std::fmt::Debug for Reporter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NativeKernelReporter")
            .finish_non_exhaustive()
    }
}
impl MetricsReporter for Reporter {
    fn report(&self, event: MetricEvent) {
        let owner = METRIC_OWNER.try_with(Clone::clone).unwrap_or(None);
        self.0.kernel(owner.as_deref(), event);
    }
}
pub(crate) fn dispatch(history: crate::telemetry_history::History) -> tracing::Dispatch {
    tracing::Dispatch::new(tracing_subscriber::registry().with(CorrelatedLayer(
        ReportGeneratorLayer::new(Arc::new(Reporter(history))),
    )))
}

/// A native span can close when its task's completed future is dropped, after the last
/// poll has left its task-local scope. Bind the owner at span creation and restore it
/// for the upstream reporter callback; do not infer identity from whichever task drops it.
struct CorrelatedLayer(ReportGeneratorLayer);
impl<S> tracing_subscriber::Layer<S> for CorrelatedLayer
where
    S: tracing::Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
{
    fn on_new_span(
        &self,
        attrs: &tracing::span::Attributes<'_>,
        id: &tracing::span::Id,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        if let Some(span) = ctx.span(id) {
            span.extensions_mut()
                .insert(crate::runtime::capture_operation());
        }
        self.0.on_new_span(attrs, id, ctx);
    }
    fn on_record(
        &self,
        id: &tracing::span::Id,
        values: &tracing::span::Record<'_>,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        self.0.on_record(id, values, ctx);
    }
    fn on_event(&self, event: &tracing::Event<'_>, ctx: tracing_subscriber::layer::Context<'_, S>) {
        self.0.on_event(event, ctx);
    }
    fn on_enter(&self, id: &tracing::span::Id, ctx: tracing_subscriber::layer::Context<'_, S>) {
        self.0.on_enter(id, ctx);
    }
    fn on_close(&self, id: tracing::span::Id, ctx: tracing_subscriber::layer::Context<'_, S>) {
        let operation = ctx.span(&id).and_then(|span| {
            span.extensions()
                .get::<crate::runtime::OperationContext>()
                .cloned()
        });
        let owner = operation
            .as_ref()
            .and_then(|operation| operation.id().map(str::to_owned));
        // Captured absence is authoritative too. A span without an owner must not
        // borrow the operation of the task that happens to close its final handle.
        METRIC_OWNER.sync_scope(owner, || self.0.on_close(id, ctx));
    }
}

#[derive(Default)]
struct Capture {
    truncated: bool,
}
impl Capture {
    fn text(&mut self, value: Option<&str>) -> Option<String> {
        value.map(|value| {
            let end = value.floor_char_boundary(value.len().min(1024));
            self.truncated |= end != value.len();
            value[..end].to_owned()
        })
    }
    fn context(&mut self, id: MetricId, correlation: Option<&str>, kind: TableType) -> Context {
        Context {
            kernel_operation_id: id.as_bytes(),
            correlation_id: self.text(correlation),
            table_kind: match kind {
                TableType::PathBased => TableKind::PathBased,
                TableType::CatalogManaged => TableKind::CatalogManaged,
            },
        }
    }
}

pub(crate) fn capture(event: MetricEvent) -> Observation {
    let mut captured = Capture::default();
    macro_rules! context {
        ($value:ident) => {
            captured.context(
                $value.operation_id,
                $value.correlation_id.as_deref(),
                $value.table_type,
            )
        };
    }
    let metric = match event {
        MetricEvent::LogSegmentLoadSuccess(v) => Metric::LogSegmentLoadSuccess {
            context: context!(v),
            num_commit_files: v.num_commit_files,
            num_checkpoint_files: v.num_checkpoint_files,
            num_compaction_files: v.num_compaction_files,
            has_latest_crc_file: v.has_latest_crc_file,
            duration: v.duration.into(),
        },
        MetricEvent::LogSegmentLoadFailure(v) => Metric::LogSegmentLoadFailure {
            context: context!(v),
        },
        MetricEvent::ProtocolMetadataLoadSuccess(v) => Metric::ProtocolMetadataLoadSuccess {
            context: context!(v),
            duration: v.duration.into(),
        },
        MetricEvent::ProtocolMetadataLoadFailure(v) => Metric::ProtocolMetadataLoadFailure {
            context: context!(v),
        },
        MetricEvent::SnapshotBuildSuccess(v) => Metric::SnapshotBuildSuccess {
            context: context!(v),
            version: v.version,
            duration: v.duration.into(),
        },
        MetricEvent::SnapshotBuildFailure(v) => Metric::SnapshotBuildFailure {
            context: context!(v),
        },
        MetricEvent::TransactionCommitSuccess(v) => Metric::TransactionCommitSuccess {
            context: context!(v),
            commit_version: v.commit_version,
            num_add_files: v.num_add_files,
            num_remove_files: v.num_remove_files,
            num_dv_updates: v.num_dv_updates,
            add_files_bytes: v.add_files_bytes,
            remove_files_bytes: v.remove_files_bytes,
            is_blind_append: v.is_blind_append,
            data_change: v.data_change,
            operation: captured.text(v.operation.as_deref()),
            prepare_duration: v.prepare_duration.into(),
            committer_duration: v.committer_duration.into(),
            total_duration: v.total_duration.into(),
        },
        MetricEvent::TransactionCommitFailure(v) => Metric::TransactionCommitFailure {
            context: context!(v),
            reason: match v.reason {
                CommitFailureReason::Conflict => CommitFailure::Conflict,
                CommitFailureReason::RetryableIo => CommitFailure::RetryableIo,
                CommitFailureReason::Error => CommitFailure::Error,
            },
        },
        MetricEvent::DomainMetadataLoadSuccess(v) => Metric::DomainMetadataLoadSuccess {
            from_cache: v.from_cache,
            num_domains_returned: v.num_domains_returned,
            duration: v.duration.into(),
        },
        MetricEvent::DomainMetadataLoadFailure => Metric::DomainMetadataLoadFailure,
        MetricEvent::SetTransactionLoadSuccess(v) => Metric::SetTransactionLoadSuccess {
            from_cache: v.from_cache,
            found: v.found,
            duration: v.duration.into(),
        },
        MetricEvent::SetTransactionLoadFailure => Metric::SetTransactionLoadFailure,
        MetricEvent::CrcReadSuccess(v) => Metric::CrcReadSuccess {
            bytes_read: v.bytes_read,
            duration: v.duration.into(),
        },
        MetricEvent::CrcReadFailure => Metric::CrcReadFailure,
        MetricEvent::JsonReadCompleted(v) => Metric::JsonReadCompleted {
            num_files: v.num_files,
            bytes_read: v.bytes_read,
        },
        MetricEvent::ParquetReadCompleted(v) => Metric::ParquetReadCompleted {
            num_files: v.num_files,
            bytes_read: v.bytes_read,
        },
        MetricEvent::ScanMetadataCompleted(v) => Metric::ScanMetadataCompleted {
            context: context!(v),
            scan_kind: match v.scan_type {
                ScanType::SequentialPhase => ScanKind::Sequential,
                ScanType::ParallelPhase => ScanKind::Parallel,
                ScanType::Full => ScanKind::Full,
            },
            duration: v.duration.into(),
            num_add_files_seen: v.num_add_files_seen,
            num_active_add_files: v.num_active_add_files,
            active_add_files_bytes: v.active_add_files_bytes,
            num_remove_files_seen: v.num_remove_files_seen,
            num_non_file_actions: v.num_non_file_actions,
            num_predicate_filtered: v.num_predicate_filtered,
            peak_hash_set_size: v.peak_hash_set_size,
            dedup_visitor_time: v.dedup_visitor_time.into(),
            predicate_eval_time: v.predicate_eval_time.into(),
        },
        MetricEvent::StorageListCompleted(v) => Metric::StorageListCompleted {
            duration: v.duration.into(),
            num_files: v.num_files,
        },
        MetricEvent::StorageReadCompleted(v) => Metric::StorageReadCompleted {
            duration: v.duration.into(),
            num_files: v.num_files,
            bytes_read: v.bytes_read,
        },
        MetricEvent::StorageCopyCompleted(v) => Metric::StorageCopyCompleted {
            duration: v.duration.into(),
        },
    };
    Observation {
        metric,
        attributes_truncated: captured.truncated,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        native_delta::{DeltaStore, StorageContract},
        runtime::QueryRuntime,
    };
    use arrow::{
        array::Int64Array,
        datatypes::{DataType, Field, Schema},
        record_batch::RecordBatch,
    };
    use datafusion::error::Result;

    #[test]
    fn span_without_an_owner_does_not_borrow_its_closing_operation() -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        let work = runtime.clone();
        runtime.bootstrap(async move {
            let dispatch = dispatch(work.native_history());
            let unowned = tracing::dispatcher::with_default(&dispatch, || {
                tracing::info_span!(
                    "json_read_completed",
                    num_files = 1_u64,
                    bytes_read = 7_u64,
                    report = true
                )
            });
            work.job_operation(
                "different-owner".into(),
                enrichment_core::telemetry::OperationDescriptor {
                    method: "fixture.unowned_metric".into(),
                    request_digest: "a".repeat(64),
                    policy_digest: "b".repeat(64),
                },
                std::time::Duration::from_secs(30),
                async move {
                    drop(unowned);
                },
            )
            .await;
            let rows = work.kernel_diagnostics().await?;
            assert_eq!(rows.len(), 1);
            assert_eq!(rows[0].operation_id, None);
            assert!(matches!(
                &rows[0].value.metric,
                Metric::JsonReadCompleted {
                    num_files: 1,
                    bytes_read: 7
                }
            ));
            work.close_diagnostics().await
        })?
    }

    #[test]
    fn real_kernel_events_keep_operation_identity_without_recursive_history() -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(&root.path().join("spill"), Default::default())?;
        let work = runtime.clone();
        runtime.bootstrap(async move {
            work.job_operation(
                "kernel-metrics-owner".into(),
                enrichment_core::telemetry::OperationDescriptor {
                    method: "fixture.kernel_metrics".into(),
                    request_digest: "a".repeat(64), policy_digest: "b".repeat(64),
                },
                std::time::Duration::from_secs(30),
                async {
                    let store = DeltaStore::new(&root.path().join("tables"), work.clone())?;
                    let schema = Arc::new(Schema::new(vec![Field::new("value", DataType::Int64, false)]));
                    let contract = StorageContract::new(schema.clone())?;
                    let table = store.create("measured", &contract, false).await?;
                    let input = RecordBatch::try_new(schema, vec![Arc::new(Int64Array::from(vec![7_i64]))])?;
                    let table = store.append(table, &contract, crate::native_catalog::batch(&store.session(), "kernel_metrics", input)?, vec![]).await?;
                    let result = work.execute(store.session().read_table(store.provider(&table, &contract).await?)?).await?;
                    assert_eq!(result.rows, 1);
                    Ok::<_, datafusion::error::DataFusionError>(())
                },
            ).await?;
            let first = work.kernel_diagnostics().await?;
            assert!(!first.is_empty(), "actual kernel callbacks were not observed");
            assert!(first.iter().all(|row| row.operation_id.as_deref() == Some("kernel-metrics-owner")));
            assert!(first.iter().any(|row| matches!(&row.value.metric,
                Metric::SnapshotBuildSuccess { context, .. } if context.kernel_operation_id != [0; 16]
            )));
            assert!(first.iter().any(|row| matches!(&row.value.metric,
                Metric::ScanMetadataCompleted { num_active_add_files: 1, .. }
            )));
            let second = work.kernel_diagnostics().await?;
            assert_eq!(first, second, "diagnostic reads must not observe their own native work");
            let (first, second) = tokio::join!(work.close_diagnostics(), work.close_diagnostics());
            first?;
            second
        })?
    }
}
