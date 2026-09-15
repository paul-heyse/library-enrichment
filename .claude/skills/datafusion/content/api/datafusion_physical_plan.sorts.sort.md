# `datafusion_physical_plan::sorts::sort`

Crate `datafusion-physical-plan` · 3 public items · structured records in [`model/datafusion_physical_plan.sorts.sort.json`](../model/datafusion_physical_plan.sorts.sort.json)

## sort_batch

`function` · `datafusion_physical_plan::sorts::sort::sort_batch`

```rust
fn sort_batch(batch: &arrow::array::RecordBatch, expressions: &datafusion_physical_expr::LexOrdering, fetch: Option<usize>) -> datafusion_common::Result<arrow::array::RecordBatch>
```

---

## sort_batch_chunked

`function` · `datafusion_physical_plan::sorts::sort::sort_batch_chunked`

```rust
fn sort_batch_chunked(batch: &arrow::array::RecordBatch, expressions: &datafusion_physical_expr::LexOrdering, batch_size: usize) -> datafusion_common::Result<Vec<arrow::array::RecordBatch>>
```

Sort a batch and return the result as multiple batches of size `batch_size`.
This is useful when you want to avoid creating one large sorted batch in memory,
and instead want to process the sorted data in smaller chunks.

---

## SortExec

`struct` · `datafusion_physical_plan::sorts::sort::SortExec`

```rust
struct SortExec
```

**Implements**: `datafusion_physical_plan::display::DisplayAs`, `datafusion_physical_plan::execution_plan::ExecutionPlan`

**Derives**: Clone, Debug

**Methods** (10)

```rust
fn dynamic_filter_expr(&self) -> Option<Arc<DynamicFilterPhysicalExpr>>
fn expr(&self) -> &LexOrdering
fn fetch(&self) -> Option<usize>
fn input(&self) -> &Arc<dyn ExecutionPlan>
fn new(expr: LexOrdering, input: Arc<dyn ExecutionPlan>) -> Self
fn preserve_partitioning(&self) -> bool
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
fn with_dynamic_filter_expr(self, filter: Arc<DynamicFilterPhysicalExpr>) -> Result<Self>
fn with_fetch(&self, fetch: Option<usize>) -> Self
fn with_preserve_partitioning(self, preserve_partitioning: bool) -> Self
```

**via `datafusion_physical_plan::display::DisplayAs`**

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut Formatter<'_>) -> fmt::Result
```

**via `datafusion_physical_plan::execution_plan::ExecutionPlan`**

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
fn benefits_from_input_partitioning(&self) -> Vec<bool>
fn cardinality_effect(&self) -> CardinalityEffect
fn child_stats_requests(&self, partition: Option<usize>) -> Vec<ChildStats>
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
fn dynamic_expressions_produced(&self) -> Vec<Arc<dyn PhysicalExpr>>
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
fn fetch(&self) -> Option<usize>
fn gather_filters_for_pushdown(&self, phase: FilterPushdownPhase, parent_filters: Vec<Arc<dyn PhysicalExpr>>, config: &datafusion_common::config::ConfigOptions) -> Result<FilterDescription>
fn handle_child_pushdown_result(&self, _phase: FilterPushdownPhase, child_pushdown_result: ChildPushdownResult, _config: &datafusion_common::config::ConfigOptions) -> Result<FilterPushdownPropagation<Arc<dyn ExecutionPlan>>>
fn input_distribution_requirements(&self) -> InputDistributionRequirements
fn metrics(&self) -> Option<MetricsSet>
fn name(&self) -> &'static str
fn properties(&self) -> &Arc<PlanProperties>
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, options: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
fn required_input_distribution(&self) -> Vec<Distribution>
fn reset_state(Arc<self>) -> Result<Arc<dyn ExecutionPlan>>
fn statistics_from_inputs(&self, input_stats: &[Arc<Statistics>], _args: &StatisticsArgs) -> Result<Arc<Statistics>>
fn try_swapping_with_projection(&self, projection: &ProjectionExec) -> Result<Option<Arc<dyn ExecutionPlan>>>
fn try_to_proto(&self, ctx: &proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
fn with_fetch(&self, limit: Option<usize>) -> Option<Arc<dyn ExecutionPlan>>
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Sort execution plan.

Support sorting datasets that are larger than the memory allotted
by the memory manager, by spilling to disk.

---
