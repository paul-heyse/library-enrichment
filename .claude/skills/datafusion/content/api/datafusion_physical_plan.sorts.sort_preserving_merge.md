# `datafusion_physical_plan::sorts::sort_preserving_merge`

Crate `datafusion-physical-plan` · 1 public items · structured records in [`model/datafusion_physical_plan.sorts.sort_preserving_merge.json`](../model/datafusion_physical_plan.sorts.sort_preserving_merge.json)

## SortPreservingMergeExec

`struct` · `datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec`

```rust
struct SortPreservingMergeExec
```

**Implements**: `datafusion_physical_plan::display::DisplayAs`, `datafusion_physical_plan::execution_plan::ExecutionPlan`

**Derives**: Clone, Debug

**Methods** (7)

```rust
fn expr(&self) -> &LexOrdering
fn fetch(&self) -> Option<usize>
fn input(&self) -> &Arc<dyn ExecutionPlan>
fn new(expr: LexOrdering, input: Arc<dyn ExecutionPlan>) -> Self
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
fn with_fetch(self, fetch: Option<usize>) -> Self
fn with_round_robin_repartition(self, enable_round_robin_repartition: bool) -> Self
```

**via `datafusion_physical_plan::display::DisplayAs`**

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `datafusion_physical_plan::execution_plan::ExecutionPlan`**

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
fn benefits_from_input_partitioning(&self) -> Vec<bool>
fn cardinality_effect(&self) -> CardinalityEffect
fn child_stats_requests(&self, _partition: Option<usize>) -> Vec<ChildStats>
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
fn fetch(&self) -> Option<usize>
fn input_distribution_requirements(&self) -> InputDistributionRequirements
fn maintains_input_order(&self) -> Vec<bool>
fn metrics(&self) -> Option<MetricsSet>
fn name(&self) -> &'static str
fn properties(&self) -> &Arc<PlanProperties>
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, options: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
fn required_input_distribution(&self) -> Vec<Distribution>
fn required_input_ordering(&self) -> Vec<Option<OrderingRequirements>>
fn statistics_from_inputs(&self, input_stats: &[Arc<Statistics>], _args: &StatisticsArgs) -> Result<Arc<Statistics>>
fn supports_limit_pushdown(&self) -> bool
fn try_swapping_with_projection(&self, projection: &ProjectionExec) -> Result<Option<Arc<dyn ExecutionPlan>>>
fn try_to_proto(&self, ctx: &proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
fn with_fetch(&self, limit: Option<usize>) -> Option<Arc<dyn ExecutionPlan>>
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
fn with_new_children_and_same_properties(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
fn with_preserve_order(&self, preserve_order: bool) -> Option<Arc<dyn ExecutionPlan>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.sorts.sort_preserving_merge.SortPreservingMergeExec.md).


Sort preserving merge execution plan

# Overview

This operator implements a K-way merge. It is used to merge multiple sorted
streams into a single sorted stream and is highly optimized.

## Inputs:

1. A list of sort expressions
2. An input plan, where each partition is sorted with respect to
   these sort expressions.

## Output:

1. A single partition that is also sorted with respect to the expressions

## Diagram

```text
┌─────────────────────────┐
│ ┌───┬───┬───┬───┐       │
│ │ A │ B │ C │ D │ ...   │──┐
│ └───┴───┴───┴───┘       │  │
└─────────────────────────┘  │  ┌───────────────────┐    ┌───────────────────────────────┐
  Stream 1                   │  │                   │    │ ┌───┬───╦═══╦───┬───╦═══╗     │
                             ├─▶│SortPreservingMerge│───▶│ │ A │ B ║ B ║ C │ D ║ E ║ ... │
                             │  │                   │    │ └───┴─▲─╩═══╩───┴───╩═══╝     │
┌─────────────────────────┐  │  └───────────────────┘    └─┬─────┴───────────────────────┘
│ ╔═══╦═══╗               │  │
│ ║ B ║ E ║     ...       │──┘                             │
│ ╚═══╩═══╝               │              Stable sort if `enable_round_robin_repartition=false`:
└─────────────────────────┘              the merged stream places equal rows from stream 1
  Stream 2


 Input Partitions                                          Output Partition
   (sorted)                                                  (sorted)
```

# Error Handling

If any of the input partitions return an error, the error is propagated to
the output and inputs are not polled again.

---
