# `datafusion_physical_plan::windows::window_agg_exec`

Crate `datafusion-physical-plan` · 1 public items · structured records in [`model/datafusion_physical_plan.windows.window_agg_exec.json`](../model/datafusion_physical_plan.windows.window_agg_exec.json)

## WindowAggExec

`struct` · `datafusion_physical_plan::windows::window_agg_exec::WindowAggExec`

Also reachable as `datafusion_physical_plan::windows::WindowAggExec`

```rust
struct WindowAggExec
```

**Implements**: `datafusion_physical_plan::display::DisplayAs`, `datafusion_physical_plan::execution_plan::ExecutionPlan`

**Derives**: Clone, Debug

**Methods** (6)

```rust
fn input(&self) -> &Arc<dyn ExecutionPlan>
fn partition_by_sort_keys(&self) -> Result<Vec<PhysicalSortExpr>>
fn partition_keys(&self) -> Vec<Arc<dyn PhysicalExpr>>
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
fn try_new(window_expr: Vec<Arc<dyn WindowExpr>>, input: Arc<dyn ExecutionPlan>, can_repartition: bool) -> Result<Self>
fn window_expr(&self) -> &[Arc<dyn WindowExpr>]
```

**via `datafusion_physical_plan::display::DisplayAs`**

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `datafusion_physical_plan::execution_plan::ExecutionPlan`**

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
fn cardinality_effect(&self) -> CardinalityEffect
fn child_stats_requests(&self, partition: Option<usize>) -> Vec<ChildStats>
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
fn input_distribution_requirements(&self) -> InputDistributionRequirements
fn maintains_input_order(&self) -> Vec<bool>
fn metrics(&self) -> Option<MetricsSet>
fn name(&self) -> &'static str
fn properties(&self) -> &Arc<PlanProperties>
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, options: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
fn required_input_distribution(&self) -> Vec<Distribution>
fn required_input_ordering(&self) -> Vec<Option<OrderingRequirements>>
fn statistics_from_inputs(&self, input_stats: &[Arc<Statistics>], _args: &StatisticsArgs) -> Result<Arc<Statistics>>
fn try_to_proto(&self, ctx: &proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
fn with_new_children_and_same_properties(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Window execution plan

---
