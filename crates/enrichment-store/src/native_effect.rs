//! Finite owned service commands expressed as native logical/physical operators.
//!
//! Planning and EXPLAIN only construct the operator. The driver is taken exactly once when
//! its output stream is first polled. Durable claims and physical cleanup remain authoritative;
//! a completed operator means the driver returned, not that a publication succeeded.
use arrow::{
    array::{ArrayRef, BooleanArray, StringArray},
    datatypes::{DataType, Field, Schema, SchemaRef},
    record_batch::RecordBatch,
};
use async_trait::async_trait;
use datafusion::{
    catalog::Session,
    common::{DFSchemaRef, DataFusionError, Result, tree_node::TreeNodeRecursion},
    dataframe::DataFrame,
    execution::TaskContext,
    logical_expr::{
        Expr, Extension, LogicalPlan, UserDefinedLogicalNode, UserDefinedLogicalNodeCore,
        physical_planning_context::PhysicalPlanningContext,
    },
    physical_expr::{EquivalenceProperties, PhysicalExpr},
    physical_plan::{
        DisplayAs, DisplayFormatType, ExecutionPlan, Partitioning, PlanProperties,
        SendableRecordBatchStream,
        execution_plan::{Boundedness, EmissionType},
        metrics::{Count, ExecutionPlanMetricsSet, MetricBuilder, MetricsSet, Time, Timestamp},
        stream::RecordBatchStreamAdapter,
    },
    physical_planner::{ExtensionPlanner, PhysicalPlanner},
    prelude::SessionContext,
};
use std::{
    cmp::Ordering,
    fmt,
    future::Future,
    hash::{Hash, Hasher},
    sync::{Arc, Mutex, OnceLock},
};

#[derive(Debug)]
struct Authority {
    job_id: String,
    grant: OnceLock<crate::control_jobs::Grant>,
}
tokio::task_local! { static AUTHORITY: Arc<Authority>; }

/// Correlation carries an already owned scope; copying it never acquires another claim.
#[derive(Clone)]
pub(crate) struct EffectContext(Option<Arc<Authority>>);
pub(crate) fn capture() -> EffectContext {
    EffectContext(AUTHORITY.try_with(Arc::clone).ok())
}
impl EffectContext {
    pub(crate) async fn scope<T>(self, work: impl Future<Output = T>) -> T {
        match self.0 {
            Some(authority) => AUTHORITY.scope(authority, work).await,
            None => work.await,
        }
    }
    pub(crate) fn run<T>(self, work: impl FnOnce() -> T) -> T {
        match self.0 {
            Some(authority) => AUTHORITY.sync_scope(authority, work),
            None => work(),
        }
    }
}

/// Bind the real claim selected by the command. Administrative control operations outside
/// an executing operator do not install an effect scope and therefore cannot run a driver.
pub fn bind_claim(grant: crate::control_jobs::Grant) -> Result<()> {
    AUTHORITY
        .try_with(|authority| {
            if authority.job_id != grant.job_id() {
                return Err(invalid("claim belongs to another native command"));
            }
            authority
                .grant
                .set(grant)
                .map_err(|_| invalid("native command claim was already bound"))
        })
        .unwrap_or(Ok(()))
}

/// Admission is re-evaluated against current Delta control before a physical side effect.
pub async fn authorize() -> Result<crate::control_jobs::Grant> {
    let grant = AUTHORITY
        .try_with(|authority| authority.grant.get().cloned())
        .ok()
        .flatten()
        .ok_or_else(|| invalid("physical effect requires a claimed native command"))?;
    grant.check().await?;
    Ok(grant)
}

pub use enrichment_core::operation::CommandKind;

struct Invocation {
    job_id: String,
    kind: CommandKind,
    driver: Mutex<Option<futures::future::BoxFuture<'static, ()>>>,
}
impl fmt::Debug for Invocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Invocation")
            .field("job_id", &self.job_id)
            .field("kind", &self.kind)
            .finish_non_exhaustive()
    }
}
#[derive(Debug, Clone)]
struct NativeCommand {
    invocation: Arc<Invocation>,
    schema: DFSchemaRef,
}
impl PartialEq for NativeCommand {
    fn eq(&self, other: &Self) -> bool {
        self.invocation.job_id == other.invocation.job_id
            && self.invocation.kind == other.invocation.kind
    }
}
impl Eq for NativeCommand {}
impl Hash for NativeCommand {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.invocation.job_id.hash(state);
        self.invocation.kind.hash(state);
    }
}
impl PartialOrd for NativeCommand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            (&self.invocation.job_id, self.invocation.kind)
                .cmp(&(&other.invocation.job_id, other.invocation.kind)),
        )
    }
}
fn schema() -> SchemaRef {
    Arc::new(Schema::new(vec![
        Field::new("job_id", DataType::Utf8, false),
        Field::new("driver_returned", DataType::Boolean, false),
    ]))
}
impl UserDefinedLogicalNodeCore for NativeCommand {
    fn name(&self) -> &str {
        "NativeCommand"
    }
    fn inputs(&self) -> Vec<&LogicalPlan> {
        vec![]
    }
    fn schema(&self) -> &DFSchemaRef {
        &self.schema
    }
    fn expressions(&self) -> Vec<Expr> {
        vec![]
    }
    fn supports_limit_pushdown(&self) -> bool {
        false
    }
    fn fmt_for_explain(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "NativeCommand: {:?} {} revision={}",
            self.invocation.kind,
            self.invocation.job_id,
            crate::control_jobs::OPERATION_REVISION
        )
    }
    fn with_exprs_and_inputs(
        &self,
        expressions: Vec<Expr>,
        inputs: Vec<LogicalPlan>,
    ) -> Result<Self> {
        if !expressions.is_empty() || !inputs.is_empty() {
            return Err(invalid(
                "native command has no relational inputs or rewriteable expressions",
            ));
        }
        Ok(self.clone())
    }
}
fn invalid(message: &str) -> DataFusionError {
    DataFusionError::Execution(message.into())
}

/// Only the owned command runtime constructs this source; there is no SQL registration.
pub(crate) fn frame(
    session: &SessionContext,
    job_id: String,
    kind: CommandKind,
    driver: impl Future<Output = ()> + Send + 'static,
) -> Result<DataFrame> {
    if job_id.is_empty() || job_id.len() > 256 {
        return Err(invalid(
            "native command requires a bounded durable identity",
        ));
    }
    let node = NativeCommand {
        invocation: Arc::new(Invocation {
            job_id,
            kind,
            driver: Mutex::new(Some(Box::pin(driver))),
        }),
        schema: Arc::new(schema().as_ref().clone().try_into()?),
    };
    Ok(DataFrame::new(
        session.state(),
        LogicalPlan::Extension(Extension {
            node: Arc::new(node),
        }),
    ))
}

#[derive(Debug)]
pub(crate) struct CommandPlanner;
#[async_trait]
impl ExtensionPlanner for CommandPlanner {
    async fn plan_extension(
        &self,
        _: &dyn PhysicalPlanner,
        node: &dyn UserDefinedLogicalNode,
        logical_inputs: &[&LogicalPlan],
        physical_inputs: &[Arc<dyn ExecutionPlan>],
        _: &dyn Session,
        _: &PhysicalPlanningContext,
    ) -> Result<Option<Arc<dyn ExecutionPlan>>> {
        let Some(command) = node.as_any().downcast_ref::<NativeCommand>() else {
            return Ok(None);
        };
        if !logical_inputs.is_empty() || !physical_inputs.is_empty() {
            return Err(invalid("native command planner arity"));
        }
        Ok(Some(Arc::new(CommandExec {
            invocation: Arc::clone(&command.invocation),
            metrics: Arc::new(CommandMetrics::new()),
            properties: Arc::new(PlanProperties::new(
                EquivalenceProperties::new(schema()),
                Partitioning::UnknownPartitioning(1),
                EmissionType::Final,
                Boundedness::Bounded,
            )),
        })))
    }
}
#[derive(Debug)]
struct CommandExec {
    invocation: Arc<Invocation>,
    metrics: Arc<CommandMetrics>,
    properties: Arc<PlanProperties>,
}
impl DisplayAs for CommandExec {
    fn fmt_as(&self, _: DisplayFormatType, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "NativeCommandExec: {:?} {}",
            self.invocation.kind, self.invocation.job_id
        )
    }
}
impl ExecutionPlan for CommandExec {
    fn name(&self) -> &str {
        "NativeCommandExec"
    }
    fn properties(&self) -> &Arc<PlanProperties> {
        &self.properties
    }
    fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>> {
        vec![]
    }
    fn with_new_children(
        self: Arc<Self>,
        children: Vec<Arc<dyn ExecutionPlan>>,
    ) -> Result<Arc<dyn ExecutionPlan>> {
        if !children.is_empty() {
            return Err(invalid("native command physical arity"));
        }
        Ok(self)
    }
    fn apply_expressions(
        &self,
        _: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>,
    ) -> Result<TreeNodeRecursion> {
        Ok(TreeNodeRecursion::Continue)
    }
    fn execute(&self, partition: usize, _: Arc<TaskContext>) -> Result<SendableRecordBatchStream> {
        if partition != 0 {
            return Err(invalid("native command has exactly one partition"));
        }
        let invocation = Arc::clone(&self.invocation);
        let metrics = self.metrics.clone();
        Ok(Box::pin(RecordBatchStreamAdapter::new(
            schema(),
            futures::stream::once(async move {
                let driver = invocation
                    .driver
                    .lock()
                    .map_err(|_| invalid("native driver ownership poisoned"))?
                    .take()
                    .ok_or_else(|| {
                        metrics.refused.add(1);
                        invalid("native driver was already executed; reconcile its durable command")
                    })?;
                metrics.started.add(1);
                metrics.start.record();
                let _timer = metrics.wall.timer();
                // Drop observes only the driver future, never physical process cleanup.
                let mut observation = DriverObservation {
                    metrics: metrics.clone(),
                    returned: false,
                };
                AUTHORITY
                    .scope(
                        Arc::new(Authority {
                            job_id: invocation.job_id.clone(),
                            grant: OnceLock::new(),
                        }),
                        driver,
                    )
                    .await;
                observation.returned = true;
                metrics.returned.add(1);
                let batch = RecordBatch::try_new(
                    schema(),
                    vec![
                        Arc::new(StringArray::from(vec![invocation.job_id.as_str()])) as ArrayRef,
                        Arc::new(BooleanArray::from(vec![true])),
                    ],
                )?;
                metrics.output_rows.add(batch.num_rows());
                metrics.output_bytes.add(batch.get_array_memory_size());
                Ok(batch)
            }),
        )))
    }

    fn metrics(&self) -> Option<MetricsSet> {
        Some(self.metrics.set.clone_inner())
    }

    fn statistics_from_inputs(
        &self,
        inputs: &[Arc<datafusion::common::Statistics>],
        args: &datafusion::physical_plan::statistics::StatisticsArgs,
    ) -> Result<Arc<datafusion::common::Statistics>> {
        if !inputs.is_empty() || args.partition().is_some_and(|p| p != 0) {
            return Err(invalid("native command statistics arity"));
        }
        Ok(Arc::new(
            datafusion::common::Statistics::new_unknown(&schema())
                .with_num_rows(datafusion::common::stats::Precision::Exact(1)),
        ))
    }
}

#[derive(Debug)]
struct CommandMetrics {
    set: ExecutionPlanMetricsSet,
    started: Count,
    returned: Count,
    refused: Count,
    dropped: Count,
    output_rows: Count,
    output_bytes: Count,
    wall: Time,
    start: Timestamp,
    end: Timestamp,
}
impl CommandMetrics {
    fn new() -> Self {
        let set = ExecutionPlanMetricsSet::new();
        Self {
            started: MetricBuilder::new(&set).counter("driver_started", 0),
            returned: MetricBuilder::new(&set).counter("driver_returned", 0),
            refused: MetricBuilder::new(&set).counter("driver_replay_refused", 0),
            dropped: MetricBuilder::new(&set).counter("driver_future_dropped", 0),
            output_rows: MetricBuilder::new(&set).output_rows(0),
            output_bytes: MetricBuilder::new(&set).output_bytes(0),
            wall: MetricBuilder::new(&set).subset_time("driver_wall_time", 0),
            start: MetricBuilder::new(&set).start_timestamp(0),
            end: MetricBuilder::new(&set).end_timestamp(0),
            set,
        }
    }
}

struct DriverObservation {
    metrics: Arc<CommandMetrics>,
    returned: bool,
}
impl Drop for DriverObservation {
    fn drop(&mut self) {
        if !self.returned {
            self.metrics.dropped.add(1);
        }
        self.metrics.end.record();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::{QueryLimits, QueryRuntime};
    use futures::TryStreamExt;

    #[tokio::test]
    async fn planning_and_unpolled_streams_do_not_start_or_duplicate_a_command() {
        let root = tempfile::tempdir().unwrap();
        let runtime =
            QueryRuntime::new(&root.path().join("spill"), QueryLimits::default()).unwrap();
        let session = runtime.session();
        let path = root.path().join("effect");
        let output = path.clone();
        let input = frame(
            &session,
            "job_finite".into(),
            CommandKind::Resolve,
            async move {
                // A real exclusive write makes duplicate execution observable independently of counters.
                use std::io::Write;
                std::fs::OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(output)
                    .unwrap()
                    .write_all(b"owned driver returned")
                    .unwrap();
            },
        )
        .unwrap();
        let explain = runtime
            .execute(input.clone().explain(true, false).unwrap())
            .await
            .unwrap();
        assert!(explain.rows > 0);
        let plan = input.create_physical_plan().await.unwrap();
        assert!(!path.exists());
        drop(plan.execute(0, session.task_ctx()).unwrap());
        assert!(!path.exists());
        let batches: Vec<_> = plan
            .execute(0, session.task_ctx())
            .unwrap()
            .try_collect()
            .await
            .unwrap();
        assert_eq!(batches[0].num_rows(), 1);
        assert_eq!(std::fs::read(path).unwrap(), b"owned driver returned");
        assert!(
            plan.execute(0, session.task_ctx())
                .unwrap()
                .try_collect::<Vec<_>>()
                .await
                .is_err()
        );
    }

    #[tokio::test]
    async fn one_query_slot_can_run_a_command_that_issues_native_queries() {
        let root = tempfile::tempdir().unwrap();
        let runtime = QueryRuntime::new(
            &root.path().join("spill"),
            QueryLimits {
                concurrency: 1,
                deadline: std::time::Duration::from_secs(5),
                ..Default::default()
            },
        )
        .unwrap();
        let child = runtime.clone();
        runtime
            .command("job_nested".into(), CommandKind::Compare, async move {
                let frame = child
                    .session()
                    .sql("SELECT sum(value) FROM (VALUES (1),(2),(3)) AS n(value)")
                    .await
                    .unwrap();
                let result = child.execute(frame).await.unwrap();
                assert_eq!(result.rows, 1);
                let value = result.batches[0]
                    .column(0)
                    .as_any()
                    .downcast_ref::<arrow::array::Int64Array>()
                    .unwrap();
                assert_eq!(value.value(0), 6);
            })
            .await
            .unwrap();
        assert!(
            runtime
                .diagnostics()
                .await
                .unwrap()
                .iter()
                .any(|q| q.physical.contains("NativeCommandExec") && q.completed)
        );
    }
}
