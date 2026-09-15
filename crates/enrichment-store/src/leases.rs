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
    lease: Arc<File>,
}

/// Keep ownership outside native operators that release their inputs after completion.
/// This delegates every planning decision; it adds only retention and the same native
/// partition coalescing used by execute_stream. Each invocation still creates a fresh plan.
#[derive(Debug)]
pub(crate) struct RetentionPlanner {
    pub(crate) inner: Arc<dyn datafusion::execution::context::QueryPlanner + Send + Sync>,
}

#[async_trait]
impl datafusion::execution::context::QueryPlanner for RetentionPlanner {
    async fn create_physical_plan(
        &self,
        logical: &datafusion::logical_expr::LogicalPlan,
        state: &dyn Session,
    ) -> Result<Arc<dyn ExecutionPlan>> {
        use datafusion::{
            common::tree_node::TreeNode,
            physical_plan::{ExecutionPlanProperties, coalesce_partitions::CoalescePartitionsExec},
        };
        let mut plan = self.inner.create_physical_plan(logical, state).await?;
        let mut leases: Vec<Arc<File>> = Vec::new();
        plan.apply(|node| {
            if let Some(owned) = node.downcast_ref::<LeasedExec>()
                && !leases.iter().any(|lease| Arc::ptr_eq(lease, &owned.lease))
            {
                if leases.len() == 1024 {
                    return Err(DataFusionError::ResourcesExhausted(
                        "query retention ownership exceeds 1024 leases".into(),
                    ));
                }
                leases.push(Arc::clone(&owned.lease));
            }
            Ok(TreeNodeRecursion::Continue)
        })?;
        if !leases.is_empty() && plan.output_partitioning().partition_count() > 1 {
            plan = Arc::new(CoalescePartitionsExec::new(plan));
        }
        for lease in leases {
            plan = Arc::new(LeasedExec { input: plan, lease });
        }
        Ok(plan)
    }
}
impl LeasedProvider {
    pub(crate) fn new(input: Arc<dyn TableProvider>, lease: Arc<File>) -> Self {
        Self { input, lease }
    }
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
                provider_as_source(Arc::new(LeasedProvider::new(input, Arc::clone(lease))));
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
            lease: Arc::clone(&self.lease),
        }))
    }
    async fn scan_with_args<'a>(
        &self,
        state: &dyn Session,
        args: ScanArgs<'a>,
    ) -> Result<ScanResult> {
        Ok(ScanResult::new(Arc::new(LeasedExec {
            input: self.input.scan_with_args(state, args).await?.into_inner(),
            lease: Arc::clone(&self.lease),
        })))
    }
}

#[derive(Debug)]
struct LeasedExec {
    input: Arc<dyn ExecutionPlan>,
    lease: Arc<File>,
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
            lease: Arc::clone(&self.lease),
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
            lease: Arc::clone(&self.lease),
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
            _lease: Arc::clone(&self.lease),
        }))
    }
}
struct LeasedStream {
    inner: SendableRecordBatchStream,
    _lease: Arc<File>,
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
