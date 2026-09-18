//! Cross-process retention leases, carried by providers, physical plans and result streams.
//! The permanent lock inode is never removed by evidence cleanup.
use std::{
    fs::{File, OpenOptions},
    io,
    path::Path,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use arrow::{datatypes::SchemaRef, record_batch::RecordBatch};
use async_trait::async_trait;
use datafusion::{
    catalog::{ScanArgs, ScanResult, Session, TableProvider},
    common::{Constraints, Statistics, config::ConfigOptions, tree_node::TreeNodeRecursion},
    error::{DataFusionError, Result},
    execution::{RecordBatchStream, SendableRecordBatchStream, TaskContext},
    logical_expr::{Expr, TableProviderFilterPushDown, TableType},
    physical_expr::PhysicalExpr,
    physical_plan::{
        ChildrenPropertiesMode, DisplayAs, DisplayFormatType, ExecutionPlan, PlanProperties,
        ReplaceChildrenOptions,
        execution_plan::CardinalityEffect,
        filter_pushdown::{FilterDescription, FilterPushdownPhase},
        statistics::{ChildStats, StatisticsArgs},
    },
};
use futures::Stream;

pub const LOCK_FILE: &str = ".evidence-retention.lock";

/// Create the permanent coordination inode during store initialization.
/// # Errors
/// A link, non-empty file or unavailable root is refused.
pub fn initialize(root: &Path) -> io::Result<()> {
    match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(root.join(LOCK_FILE))
    {
        Ok(file) => {
            file.sync_all()?;
            File::open(root)?.sync_all()?;
        }
        Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
            open(root)?;
        }
        Err(e) => return Err(e),
    }
    Ok(())
}

fn open(root: &Path) -> io::Result<File> {
    let path = root.join(LOCK_FILE);
    let before = std::fs::symlink_metadata(&path)?;
    if !before.is_file() || before.len() != 0 {
        return Err(io::Error::other("invalid evidence retention lock"));
    }
    let file = File::open(&path)?;
    let after = std::fs::symlink_metadata(&path)?;
    let actual = file.metadata()?;
    if !after.is_file() || actual.len() != 0 {
        return Err(io::Error::other("evidence retention lock changed"));
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        if (before.dev(), before.ino()) != (actual.dev(), actual.ino())
            || (after.dev(), after.ino()) != (actual.dev(), actual.ino())
        {
            return Err(io::Error::other("evidence retention lock was replaced"));
        }
    }
    Ok(file)
}

/// Pin retained evidence without waiting behind cleanup or changing any file bytes.
/// # Errors
/// Cleanup in progress and invalid coordination files are explicit errors.
pub fn shared(root: &Path) -> io::Result<Arc<File>> {
    let file = open(root)?;
    file.try_lock_shared().map_err(io::Error::from)?;
    Ok(Arc::new(file))
}

/// Manual cleanup must hold this guard through revalidation and every deletion.
/// # Errors
/// Any active reader, publisher or exporter prevents cleanup.
pub fn exclusive(root: &Path) -> io::Result<File> {
    let file = open(root)?;
    file.try_lock().map_err(io::Error::from)?;
    Ok(file)
}

#[derive(Debug)]
pub(crate) struct LeasedProvider {
    input: Arc<dyn TableProvider>,
    lease: Retained,
}

/// The two physical resources native scans can own. Staging is removed only after the
/// last provider, optimized physical plan and stream has released the directory.
#[derive(Clone, Debug)]
enum Retained {
    Evidence(Arc<File>),
    Input(Arc<crate::private_directory::PrivateDirectory>),
    Durable(Arc<crate::retention::LeaseGuard>),
    Exact(Arc<ImmutableRead>),
    Memory(Arc<crate::snapshot_registry::SnapshotMemory>),
    Flight(Arc<crate::snapshot_registry::Flight>),
    Combined(Arc<Retained>, Arc<Retained>),
}

/// Exact dependencies under an explicit sealed-root contract. The physical root lock
/// prevents concurrent removal; only the immutable seal and exact vector admit reads.
pub struct ImmutableRead {
    namespace: std::path::PathBuf,
    root: Arc<File>,
    dependencies: Vec<crate::retention::Dependency>,
    runtime: crate::runtime::QueryRuntime,
    immutable: Arc<crate::immutable_root::ImmutableRoot>,
}
impl std::fmt::Debug for ImmutableRead {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ImmutableRead")
            .field("namespace", &self.namespace)
            .field("dependencies", &self.dependencies)
            .finish_non_exhaustive()
    }
}

/// The exact vector survives logical rewrites, optimized-away leaves and physical streams.
#[derive(Clone, Debug)]
pub enum ReadProtection {
    Durable(Arc<crate::retention::LeaseGuard>),
    Immutable(Arc<ImmutableRead>),
}
impl ReadProtection {
    pub(crate) fn immutable(
        root: Arc<crate::immutable_root::ImmutableRoot>,
        dependencies: Vec<crate::retention::Dependency>,
        runtime: crate::runtime::QueryRuntime,
    ) -> Result<Self> {
        root.validate()?;
        if dependencies.is_empty() || dependencies.len() > 1024 {
            return datafusion::common::plan_err!("immutable read dependency bound");
        }
        let value = ImmutableRead {
            namespace: root.root().join("delta").canonicalize()?,
            root: root.lease(),
            dependencies,
            runtime,
            immutable: root,
        };
        value.require_namespace(&value.namespace)?;
        Ok(Self::Immutable(Arc::new(value)))
    }
    pub(crate) async fn require_tables(
        &self,
        namespace: &Path,
        bindings: &[enrichment_core::evidence::snapshot::DeltaBinding],
    ) -> Result<()> {
        self.require_selections(
            namespace,
            &bindings
                .iter()
                .map(|binding| binding.selection())
                .collect::<Vec<_>>(),
        )
        .await
    }
    pub(crate) async fn require_selections(
        &self,
        namespace: &Path,
        selections: &[enrichment_core::delta_reference::TableSelection],
    ) -> Result<()> {
        match self {
            Self::Durable(guard) => guard.require_selections(namespace, selections).await,
            Self::Immutable(guard) => {
                guard.require_namespace(namespace)?;
                crate::retention::require_selection_vector(
                    &guard.runtime,
                    &guard.dependencies,
                    selections,
                )
                .await
            }
        }
    }

    pub(crate) fn provider(
        self,
        input: Arc<dyn TableProvider>,
        session: &datafusion::prelude::SessionContext,
    ) -> Result<Arc<dyn TableProvider>> {
        let lease = match self {
            Self::Durable(guard) => Retained::Durable(guard),
            Self::Immutable(guard) => Retained::Exact(guard),
        };
        if input.get_logical_plan().is_some() {
            retained_view(&input, session, lease)
        } else {
            Ok(Arc::new(LeasedProvider::retaining(input, lease)))
        }
    }
}
impl ImmutableRead {
    fn require_namespace(&self, namespace: &Path) -> Result<()> {
        self.immutable.validate()?;
        use std::os::unix::fs::MetadataExt;
        if namespace.canonicalize()? != self.namespace {
            return datafusion::common::plan_err!("immutable read namespace mismatch");
        }
        let parent = namespace
            .parent()
            .ok_or_else(|| io::Error::other("missing read root"))?;
        let actual = open(parent)?.metadata()?;
        let held = self.root.metadata()?;
        if (actual.dev(), actual.ino(), actual.created()?)
            != (held.dev(), held.ino(), held.created()?)
        {
            return datafusion::common::plan_err!("immutable read root was replaced");
        }
        Ok(())
    }
}

impl Retained {
    fn protects_delta(&self) -> bool {
        match self {
            Self::Evidence(_) | Self::Durable(_) | Self::Exact(_) => true,
            Self::Combined(a, b) => a.protects_delta() || b.protects_delta(),
            Self::Input(_) | Self::Memory(_) | Self::Flight(_) => false,
        }
    }
    fn protects_staging(&self) -> bool {
        match self {
            Self::Input(_) => true,
            Self::Combined(a, b) => a.protects_staging() || b.protects_staging(),
            Self::Evidence(_)
            | Self::Durable(_)
            | Self::Exact(_)
            | Self::Memory(_)
            | Self::Flight(_) => false,
        }
    }
    fn same(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Evidence(a), Self::Evidence(b)) => Arc::ptr_eq(a, b),
            (Self::Input(a), Self::Input(b)) => Arc::ptr_eq(a, b),
            (Self::Durable(a), Self::Durable(b)) => Arc::ptr_eq(a, b),
            (Self::Exact(a), Self::Exact(b)) => Arc::ptr_eq(a, b),
            (Self::Memory(a), Self::Memory(b)) => Arc::ptr_eq(a, b),
            (Self::Flight(a), Self::Flight(b)) => Arc::ptr_eq(a, b),
            (Self::Combined(a, x), Self::Combined(b, y)) => Arc::ptr_eq(a, b) && Arc::ptr_eq(x, y),
            _ => false,
        }
    }
}

// The ownership node is installed before logical optimization. QueryPlanner sees an
// already optimized logical plan, too late to retain a scan removed by exact statistics.
#[derive(Debug, Clone)]
pub(crate) struct Retention {
    input: datafusion::logical_expr::LogicalPlan,
    leases: Vec<Retained>,
}
impl PartialEq for Retention {
    fn eq(&self, other: &Self) -> bool {
        self.input == other.input
            && self.leases.len() == other.leases.len()
            && self
                .leases
                .iter()
                .zip(&other.leases)
                .all(|(a, b)| a.same(b))
    }
}
impl Eq for Retention {}
impl std::hash::Hash for Retention {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.input.hash(state);
        fn lease_hash<H: std::hash::Hasher>(lease: &Retained, state: &mut H) {
            std::mem::discriminant(lease).hash(state);
            match lease {
                Retained::Evidence(a) => Arc::as_ptr(a).hash(state),
                Retained::Input(a) => Arc::as_ptr(a).hash(state),
                Retained::Durable(a) => Arc::as_ptr(a).hash(state),
                Retained::Exact(a) => Arc::as_ptr(a).hash(state),
                Retained::Memory(a) => Arc::as_ptr(a).hash(state),
                Retained::Flight(a) => Arc::as_ptr(a).hash(state),
                Retained::Combined(a, b) => {
                    Arc::as_ptr(a).hash(state);
                    Arc::as_ptr(b).hash(state);
                }
            }
        }
        for lease in &self.leases {
            lease_hash(lease, state);
        }
    }
}
impl PartialOrd for Retention {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.leases.len() != other.leases.len()
            || !self
                .leases
                .iter()
                .zip(&other.leases)
                .all(|(a, b)| a.same(b))
        {
            None
        } else {
            self.input.partial_cmp(&other.input)
        }
    }
}
impl datafusion::logical_expr::UserDefinedLogicalNodeCore for Retention {
    fn name(&self) -> &str {
        "NativeRetention"
    }
    fn inputs(&self) -> Vec<&datafusion::logical_expr::LogicalPlan> {
        vec![&self.input]
    }
    fn schema(&self) -> &datafusion::common::DFSchemaRef {
        self.input.schema()
    }
    fn expressions(&self) -> Vec<Expr> {
        vec![]
    }
    fn fmt_for_explain(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "NativeRetention: {} captured resources",
            self.leases.len()
        )
    }
    fn with_exprs_and_inputs(
        &self,
        expressions: Vec<Expr>,
        mut inputs: Vec<datafusion::logical_expr::LogicalPlan>,
    ) -> Result<Self> {
        if !expressions.is_empty() || inputs.len() != 1 {
            return datafusion::common::internal_err!("retention node arity");
        }
        Ok(Self {
            input: inputs.remove(0),
            leases: self.leases.clone(),
        })
    }
    fn supports_limit_pushdown(&self) -> bool {
        true
    }
}

#[derive(Debug)]
pub(crate) struct RetentionAnalyzer;
impl datafusion::optimizer::analyzer::AnalyzerRule for RetentionAnalyzer {
    fn name(&self) -> &str {
        "capture_native_retention"
    }
    fn analyze(
        &self,
        plan: datafusion::logical_expr::LogicalPlan,
        _: &ConfigOptions,
    ) -> Result<datafusion::logical_expr::LogicalPlan> {
        use datafusion::logical_expr::{Extension, LogicalPlan};
        fn collect(plan: &LogicalPlan, leases: &mut Vec<Retained>, depth: usize) -> Result<()> {
            if depth > 64 {
                return datafusion::common::plan_err!("retention view nesting bound");
            }
            plan.apply_with_subqueries(|node| {
                if let LogicalPlan::TableScan(scan) = node {
                    let provider = datafusion::datasource::source_as_provider(&scan.source)?;
                    capture(&provider, leases, depth)?;
                }
                Ok(TreeNodeRecursion::Continue)
            })?;
            Ok(())
        }
        fn capture(
            provider: &Arc<dyn TableProvider>,
            leases: &mut Vec<Retained>,
            depth: usize,
        ) -> Result<()> {
            if depth > 64 {
                return datafusion::common::plan_err!("retention provider nesting bound");
            }
            if let Some(owned) = provider.downcast_ref::<LeasedProvider>() {
                if !leases.iter().any(|lease| lease.same(&owned.lease)) {
                    if leases.len() == 1024 {
                        return Err(DataFusionError::ResourcesExhausted(
                            "query retention exceeds 1024 resources".into(),
                        ));
                    }
                    leases.push(owned.lease.clone());
                }
                capture(&owned.input, leases, depth + 1)?;
            } else if let Some(input) = provider.get_logical_plan() {
                collect(&input, leases, depth + 1)?;
            }
            Ok(())
        }
        if let LogicalPlan::Extension(extension) = &plan
            && extension.node.as_any().is::<Retention>()
        {
            return Ok(plan);
        }
        let mut leases = Vec::new();
        collect(&plan, &mut leases, 0)?;
        if leases.is_empty() {
            Ok(plan)
        } else {
            Ok(LogicalPlan::Extension(Extension {
                node: Arc::new(Retention {
                    input: plan,
                    leases,
                }),
            }))
        }
    }
}

pub(crate) struct RetentionExtensionPlanner;
#[async_trait]
impl datafusion::physical_planner::ExtensionPlanner for RetentionExtensionPlanner {
    async fn plan_extension(
        &self,
        _: &dyn datafusion::physical_planner::PhysicalPlanner,
        node: &dyn datafusion::logical_expr::UserDefinedLogicalNode,
        _: &[&datafusion::logical_expr::LogicalPlan],
        inputs: &[Arc<dyn ExecutionPlan>],
        _: &dyn Session,
        _: &datafusion::logical_expr::physical_planning_context::PhysicalPlanningContext,
    ) -> Result<Option<Arc<dyn ExecutionPlan>>> {
        use datafusion::physical_plan::{
            ExecutionPlanProperties, coalesce_partitions::CoalescePartitionsExec,
        };
        let Some(retention) = node.as_any().downcast_ref::<Retention>() else {
            return Ok(None);
        };
        if inputs.len() != 1 {
            return datafusion::common::internal_err!("retention physical arity");
        }
        let mut plan = Arc::clone(&inputs[0]);
        if plan.output_partitioning().partition_count() > 1 {
            plan = Arc::new(CoalescePartitionsExec::new(plan));
        }
        for lease in &retention.leases {
            plan = Arc::new(LeasedExec {
                input: plan,
                lease: lease.clone(),
            });
        }
        Ok(Some(plan))
    }
}

impl LeasedProvider {
    /// Inspect the actual source through ownership wrappers. A wrapper alone never
    /// proves immutability (it could enclose a mutable table or a volatile view).
    pub(crate) fn materialization_source(&self) -> Result<Arc<dyn TableProvider>> {
        let mut input = self.input.clone();
        loop {
            if let Some(nested) = input.downcast_ref::<Self>() {
                input = nested.input.clone();
            } else if let Some(admitted) =
                input.downcast_ref::<crate::admitted_provider::AdmittedProvider>()
            {
                if admitted.is_captured_batch() {
                    return Ok(input);
                }
                input = admitted.input().clone();
            } else {
                break;
            }
        }
        if input.get_logical_plan().is_some() {
            return Ok(input);
        }
        if let Some(scan) = input.downcast_ref::<deltalake::delta_datafusion::DeltaScanNext>() {
            if self.lease.protects_delta() {
                scan.validate_immutable_codec()?;
                return Ok(input);
            }
        } else if input.is::<datafusion::datasource::listing::ListingTable>()
            && self.lease.protects_staging()
        {
            return Ok(input);
        }
        Err(DataFusionError::Plan(
            "materialization source has no immutable provider witness".into(),
        ))
    }
    pub(crate) fn new(input: Arc<dyn TableProvider>, lease: Arc<File>) -> Self {
        Self::retaining(input, Retained::Evidence(lease))
    }
    fn retaining(input: Arc<dyn TableProvider>, lease: Retained) -> Self {
        // A second wrapper must preserve the first ownership even if exact statistics
        // eliminate the scan. The planner captures the outer lease before optimization.
        let mut source = input.as_ref();
        while let Some(admitted) =
            source.downcast_ref::<crate::admitted_provider::AdmittedProvider>()
        {
            source = admitted.input().as_ref();
        }
        let lease = match source.downcast_ref::<Self>() {
            Some(existing) => Retained::Combined(Arc::new(existing.lease.clone()), Arc::new(lease)),
            None => lease,
        };
        Self { input, lease }
    }
}

pub(crate) fn accounted_provider(
    input: Arc<dyn TableProvider>,
    memory: Arc<crate::snapshot_registry::SnapshotMemory>,
) -> Arc<dyn TableProvider> {
    Arc::new(LeasedProvider::retaining(input, Retained::Memory(memory)))
}

pub(crate) fn coordinated_provider(
    input: Arc<dyn TableProvider>,
    memory: Arc<crate::snapshot_registry::SnapshotMemory>,
    flight: Arc<crate::snapshot_registry::Flight>,
) -> Arc<dyn TableProvider> {
    Arc::new(LeasedProvider::retaining(
        accounted_provider(input, memory),
        Retained::Flight(flight),
    ))
}

pub(crate) fn protected_provider(
    input: Arc<dyn TableProvider>,
    guard: Arc<crate::retention::LeaseGuard>,
    session: &datafusion::prelude::SessionContext,
) -> Result<Arc<dyn TableProvider>> {
    ReadProtection::Durable(guard).provider(input, session)
}

/// Attach request ownership to exact scan leaves, then keep native view inlining available.
/// Wrapping the final view hides its logical plan and causes another optimization/planning
/// pass inside DataFrameTableProvider::scan. Exposing an unmodified cached plan would instead
/// lose the lease when native inlining removes the wrapper. The cache itself owns no lease.
pub(crate) fn leased_view(
    view: &Arc<dyn TableProvider>,
    session: &datafusion::prelude::SessionContext,
    lease: &Arc<File>,
) -> Result<Arc<dyn TableProvider>> {
    retained_view(view, session, Retained::Evidence(Arc::clone(lease)))
}

pub(crate) fn input_view(
    view: &Arc<dyn TableProvider>,
    session: &datafusion::prelude::SessionContext,
    directory: Arc<crate::private_directory::PrivateDirectory>,
) -> Result<Arc<dyn TableProvider>> {
    retained_view(view, session, Retained::Input(directory))
}

fn retained_view(
    view: &Arc<dyn TableProvider>,
    session: &datafusion::prelude::SessionContext,
    lease: Retained,
) -> Result<Arc<dyn TableProvider>> {
    use datafusion::{
        common::tree_node::Transformed,
        dataframe::DataFrame,
        datasource::{provider_as_source, source_as_provider},
        logical_expr::LogicalPlan,
    };
    let logical = view
        .get_logical_plan()
        .ok_or_else(|| {
            DataFusionError::Internal("cached domain view has no native logical plan".into())
        })?
        .into_owned();
    let logical = logical
        .transform_up_with_subqueries(|node| {
            let LogicalPlan::TableScan(mut scan) = node else {
                return Ok(Transformed::no(node));
            };
            let input = source_as_provider(&scan.source)?;
            if input.get_logical_plan().is_some() {
                return Err(DataFusionError::Internal(
                    "cached domain view has an unexpanded nested view".into(),
                ));
            }
            scan.source =
                provider_as_source(Arc::new(LeasedProvider::retaining(input, lease.clone())));
            Ok(Transformed::yes(LogicalPlan::TableScan(scan)))
        })?
        .data;
    Ok(DataFrame::new(session.state(), logical).into_view())
}
#[async_trait]
impl TableProvider for LeasedProvider {
    fn schema(&self) -> SchemaRef {
        self.input.schema()
    }
    fn constraints(&self) -> Option<&Constraints> {
        self.input.constraints()
    }
    fn table_type(&self) -> TableType {
        self.input.table_type()
    }
    fn statistics(&self) -> Option<Statistics> {
        self.input.statistics()
    }
    fn supports_filters_pushdown(
        &self,
        filters: &[&Expr],
    ) -> Result<Vec<TableProviderFilterPushDown>> {
        self.input.supports_filters_pushdown(filters)
    }
    async fn scan(
        &self,
        state: &dyn Session,
        projection: Option<&Vec<usize>>,
        filters: &[Expr],
        limit: Option<usize>,
    ) -> Result<Arc<dyn ExecutionPlan>> {
        Ok(Arc::new(LeasedExec {
            input: crate::task_context::InputContext::retaining(Arc::new(self.lease.clone()))
                .scope(self.input.scan(state, projection, filters, limit))
                .await?,
            lease: self.lease.clone(),
        }))
    }
    async fn scan_with_args<'a>(
        &self,
        state: &dyn Session,
        args: ScanArgs<'a>,
    ) -> Result<ScanResult> {
        Ok(ScanResult::new(Arc::new(LeasedExec {
            input: crate::task_context::InputContext::retaining(Arc::new(self.lease.clone()))
                .scope(self.input.scan_with_args(state, args))
                .await?
                .into_inner(),
            lease: self.lease.clone(),
        })))
    }
}

#[derive(Debug)]
struct LeasedExec {
    input: Arc<dyn ExecutionPlan>,
    lease: Retained,
}
impl DisplayAs for LeasedExec {
    fn fmt_as(&self, _: DisplayFormatType, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "EvidenceLeaseExec")
    }
}
impl ExecutionPlan for LeasedExec {
    fn name(&self) -> &str {
        "EvidenceLeaseExec"
    }
    fn properties(&self) -> &Arc<PlanProperties> {
        self.input.properties()
    }
    fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>> {
        vec![&self.input]
    }
    fn maintains_input_order(&self) -> Vec<bool> {
        vec![true]
    }
    fn benefits_from_input_partitioning(&self) -> Vec<bool> {
        vec![false]
    }
    fn supports_limit_pushdown(&self) -> bool {
        true
    }
    fn try_swapping_with_projection(
        &self,
        projection: &datafusion::physical_plan::projection::ProjectionExec,
    ) -> Result<Option<Arc<dyn ExecutionPlan>>> {
        Ok(Some(Arc::new(Self {
            input: datafusion::physical_plan::projection::make_with_child(projection, &self.input)?,
            lease: self.lease.clone(),
        })))
    }
    fn cardinality_effect(&self) -> CardinalityEffect {
        CardinalityEffect::Equal
    }
    fn gather_filters_for_pushdown(
        &self,
        _: FilterPushdownPhase,
        filters: Vec<Arc<dyn PhysicalExpr>>,
        _: &ConfigOptions,
    ) -> Result<FilterDescription> {
        FilterDescription::from_children(filters, &self.children())
    }
    fn child_stats_requests(&self, partition: Option<usize>) -> Vec<ChildStats> {
        vec![ChildStats::At(partition)]
    }
    fn statistics_from_inputs(
        &self,
        stats: &[Arc<Statistics>],
        _: &StatisticsArgs,
    ) -> Result<Arc<Statistics>> {
        stats
            .first()
            .cloned()
            .ok_or_else(|| DataFusionError::Internal("lease input statistics missing".into()))
    }
    fn apply_expressions(
        &self,
        _: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>,
    ) -> Result<TreeNodeRecursion> {
        Ok(TreeNodeRecursion::Continue)
    }
    fn replace_children(
        self: Arc<Self>,
        mut children: Vec<Arc<dyn ExecutionPlan>>,
        _: ReplaceChildrenOptions,
    ) -> Result<Arc<dyn ExecutionPlan>> {
        if children.len() != 1 {
            return Err(DataFusionError::Internal("lease needs one input".into()));
        }
        Ok(Arc::new(Self {
            input: children.remove(0),
            lease: self.lease.clone(),
        }))
    }
    fn with_new_children(
        self: Arc<Self>,
        children: Vec<Arc<dyn ExecutionPlan>>,
    ) -> Result<Arc<dyn ExecutionPlan>> {
        self.replace_children(
            children,
            ReplaceChildrenOptions::new(ChildrenPropertiesMode::Recompute),
        )
    }
    fn execute(
        &self,
        partition: usize,
        context: Arc<TaskContext>,
    ) -> Result<SendableRecordBatchStream> {
        let lease = Arc::new(self.lease.clone());
        let inner = crate::task_context::InputContext::retaining(lease.clone())
            .run(|| self.input.execute(partition, context))?;
        Ok(Box::pin(LeasedStream {
            inner,
            _lease: lease,
        }))
    }
}
struct LeasedStream {
    inner: SendableRecordBatchStream,
    _lease: Arc<Retained>,
}
impl Stream for LeasedStream {
    type Item = Result<RecordBatch>;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        crate::task_context::InputContext::retaining(self._lease.clone())
            .run(|| self.inner.as_mut().poll_next(cx))
    }
}
impl RecordBatchStream for LeasedStream {
    fn schema(&self) -> SchemaRef {
        self.inner.schema()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use arrow::{
        array::UInt64Array,
        datatypes::{DataType, Field, Schema},
        ipc::writer::FileWriter,
    };
    use futures::TryStreamExt;

    #[tokio::test]
    async fn native_stream_children_retain_exact_read_after_stream_cancellation() -> Result<()> {
        let root = tempfile::tempdir()?;
        initialize(root.path())?;
        std::fs::create_dir(root.path().join("delta"))?;
        let runtime =
            crate::runtime::QueryRuntime::new(&root.path().join("spill"), Default::default())?;
        crate::immutable_root::ImmutableRoot::seal(root.path())?;
        let guard = ReadProtection::immutable(
            crate::immutable_root::ImmutableRoot::open(root.path())?,
            vec![crate::retention::Dependency::Artifact {
                artifact_id: format!("art_{}", "a".repeat(64)),
            }],
            runtime.clone(),
        )?;
        let ReadProtection::Immutable(guard) = guard else {
            unreachable!()
        };
        let (started, wait_started) = tokio::sync::oneshot::channel();
        let (release, wait_release) = std::sync::mpsc::channel();
        let schema = Arc::new(Schema::empty());
        let returned = schema.clone();
        let inner = futures::stream::once(async move {
            datafusion::common::runtime::SpawnedTask::spawn_blocking(move || {
                let _ = started.send(());
                wait_release.recv().unwrap();
                Ok(RecordBatch::new_empty(returned))
            })
            .await
            .map_err(|error| DataFusionError::Execution(error.to_string()))?
        });
        let stream = LeasedStream {
            inner: Box::pin(
                datafusion::physical_plan::stream::RecordBatchStreamAdapter::new(schema, inner),
            ),
            _lease: Arc::new(Retained::Exact(guard)),
        };
        struct Notice(Option<tokio::sync::oneshot::Sender<()>>);
        impl Drop for Notice {
            fn drop(&mut self) {
                if let Some(sender) = self.0.take() {
                    let _ = sender.send(());
                }
            }
        }
        struct Input {
            stream: LeasedStream,
            _notice: Notice,
        }
        let (cancelled, wait_cancelled) = tokio::sync::oneshot::channel();
        let mut input = Input {
            stream,
            _notice: Notice(Some(cancelled)),
        };
        let waiter = runtime.spawn(async move {
            let result = input.stream.try_next().await;
            drop(input);
            result
        });
        wait_started.await.unwrap();
        drop(waiter);
        wait_cancelled.await.unwrap();
        assert!(
            exclusive(root.path()).is_err(),
            "native blocking child owns protection after its stream is gone"
        );
        release.send(()).unwrap();
        runtime.close_diagnostics().await?;
        assert!(exclusive(root.path()).is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn immutable_protection_survives_pruned_plans_streams_and_cancelled_blocking_reads()
    -> Result<()> {
        for pruned in [false, true] {
            let root = tempfile::tempdir()?;
            initialize(root.path())?;
            std::fs::create_dir(root.path().join("delta"))?;
            let runtime =
                crate::runtime::QueryRuntime::new(&root.path().join("spill"), Default::default())?;
            crate::immutable_root::ImmutableRoot::seal(root.path())?;
            let guard = ReadProtection::immutable(
                crate::immutable_root::ImmutableRoot::open(root.path())?,
                vec![crate::retention::Dependency::Artifact {
                    artifact_id: format!("art_{}", "a".repeat(64)),
                }],
                runtime.clone(),
            )?;
            let session = runtime.session();
            let batch = RecordBatch::try_new(
                Arc::new(Schema::new(vec![Field::new("n", DataType::UInt64, false)])),
                vec![Arc::new(UInt64Array::from(vec![1, 2, 3]))],
            )?;
            let input = crate::native_catalog::batch(&session, "leases", batch)?.into_view();
            let owned = guard.provider(input.clone(), &session)?;
            let frame = session
                .read_table(owned.clone())?
                .filter(datafusion::prelude::lit(!pruned))?;
            let physical = frame.create_physical_plan().await?;
            drop((frame, input, owned));
            assert!(
                exclusive(root.path()).is_err(),
                "even a pruned plan retains exact root protection"
            );
            let mut stream =
                datafusion::physical_plan::execute_stream(physical.clone(), session.task_ctx())?;
            drop((physical, session));
            let mut count = 0;
            while let Some(batch) = stream.try_next().await? {
                count += batch.num_rows();
            }
            assert_eq!(count, if pruned { 0 } else { 3 });
            assert!(
                exclusive(root.path()).is_err(),
                "exhausted stream remains a physical owner"
            );
            drop(stream);
            assert!(
                exclusive(root.path()).is_ok(),
                "last physical owner releases the root"
            );

            let guard = ReadProtection::immutable(
                crate::immutable_root::ImmutableRoot::open(root.path())?,
                vec![crate::retention::Dependency::Artifact {
                    artifact_id: format!("art_{}", "a".repeat(64)),
                }],
                runtime.clone(),
            )?;
            let (started, wait_started) = tokio::sync::oneshot::channel();
            let (release, wait_release) = std::sync::mpsc::channel();
            let (finished, wait_finished) = tokio::sync::oneshot::channel();
            let driver = runtime.clone();
            let waiter = tokio::spawn(async move {
                driver
                    .blocking(move || {
                        let protection = guard;
                        let _ = started.send(());
                        wait_release.recv().unwrap();
                        drop(protection);
                        let _ = finished.send(());
                    })
                    .await
            });
            wait_started.await.unwrap();
            waiter.abort();
            let _ = waiter.await;
            assert!(
                exclusive(root.path()).is_err(),
                "cancelled waiter cannot release a live reader"
            );
            release.send(()).unwrap();
            wait_finished.await.unwrap();
            assert!(exclusive(root.path()).is_ok());
            runtime.close_diagnostics().await?;
        }
        Ok(())
    }

    #[tokio::test]
    async fn plan19_immutable_read_requires_exact_vector_and_root() -> Result<()> {
        let root = tempfile::tempdir()?;
        initialize(root.path())?;
        let namespace = root.path().join("delta");
        std::fs::create_dir(&namespace)?;
        let runtime =
            crate::runtime::QueryRuntime::new(&root.path().join("spill"), Default::default())?;
        let binding = enrichment_core::evidence::snapshot::DeltaBinding {source: enrichment_core::delta_reference::DeltaVersionRef { table: enrichment_core::delta_reference::DeltaTableRef { table_uri: "evidence_symbols".into(), table_id: "fixture".into(), contract_id: enrichment_core::identity::SchemaContractId::try_from("schema_contract_cc8321d6375c494d043fdd0260f21bc0ec51dacc9f6abb7f909cdcd3041b78bf".to_owned()).unwrap() }, version: 7 }, relation: "symbols".into(), cohort_id: enrichment_core::identity::CohortId::try_from("cohort_d7cbbb688b2e506c022e95cef8c4f629".to_owned()).unwrap(), rows: 1,};
        crate::immutable_root::ImmutableRoot::seal(root.path())?;
        let guard = ReadProtection::immutable(
            crate::immutable_root::ImmutableRoot::open(root.path())?,
            vec![crate::retention::dependency(&binding)],
            runtime.clone(),
        )?;
        guard
            .require_tables(&namespace, std::slice::from_ref(&binding))
            .await?;
        let mut wrong = binding.clone();
        wrong.cohort_id = enrichment_core::identity::CohortId::try_from(
            "cohort_eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee".to_owned(),
        )
        .unwrap();
        assert!(guard.require_tables(&namespace, &[wrong]).await.is_err());
        // A replacement permanent lock is a different root even at the same path.
        std::fs::rename(root.path().join(LOCK_FILE), root.path().join("old-lock"))?;
        initialize(root.path())?;
        assert!(guard.require_tables(&namespace, &[binding]).await.is_err());
        drop(guard);
        runtime.close_diagnostics().await?;
        Ok(())
    }

    #[tokio::test]
    async fn private_input_directory_survives_planning_and_stream_ownership() {
        let root = tempfile::tempdir().unwrap();
        let runtime =
            crate::runtime::QueryRuntime::new(&root.path().join("spill"), Default::default())
                .unwrap();
        let control =
            crate::control::ControlStore::open(&root.path().join("state"), runtime.clone())
                .unwrap();
        let retention = crate::retention::RetentionStore::new(control, runtime.clone());
        let directory = crate::PrivateDirectory::create(
            &retention,
            &runtime,
            crate::private_directory::Kind::Documents,
        )
        .await
        .unwrap();
        let path = directory.path().join("documents.arrow");
        let schema = Arc::new(Schema::new(vec![Field::new("n", DataType::UInt64, false)]));
        let batch = RecordBatch::try_new(
            schema.clone(),
            vec![Arc::new(UInt64Array::from(vec![1, 2, 3]))],
        )
        .unwrap();
        let mut writer = FileWriter::try_new(File::create(&path).unwrap(), &schema).unwrap();
        writer.write(&batch).unwrap();
        writer.finish().unwrap();
        drop(writer);
        let session = runtime.session();
        let input = crate::arrow_input::provider(&runtime, &path, schema)
            .await
            .unwrap();
        let owned = input_view(&input, &session, directory.clone()).unwrap();
        let frame = session
            .read_table(owned.clone())
            .unwrap()
            .aggregate(
                vec![],
                vec![datafusion::functions_aggregate::expr_fn::count(
                    datafusion::prelude::lit(1),
                )],
            )
            .unwrap();
        let physical = frame.create_physical_plan().await.unwrap();
        drop((frame, input, owned, directory));
        assert!(
            path.exists(),
            "native physical plan owns the private directory"
        );
        let mut stream =
            datafusion::physical_plan::execute_stream(physical.clone(), session.task_ctx())
                .unwrap();
        drop((physical, session));
        assert!(
            path.exists(),
            "an executing stream owns its input after caller cancellation"
        );
        assert_eq!(stream.try_next().await.unwrap().unwrap().num_rows(), 1);
        assert!(stream.try_next().await.unwrap().is_none());
        assert!(
            path.exists(),
            "exhaustion does not release the caller's retained stream"
        );
        drop(stream);
        runtime.close_diagnostics().await.unwrap();
        assert!(
            !path.exists(),
            "last physical owner reclaims the private directory"
        );
    }
}
