//! Shared DataFusion resources and bounded result consumption (ADR-0024).

use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
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

tokio::task_local! { static OPERATION: Arc<Operation>; }

struct Operation {
    id: String,
    descriptor: OperationDescriptor,
    inputs: std::sync::Mutex<Vec<OperationInput>>,
    started: Instant,
    deadline: Duration,
    retained_charge: AtomicUsize,
    artifact_charge: AtomicUsize,
    byte_limit: usize,
    statistics: std::sync::Mutex<crate::query_diagnostics::Summary>,
    history: crate::query_diagnostics::History,
}

/// Bounded completion observations, attributed to the owning operation even with concurrent
/// clients. Native elapsed durations overlap; they are not an end-to-end wall clock or RSS.
#[derive(Debug, Clone, serde::Serialize)]
pub struct OperationDiagnostics {
    pub operation_id: String,
    pub request: OperationDescriptor,
    pub snapshots: Vec<SnapshotBinding>,
    pub owned_lifetime_micros: u64,
    pub retained_output_bytes: usize,
    pub artifact_bytes: usize,
    pub native: crate::query_diagnostics::Summary,
}

impl Drop for Operation {
    fn drop(&mut self) {
        self.history.operation(OperationDiagnostics {
            operation_id: self.id.clone(),
            request: self.descriptor.clone(),
            snapshots: self
                .inputs
                .lock()
                .map(|inputs| inputs.iter().map(|input| input.snapshot.clone()).collect())
                .unwrap_or_default(),
            owned_lifetime_micros: u64::try_from(self.started.elapsed().as_micros())
                .unwrap_or(u64::MAX),
            retained_output_bytes: self.retained_charge.load(Ordering::Acquire),
            artifact_bytes: self.artifact_charge.load(Ordering::Acquire),
            native: self
                .statistics
                .lock()
                .map(|stats| stats.clone())
                .unwrap_or_default(),
        });
    }
}

/// Immutable request and effective-policy identities bound before admitting any child work.
/// These are diagnostic identities, never authority to execute code or keys for a cache.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OperationDescriptor {
    pub method: String,
    pub request_digest: String,
    pub policy_digest: String,
}

struct OperationInput {
    snapshot: SnapshotBinding,
    _lease: Arc<std::fs::File>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SnapshotBinding {
    pub snapshot_id: String,
    pub context_id: String,
    pub environment_id: String,
    pub catalog_generation: u64,
    pub manifest_digest: String,
    pub projection_version: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OperationBinding {
    pub request: OperationDescriptor,
    pub snapshots: Vec<SnapshotBinding>,
    pub retained_byte_limit: usize,
    pub artifact_byte_limit: u64,
    pub deadline_millis: u64,
}

/// Keep the same admitted snapshot and its retention ownership through final result work,
/// including blocking projections after the native stream has released its own providers.
pub(crate) fn bind_snapshot(
    manifest: &enrichment_core::evidence::snapshot::EvidenceManifest,
    manifest_digest: &str,
    generation: u64,
    lease: Arc<std::fs::File>,
) -> Result<()> {
    OPERATION
        .try_with(|operation| {
            let snapshot = SnapshotBinding {
                snapshot_id: manifest.snapshot_id.to_string(),
                context_id: manifest.context_id.to_string(),
                environment_id: manifest.environment_id.to_string(),
                catalog_generation: generation,
                manifest_digest: manifest_digest.into(),
                projection_version: manifest.schema_version.clone(),
            };
            let mut inputs = operation
                .inputs
                .lock()
                .map_err(|_| DataFusionError::Internal("operation inputs poisoned".into()))?;
            if let Some(bound) = inputs.iter().find(|input| {
                input.snapshot.snapshot_id == snapshot.snapshot_id
                    && input.snapshot.catalog_generation == generation
            }) {
                if bound.snapshot != snapshot {
                    return Err(crate::preparation::InvariantFailure::error(
                        "immutable operation input",
                        "snapshot_binding",
                        vec![snapshot.snapshot_id],
                    ));
                }
            } else {
                if inputs.len() == 32 {
                    return Err(budget_limit("operation snapshots", 33, 32));
                }
                inputs.push(OperationInput {
                    snapshot,
                    _lease: lease,
                });
            }
            Ok(())
        })
        .unwrap_or(Ok(()))
}

pub(crate) fn operation_binding() -> Option<OperationBinding> {
    OPERATION
        .try_with(|operation| OperationBinding {
            request: operation.descriptor.clone(),
            snapshots: operation
                .inputs
                .lock()
                .map(|inputs| inputs.iter().map(|i| i.snapshot.clone()).collect())
                .unwrap_or_default(),
            retained_byte_limit: operation.byte_limit,
            artifact_byte_limit: crate::result::MAX_BYTES,
            deadline_millis: u64::try_from(operation.deadline.as_millis()).unwrap_or(u64::MAX),
        })
        .ok()
}

#[derive(Debug, thiserror::Error)]
#[error("native operation exceeded {rule} budget (observed {observed:?}, allowed {allowed:?})")]
pub struct BudgetFailure {
    pub rule: String,
    pub cause: enrichment_core::wire::DiagnosticCause,
    pub observed: Option<u64>,
    pub allowed: Option<u64>,
    pub operation_id: Option<String>,
}

/// Correlation is captured by the actual query trace, never reconstructed by rerunning a plan.
#[must_use]
pub fn operation_id() -> Option<String> {
    OPERATION.try_with(|operation| operation.id.clone()).ok()
}

/// Carry this operation's budget and correlation into owned blocking work. A plain Tokio
/// spawn_blocking call does not propagate task-local values.
#[derive(Clone)]
pub struct OperationContext(Option<Arc<Operation>>);

#[must_use]
pub fn capture_operation() -> OperationContext {
    OperationContext(OPERATION.try_with(Arc::clone).ok())
}

impl OperationContext {
    pub(crate) fn query(&self, query: &crate::query_diagnostics::QueryDiagnostics) {
        if let Some(operation) = &self.0
            && let Ok(mut statistics) = operation.statistics.lock()
        {
            statistics.query(query);
        }
    }
    pub(crate) fn index(&self, bytes: Option<u64>) {
        if let Some(operation) = &self.0
            && let Ok(mut statistics) = operation.statistics.lock()
        {
            if let Some(bytes) = bytes {
                statistics.index_materializations =
                    statistics.index_materializations.saturating_add(1);
                statistics.index_spill_bytes = statistics.index_spill_bytes.saturating_add(bytes);
            } else {
                statistics.index_reads = statistics.index_reads.saturating_add(1);
            }
        }
    }
    pub fn run<T>(self, work: impl FnOnce() -> T) -> T {
        match self.0 {
            Some(operation) => OPERATION.sync_scope(operation, work),
            None => work(),
        }
    }
}

/// Conservative charge for retained native DTO/result encoding, separate from Arrow's pool.
/// This is cumulative encoded-output accounting, not an estimate of process RSS.
pub fn charge_result(bytes: usize) -> Result<()> {
    charge_output(bytes)
}

pub(crate) fn remaining_artifact_bytes() -> usize {
    let limit = crate::result::MAX_BYTES as usize;
    OPERATION
        .try_with(|operation| {
            limit.saturating_sub(operation.artifact_charge.load(Ordering::Acquire))
        })
        .unwrap_or(limit)
}

/// Streamed presentation bytes have their own cumulative operation bound. They do not
/// remain as in-memory JSON, but repeated child queries cannot reset their disk allowance.
pub(crate) fn charge_artifact(bytes: usize) -> Result<()> {
    OPERATION
        .try_with(|operation| {
            if operation.started.elapsed() >= operation.deadline {
                return Err(deadline_budget("artifact delivery deadline"));
            }
            let limit = crate::result::MAX_BYTES as usize;
            operation
                .artifact_charge
                .fetch_update(Ordering::AcqRel, Ordering::Acquire, |used| {
                    used.checked_add(bytes).filter(|total| *total <= limit)
                })
                .map(|_| ())
                .map_err(|used| {
                    budget_limit(
                        "operation artifact bytes",
                        used.saturating_add(bytes),
                        limit,
                    )
                })
        })
        .unwrap_or(Ok(()))
}

fn charge_output(bytes: usize) -> Result<()> {
    OPERATION
        .try_with(|operation| {
            operation
                .retained_charge
                .fetch_update(Ordering::AcqRel, Ordering::Acquire, |used| {
                    used.checked_add(bytes)
                        .filter(|total| *total <= operation.byte_limit)
                })
                .map(|_| ())
                .map_err(|used| {
                    budget_limit(
                        "operation retained output bytes",
                        used.saturating_add(bytes),
                        operation.byte_limit,
                    )
                })
        })
        .unwrap_or(Ok(()))
}

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
    operations: Arc<Semaphore>,
    limits: QueryLimits,
    diagnostics: crate::query_diagnostics::History,
}

/// Execution observations are operational diagnostics, not library evidence coverage.
#[derive(Debug)]
pub struct QueryOutput {
    pub batches: Vec<RecordBatch>,
    pub rows: usize,
    pub arrow_bytes: usize,
    pub elapsed: Duration,
}

impl QueryRuntime {
    #[must_use]
    pub fn operation_diagnostics(&self) -> Vec<OperationDiagnostics> {
        self.diagnostics.operations()
    }
    pub(crate) fn index_history(&self) -> crate::query_diagnostics::History {
        self.diagnostics.clone()
    }
    /// Bound finite blocking I/O with the same shared work admission. The worker owns its
    /// permit until it actually exits, even if a caller disconnects or its outer deadline
    /// expires. The callback must not start nested queries on this admission pool.
    pub fn blocking<T: Send + 'static>(
        &self,
        work: impl FnOnce() -> T + Send + 'static,
    ) -> futures::future::BoxFuture<'_, Result<T>> {
        // Catalog/publication callbacks can contain large native values. Keep this admission
        // boundary's future and callback off the caller's stack before it is first polled.
        let work: Box<dyn FnOnce() -> T + Send> = Box::new(work);
        Box::pin(async move {
            let permit =
                tokio::time::timeout(self.remaining(), Arc::clone(&self.permits).acquire_owned())
                    .await
                    .map_err(|_| deadline_budget("blocking work admission deadline"))?
                    .map_err(|error| DataFusionError::Execution(error.to_string()))?;
            let operation = capture_operation();
            tokio::task::spawn_blocking(move || {
                let _permit = permit;
                operation.run(work)
            })
            .await
            .map_err(|error| DataFusionError::Execution(error.to_string()))
        })
    }

    /// A durable job owns a separate operation scope. Its producer runner already owns
    /// queue admission, cancellation and cleanup; dropping that runner on an outer timeout
    /// would break cleanup guarantees. Native child queries inherit the job's remaining
    /// deadline and cumulative output charge, while producer deadlines retain their owner.
    pub async fn job_operation<T>(
        &self,
        id: String,
        descriptor: OperationDescriptor,
        deadline: Duration,
        work: impl Future<Output = T>,
    ) -> T {
        let operation = Arc::new(Operation {
            id,
            descriptor,
            inputs: std::sync::Mutex::new(Vec::new()),
            started: Instant::now(),
            deadline,
            retained_charge: AtomicUsize::new(0),
            artifact_charge: AtomicUsize::new(0),
            byte_limit: self.limits.result_bytes,
            statistics: std::sync::Mutex::new(crate::query_diagnostics::Summary::default()),
            history: self.diagnostics.clone(),
        });
        OPERATION.scope(operation, work).await
    }
    /// One end-to-end budget includes queueing, all child queries and result preparation.
    /// Output allocations remain conservatively charged until the operation ends, including
    /// batches moved into native projections. This bounds cumulative materialization; it does
    /// not claim to measure process RSS or return memory credit for early-dropped projections.
    pub async fn operation<T>(
        &self,
        id: String,
        descriptor: OperationDescriptor,
        work: impl Future<Output = T>,
    ) -> Result<T> {
        if OPERATION.try_with(|_| ()).is_ok() {
            return Ok(work.await);
        }
        let operation = Arc::new(Operation {
            id,
            descriptor,
            inputs: std::sync::Mutex::new(Vec::new()),
            started: Instant::now(),
            deadline: self.limits.deadline,
            retained_charge: AtomicUsize::new(0),
            artifact_charge: AtomicUsize::new(0),
            byte_limit: self.limits.result_bytes,
            statistics: std::sync::Mutex::new(crate::query_diagnostics::Summary::default()),
            history: self.diagnostics.clone(),
        });
        OPERATION
            .scope(operation, async {
                tokio::time::timeout(self.limits.deadline, async {
                    // A separate admission pool avoids reacquiring an operation's own permit in
                    // each child query. Both pools release permits on cancellation and failure.
                    let _permit = self
                        .operations
                        .acquire()
                        .await
                        .map_err(|e| DataFusionError::Execution(e.to_string()))?;
                    Ok(work.await)
                })
                .await
                .map_err(|_| {
                    deadline_budget("operation deadline including queue and result work")
                })?
            })
            .await
            .map_err(|error| self.failed(error))
    }

    fn remaining(&self) -> Duration {
        OPERATION
            .try_with(|operation| {
                operation
                    .deadline
                    .saturating_sub(operation.started.elapsed())
                    .min(self.limits.deadline)
            })
            .unwrap_or(self.limits.deadline)
    }
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
            operations: Arc::new(Semaphore::new(limits.concurrency)),
            limits,
            diagnostics: crate::query_diagnostics::History::persistent(
                spill_root.join("query-failures.json"),
            )?,
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

    /// Persist bounded failure context beyond the process-local eight-plan ring.
    pub fn record_failure(&self, diagnostic: enrichment_core::wire::Diagnostic) {
        self.diagnostics.failure(diagnostic);
    }

    fn failed(&self, error: DataFusionError) -> DataFusionError {
        let query = crate::query::QueryError::from(error);
        self.record_failure(query.diagnostic());
        match query {
            crate::query::QueryError::DataFusion(error) => error,
            _ => unreachable!("constructed DataFusion failure"),
        }
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
        family: Option<crate::preparation::QueryFamily>,
    ) -> Result<(
        datafusion::physical_plan::SendableRecordBatchStream,
        crate::query_diagnostics::Trace,
    )> {
        use datafusion::physical_plan::{
            ExecutionPlanProperties, coalesce_partitions::CoalescePartitionsExec,
        };
        let planning = Instant::now();
        let (state, logical) = frame.into_parts();
        let mut trace =
            crate::query_diagnostics::Trace::new(&logical, self.diagnostics.clone(), start);
        if let Some(family) = family {
            trace.family(family);
            family.require(logical.schema().as_arrow())?;
        }
        let analyzed = state.analyzer().execute_and_check(
            logical.clone(),
            state.config_options(),
            |_, _| {},
        )?;
        crate::preparation::result(
            analyzed.schema().as_arrow(),
            logical.schema().as_arrow(),
            "analyzed_logical",
        )?;
        trace.analyzed(
            &analyzed,
            u64::try_from(planning.elapsed().as_micros()).unwrap_or(u64::MAX),
        );
        let optimizing = Instant::now();
        let optimized = state.optimizer().optimize(analyzed, &state, |_, _| {})?;
        trace.optimized(
            &optimized,
            u64::try_from(optimizing.elapsed().as_micros()).unwrap_or(u64::MAX),
        );
        crate::preparation::result(
            optimized.schema().as_arrow(),
            logical.schema().as_arrow(),
            "optimized_logical",
        )?;
        let mut plan = state
            .query_planner()
            .create_physical_plan(&optimized, &state)
            .await?;
        crate::preparation::result(
            &plan.schema(),
            optimized.schema().as_arrow(),
            "physical_plan",
        )?;
        if plan.output_partitioning().partition_count() > 1 {
            plan = Arc::new(CoalescePartitionsExec::new(plan));
        }
        trace.physical(
            Arc::clone(&plan),
            u64::try_from(planning.elapsed().as_micros()).unwrap_or(u64::MAX),
        );
        let expected = plan.schema();
        let stream = datafusion::physical_plan::execute_stream(plan, state.task_ctx())?;
        crate::preparation::physical(&stream.schema(), &expected, "execution_stream")?;
        Ok((stream, trace))
    }

    /// Consume a finite plan under global concurrency, elapsed-time and output allocation bounds.
    /// The DataFusion pool does not account for every scan allocation; batch/output bounds are
    /// separate and callers must bound producer values and input file sizes at admission.
    ///
    /// # Errors
    /// Time/resource exhaustion is an explicit error, never an empty or truncated success.
    pub async fn execute(&self, frame: DataFrame) -> Result<QueryOutput> {
        self.execute_family(frame, None).await
    }

    /// Evaluate a trusted invariant relation containing one offending identity column.
    /// Native projection and limit bound diagnostic hydration before execution. An empty
    /// relation proves only this invariant; row IDs remain witnesses, not schema metadata.
    pub async fn require_empty(&self, frame: DataFrame, rule: &str, stage: &str) -> Result<()> {
        if frame.schema().fields().len() != 1 {
            return Err(self.failed(crate::preparation::InvariantFailure::contract(
                "invariant query must select one identity column",
                stage,
                Vec::new(),
            )));
        }
        let (qualifier, field) = frame.schema().qualified_field(0);
        let key = datafusion::common::Column::new(qualifier.cloned(), field.name());
        let frame = frame
            .select(vec![
                datafusion::logical_expr::Expr::Column(key).alias("witness_id"),
            ])?
            .limit(0, Some(8))?;
        let output = self
            .execute_family(
                frame,
                Some(crate::preparation::QueryFamily::InvariantWitness),
            )
            .await?;
        if output.rows != 0 {
            return Err(self.failed(crate::preparation::InvariantFailure::error(
                rule,
                stage,
                crate::preparation::witnesses(&output.batches),
            )));
        }
        Ok(())
    }

    /// Execute a domain query with independently declared decoder requirements.
    pub async fn execute_family(
        &self,
        frame: DataFrame,
        family: Option<crate::preparation::QueryFamily>,
    ) -> Result<QueryOutput> {
        let start = Instant::now();
        tokio::time::timeout(self.remaining(), async {
            let _permit = self
                .permits
                .acquire()
                .await
                .map_err(|e| DataFusionError::Execution(e.to_string()))?;
            let (stream, mut trace) = self.plan(frame, start, family).await?;
            let expected = stream.schema();
            let mut stream = stream;
            let mut batches = Vec::new();
            let mut rows = 0usize;
            let mut bytes = 0usize;
            while let Some(batch) = stream.try_next().await? {
                crate::preparation::physical(&batch.schema(), &expected, "result_batch")?;
                rows = rows
                    .checked_add(batch.num_rows())
                    .ok_or_else(|| budget("result rows"))?;
                bytes = bytes
                    .checked_add(batch.get_array_memory_size())
                    .ok_or_else(|| budget("result bytes"))?;
                if rows > self.limits.result_rows || bytes > self.limits.result_bytes {
                    return Err(if rows > self.limits.result_rows {
                        budget_limit("result rows", rows, self.limits.result_rows)
                    } else {
                        budget_limit("result Arrow bytes", bytes, self.limits.result_bytes)
                    });
                }
                charge_output(batch.get_array_memory_size())?;
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
        .map_err(|_| deadline_budget("query deadline"))
        .and_then(std::convert::identity)
        .map_err(|error| self.failed(error))
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
        self.consume(frame, None, max_rows, (), |(), batch, _permit| {
            consume(batch)
        })
        .await
        .map(|(rows, ())| rows)
    }

    /// Fold one native batch at a time into a bounded projection or artifact sink. A blocking
    /// writer retains the existing query permit until it actually exits, even after timeout.
    /// No second permit is acquired and no full Arrow result is collected ahead of delivery.
    pub async fn fold_blocking<T, F>(
        &self,
        frame: DataFrame,
        family: crate::preparation::QueryFamily,
        max_rows: usize,
        initial: T,
        consume: F,
    ) -> Result<T>
    where
        T: Send + 'static,
        F: Fn(T, &RecordBatch) -> Result<T> + Clone + Send + 'static,
    {
        self.consume(
            frame,
            Some(family),
            max_rows,
            initial,
            |state, batch, permit| {
                let consume = consume.clone();
                let operation = capture_operation();
                async move {
                    tokio::task::spawn_blocking(move || {
                        let _permit = permit;
                        operation.run(|| consume(state, &batch))
                    })
                    .await
                    .map_err(|e| DataFusionError::Execution(e.to_string()))?
                }
            },
        )
        .await
        .map(|(_, state)| state)
    }

    async fn consume<T, F, Fut>(
        &self,
        frame: DataFrame,
        family: Option<crate::preparation::QueryFamily>,
        max_rows: usize,
        mut state: T,
        mut consume: F,
    ) -> Result<(usize, T)>
    where
        F: FnMut(T, RecordBatch, Arc<tokio::sync::OwnedSemaphorePermit>) -> Fut,
        Fut: Future<Output = Result<T>>,
    {
        let start = Instant::now();
        tokio::time::timeout(self.remaining(), async {
            let permit = Arc::new(
                Arc::clone(&self.permits)
                    .acquire_owned()
                    .await
                    .map_err(|e| DataFusionError::Execution(e.to_string()))?,
            );
            let (stream, mut trace) = self.plan(frame, start, family).await?;
            let expected = stream.schema();
            let mut stream = stream;
            let mut rows = 0usize;
            let mut bytes = 0usize;
            while let Some(batch) = stream.try_next().await? {
                crate::preparation::physical(&batch.schema(), &expected, "result_batch")?;
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
                state = consume(state, batch, Arc::clone(&permit)).await?;
            }
            drop(stream);
            trace.completed(rows, bytes);
            Ok((rows, state))
        })
        .await
        .map_err(|_| deadline_budget("scan deadline"))
        .and_then(std::convert::identity)
        .map_err(|error| self.failed(error))
    }
}

fn budget(resource: &str) -> DataFusionError {
    DataFusionError::External(Box::new(BudgetFailure {
        rule: resource.into(),
        cause: enrichment_core::wire::DiagnosticCause::Capacity,
        observed: None,
        allowed: None,
        operation_id: operation_id(),
    }))
}

fn budget_limit(resource: &str, observed: usize, allowed: usize) -> DataFusionError {
    DataFusionError::External(Box::new(BudgetFailure {
        rule: resource.into(),
        cause: enrichment_core::wire::DiagnosticCause::Capacity,
        observed: Some(observed as u64),
        allowed: Some(allowed as u64),
        operation_id: operation_id(),
    }))
}

fn deadline_budget(resource: &str) -> DataFusionError {
    DataFusionError::External(Box::new(BudgetFailure {
        rule: resource.into(),
        cause: enrichment_core::wire::DiagnosticCause::Deadline,
        observed: None,
        allowed: None,
        operation_id: operation_id(),
    }))
}
