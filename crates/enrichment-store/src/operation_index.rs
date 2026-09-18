//! Declared operation intermediates, planned through DataFusion CacheFactory (ADR-0052).
//! Planning never fills a cache. The immutable binding owns one execution-time result across
//! separately prepared readers; native spill and the existing runtime own physical resources.
use std::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    sync::{
        Arc, OnceLock,
        atomic::{AtomicU8, AtomicU64, Ordering as AtomicOrdering},
    },
};

use async_trait::async_trait;
use datafusion::{
    catalog::Session,
    common::{DFSchemaRef, Result, tree_node::TreeNodeRecursion},
    dataframe::DataFrame,
    error::DataFusionError,
    execution::{SessionState, TaskContext, session_state::CacheFactory},
    logical_expr::{
        Expr, Extension, LogicalPlan, UserDefinedLogicalNode, UserDefinedLogicalNodeCore,
        physical_planning_context::PhysicalPlanningContext,
    },
    physical_plan::{
        DisplayAs, DisplayFormatType, ExecutionPlan, Partitioning, PlanProperties,
        SendableRecordBatchStream,
        execution_plan::{Boundedness, EmissionType},
        metrics::{ExecutionPlanMetricsSet, MetricBuilder, MetricsSet},
        stream::RecordBatchStreamAdapter,
    },
    physical_planner::{ExtensionPlanner, PhysicalPlanner},
};
use enrichment_core::telemetry::{
    MaterializationActivity, MaterializationFamily, MaterializationObservation,
};
use futures::{
    FutureExt, TryStreamExt,
    future::{BoxFuture, Shared},
};

use crate::{
    preparation::QueryFamily,
    runtime::{OperationContext, QueryRuntime},
};

mod spill;
#[cfg(test)]
mod tests;

type FillResult = std::result::Result<Arc<spill::Filled>, Arc<DataFusionError>>;
type SharedFill = Shared<BoxFuture<'static, FillResult>>;
static NEXT_BINDING: AtomicU64 = AtomicU64::new(1);
const UNSTARTED: u8 = 0;
const FILLING: u8 = 1;
const READY: u8 = 2;
const FAILED: u8 = 3;
const CANCELLED: u8 = 4;

#[derive(Clone)]
struct Observer {
    id: u64,
    family: MaterializationFamily,
    operation: OperationContext,
    history: crate::telemetry_history::History,
}
impl Observer {
    fn emit(&self, activity: MaterializationActivity) {
        self.history.materialization(
            self.operation.id(),
            MaterializationObservation {
                binding: self.id,
                family: self.family,
                activity,
            },
        );
    }
}

struct Binding {
    id: u64,
    family: MaterializationFamily,
    runtime: QueryRuntime,
    observer: Observer,
    input: LogicalPlan,
    session: SessionState,
    options: Arc<crate::session_witness::Witness>,
    state: Arc<AtomicU8>,
    fill: OnceLock<SharedFill>,
    metrics: ExecutionPlanMetricsSet,
}
impl fmt::Debug for Binding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MaterializationBinding")
            .field("id", &self.id)
            .field("family", &self.family.as_str())
            .field("operation", &self.observer.operation.id())
            .field("definition", &crate::runtime::DEFINITION_REVISION)
            .field("state", &self.state.load(AtomicOrdering::Acquire))
            .finish_non_exhaustive()
    }
}

fn invalid(rule: &str) -> DataFusionError {
    crate::preparation::InvariantFailure::contract(rule, "operation_materialization", vec![])
}
impl Binding {
    fn require(&self, session: &dyn Session) -> Result<()> {
        self.observer.operation.require_current()?;
        if self.options != crate::session_witness::Witness::get(session)
            || !Arc::ptr_eq(&self.runtime.session().runtime_env(), session.runtime_env())
        {
            return Err(invalid("materialization session policy/runtime changed"));
        }
        QueryFamily::Intermediate(self.family).require(self.input.schema().as_arrow())
    }
    fn shared(
        &self,
        input: Arc<dyn ExecutionPlan>,
        context: Arc<TaskContext>,
        admission: crate::runtime::QueryAdmission,
    ) -> SharedFill {
        self.fill.get_or_init(|| {
            self.state.store(FILLING, AtomicOrdering::Release);
            MetricBuilder::new(&self.metrics).counter("cache_fills", 0).add(1);
            let runtime = self.runtime.clone();
            let observer = self.observer.clone();
            let operation = observer.operation.clone();
            observer.emit(MaterializationActivity::Fill);
            let metrics = self.metrics.clone();
            let state = self.state.clone();
            // The task owns no Binding or SharedFill: dropping the last binding can abort
            // it, and dropping one waiter cannot. Already-running blocking callbacks retain
            // their input/spill/reservation and physical-exit token independently.
            let task = self.runtime.spawn(admission.scope(async move {
                let outcome = async {
                    let cancellation = operation.cancellation()?;
                    let remaining = runtime.remaining()?;
                    tokio::select! {
                        biased;
                        () = cancellation.cancelled() => {
                            state.store(CANCELLED, AtomicOrdering::Release);
                            Err(invalid("operation materialization cancelled"))
                        }
                        result = tokio::time::timeout(remaining, spill::fill(
                            runtime.clone(), input, context, metrics.clone()
                        )) => match result {
                            Ok(result) => result,
                            Err(_) => {
                                state.store(CANCELLED, AtomicOrdering::Release);
                                Err(DataFusionError::External(Box::new(crate::runtime::BudgetFailure {
                                    rule: "operation materialization deadline".into(),
                                    cause: enrichment_core::wire::DiagnosticCause::Capacity,
                                    observed: None, allowed: None,
                                    operation_id: operation.id().map(str::to_owned),
                                })))
                            }
                        }
                    }
                }.await;
                match outcome {
                    Ok(value) => {
                        observer.emit(MaterializationActivity::Ready { spill_bytes: value.bytes() });
                        state.store(READY, AtomicOrdering::Release);
                        Ok(Arc::new(value))
                    }
                    Err(error) => {
                        if state.load(AtomicOrdering::Acquire) != CANCELLED {
                            state.store(FAILED, AtomicOrdering::Release);
                        }
                        observer.emit(if state.load(AtomicOrdering::Acquire) == CANCELLED {
                            MaterializationActivity::Cancelled
                        } else { MaterializationActivity::Failed });
                        MetricBuilder::new(&metrics).counter("cache_failures", 0).add(1);
                        Err(Arc::new(error))
                    }
                }
            }));
            async move {
                task.await.map_err(|error| Arc::new(DataFusionError::Execution(
                    format!("owned materialization task: {error}")
                )))?
            }.boxed().shared()
        }).clone()
    }
}

#[derive(Debug, Clone)]
struct Materialization {
    binding: Arc<Binding>,
    admitted: bool,
}
impl PartialEq for Materialization {
    fn eq(&self, other: &Self) -> bool {
        self.binding.id == other.binding.id && self.admitted == other.admitted
    }
}
impl Eq for Materialization {}
impl Hash for Materialization {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.binding.id.hash(state);
        self.admitted.hash(state);
    }
}
impl PartialOrd for Materialization {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else {
            None
        }
    }
}
impl UserDefinedLogicalNodeCore for Materialization {
    fn name(&self) -> &str {
        "OperationMaterialization"
    }
    fn inputs(&self) -> Vec<&LogicalPlan> {
        // A captured base is a source to subsequent consumers, not their rewritable child.
        // The exact admitted plan remains inspectable in Binding and in the physical tree.
        // Logical rewrites such as sort transposition rebuild intermediate children before
        // restoring equivalent outer nodes; accepting those children could change one fill.
        vec![]
    }
    fn schema(&self) -> &DFSchemaRef {
        self.binding.input.schema()
    }
    fn expressions(&self) -> Vec<Expr> {
        vec![]
    }
    fn fmt_for_explain(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "OperationMaterialization: {} binding={} operation={}",
            self.binding.family.as_str(),
            self.binding.id,
            self.binding.observer.operation.id().unwrap_or("unbound")
        )
    }
    fn with_exprs_and_inputs(&self, exprs: Vec<Expr>, inputs: Vec<LogicalPlan>) -> Result<Self> {
        if !exprs.is_empty() || !inputs.is_empty() {
            return Err(invalid(
                "materialization input changed; bind a new intermediate",
            ));
        }
        Ok(self.clone())
    }
    // Default predicate/projection/limit barriers keep every reader outside the shared base.
}

#[derive(Debug)]
pub(crate) struct OperationCacheFactory;
impl CacheFactory for OperationCacheFactory {
    fn create(&self, plan: LogicalPlan, state: &SessionState) -> Result<LogicalPlan> {
        let LogicalPlan::Extension(extension) = plan else {
            return Err(invalid(
                "cache requires an explicitly declared operation binding",
            ));
        };
        let Some(request) = extension.node.as_any().downcast_ref::<Materialization>() else {
            return Err(invalid(
                "cache requires an explicitly declared operation binding",
            ));
        };
        request.binding.require(state)?;
        Ok(LogicalPlan::Extension(Extension {
            node: Arc::new(Materialization {
                admitted: true,
                ..request.clone()
            }),
        }))
    }
}

/// Admit immutable native input before optimization can erase an effect or volatile expression.
fn admit_source(source: &dyn datafusion::catalog::TableProvider, depth: usize) -> Result<()> {
    if depth > 64 {
        return Err(invalid("materialization provider nesting bound"));
    }
    if let Some(plan) = source.get_logical_plan() {
        admit(&plan, depth + 1)
    } else if let Some(leased) = source.downcast_ref::<crate::leases::LeasedProvider>() {
        let input = leased.materialization_source()?;
        if let Some(plan) = input.get_logical_plan() {
            admit(&plan, depth + 1)
        } else {
            Ok(())
        }
    } else if let Some(admitted) =
        source.downcast_ref::<crate::admitted_provider::AdmittedProvider>()
    {
        if admitted.is_captured_batch() {
            Ok(())
        } else {
            admit_source(admitted.input().as_ref(), depth + 1)
        }
    } else {
        Err(invalid(
            "materialization input lacks immutable retention binding",
        ))
    }
}

fn admit(input: &LogicalPlan, depth: usize) -> Result<()> {
    if depth > 64 {
        return Err(invalid("materialization view nesting bound"));
    }
    input.apply_with_subqueries(|plan| {
        match plan {
            LogicalPlan::Dml(_)
            | LogicalPlan::Ddl(_)
            | LogicalPlan::Copy(_)
            | LogicalPlan::Statement(_)
            | LogicalPlan::Explain(_)
            | LogicalPlan::Analyze(_)
            | LogicalPlan::DescribeTable(_)
            | LogicalPlan::RecursiveQuery(_) => {
                return Err(invalid("effect or unsupported input in materialization"));
            }
            LogicalPlan::Extension(extension) => {
                if let Some(cache) = extension.node.as_any().downcast_ref::<Materialization>() {
                    if !cache.admitted {
                        return Err(invalid("unadmitted nested materialization"));
                    }
                    cache.binding.observer.operation.require_current()?;
                } else if !extension
                    .node
                    .as_any()
                    .is::<crate::arrow_contract::ArrowContract>()
                    && !extension.node.as_any().is::<crate::leases::Retention>()
                {
                    return Err(invalid("unsupported extension in materialization"));
                }
            }
            LogicalPlan::TableScan(scan) => {
                let source = datafusion::datasource::source_as_provider(&scan.source)?;
                admit_source(source.as_ref(), depth + 1)?;
            }
            _ => {}
        }
        if plan.expressions().iter().any(Expr::is_volatile) {
            return Err(invalid("volatile input in materialization"));
        }
        Ok(TreeNodeRecursion::Continue)
    })?;
    Ok(())
}

/// Bind a family once, then let DataFusion plan every count/page consumer of the cached frame.
pub(crate) async fn cache(
    runtime: &QueryRuntime,
    frame: DataFrame,
    family: QueryFamily,
) -> Result<DataFrame> {
    let family = family
        .materialization()
        .ok_or_else(|| invalid("undeclared materialization family"))?;
    let operation = crate::runtime::capture_operation();
    operation.require_current()?;
    let frame = crate::provider::derived(frame, "operation_index")?;
    let (state, input) = frame.into_parts();
    admit(&input, 0)?;
    enrichment_core::native_analysis::validate_plan(&input)?;
    // Seal the fully analyzed and optimized meaning before sharing. Later consumers may
    // rewrite above this boundary; a changed base must create its own binding.
    let input = state.optimize(&input)?;
    QueryFamily::Intermediate(family).require(input.schema().as_arrow())?;
    let id = NEXT_BINDING.fetch_add(1, AtomicOrdering::Relaxed);
    let binding = Arc::new(Binding {
        id,
        family,
        observer: Observer {
            id,
            family,
            operation,
            history: runtime.native_history(),
        },
        runtime: runtime.clone(),
        input: input.clone(),
        options: crate::session_witness::Witness::get(&state),
        session: state.clone(),
        state: Arc::new(AtomicU8::new(UNSTARTED)),
        fill: OnceLock::new(),
        metrics: ExecutionPlanMetricsSet::new(),
    });
    DataFrame::new(
        state,
        LogicalPlan::Extension(Extension {
            node: Arc::new(Materialization {
                binding,
                admitted: false,
            }),
        }),
    )
    .cache()
    .await
}

/// Count is a native aggregate over the shared base, never fill completion metadata.
pub(crate) async fn count(runtime: &QueryRuntime, frame: DataFrame) -> Result<u64> {
    use datafusion::{functions_aggregate::expr_fn::count, prelude::lit};
    let counted = frame.aggregate(
        vec![],
        vec![
            datafusion::logical_expr::expr_fn::cast(
                count(lit(1)),
                arrow::datatypes::DataType::UInt64,
            )
            .alias("count"),
        ],
    )?;
    enrichment_core::native_struct! { struct Count {count:u64=>enrichment_core::native_union::Rule::Text} }
    runtime
        .records::<Count>(counted, 1)
        .await?
        .pop()
        .map(|row| row.count)
        .ok_or_else(|| invalid("native count requires one aggregate"))
}

pub(crate) struct MaterializationPlanner;
#[async_trait]
impl ExtensionPlanner for MaterializationPlanner {
    async fn plan_extension(
        &self,
        planner: &dyn PhysicalPlanner,
        node: &dyn UserDefinedLogicalNode,
        logical_inputs: &[&LogicalPlan],
        inputs: &[Arc<dyn ExecutionPlan>],
        session: &dyn Session,
        _: &PhysicalPlanningContext,
    ) -> Result<Option<Arc<dyn ExecutionPlan>>> {
        let Some(cache) = node.as_any().downcast_ref::<Materialization>() else {
            return Ok(None);
        };
        if !cache.admitted || !inputs.is_empty() || !logical_inputs.is_empty() {
            return Err(invalid("unadmitted materialization or physical arity"));
        }
        cache.binding.require(session)?;
        // This only prepares the sealed base. Execution remains lazy and shared across
        // every separately prepared consumer through Binding::shared, including EXPLAIN.
        let input = planner
            .create_physical_plan(&cache.binding.input, &cache.binding.session)
            .await?;
        Ok(Some(Arc::new(MaterializationExec::new(
            input,
            cache.binding.clone(),
        )?)))
    }
}

#[derive(Debug)]
struct MaterializationExec {
    input: Arc<dyn ExecutionPlan>,
    binding: Arc<Binding>,
    properties: Arc<PlanProperties>,
}
impl MaterializationExec {
    fn new(input: Arc<dyn ExecutionPlan>, binding: Arc<Binding>) -> Result<Self> {
        if input.schema().as_ref() != binding.input.schema().as_arrow() {
            return Err(invalid("materialization physical schema changed"));
        }
        if input.properties().boundedness != Boundedness::Bounded {
            return Err(invalid(
                "operation materialization requires a bounded input",
            ));
        }
        // FIFO replay preserves a single input's equivalences, proven keys and ordering.
        // Combining partitions establishes neither global order nor partition constants.
        let mut equivalence = input.properties().eq_properties.clone();
        if input.properties().partitioning.partition_count() > 1 {
            equivalence.clear_orderings();
            equivalence.clear_per_partition_constants();
        }
        let properties = Arc::new(PlanProperties::new(
            equivalence,
            Partitioning::UnknownPartitioning(1),
            EmissionType::Final,
            Boundedness::Bounded,
        ));
        Ok(Self {
            input,
            binding,
            properties,
        })
    }
}
impl DisplayAs for MaterializationExec {
    fn fmt_as(&self, _: DisplayFormatType, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "OperationMaterializationExec: {} binding={}",
            self.binding.family.as_str(),
            self.binding.id
        )
    }
}
impl ExecutionPlan for MaterializationExec {
    fn cardinality_effect(&self) -> datafusion::physical_plan::execution_plan::CardinalityEffect {
        datafusion::physical_plan::execution_plan::CardinalityEffect::Equal
    }
    fn child_stats_requests(
        &self,
        _: Option<usize>,
    ) -> Vec<datafusion::physical_plan::statistics::ChildStats> {
        // The one replay partition contains every input partition. Input partition 0
        // alone is not its cardinality or column statistics.
        vec![datafusion::physical_plan::statistics::ChildStats::At(None)]
    }
    fn statistics_from_inputs(
        &self,
        stats: &[Arc<datafusion::common::Statistics>],
        args: &datafusion::physical_plan::statistics::StatisticsArgs,
    ) -> Result<Arc<datafusion::common::Statistics>> {
        if args.partition().is_some_and(|partition| partition != 0) {
            return Err(invalid("materialization statistics partition"));
        }
        stats
            .first()
            .cloned()
            .ok_or_else(|| invalid("materialization input statistics missing"))
    }
    fn name(&self) -> &str {
        "OperationMaterializationExec"
    }
    fn properties(&self) -> &Arc<PlanProperties> {
        &self.properties
    }
    fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>> {
        vec![&self.input]
    }
    fn apply_expressions(
        &self,
        _: &mut dyn FnMut(
            &Arc<dyn datafusion::physical_expr::PhysicalExpr>,
        ) -> Result<TreeNodeRecursion>,
    ) -> Result<TreeNodeRecursion> {
        Ok(TreeNodeRecursion::Continue)
    }
    fn with_new_children(
        self: Arc<Self>,
        mut children: Vec<Arc<dyn ExecutionPlan>>,
    ) -> Result<Arc<dyn ExecutionPlan>> {
        if children.len() != 1 {
            return Err(invalid("materialization physical input arity"));
        }
        Ok(Arc::new(Self::new(
            children.remove(0),
            self.binding.clone(),
        )?))
    }
    fn execute(
        &self,
        partition: usize,
        context: Arc<TaskContext>,
    ) -> Result<SendableRecordBatchStream> {
        if partition != 0 {
            return Err(invalid("materialization has one replay partition"));
        }
        self.binding.observer.operation.require_current()?;
        let binding = self.binding.clone();
        let input = self.input.clone();
        let schema = self.schema();
        let admission = crate::runtime::capture_query_admission();
        let stream = futures::stream::once(async move {
            binding.observer.operation.require_current()?;
            binding.observer.emit(MaterializationActivity::Wait);
            MetricBuilder::new(&binding.metrics)
                .counter("cache_waiters", 0)
                .add(1);
            let filled = binding
                .shared(input, context.clone(), admission)
                .await
                .map_err(DataFusionError::Shared)?;
            binding.observer.operation.require_current()?;
            MetricBuilder::new(&binding.metrics)
                .counter("cache_reads", 0)
                .add(1);
            spill::read(filled, binding, context)
        })
        .try_flatten();
        Ok(Box::pin(RecordBatchStreamAdapter::new(schema, stream)))
    }
    fn metrics(&self) -> Option<MetricsSet> {
        Some(self.binding.metrics.clone_inner())
    }
}
