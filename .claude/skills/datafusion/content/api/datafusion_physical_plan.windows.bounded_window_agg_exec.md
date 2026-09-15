# `datafusion_physical_plan::windows::bounded_window_agg_exec`

Crate `datafusion-physical-plan` · 2 public items · structured records in [`model/datafusion_physical_plan.windows.bounded_window_agg_exec.json`](../model/datafusion_physical_plan.windows.bounded_window_agg_exec.json)

## BoundedWindowAggExec

`struct` · `datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec`

Also reachable as `datafusion_physical_plan::windows::BoundedWindowAggExec`

```rust
struct BoundedWindowAggExec
```

**Fields**: `input_order_mode`

**Implements**: `datafusion_physical_plan::display::DisplayAs`, `datafusion_physical_plan::execution_plan::ExecutionPlan`

**Derives**: Clone, Debug

**Methods** (7)

```rust
fn input(&self) -> &Arc<dyn ExecutionPlan>
fn partition_by_sort_keys(&self) -> Result<Vec<PhysicalSortExpr>>
fn partition_keys(&self) -> Vec<Arc<dyn PhysicalExpr>>
fn state_observer(&self) -> Option<&Arc<dyn WindowStateObserver>>
fn try_new(window_expr: Vec<Arc<dyn WindowExpr>>, input: Arc<dyn ExecutionPlan>, input_order_mode: InputOrderMode, can_repartition: bool) -> Result<Self>
fn window_expr(&self) -> &[Arc<dyn WindowExpr>]
fn with_state_observer(self, observer: Option<Arc<dyn WindowStateObserver>>) -> Result<Self>
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

## WindowStateObserver

`trait` · `datafusion_physical_plan::windows::bounded_window_agg_exec::WindowStateObserver`

Also reachable as `datafusion_physical_plan::windows::WindowStateObserver`

```rust
trait WindowStateObserver: Send + Sync
```

**Methods** (1)

```rust
fn finalize_window_aggregate(&self, partition_idx: usize, window_expr: &Arc<dyn WindowExpr>, partition_key: &PartitionKey, state: Vec<ScalarValue>) -> Result<()>
```

Callback receiver for per-partition window state.

`state` is the result of [`Accumulator::state`], which is a `&mut self`
call whose trait doc states "this function should not be called twice."
Several built-in aggregates (`median`, `percentile_cont`, `string_agg`,
`min_max_bytes`/`min_max_struct`) `std::mem::take` their internal
buffers to build that state — so `state` is a destructive read, not a
snapshot. The exec fires this at most once per group; a callee that
needs the value beyond the callback must retain it (e.g. clone into
owned storage).

[`Accumulator::state`]: datafusion_expr::Accumulator::state

---
