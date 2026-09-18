# `datafusion_physical_plan::union`

Crate `datafusion-physical-plan` · 3 public items · structured records in [`model/datafusion_physical_plan.union.json`](../model/datafusion_physical_plan.union.json)

## can_interleave

`function` · `datafusion_physical_plan::union::can_interleave`

```rust
fn can_interleave<T: Borrow<std::sync::Arc<dyn ExecutionPlan>>>(inputs: impl Iterator<Item = T>) -> bool
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.union.can_interleave.md).


Returns true if all inputs have the same [`Partitioning::Hash`] or [`Partitioning::Range`]
spec, making them safe to interleave. Two inputs are interleave-compatible when partition
`k` covers the identical key range or hash bucket across every input.

Note: compatibility is checked sequentially against the first input, so
`InputDistributionRequirements::co_partitioned` is not needed here.

It might be too strict here in the case that the input partition specs are compatible but not exactly the same.
For example one input partition has the partition spec Hash('a','b','c') and
other has the partition spec Hash('a'), It is safe to derive the out partition with the spec Hash('a','b','c').

---

## InterleaveExec

`struct` · `datafusion_physical_plan::union::InterleaveExec`

```rust
struct InterleaveExec
```

**Implements**: `datafusion_physical_plan::display::DisplayAs`, `datafusion_physical_plan::execution_plan::ExecutionPlan`

**Derives**: Clone, Debug

**Methods** (3)

```rust
fn inputs(&self) -> &Vec<Arc<dyn ExecutionPlan>>
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
fn try_new(inputs: Vec<Arc<dyn ExecutionPlan>>) -> Result<Self>
```

**via `datafusion_physical_plan::display::DisplayAs`**

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `datafusion_physical_plan::execution_plan::ExecutionPlan`**

```rust
fn apply_expressions(&self, _f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
fn benefits_from_input_partitioning(&self) -> Vec<bool>
fn child_stats_requests(&self, partition: Option<usize>) -> Vec<ChildStats>
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
fn maintains_input_order(&self) -> Vec<bool>
fn metrics(&self) -> Option<MetricsSet>
fn name(&self) -> &'static str
fn properties(&self) -> &Arc<PlanProperties>
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, options: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
fn statistics_from_inputs(&self, input_stats: &[Arc<Statistics>], _args: &StatisticsArgs) -> Result<Arc<Statistics>>
fn try_to_proto(&self, ctx: &proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
fn with_new_children_and_same_properties(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.union.InterleaveExec.md).


Combines multiple input streams by interleaving them.

All inputs must share an identical [`Partitioning::Hash`] or [`Partitioning::Range`] so that
partition `k` covers the same data across every input. Each output partition is the
interleaving of the same-indexed partition from all inputs:
`output[k] = input[0][k] + input[1][k] + ... + input[n-1][k]`

# Data Flow
```text
+---------+
|         |---+
| Input 1 |   |
|         |-------------+
+---------+   |         |
              |         |         +---------+
              +------------------>|         |
                +---------------->| Combine |-->
                | +-------------->|         |
                | |     |         +---------+
+---------+     | |     |
|         |-----+ |     |
| Input 2 |       |     |
|         |---------------+
+---------+       |     | |
                  |     | |       +---------+
                  |     +-------->|         |
                  |       +------>| Combine |-->
                  |         +---->|         |
                  |         |     +---------+
+---------+       |         |
|         |-------+         |
| Input 3 |                 |
|         |-----------------+
+---------+
```

---

## UnionExec

`struct` · `datafusion_physical_plan::union::UnionExec`

```rust
struct UnionExec
```

**Implements**: `datafusion_physical_plan::display::DisplayAs`, `datafusion_physical_plan::execution_plan::ExecutionPlan`

**Derives**: Clone, Debug

**Methods** (3)

```rust
fn inputs(&self) -> &Vec<Arc<dyn ExecutionPlan>>
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
fn try_new(inputs: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

**via `datafusion_physical_plan::display::DisplayAs`**

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `datafusion_physical_plan::execution_plan::ExecutionPlan`**

```rust
fn apply_expressions(&self, _f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
fn benefits_from_input_partitioning(&self) -> Vec<bool>
fn cardinality_effect(&self) -> CardinalityEffect
fn check_invariants(&self, check: InvariantLevel) -> Result<()>
fn child_stats_requests(&self, partition: Option<usize>) -> Vec<ChildStats>
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
fn gather_filters_for_pushdown(&self, _phase: FilterPushdownPhase, parent_filters: Vec<Arc<dyn PhysicalExpr>>, _config: &ConfigOptions) -> Result<FilterDescription>
fn handle_child_pushdown_result(&self, phase: FilterPushdownPhase, child_pushdown_result: ChildPushdownResult, _config: &ConfigOptions) -> Result<FilterPushdownPropagation<Arc<dyn ExecutionPlan>>>
fn maintains_input_order(&self) -> Vec<bool>
fn metrics(&self) -> Option<MetricsSet>
fn name(&self) -> &'static str
fn properties(&self) -> &Arc<PlanProperties>
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, options: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
fn statistics_from_inputs(&self, input_stats: &[Arc<Statistics>], args: &StatisticsArgs) -> Result<Arc<Statistics>>
fn supports_limit_pushdown(&self) -> bool
fn try_swapping_with_projection(&self, projection: &ProjectionExec) -> Result<Option<Arc<dyn ExecutionPlan>>>
fn try_to_proto(&self, ctx: &proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
fn with_new_children_and_same_properties(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.union.UnionExec.md).


`UnionExec`: `UNION ALL` execution plan.

`UnionExec` combines multiple inputs with the same schema by
concatenating the partitions.  It does not mix or copy data within
or across partitions. Thus if the input partitions are sorted, the
output partitions of the union are also sorted.

For example, given a `UnionExec` of two inputs, with `N`
partitions, and `M` partitions, there will be `N+M` output
partitions. The first `N` output partitions are from Input 1
partitions, and then next `M` output partitions are from Input 2.

```text
                       ▲       ▲           ▲         ▲
                       │       │           │         │
     Output            │  ...  │           │         │
   Partitions          │0      │N-1        │ N       │N+M-1
(passes through   ┌────┴───────┴───────────┴─────────┴───┐
 the N+M input    │              UnionExec               │
  partitions)     │                                      │
                  └──────────────────────────────────────┘
                                     ▲
                                     │
                                     │
      Input           ┌────────┬─────┴────┬──────────┐
    Partitions        │ ...    │          │     ...  │
                   0  │        │ N-1      │ 0        │  M-1
                 ┌────┴────────┴───┐  ┌───┴──────────┴───┐
                 │                 │  │                  │
                 │                 │  │                  │
                 │                 │  │                  │
                 │                 │  │                  │
                 │                 │  │                  │
                 │                 │  │                  │
                 │Input 1          │  │Input 2           │
                 └─────────────────┘  └──────────────────┘
```

---
