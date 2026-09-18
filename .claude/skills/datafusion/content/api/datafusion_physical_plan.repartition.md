# `datafusion_physical_plan::repartition`

Crate `datafusion-physical-plan` · 4 public items · structured records in [`model/datafusion_physical_plan.repartition.json`](../model/datafusion_physical_plan.repartition.json)

## REPARTITION_RANDOM_STATE

`constant` · `datafusion_physical_plan::repartition::REPARTITION_RANDOM_STATE`

```rust
const REPARTITION_RANDOM_STATE: joins::SeededRandomState = _
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.repartition.REPARTITION_RANDOM_STATE.md).


Fixed RandomState used for hash repartitioning to ensure consistent behavior across
executions and runs.

---

## BatchPartitioner

`struct` · `datafusion_physical_plan::repartition::BatchPartitioner`

```rust
struct BatchPartitioner
```

**Methods** (6)

```rust
fn new_hash_partitioner(exprs: Vec<Arc<dyn PhysicalExpr>>, num_partitions: usize, timer: metrics::Time) -> Result<Self>
fn new_range_partitioner(range_partitioning: &RangePartitioning, timer: metrics::Time) -> Self
fn new_round_robin_partitioner(num_partitions: usize, timer: metrics::Time, input_partition: usize, num_input_partitions: usize) -> Self
fn partition<F>(&mut self, batch: RecordBatch, f: F) -> Result<()> where F: FnMut(usize, RecordBatch) -> Result<()>
fn partition_iter(&mut self, batch: RecordBatch) -> Result<impl Iterator<Item = Result<(usize, RecordBatch)>> + Send + '_>
fn try_new(partitioning: Partitioning, timer: metrics::Time, input_partition: usize, num_input_partitions: usize) -> Result<Self>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.repartition.BatchPartitioner.md).


A utility that can be used to partition batches based on [`Partitioning`]

---

## RangeExpr

`struct` · `datafusion_physical_plan::repartition::RangeExpr`

```rust
struct RangeExpr
```

**Implements**: `core::fmt::Display`, `datafusion_physical_expr_common::physical_expr::PhysicalExpr`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (5)

```rust
fn on_columns(&self) -> &[PhysicalExprRef]
fn sort_options(&self) -> &[SortOptions]
fn split_points(&self) -> &[SplitPoint]
fn try_from_proto(node: &protobuf::PhysicalExprNode, ctx: &datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecodeCtx<'_>) -> Result<PhysicalExprRef>
fn try_new(on_columns: Vec<PhysicalExprRef>, range_partitioning: &RangePartitioning) -> Result<Self>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `datafusion_physical_expr_common::physical_expr::PhysicalExpr`**

```rust
fn children(&self) -> Vec<&PhysicalExprRef>
fn data_type(&self, _input_schema: &Schema) -> Result<DataType>
fn evaluate(&self, batch: &RecordBatch) -> Result<ColumnarValue>
fn fmt_sql(&self, f: &mut Formatter<'_>) -> std::fmt::Result
fn nullable(&self, _input_schema: &Schema) -> Result<bool>
fn try_to_proto(&self, ctx: &datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncodeCtx<'_>) -> Result<Option<protobuf::PhysicalExprNode>>
fn with_new_children(Arc<self>, children: Vec<PhysicalExprRef>) -> Result<PhysicalExprRef>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.repartition.RangeExpr.md).


Physical expression that returns the Range partition for each input row.

This uses the same routing function as [`BatchPartitioner`], so dynamic
filtering and repartitioning agree for every [`ScalarValue`] comparison.

---

## RepartitionExec

`struct` · `datafusion_physical_plan::repartition::RepartitionExec`

```rust
struct RepartitionExec
```

**Implements**: `datafusion_physical_plan::display::DisplayAs`, `datafusion_physical_plan::execution_plan::ExecutionPlan`

**Derives**: Clone, Debug

**Methods** (7)

```rust
fn input(&self) -> &Arc<dyn ExecutionPlan>
fn name(&self) -> &str
fn partitioning(&self) -> &Partitioning
fn preserve_order(&self) -> bool
fn try_from_proto(node: &protobuf::PhysicalPlanNode, ctx: &proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
fn try_new(input: Arc<dyn ExecutionPlan>, partitioning: Partitioning) -> Result<Self>
fn with_preserve_order(self) -> Self
```

**via `datafusion_physical_plan::display::DisplayAs`**

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `datafusion_physical_plan::execution_plan::ExecutionPlan`**

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
fn benefits_from_input_partitioning(&self) -> Vec<bool>
fn cardinality_effect(&self) -> CardinalityEffect
fn child_stats_requests(&self, _partition: Option<usize>) -> Vec<ChildStats>
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
fn gather_filters_for_pushdown(&self, _phase: FilterPushdownPhase, parent_filters: Vec<Arc<dyn PhysicalExpr>>, _config: &ConfigOptions) -> Result<FilterDescription>
fn handle_child_pushdown_result(&self, _phase: FilterPushdownPhase, child_pushdown_result: ChildPushdownResult, _config: &ConfigOptions) -> Result<FilterPushdownPropagation<Arc<dyn ExecutionPlan>>>
fn maintains_input_order(&self) -> Vec<bool>
fn metrics(&self) -> Option<MetricsSet>
fn name(&self) -> &'static str
fn properties(&self) -> &Arc<PlanProperties>
fn repartitioned(&self, target_partitions: usize, _config: &ConfigOptions) -> Result<Option<Arc<dyn ExecutionPlan>>>
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, options: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
fn statistics_from_inputs(&self, input_stats: &[Arc<Statistics>], args: &StatisticsArgs) -> Result<Arc<Statistics>>
fn try_pushdown_sort(&self, order: &[PhysicalSortExpr]) -> Result<SortOrderPushdownResult<Arc<dyn ExecutionPlan>>>
fn try_swapping_with_projection(&self, projection: &ProjectionExec) -> Result<Option<Arc<dyn ExecutionPlan>>>
fn try_to_proto(&self, ctx: &proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<protobuf::PhysicalPlanNode>>
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
fn with_new_children_and_same_properties(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.repartition.RepartitionExec.md).


Maps `N` input partitions to `M` output partitions based on a
[`Partitioning`] scheme.

# Background

DataFusion, like most other commercial systems, with the
notable exception of DuckDB, uses the "Exchange Operator" based
approach to parallelism which works well in practice given
sufficient care in implementation.

DataFusion's planner picks the target number of partitions and
then [`RepartitionExec`] redistributes [`RecordBatch`]es to that number
of output partitions.

For example, given `target_partitions=3` (trying to use 3 cores)
but scanning an input with 2 partitions, `RepartitionExec` can be
used to get 3 even streams of `RecordBatch`es


```text
       ▲                  ▲                  ▲
       │                  │                  │
       │                  │                  │
       │                  │                  │
┌───────────────┐  ┌───────────────┐  ┌───────────────┐
│    GroupBy    │  │    GroupBy    │  │    GroupBy    │
│   (Partial)   │  │   (Partial)   │  │   (Partial)   │
└───────────────┘  └───────────────┘  └───────────────┘
       ▲                  ▲                  ▲
       └──────────────────┼──────────────────┘
                          │
             ┌─────────────────────────┐
             │     RepartitionExec     │
             │   (hash/round robin)    │
             └─────────────────────────┘
                        ▲   ▲
            ┌───────────┘   └───────────┐
            │                           │
            │                           │
       .─────────.                 .─────────.
    ,─'           '─.           ,─'           '─.
   ;      Input      :         ;      Input      :
   :   Partition 0   ;         :   Partition 1   ;
    ╲               ╱           ╲               ╱
     '─.         ,─'             '─.         ,─'
        `───────'                   `───────'
```

# Error Handling

If any of the input partitions return an error, the error is propagated to
all output partitions and inputs are not polled again.

# Output Ordering

If more than one stream is being repartitioned, the output will be some
arbitrary interleaving (and thus unordered) unless
[`Self::with_preserve_order`] specifies otherwise.

# Batch coalescing

Repartitioning one [`RecordBatch`] implies creating multiple smaller batches, potentially
as many as the number of output partitions. [`RepartitionExec`] makes sure that the returned
batches adhere to the configured `datafusion.execution.batch_size` for efficient operations,
and for that, it will automatically coalesce batches right after repartitioning for bounded
inputs. Coalescing is skipped for unbounded inputs so partial batches are emitted promptly.

For this, one shared [`LimitedBatchCoalescer`] per output partition is used:

```text
                        ┌───┐                           ┌───┐
                     ┌─▶│   │────────▶.───────────.     │   │     ┌──────────────────┐
                     │  └───┘ ┌───┐  ( Coalescer 0 )──▶ ├───┤ ───▶│     Output 0     │
                     │┌──────▶│   │──▶`───────────'     │   │     └──────────────────┘
                     ││       └───┘                     └───┘
┌──────────────────┐ ││                                           ┌──────────────────┐
│BatchPartitioner 0│─┘│                                           │     Output 1     │
└──────────────────┘  │                                           └──────────────────┘
                      │
┌──────────────────┐  │                ...                        ┌──────────────────┐
│BatchPartitioner 1│──┘                                           │     Output 2     │
└──────────────────┘                                              └──────────────────┘

                                                                  ┌──────────────────┐
                                                                  │     Output 3     │
                                                                  └──────────────────┘
```

# Spilling Architecture

RepartitionExec uses [`SpillPool`](crate::spill::spill_pool) channels to handle
memory pressure during repartitioning. Each (input partition, output partition)
pair gets its own SpillPool channel for FIFO ordering.

```text
Input Partitions (N)          Output Partitions (M)
────────────────────          ─────────────────────

   Input 0 ──┐                      ┌──▶ Output 0
             │  ┌──────────────┐    │
             ├─▶│ SpillPool    │────┤
             │  │ [In0→Out0]   │    │
   Input 1 ──┤  └──────────────┘    ├──▶ Output 1
             │                       │
             │  ┌──────────────┐    │
             ├─▶│ SpillPool    │────┤
             │  │ [In1→Out0]   │    │
   Input 2 ──┤  └──────────────┘    ├──▶ Output 2
             │                      │
             │       ... (N×M SpillPools total)
             │                      │
             │  ┌──────────────┐    │
             └─▶│ SpillPool    │────┘
                │ [InN→OutM]   │
                └──────────────┘

Each SpillPool maintains FIFO order for its (input, output) pair.
See `RepartitionBatch` for details on the memory/spill decision logic.
```

# Footnote

The "Exchange Operator" was first described in the 1989 paper
[Encapsulation of parallelism in the Volcano query processing
system Paper](https://dl.acm.org/doi/pdf/10.1145/93605.98720)
which uses the term "Exchange" for the concept of repartitioning
data across threads.

For more background, please also see the [Optimizing Repartitions in DataFusion] blog.

[Optimizing Repartitions in DataFusion]: https://datafusion.apache.org/blog/2025/12/15/avoid-consecutive-repartitions

---
