# `datafusion_physical_plan::joins::nested_loop_join`

Crate `datafusion-physical-plan` · 2 public items · structured records in [`model/datafusion_physical_plan.joins.nested_loop_join.json`](../model/datafusion_physical_plan.joins.nested_loop_join.json)

## NestedLoopJoinExec

`struct` · `datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec`

Also reachable as `datafusion_physical_plan::joins::NestedLoopJoinExec`

```rust
struct NestedLoopJoinExec
```

**Implements**: `datafusion_physical_plan::display::DisplayAs`, `datafusion_physical_plan::execution_plan::ExecutionPlan`, `datafusion_physical_plan::projection::EmbeddedProjection`

**Derives**: Debug

**Methods** (10)

```rust
fn contains_projection(&self) -> bool
fn filter(&self) -> Option<&JoinFilter>
fn join_type(&self) -> &JoinType
fn left(&self) -> &Arc<dyn ExecutionPlan>
fn projection(&self) -> &Option<ProjectionRef>
fn right(&self) -> &Arc<dyn ExecutionPlan>
fn swap_inputs(&self) -> Result<Arc<dyn ExecutionPlan>>
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
fn try_new(left: Arc<dyn ExecutionPlan>, right: Arc<dyn ExecutionPlan>, filter: Option<JoinFilter>, join_type: &JoinType, projection: Option<Vec<usize>>) -> Result<Self>
fn with_projection(&self, projection: Option<Vec<usize>>) -> Result<Self>
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
fn statistics_from_inputs(&self, input_stats: &[Arc<Statistics>], _args: &StatisticsArgs) -> Result<Arc<Statistics>>
fn try_swapping_with_projection(&self, projection: &ProjectionExec) -> Result<Option<Arc<dyn ExecutionPlan>>>
fn try_to_proto(&self, ctx: &proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
fn with_new_children_and_same_properties(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

**via `datafusion_physical_plan::projection::EmbeddedProjection`**

```rust
fn with_projection(&self, projection: Option<Vec<usize>>) -> Result<Self>
```

NestedLoopJoinExec is a build-probe join operator designed for joins that
do not have equijoin keys in their `ON` clause.

# Execution Flow

```text
                                               Incoming right batch
               Left Side Buffered Batches
                      ┌───────────┐              ┌───────────────┐
                      │ ┌───────┐ │              │               │
                      │ │       │ │              │               │
 Current Left Row ───▶│ ├───────├─┤──────────┐   │               │
                      │ │       │ │          │   └───────────────┘
                      │ │       │ │          │           │
                      │ │       │ │          │           │
                      │ └───────┘ │          │           │
                      │ ┌───────┐ │          │           │
                      │ │       │ │          │     ┌─────┘
                      │ │       │ │          │     │
                      │ │       │ │          │     │
                      │ │       │ │          │     │
                      │ │       │ │          │     │
                      │ └───────┘ │          ▼     ▼
                      │   ......  │  ┌──────────────────────┐
                      │           │  │X (Cartesian Product) │
                      │           │  └──────────┬───────────┘
                      └───────────┘             │
                                                │
                                                ▼
                                     ┌───────┬───────────────┐
                                     │       │               │
                                     │       │               │
                                     │       │               │
                                     └───────┴───────────────┘
                                       Intermediate Batch
                                 (For join predicate evaluation)
```

The execution follows a two-phase design:

## 1. Buffering Left Input
- The operator eagerly buffers all left-side input batches into memory,
  util a memory limit is reached.
  Currently, an out-of-memory error will be thrown if all the left-side input batches
  cannot fit into memory at once.
  In the future, it's possible to make this case finish execution. (see
  'Memory-limited Execution' section)
- The rationale for buffering the left side is that scanning the right side
  can be expensive (e.g., decoding Parquet files), so buffering more left
  rows reduces the number of right-side scan passes required.

## 2. Probing Right Input
- Right-side input is streamed batch by batch.
- For each right-side batch:
  - It evaluates the join filter against the full buffered left input.
    This results in a Cartesian product between the right batch and each
    left row -- with the join predicate/filter applied -- for each inner
    loop iteration.
  - Matched results are accumulated into an output buffer. (see more in
    `Output Buffering Strategy` section)
- This process continues until all right-side input is consumed.

# Producing unmatched build-side data
- For special join types like left/full joins, it's required to also output
  unmatched pairs. During execution, bitmaps are kept for both left and right
  sides of the input; they'll be handled by dedicated states in `NLJStream`.
- The final output of the left side unmatched rows is handled by a single
  partition for simplicity, since it only counts a small portion of the
  execution time. (e.g. if probe side has 10k rows, the final output of
  unmatched build side only roughly counts for 1/10k of the total time)

# Output Buffering Strategy
The operator uses an intermediate output buffer to accumulate results. Once
the output threshold is reached (currently set to the same value as
`batch_size` in the configuration), the results will be eagerly output.

# Extra Notes
- The operator always considers the **left** side as the build (buffered) side.
  Therefore, the physical optimizer should assign the smaller input to the left.
- The design try to minimize the intermediate data size to approximately
  1 batch, for better cache locality and memory efficiency.

# Memory-limited Execution
When the memory budget is exceeded during left-side buffering, the operator
falls back to a multi-pass strategy:
1. Buffer as many left rows as fit in memory (one "chunk")
2. On the first pass, the right side is both processed and spilled to disk
3. For each subsequent left chunk, the right side is re-read from the spill file

The fallback is triggered automatically when the initial in-memory load
fails with `ResourcesExhausted` and disk spilling is available. Each
output partition independently re-executes the left child and manages
its own spill state.

All join types are supported. For RIGHT/FULL/RIGHT SEMI/RIGHT ANTI/
RIGHT MARK joins, a global right-side bitmap (indexed by right batch
sequence number) accumulates matches across all left chunks. After the
last left chunk is processed, the right side is replayed one more time
to emit unmatched right rows using the accumulated bitmap.

Tracking issue: <https://github.com/apache/datafusion/issues/15760>

# Clone / Shared State
Note this structure includes a [`OnceAsync`] that is used to coordinate the
loading of the left side with the processing in each output stream.
Therefore it can not be [`Clone`]

---

## NestedLoopJoinExecBuilder

`struct` · `datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExecBuilder`

Also reachable as `datafusion_physical_plan::joins::NestedLoopJoinExecBuilder`

```rust
struct NestedLoopJoinExecBuilder
```

**Implements**: `core::convert::From`

**Methods** (5)

```rust
fn build(self) -> Result<NestedLoopJoinExec>
fn new(left: Arc<dyn ExecutionPlan>, right: Arc<dyn ExecutionPlan>, join_type: JoinType) -> Self
fn with_filter(self, filter: Option<JoinFilter>) -> Self
fn with_projection(self, projection: Option<Vec<usize>>) -> Self
fn with_projection_ref(self, projection: Option<ProjectionRef>) -> Self
```

**via `core::convert::From`**

```rust
fn from(exec: &NestedLoopJoinExec) -> Self
```

Helps to build [`NestedLoopJoinExec`].

---
