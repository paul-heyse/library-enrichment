# `datafusion_physical_plan::joins::sort_merge_join::exec`

Crate `datafusion-physical-plan` · 1 public items · structured records in [`model/datafusion_physical_plan.joins.sort_merge_join.exec.json`](../model/datafusion_physical_plan.joins.sort_merge_join.exec.json)

## SortMergeJoinExec

`struct` · `datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec`

Also reachable as `datafusion_physical_plan::joins::SortMergeJoinExec`

```rust
struct SortMergeJoinExec
```

**Fields**: `left`, `right`, `on`, `filter`, `join_type`, `sort_options`, `null_equality`

**Implements**: `datafusion_physical_plan::display::DisplayAs`, `datafusion_physical_plan::execution_plan::ExecutionPlan`

**Derives**: Clone, Debug

**Methods** (11)

```rust
fn filter(&self) -> &Option<JoinFilter>
fn join_type(&self) -> JoinType
fn left(&self) -> &Arc<dyn ExecutionPlan>
fn null_equality(&self) -> NullEquality
fn on(&self) -> &[(PhysicalExprRef, PhysicalExprRef)]
fn probe_side(join_type: &JoinType) -> JoinSide
fn right(&self) -> &Arc<dyn ExecutionPlan>
fn sort_options(&self) -> &[SortOptions]
fn swap_inputs(&self) -> Result<Arc<dyn ExecutionPlan>>
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
fn try_new(left: Arc<dyn ExecutionPlan>, right: Arc<dyn ExecutionPlan>, on: JoinOn, filter: Option<JoinFilter>, join_type: JoinType, sort_options: Vec<SortOptions>, null_equality: NullEquality) -> Result<Self>
```

**via `datafusion_physical_plan::display::DisplayAs`**

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `datafusion_physical_plan::execution_plan::ExecutionPlan`**

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
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
fn try_swapping_with_projection(&self, projection: &ProjectionExec) -> Result<Option<Arc<dyn ExecutionPlan>>>
fn try_to_proto(&self, ctx: &proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
fn with_new_children_and_same_properties(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.joins.sort_merge_join.exec.SortMergeJoinExec.md).


Join execution plan that executes equi-join predicates on multiple partitions using Sort-Merge
join algorithm and applies an optional filter post join. Can be used to join arbitrarily large
inputs where one or both of the inputs don't fit in the available memory.

# Join Expressions

Equi-join predicate (e.g. `<col1> = <col2>`) expressions are represented by [`Self::on`].

Non-equality predicates, which can not be pushed down to join inputs (e.g.
`<col1> != <col2>`) are known as "filter expressions" and are evaluated
after the equijoin predicates. They are represented by [`Self::filter`]. These are optional
expressions.

# Sorting

Assumes that both the left and right input to the join are pre-sorted. It is not the
responsibility of this execution plan to sort the inputs.

# "Streamed" vs "Buffered"

The number of record batches of streamed input currently present in the memory will depend
on the output batch size of the execution plan. There is no spilling support for streamed input.
The comparisons are performed from values of join keys in streamed input with the values of
join keys in buffered input. One row in streamed record batch could be matched with multiple rows in
buffered input batches. Streamed input batches are represented by `StreamedBatch`.

Buffered input is buffered for all record batches having the same value of join key.
If the memory limit increases beyond the specified value and spilling is enabled,
buffered batches could be spilled to disk. If spilling is disabled, the execution
will fail under the same conditions. Multiple record batches of buffered could currently reside
in memory/disk during the execution. The number of buffered batches residing in
memory/disk depends on the number of rows of buffered input having the same value
of join key as that of streamed input rows currently present in memory. Due to pre-sorted inputs,
the algorithm understands when it is not needed anymore, and releases the buffered batches
from memory/disk. Buffered input batches are represented by `BufferedBatch`.

Depending on the type of join, left or right input may be selected as streamed or buffered
respectively. For example, in a left-outer join, the left execution plan will be selected as
streamed input while in a right-outer join, the right execution plan will be selected as the
streamed input.

Reference for the algorithm:
<https://en.wikipedia.org/wiki/Sort-merge_join>.

Helpful short video demonstration:
<https://www.youtube.com/watch?v=jiWCPJtDE2c>.

---
