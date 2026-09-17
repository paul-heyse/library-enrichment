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
    Staging(Arc<tempfile::TempDir>),
    Captured(Arc<tempfile::TempDir>, Arc<File>),
    Durable(Arc<crate::retention::LeaseGuard>),
    Memory(Arc<datafusion::execution::memory_pool::MemoryReservation>),
    Combined(Arc<Retained>, Arc<Retained>),
}

/// Protection captured before opening a version. Read-only exports currently use
/// the shared physical root guard; writable service reads use exact durable facts.
#[derive(Clone, Debug)]
pub enum ReadProtection {
    Durable(Arc<crate::retention::LeaseGuard>),
    Root(Arc<File>),
}
impl ReadProtection {
    pub(crate) async fn require_tables(
        &self,
        namespace: &Path,
        bindings: &[enrichment_core::evidence::snapshot::DeltaBinding],
    ) -> Result<()> {
        match self {
            Self::Durable(guard) => guard.require_tables(namespace, bindings).await,
            Self::Root(guard) => {
                // Read-only roots cannot append durable leases. Verify the pinned inode
                // belongs to this namespace before opening any dependent provider.
                let root = namespace
                    .parent()
                    .ok_or_else(|| io::Error::other("missing read root"))?;
                let actual = open(root)?.metadata()?;
                let held = guard.metadata()?;
                #[cfg(unix)]
                {
                    use std::os::unix::fs::MetadataExt;
                    if (actual.dev(), actual.ino()) != (held.dev(), held.ino()) {
                        return Err(DataFusionError::Plan(
                            "read protection namespace mismatch".into(),
                        ));
                    }
                }
                #[cfg(not(unix))]
                {
                    let _ = (actual, held);
                    return Err(DataFusionError::NotImplemented(
                        "read-only root identity requires inode qualification".into(),
                    ));
                }
                Ok(())
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
            Self::Root(guard) => Retained::Evidence(guard),
        };
        if input.get_logical_plan().is_some() {
            retained_view(&input, session, lease)
        } else {
            Ok(Arc::new(LeasedProvider::retaining(input, lease)))
        }
    }
}
impl Retained {
    fn protects_delta(&self) -> bool {
        match self {
            Self::Evidence(_) | Self::Captured(..) | Self::Durable(_) => true,
            Self::Combined(a, b) => a.protects_delta() || b.protects_delta(),
            Self::Staging(_) | Self::Memory(_) => false,
        }
    }
    fn protects_staging(&self) -> bool {
        match self {
            Self::Staging(_) | Self::Captured(..) => true,
            Self::Combined(a, b) => a.protects_staging() || b.protects_staging(),
            Self::Evidence(_) | Self::Durable(_) | Self::Memory(_) => false,
        }
    }
    fn same(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Evidence(a), Self::Evidence(b)) => Arc::ptr_eq(a, b),
            (Self::Staging(a), Self::Staging(b)) => Arc::ptr_eq(a, b),
            (Self::Captured(a, x), Self::Captured(b, y)) => Arc::ptr_eq(a, b) && Arc::ptr_eq(x, y),
            (Self::Durable(a), Self::Durable(b)) => Arc::ptr_eq(a, b),
            (Self::Memory(a), Self::Memory(b)) => Arc::ptr_eq(a, b),
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
                Retained::Staging(a) => Arc::as_ptr(a).hash(state),
                Retained::Captured(a, b) => {
                    Arc::as_ptr(a).hash(state);
                    Arc::as_ptr(b).hash(state);
                }
                Retained::Durable(a) => Arc::as_ptr(a).hash(state),
                Retained::Memory(a) => Arc::as_ptr(a).hash(state),
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
    memory: Arc<datafusion::execution::memory_pool::MemoryReservation>,
) -> Arc<dyn TableProvider> {
    Arc::new(LeasedProvider::retaining(input, Retained::Memory(memory)))
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

pub(crate) fn staged_view(
    view: &Arc<dyn TableProvider>,
    session: &datafusion::prelude::SessionContext,
    directory: Arc<tempfile::TempDir>,
) -> Result<Arc<dyn TableProvider>> {
    retained_view(view, session, Retained::Staging(directory))
}

pub(crate) fn captured_view(
    view: &Arc<dyn TableProvider>,
    session: &datafusion::prelude::SessionContext,
    directory: Arc<tempfile::TempDir>,
    evidence: Arc<File>,
) -> Result<Arc<dyn TableProvider>> {
    retained_view(view, session, Retained::Captured(directory, evidence))
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
            input: self.input.scan(state, projection, filters, limit).await?,
            lease: self.lease.clone(),
        }))
    }
    async fn scan_with_args<'a>(
        &self,
        state: &dyn Session,
        args: ScanArgs<'a>,
    ) -> Result<ScanResult> {
        Ok(ScanResult::new(Arc::new(LeasedExec {
            input: self.input.scan_with_args(state, args).await?.into_inner(),
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
        Ok(Box::pin(LeasedStream {
            inner: self.input.execute(partition, context)?,
            _lease: self.lease.clone(),
        }))
    }
}
struct LeasedStream {
    inner: SendableRecordBatchStream,
    _lease: Retained,
}
impl Stream for LeasedStream {
    type Item = Result<RecordBatch>;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.inner.as_mut().poll_next(cx)
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
    async fn private_input_directory_survives_planning_and_stream_ownership() {
        let root = tempfile::tempdir().unwrap();
        let runtime =
            crate::runtime::QueryRuntime::new(&root.path().join("spill"), Default::default())
                .unwrap();
        let directory = Arc::new(tempfile::tempdir_in(root.path()).unwrap());
        let path = directory.path().join("input.arrow");
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
        let owned = staged_view(&input, &session, directory.clone()).unwrap();
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
        assert!(
            !path.exists(),
            "last physical owner reclaims the private directory"
        );
    }
}
