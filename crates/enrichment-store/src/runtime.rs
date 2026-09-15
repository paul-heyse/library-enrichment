//! Shared DataFusion resources and bounded result consumption (ADR-0024).

use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, Instant};

use arrow::record_batch::RecordBatch;
use datafusion::dataframe::DataFrame;
use datafusion::error::{DataFusionError, Result};
use datafusion::execution::{
    SessionState, SessionStateBuilder, memory_pool::FairSpillPool, runtime_env::RuntimeEnvBuilder,
};
use datafusion::prelude::{SessionConfig, SessionContext};
use futures::TryStreamExt;
use tokio::sync::Semaphore;

/// Bounds apply to the shared daemon runtime, not independently to every request.
#[derive(Debug, Clone)]
pub struct QueryLimits {
    pub memory_bytes: usize,
    pub spill_bytes: u64,
    pub metadata_cache_bytes: usize,
    pub batch_rows: usize,
    pub partitions: usize,
    pub concurrency: usize,
    pub result_rows: usize,
    pub result_bytes: usize,
    pub deadline: Duration,
}

impl Default for QueryLimits {
    fn default() -> Self {
        Self {
            memory_bytes: 128 * 1024 * 1024,
            spill_bytes: 512 * 1024 * 1024,
            metadata_cache_bytes: 16 * 1024 * 1024,
            batch_rows: 1024,
            partitions: 2,
            concurrency: 4,
            result_rows: 10_000,
            result_bytes: 16 * 1024 * 1024,
            deadline: Duration::from_secs(30),
        }
    }
}

/// One shared resource pool; each operation receives an isolated registration namespace.
#[derive(Clone)]
pub struct QueryRuntime {
    template: SessionState,
    permits: Arc<Semaphore>,
    limits: QueryLimits,
    diagnostics: crate::query_diagnostics::History,
}

/// Execution observations are operational diagnostics, not library evidence coverage.
pub struct QueryOutput {
    pub batches: Vec<RecordBatch>,
    pub rows: usize,
    pub arrow_bytes: usize,
    pub elapsed: Duration,
}

impl QueryRuntime {
    /// Complete multi-query operations share this elapsed-time budget.
    #[must_use]
    pub fn deadline(&self) -> Duration {
        self.limits.deadline
    }
    /// Construct bounded shared resources using an explicitly service-owned spill root.
    ///
    /// # Errors
    /// Invalid limits or unavailable spill storage prevent admission.
    pub fn new(spill_root: &Path, limits: QueryLimits) -> Result<Self> {
        if limits.memory_bytes == 0
            || limits.spill_bytes == 0
            || limits.batch_rows == 0
            || limits.partitions == 0
            || limits.concurrency == 0
            || limits.result_rows == 0
            || limits.result_bytes == 0
            || limits.deadline.is_zero()
        {
            return Err(DataFusionError::Configuration(
                "query limits must be positive".into(),
            ));
        }
        std::fs::create_dir_all(spill_root)?;
        let runtime = RuntimeEnvBuilder::new()
            .with_memory_pool(Arc::new(FairSpillPool::new(limits.memory_bytes)))
            .with_temp_file_path(spill_root)
            .with_max_temp_directory_size(limits.spill_bytes)
            .with_max_spill_merge_fan_in(8)
            .with_metadata_cache_limit(limits.metadata_cache_bytes)
            .build_arc()?;
        let config = SessionConfig::new()
            .with_batch_size(limits.batch_rows)
            .with_target_partitions(limits.partitions)
            .set_bool("datafusion.execution.parquet.skip_metadata", false);
        let template = SessionContext::new_with_config_rt(config, Arc::clone(&runtime)).state();
        let planner = Arc::new(crate::leases::RetentionPlanner {
            inner: Arc::clone(template.query_planner()),
        });
        let config = template.config().clone();
        let template = SessionStateBuilder::new_from_existing(template)
            .with_config(config)
            .with_query_planner(planner)
            .build();
        Ok(Self {
            template,
            permits: Arc::new(Semaphore::new(limits.concurrency)),
            limits,
            diagnostics: Default::default(),
        })
    }

    /// Fresh scoped tables with shared spill, memory accounting and metadata cache.
    #[must_use]
    pub fn session(&self) -> SessionContext {
        use datafusion::catalog::{
            CatalogProvider, CatalogProviderList, MemoryCatalogProvider, MemoryCatalogProviderList,
            MemorySchemaProvider,
        };
        // Clone only pristine defaults and shared function/runtime objects. Every request gets
        // a new registration namespace; no request table or execution plan enters the template.
        let mut state = self.template.clone();
        let names = &state.config_options().catalog;
        let catalogs = Arc::new(MemoryCatalogProviderList::new());
        let catalog = Arc::new(MemoryCatalogProvider::new());
        catalog
            .register_schema(&names.default_schema, Arc::new(MemorySchemaProvider::new()))
            .expect("registering a fresh native memory schema cannot fail");
        catalogs.register_catalog(names.default_catalog.clone(), catalog);
        state.register_catalog_list(catalogs);
        state.mark_start_execution();
        SessionContext::new_with_state(state)
    }

    /// Last eight executed plans, bounded independently of indefinitely retained library facts.
    pub fn diagnostics(&self) -> Vec<crate::query_diagnostics::QueryDiagnostics> {
        self.diagnostics.read()
    }

    /// Cumulative observed query totals, separate from the bounded recent-plan history.
    pub fn diagnostic_summary(&self) -> crate::query_diagnostics::Summary {
        self.diagnostics.summary()
    }

    /// Bounded operational status copied from this runtime's actual counters and admission.
    pub fn operational_counters(&self) -> enrichment_core::wire::status::NativeQueryCounters {
        let summary = self.diagnostic_summary();
        enrichment_core::wire::status::NativeQueryCounters {
            executions: summary.executions,
            completed: summary.completed,
            incomplete: summary.executions.saturating_sub(summary.completed),
            planning_micros: summary.planning_micros,
            elapsed_micros: summary.elapsed_micros,
            admitted: self
                .limits
                .concurrency
                .saturating_sub(self.permits.available_permits()),
            concurrency_limit: self.limits.concurrency,
            managed_memory_limit_bytes: self.limits.memory_bytes,
            spill_limit_bytes: self.limits.spill_bytes,
            metadata_cache_limit_bytes: self.limits.metadata_cache_bytes,
        }
    }

    async fn plan(
        &self,
        frame: DataFrame,
        start: Instant,
    ) -> Result<(
        datafusion::physical_plan::SendableRecordBatchStream,
        crate::query_diagnostics::Trace,
    )> {
        use datafusion::physical_plan::{
            ExecutionPlanProperties, coalesce_partitions::CoalescePartitionsExec,
        };
        let planning = Instant::now();
        let (state, logical) = frame.into_parts();
        let optimized = state.optimize(&logical)?;
        let mut plan = state
            .query_planner()
            .create_physical_plan(&optimized, &state)
            .await?;
        if plan.output_partitioning().partition_count() > 1 {
            plan = Arc::new(CoalescePartitionsExec::new(plan));
        }
        let trace = crate::query_diagnostics::Trace::new(
            Arc::clone(&plan),
            &optimized,
            self.diagnostics.clone(),
            start,
            u64::try_from(planning.elapsed().as_micros()).unwrap_or(u64::MAX),
        );
        let stream = datafusion::physical_plan::execute_stream(plan, state.task_ctx())?;
        Ok((stream, trace))
    }

    /// Consume a finite plan under global concurrency, elapsed-time and output allocation bounds.
    /// The DataFusion pool does not account for every scan allocation; batch/output bounds are
    /// separate and callers must bound producer values and input file sizes at admission.
    ///
    /// # Errors
    /// Time/resource exhaustion is an explicit error, never an empty or truncated success.
    pub async fn execute(&self, frame: DataFrame) -> Result<QueryOutput> {
        let start = Instant::now();
        tokio::time::timeout(self.limits.deadline, async {
            let _permit = self
                .permits
                .acquire()
                .await
                .map_err(|e| DataFusionError::Execution(e.to_string()))?;
            let (stream, mut trace) = self.plan(frame, start).await?;
            let mut stream = stream;
            let mut batches = Vec::new();
            let mut rows = 0usize;
            let mut bytes = 0usize;
            while let Some(batch) = stream.try_next().await? {
                rows = rows
                    .checked_add(batch.num_rows())
                    .ok_or_else(|| budget("result rows"))?;
                bytes = bytes
                    .checked_add(batch.get_array_memory_size())
                    .ok_or_else(|| budget("result bytes"))?;
                if rows > self.limits.result_rows || bytes > self.limits.result_bytes {
                    return Err(budget("result rows or Arrow bytes"));
                }
                batches.push(batch);
            }
            drop(stream);
            trace.completed(rows, bytes);
            Ok(QueryOutput {
                batches,
                rows,
                arrow_bytes: bytes,
                elapsed: start.elapsed(),
            })
        })
        .await
        .map_err(|_| budget("deadline"))?
    }

    /// Visit bounded batches without retaining a complete validation/export result.
    /// The callback must consume the batch synchronously and must not accumulate unbounded data.
    /// # Errors
    /// Query failures, callback errors, elapsed-time and scan bounds fail the operation.
    pub async fn visit(
        &self,
        frame: DataFrame,
        max_rows: usize,
        mut consume: impl FnMut(&RecordBatch) -> Result<()>,
    ) -> Result<usize> {
        self.visit_async(frame, max_rows, |batch| std::future::ready(consume(&batch)))
            .await
    }

    /// Consume each bounded batch before polling the next, retaining the query permit and
    /// stream ownership through asynchronous validation or output I/O.
    pub async fn visit_async<F, Fut>(
        &self,
        frame: DataFrame,
        max_rows: usize,
        mut consume: F,
    ) -> Result<usize>
    where
        F: FnMut(RecordBatch) -> Fut,
        Fut: Future<Output = Result<()>>,
    {
        let start = Instant::now();
        tokio::time::timeout(self.limits.deadline, async {
            let _permit = self
                .permits
                .acquire()
                .await
                .map_err(|e| DataFusionError::Execution(e.to_string()))?;
            let (stream, mut trace) = self.plan(frame, start).await?;
            let mut stream = stream;
            let mut rows = 0usize;
            let mut bytes = 0usize;
            while let Some(batch) = stream.try_next().await? {
                rows = rows
                    .checked_add(batch.num_rows())
                    .filter(|n| *n <= max_rows)
                    .ok_or_else(|| budget("scan rows"))?;
                if batch.get_array_memory_size() > self.limits.result_bytes {
                    return Err(budget("scan batch bytes"));
                }
                bytes = bytes
                    .checked_add(batch.get_array_memory_size())
                    .ok_or_else(|| budget("scan bytes counter"))?;
                consume(batch).await?;
            }
            drop(stream);
            trace.completed(rows, bytes);
            Ok(rows)
        })
        .await
        .map_err(|_| budget("deadline"))?
    }
}

fn budget(resource: &str) -> DataFusionError {
    DataFusionError::ResourcesExhausted(format!("query exceeded {resource} budget"))
}
