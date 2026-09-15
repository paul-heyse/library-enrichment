# `datafusion_physical_plan::joins::cross_join`

Crate `datafusion-physical-plan` · 1 public items · structured records in [`model/datafusion_physical_plan.joins.cross_join.json`](../model/datafusion_physical_plan.joins.cross_join.json)

## CrossJoinExec

`struct` · `datafusion_physical_plan::joins::cross_join::CrossJoinExec`

Also reachable as `datafusion_physical_plan::joins::CrossJoinExec`

```rust
struct CrossJoinExec
```

**Fields**: `left`, `right`

**Implements**: `datafusion_physical_plan::display::DisplayAs`, `datafusion_physical_plan::execution_plan::ExecutionPlan`

**Derives**: Debug

**Methods** (5)

```rust
fn left(&self) -> &Arc<dyn ExecutionPlan>
fn new(left: Arc<dyn ExecutionPlan>, right: Arc<dyn ExecutionPlan>) -> Self
fn right(&self) -> &Arc<dyn ExecutionPlan>
fn swap_inputs(&self) -> Result<Arc<dyn ExecutionPlan>>
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
```

**via `datafusion_physical_plan::display::DisplayAs`**

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `datafusion_physical_plan::execution_plan::ExecutionPlan`**

```rust
fn apply_expressions(&self, _f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
fn child_stats_requests(&self, partition: Option<usize>) -> Vec<ChildStats>
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
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
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
fn with_new_children_and_same_properties(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Cross Join Execution Plan

This operator is used when there are no predicates between two tables and
returns the Cartesian product of the two tables.

Buffers the left input into memory and then streams batches from each
partition on the right input combining them with the buffered left input
to generate the output.

# Clone / Shared State

Note this structure includes a [`OnceAsync`] that is used to coordinate the
loading of the left side with the processing in each output stream.
Therefore it can not be [`Clone`]

---
