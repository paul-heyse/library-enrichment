//! Shared DataFusion resources and bounded result consumption (ADR-0024).

use enrichment_core::telemetry::{
    OperationBinding, OperationDescriptor, OperationDiagnostics, OperationEnd, SnapshotBinding,
};
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

use arrow::record_batch::RecordBatch;
use datafusion::dataframe::DataFrame;
use datafusion::error::{DataFusionError, Result};
use datafusion::execution::{
    SessionState, SessionStateBuilder,
    memory_pool::{FairSpillPool, PeakRecordingPool},
    runtime_env::RuntimeEnvBuilder,
};
use datafusion::prelude::{SessionConfig, SessionContext};
use futures::TryStreamExt;
use tokio::sync::Semaphore;

/// Complete build-derived local source/contract and dependency closure. Input snapshots and
/// effective runtime settings are separately bound by each native operation descriptor.
pub const DEFINITION_REVISION: &str =
    concat!("native-runtime/1/", env!("ENR_NATIVE_SOURCE_DIGEST"));
pub const SOURCE_RECEIPT: &str =
    include_str!(concat!(env!("OUT_DIR"), "/native-source-receipt.json"));

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
    history: crate::telemetry_history::History,
}

/// Bounded completion observations, attributed to the owning operation even with concurrent
/// clients. Native elapsed durations overlap; they are not an end-to-end wall clock or RSS.
impl Drop for Operation {
    fn drop(&mut self) {
        self.history.operation(OperationEnd {
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
        });
    }
}

/// Immutable request and effective-policy identities bound before admitting any child work.
/// These are diagnostic identities, never authority to execute code or keys for a cache.
struct OperationInput {
    snapshot: SnapshotBinding,
    _lease: Arc<std::fs::File>,
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
                control: generation,
                manifest_digest: manifest_digest.into(),
                projection_version: manifest.schema_version.clone(),
            };
            let mut inputs = operation
                .inputs
                .lock()
                .map_err(|_| DataFusionError::Internal("operation inputs poisoned".into()))?;
            if let Some(bound) = inputs.iter().find(|input| {
                input.snapshot.snapshot_id == snapshot.snapshot_id
                    && input.snapshot.control == generation
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
    pub(crate) fn id(&self) -> Option<&str> {
        self.0.as_ref().map(|operation| operation.id.as_str())
    }
    fn spawn<T: Send + 'static>(
        self,
        work: impl std::future::Future<Output = T> + Send + 'static,
    ) -> datafusion::common::runtime::SpawnedTask<T> {
        let work = Box::pin(work);
        datafusion::common::runtime::SpawnedTask::spawn(self.scope(work))
    }
    pub(crate) async fn scope<T>(self, work: impl Future<Output = T>) -> T {
        match self.0 {
            Some(operation) => OPERATION.scope(operation, work).await,
            None => work.await,
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
    pub native: enrichment_core::config::NativeQueryConfig,
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
            native: enrichment_core::config::NativeQueryConfig::default(),
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
    executor: Arc<NativeExecutor>,
    template: SessionState,
    permits: Arc<Semaphore>,
    operations: Arc<Semaphore>,
    limits: Arc<QueryLimits>,
    diagnostics: crate::telemetry_history::History,
}

/// One executor per shared query runtime, independent of a transport/test caller's stack.
/// Every owned task and blocking callback retains this owner until it actually exits.
struct NativeExecutor(Option<tokio::runtime::Runtime>);
impl Drop for NativeExecutor {
    fn drop(&mut self) {
        if let Some(runtime) = self.0.take() {
            // The last owner can exit on a native worker. Tokio cannot synchronously join
            // that worker from itself. This requests shutdown without an async-context panic;
            // running blocking callbacks retain their owner and permits until their exit.
            runtime.shutdown_background();
        }
    }
}

/// Completed native fold facts describe the stream actually consumed, before pagination reuse.
pub struct FoldOutput<T> {
    pub rows: usize,
    pub value: T,
    pub properties: Arc<datafusion::physical_plan::PlanProperties>,
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
    pub(crate) fn executor_handle(&self) -> tokio::runtime::Handle {
        self.executor
            .0
            .as_ref()
            .expect("live native executor")
            .handle()
            .clone()
    }

    /// Run owned native work on the configured service executor with abort-on-drop semantics.
    /// Starting a task does not grant operation/effect admission; those retain their own guards.
    pub fn spawn<T: Send + 'static>(
        &self,
        work: impl Future<Output = T> + Send + 'static,
    ) -> datafusion::common::runtime::SpawnedTask<T> {
        // Native startup/planning futures can be large. Move them to the heap before
        // composing task-local and tracing wrappers on the transport caller's stack.
        let work = Box::pin(work);
        let handle = self.executor_handle();
        let _enter = handle.enter();
        let executor = Arc::clone(&self.executor);
        let effects = crate::native_effect::capture();
        capture_operation().spawn(async move {
            let _executor = executor;
            effects.scope(work).await
        })
    }

    fn spawn_blocking<T: Send + 'static>(
        &self,
        work: impl FnOnce() -> T + Send + 'static,
    ) -> datafusion::common::runtime::SpawnedTask<T> {
        let handle = self.executor_handle();
        let _enter = handle.enter();
        let executor = Arc::clone(&self.executor);
        let operation = capture_operation();
        let effects = crate::native_effect::capture();
        datafusion::common::runtime::SpawnedTask::spawn_blocking(move || {
            let _executor = executor;
            operation.run(|| effects.run(work))
        })
    }

    /// Complete finite startup recovery on the shared executor. Never poll native SQL on the
    /// synchronous caller or create a second recovery runtime.
    pub fn bootstrap<T: Send + 'static>(
        &self,
        work: impl Future<Output = T> + Send + 'static,
    ) -> Result<T> {
        if tokio::runtime::Handle::try_current()
            .is_ok_and(|handle| handle.id() == self.executor_handle().id())
        {
            return Err(DataFusionError::Execution(
                "cannot synchronously bootstrap from a native worker".into(),
            ));
        }
        futures::executor::block_on(self.spawn(Box::pin(work)))
            .map_err(|error| DataFusionError::Execution(format!("native startup task: {error}")))
    }
    pub async fn operation_diagnostics(&self) -> Result<Vec<OperationDiagnostics>> {
        self.diagnostics.operations().await
    }
    pub(crate) fn index_history(&self) -> crate::telemetry_history::History {
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
                tokio::time::timeout(self.remaining()?, Arc::clone(&self.permits).acquire_owned())
                    .await
                    .map_err(|_| deadline_budget("blocking work admission deadline"))?
                    .map_err(|error| DataFusionError::Execution(error.to_string()))?;
            self.spawn_blocking(move || {
                let _permit = permit;
                work()
            })
            .await
            .map_err(|error| DataFusionError::Execution(error.to_string()))
        })
    }

    /// Own one complete native Delta write, including native planning and commit acknowledgement.
    /// The native task keeps its permit and operation context until it exits; disconnects and
    /// deadlines abort via DataFusion's SpawnedTask. An interrupted commit is indeterminate and
    /// must be reconciled from its transaction/publication key, never blindly retried.
    pub(crate) async fn native_write<T: Send + 'static>(
        &self,
        work: impl std::future::Future<Output = Result<T>> + Send + 'static,
    ) -> Result<T> {
        self.native_io(work, "native write acknowledgement indeterminate")
            .await
    }

    /// Delta snapshot/metadata loading uses the same executor and admission as query work.
    pub(crate) async fn native_read<T: Send + 'static>(
        &self,
        work: impl Future<Output = Result<T>> + Send + 'static,
    ) -> Result<T> {
        self.native_io(work, "native metadata read deadline").await
    }

    async fn native_io<T: Send + 'static>(
        &self,
        work: impl Future<Output = Result<T>> + Send + 'static,
        interrupted: &'static str,
    ) -> Result<T> {
        let deadline = tokio::time::Instant::now() + self.remaining()?;
        let permit = tokio::time::timeout_at(deadline, Arc::clone(&self.permits).acquire_owned())
            .await
            .map_err(|_| deadline_budget("native I/O admission deadline"))?
            .map_err(|error| DataFusionError::Execution(error.to_string()))?;
        self.remaining()?;
        let task = self.spawn(async move {
            let _permit = permit;
            work.await
        });
        tokio::time::timeout_at(deadline, task)
            .await
            .map_err(|_| deadline_budget(interrupted))?
            .map_err(|error| DataFusionError::Execution(format!("native I/O task: {error}")))?
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
            history: self.diagnostics.clone(),
        });
        OPERATION.scope(operation, work).await
    }
    /// A terminal control commit must remain possible after its producer deadline expires.
    /// Only the finite JobStore settlement methods use this scope; it cannot launch effects or
    /// renew a claim. It retains request/policy correlation with a fresh bounded query budget.
    pub(crate) async fn settlement<T>(&self, id: &str, work: impl Future<Output = T>) -> T {
        let descriptor = OPERATION
            .try_with(|operation| operation.descriptor.clone())
            .unwrap_or_else(|_| OperationDescriptor {
                method: "native.control.settlement".into(),
                request_digest: enrichment_core::canonical::sha256_hex(id.as_bytes()),
                policy_digest: DEFINITION_REVISION.into(),
            });
        self.job_operation(
            format!("{id}/settlement"),
            descriptor,
            self.limits.deadline,
            work,
        )
        .await
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

    fn remaining(&self) -> Result<Duration> {
        let remaining = OPERATION
            .try_with(|operation| {
                operation
                    .deadline
                    .saturating_sub(operation.started.elapsed())
                    .min(self.limits.deadline)
            })
            .unwrap_or(self.limits.deadline);
        if remaining.is_zero() {
            return Err(deadline_budget("operation expired before native admission"));
        }
        Ok(remaining)
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
        Self::with_diagnostics(spill_root, &spill_root.join("native-diagnostics"), limits)
    }
    pub fn with_diagnostics(
        spill_root: &Path,
        diagnostics_root: &Path,
        limits: QueryLimits,
    ) -> Result<Self> {
        crate::task_context::install()?;
        if !(1..=16).contains(&limits.concurrency)
            || !(1..=16).contains(&limits.native.blocking_threads)
            || !(8 * 1024 * 1024..=64 * 1024 * 1024).contains(&limits.native.worker_stack_bytes)
        {
            return Err(DataFusionError::Configuration(
                "native executor requires 1..16 workers/blocking threads and 8..64 MiB worker stacks".into(),
            ));
        }
        if limits.memory_bytes == 0
            || limits.native.row_group_rows == 0
            || limits.native.row_group_rows > 1_000_000
            || limits.native.claim_lease_seconds == 0
            || limits.native.claim_lease_seconds > 86_400
            || limits.native.row_group_bytes == 0
            || limits.native.row_group_bytes as u64 > limits.native.target_file_bytes
            || limits.spill_bytes == 0
            || limits.batch_rows == 0
            || limits.partitions == 0
            || limits.concurrency == 0
            || limits.result_rows == 0
            || limits.result_bytes == 0
            || limits.deadline.is_zero()
        {
            return Err(DataFusionError::Configuration(
                "query limits must be positive and native layout rows at most 1000000".into(),
            ));
        }
        std::fs::create_dir_all(spill_root)?;
        let executor = Arc::new(NativeExecutor(Some(
            tokio::runtime::Builder::new_multi_thread()
                .worker_threads(limits.concurrency)
                .max_blocking_threads(limits.native.blocking_threads)
                .thread_stack_size(limits.native.worker_stack_bytes)
                .thread_name("enrichment-native")
                .enable_all()
                .build()?,
        )));
        let runtime = RuntimeEnvBuilder::new()
            .with_memory_pool(Arc::new(PeakRecordingPool::new(Arc::new(
                FairSpillPool::new(limits.memory_bytes),
            ))))
            .with_temp_file_path(spill_root)
            .with_max_temp_directory_size(limits.spill_bytes)
            .with_max_spill_merge_fan_in(8)
            .with_metadata_cache_limit(limits.metadata_cache_bytes)
            .build_arc()?;
        // The registry's implicit file backend is not our acknowledged durable backend.
        // Register once before any Delta builder or scan; both use this exact shared handle.
        runtime.register_object_store(
            &url::Url::parse("file:///")
                .map_err(|error| DataFusionError::External(Box::new(error)))?,
            Arc::new(crate::durable_store::DurableLocalStore::new()),
        );
        let limits = Arc::new(limits);
        let policy = crate::native_policy::NativePolicy::new(Arc::clone(&limits));
        let mut config = SessionConfig::new()
            .with_batch_size(limits.batch_rows)
            .with_target_partitions(limits.partitions)
            .with_default_catalog_and_schema("operation", "work")
            .with_information_schema(true)
            .with_option_extension(policy.clone())
            .set_bool("datafusion.execution.parquet.skip_metadata", false);
        config.options_mut().execution.parquet = policy.table_options().global;
        let template = SessionContext::new_with_config_rt(config, Arc::clone(&runtime)).state();
        let planner = Arc::new(crate::leases::RetentionPlanner {
            inner: Arc::new(crate::arrow_contract::NativePlanner),
        });
        let config = template.config().clone();
        let mut analyzer_rules = template.analyzer().rules.clone();
        analyzer_rules.insert(
            0,
            Arc::new(enrichment_core::native_analysis::SemanticAnalyzer),
        );
        let functions = enrichment_core::native_types::ClockMeaning::VALUES
            .iter()
            .map(|value| {
                Arc::new(enrichment_core::native_time::function(Some(
                    enrichment_core::native_types::ClockMeaning::parse(value)
                        .expect("declared clock"),
                )))
            })
            .chain(std::iter::once(Arc::new(
                enrichment_core::native_time::function(None),
            )))
            .chain(template.scalar_functions().values().cloned())
            .chain(std::iter::once(Arc::new(
                enrichment_core::native_collections::map_entries(),
            )))
            .chain(
                [
                    enrichment_core::native_version::semver_key(),
                    enrichment_core::native_version::pep440_value(),
                    enrichment_core::native_version::pep440_matches(),
                    enrichment_core::native_url::parts(),
                    enrichment_core::native_text::position(),
                ]
                .into_iter()
                .map(Arc::new),
            )
            .collect();
        let template = SessionStateBuilder::new_from_existing(template)
            .with_config(config)
            .with_analyzer_rules(analyzer_rules)
            .with_scalar_functions(functions)
            .with_extension_type_registry(enrichment_core::native_types::registry()?)
            .with_query_planner(planner)
            .build();
        let mut shared = Self {
            executor,
            template,
            permits: Arc::new(Semaphore::new(limits.concurrency)),
            operations: Arc::new(Semaphore::new(limits.concurrency)),
            limits,
            diagnostics: crate::telemetry_history::History::default(),
        };
        let pool = Arc::clone(&shared.template.runtime_env().memory_pool);
        let mut telemetry = shared.clone();
        // Reserved, single diagnostic admission shares the same executor, memory and spill.
        // A caller may request its observations while holding the sole service query permit.
        telemetry.permits = Arc::new(Semaphore::new(1));
        telemetry.operations = Arc::new(Semaphore::new(1));
        telemetry.limits = Arc::new(QueryLimits {
            result_rows: crate::telemetry_history::READ_ROWS,
            result_bytes: crate::telemetry_history::READ_BYTES,
            ..shared.limits.as_ref().clone()
        });
        shared.diagnostics =
            crate::telemetry_history::History::start(telemetry, diagnostics_root, pool)?;
        Ok(shared)
    }

    /// Decode a bounded declared record selection at an effect or transport boundary.
    /// The native plan owns decisions; generated codecs only project the selected values.
    pub async fn records<T: enrichment_core::native_union::NativeStruct>(
        &self,
        frame: datafusion::dataframe::DataFrame,
        maximum: usize,
    ) -> datafusion::common::Result<Vec<T>> {
        let expected = arrow::datatypes::Schema::new(T::fields());
        enrichment_core::native_schema::check_input(frame.schema().as_arrow(), &expected)?;
        for field in expected.fields() {
            let actual = frame.schema().field_with_unqualified_name(field.name())?;
            enrichment_core::native_analysis::compatible(actual, field, "typed record decoder")?;
        }
        let limit = maximum.checked_add(1).ok_or_else(|| {
            datafusion::common::DataFusionError::ResourcesExhausted("record bound overflow".into())
        })?;
        let output = self.execute(frame.limit(0, Some(limit))?).await?;
        if output.rows > maximum {
            return datafusion::common::exec_err!("native record selection exceeds its bound");
        }
        let mut records = Vec::with_capacity(output.rows);
        for batch in &output.batches {
            for field in expected.fields() {
                enrichment_core::native_analysis::compatible(
                    batch.schema().field_with_name(field.name())?,
                    field,
                    "physical record decoder",
                )?;
            }
            let rows = enrichment_core::evidence::arrow_model::cells::RowSet::batch(batch)?;
            for index in 0..batch.num_rows() {
                records.push(T::decode(rows.row(index))?);
            }
        }
        Ok(records)
    }

    /// Fresh scoped tables with shared spill, memory accounting and metadata cache.
    #[must_use]
    pub fn session(&self) -> SessionContext {
        self.bound_session(std::collections::BTreeMap::new())
            .expect("empty native inventory has a valid fixed schema")
    }

    pub(crate) fn bound_session(
        &self,
        mut bindings: std::collections::BTreeMap<
            String,
            Arc<dyn datafusion::catalog::CatalogProvider>,
        >,
    ) -> Result<SessionContext> {
        use datafusion::catalog::{CatalogProviderList, MemoryCatalogProviderList};
        let mut state = self.template.clone();
        let catalogs = Arc::new(MemoryCatalogProviderList::new());
        let (metadata, summary) = crate::native_catalog::metadata(&bindings)?;
        bindings.insert(
            "operation".into(),
            Arc::new(
                crate::native_catalog::BoundCatalog::default()
                    .with_work()
                    .with_metadata(metadata, summary),
            ),
        );
        for (name, catalog) in bindings {
            catalogs.register_catalog(name, catalog);
        }
        state.register_catalog_list(catalogs);
        state.mark_start_execution();
        Ok(SessionContext::new_with_state(state))
    }

    pub(crate) fn combined_session(&self, sessions: &[&SessionContext]) -> Result<SessionContext> {
        let mut bindings = std::collections::BTreeMap::new();
        for session in sessions {
            for name in session
                .catalog_names()
                .into_iter()
                .filter(|name| name != "operation")
            {
                let catalog = session
                    .catalog(&name)
                    .ok_or_else(|| DataFusionError::Internal("bound catalog disappeared".into()))?;
                if bindings.insert(name, catalog).is_some() {
                    return Err(DataFusionError::Plan("duplicate bound catalog".into()));
                }
            }
        }
        self.bound_session(bindings)
    }

    /// Last eight executed plans, bounded independently of indefinitely retained library facts.
    pub async fn diagnostics(&self) -> Result<Vec<enrichment_core::telemetry::QueryDiagnostics>> {
        self.diagnostics.read().await
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

    /// Capture a service observation in the same reserved native ingress as query diagnostics.
    pub fn record_service(&self, observation: enrichment_core::telemetry::ServiceObservation) {
        self.diagnostics.service(observation);
    }
    /// Process-scoped service totals selected from committed native observations.
    pub async fn service_counters(&self) -> Result<enrichment_core::telemetry::ServiceCounters> {
        self.diagnostics.service_counters().await
    }

    /// Cumulative observed query totals, separate from the bounded recent-plan history.
    pub async fn diagnostic_summary(&self) -> Result<enrichment_core::telemetry::Summary> {
        self.diagnostics.summary().await
    }

    /// Bounded operational status copied from this runtime's actual counters and admission.
    pub async fn operational_counters(
        &self,
    ) -> Result<enrichment_core::wire::status::NativeQueryCounters> {
        let summary = self.diagnostic_summary().await?;
        use datafusion::common::config::ExtensionOptions;
        let pool = &self.template.runtime_env().memory_pool;
        let settings = self
            .template
            .config_options()
            .extensions
            .get::<crate::native_policy::NativePolicy>()
            .expect("runtime installs its immutable policy")
            .entries();
        Ok(enrichment_core::wire::status::NativeQueryCounters {
            diagnostic_observations_dropped: self.diagnostics.dropped(),
            managed_memory_reserved_bytes: pool.reserved(),
            managed_memory_peak_bytes: PeakRecordingPool::from_pool(pool.as_ref())
                .expect("runtime installs its peak recorder")
                .peak_reserved(),
            effective_native_settings: settings
                .into_iter()
                .filter_map(|entry| entry.value.map(|value| (entry.key, value)))
                .collect(),
            executions: summary.executions,
            completed: summary.completed,
            incomplete: summary.incomplete,
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
        })
    }
    pub async fn diagnostic_failures(&self) -> Result<Vec<RecordBatch>> {
        self.diagnostics.failures().await
    }
    pub async fn flush_diagnostics(&self) -> Result<()> {
        self.diagnostics.flush().await
    }
    pub async fn close_diagnostics(&self) -> Result<()> {
        self.diagnostics.close().await
    }

    /// Plan and consume one owned finite command. Release query admission before polling
    /// its driver: the driver has its own effect admission and issues child native queries.
    /// Its owner must await physical completion; a caller timeout cannot certify cleanup.
    pub async fn command(
        &self,
        id: String,
        kind: crate::native_effect::CommandKind,
        driver: impl std::future::Future<Output = ()> + Send + 'static,
    ) -> Result<()> {
        use futures::TryStreamExt;
        let frame = crate::native_effect::frame(&self.session(), id, kind, driver)?;
        let start = Instant::now();
        let permit = Arc::new(
            tokio::time::timeout(self.remaining()?, self.permits.clone().acquire_owned())
                .await
                .map_err(|_| deadline_budget("command planning admission"))?
                .map_err(|error| DataFusionError::Execution(error.to_string()))?,
        );
        let (mut stream, mut trace) = tokio::time::timeout(
            self.remaining()?,
            self.plan(frame, start, None, permit.clone()),
        )
        .await
        .map_err(|_| deadline_budget("command planning"))??;
        drop(permit);
        let batch = stream.try_next().await?.ok_or_else(|| {
            DataFusionError::Execution("native command returned no completion".into())
        })?;
        if batch.num_rows() != 1 || stream.try_next().await?.is_some() {
            return Err(DataFusionError::Execution(
                "native command completion cardinality".into(),
            ));
        }
        trace.completed(1, batch.get_array_memory_size());
        Ok(())
    }

    /// Native planning owns the admission permit independently of its awaiting caller. This
    /// also isolates the upstream recursive optimizer from producer call-stack depth.
    async fn plan(
        &self,
        frame: DataFrame,
        start: Instant,
        family: Option<crate::preparation::QueryFamily>,
        permit: Arc<tokio::sync::OwnedSemaphorePermit>,
    ) -> Result<(
        datafusion::physical_plan::SendableRecordBatchStream,
        crate::query_diagnostics::Trace,
    )> {
        let runtime = self.clone();
        self.spawn(async move {
            let _permit = permit;
            runtime.plan_inner(frame, start, family).await
        })
        .await
        .map_err(|error| DataFusionError::Execution(format!("native planning task: {error}")))?
    }

    async fn plan_inner(
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
        let logical = logical.resolve_lambda_variables()?.data;
        enrichment_core::native_analysis::validate_plan(&logical)?;
        let mut trace =
            crate::query_diagnostics::Trace::new(&logical, self.diagnostics.clone(), start);
        trace.bound(&state);
        if let Some(family) = family {
            trace.family(family);
            family.require(logical.schema().as_arrow())?;
        }
        let analyzed = state.analyzer().execute_and_check(
            logical.clone(),
            state.config_options(),
            |plan, rule| trace.rule("analysis", rule.name(), plan),
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
        let optimized = state.optimizer().optimize(analyzed, &state, |plan, rule| {
            trace.rule("optimization", rule.name(), plan)
        })?;
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
        // The authored contract is the boundary at every preparation stage. Logical
        // optimization can strengthen nullability after eliminating a UNION branch;
        // native physical operators may retain a conservative nullable field. They
        // must still satisfy every guarantee of the original declared result.
        crate::preparation::result(&plan.schema(), logical.schema().as_arrow(), "physical_plan")?;
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
            .await
            .map_err(|e| e.context(format!("invariant {stage}: {rule}")))?;
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
    pub(crate) async fn admit(&self, invariants: crate::invariants::Invariants) -> Result<()> {
        use arrow::array::AsArray;
        let plan = self.native_read(async move { invariants.plan() }).await?;
        let output = self.execute(plan).await?;
        for batch in &output.batches {
            if batch.num_rows() == 0 {
                continue;
            }
            let text = |index| -> Result<&str> {
                batch
                    .column(index)
                    .as_string_opt::<i32>()
                    .map(|values| values.value(0))
                    .ok_or_else(|| {
                        DataFusionError::Internal("native invariant witness is not text".into())
                    })
            };
            return Err(self.failed(crate::preparation::InvariantFailure::error(
                text(0)?,
                text(1)?,
                crate::preparation::witnesses(&[batch.project(&[2])?]),
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
        tokio::time::timeout(self.remaining()?, async {
            let permit = Arc::new(
                Arc::clone(&self.permits)
                    .acquire_owned()
                    .await
                    .map_err(|e| DataFusionError::Execution(e.to_string()))?,
            );
            let (stream, mut trace) = self.plan(frame, start, family, Arc::clone(&permit)).await?;
            let expected = stream.schema();
            let mut stream = stream;
            let mut batches = Vec::new();
            let mut rows = 0usize;
            let mut memory = datafusion::common::utils::memory::RecordBatchMemoryCounter::new();
            let mut bytes = 0usize;
            while let Some(batch) = stream.try_next().await? {
                crate::preparation::physical(&batch.schema(), &expected, "result_batch")?;
                rows = rows
                    .checked_add(batch.num_rows())
                    .ok_or_else(|| budget("result rows"))?;
                let previous_bytes = bytes;
                memory.count_batch(&batch);
                bytes = memory.memory_usage();
                if rows > self.limits.result_rows || bytes > self.limits.result_bytes {
                    return Err(if rows > self.limits.result_rows {
                        budget_limit("result rows", rows, self.limits.result_rows)
                    } else {
                        budget_limit("result Arrow bytes", bytes, self.limits.result_bytes)
                    });
                }
                charge_output(bytes - previous_bytes)?;
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
        .map(|out| out.rows)
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
    ) -> Result<FoldOutput<T>>
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
                async move {
                    self.spawn_blocking(move || {
                        let _permit = permit;
                        consume(state, &batch)
                    })
                    .await
                    .map_err(|e| DataFusionError::Execution(e.to_string()))?
                }
            },
        )
        .await
    }

    async fn consume<T, F, Fut>(
        &self,
        frame: DataFrame,
        family: Option<crate::preparation::QueryFamily>,
        max_rows: usize,
        mut state: T,
        mut consume: F,
    ) -> Result<FoldOutput<T>>
    where
        F: FnMut(T, RecordBatch, Arc<tokio::sync::OwnedSemaphorePermit>) -> Fut,
        Fut: Future<Output = Result<T>>,
    {
        let start = Instant::now();
        tokio::time::timeout(self.remaining()?, async {
            let permit = Arc::new(
                Arc::clone(&self.permits)
                    .acquire_owned()
                    .await
                    .map_err(|e| DataFusionError::Execution(e.to_string()))?,
            );
            let (stream, mut trace) = self.plan(frame, start, family, Arc::clone(&permit)).await?;
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
            Ok(FoldOutput {
                rows,
                value: state,
                properties: trace.properties()?,
            })
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
