# `datafusion_physical_plan::joins::piecewise_merge_join::exec`

Crate `datafusion-physical-plan` · 1 public items · structured records in [`model/datafusion_physical_plan.joins.piecewise_merge_join.exec.json`](../model/datafusion_physical_plan.joins.piecewise_merge_join.exec.json)

## PiecewiseMergeJoinExec

`struct` · `datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec`

Also reachable as `datafusion_physical_plan::joins::PiecewiseMergeJoinExec`

```rust
struct PiecewiseMergeJoinExec
```

**Fields**: `buffered`, `streamed`, `on`, `operator`, `join_type`

**Implements**: `datafusion_physical_plan::display::DisplayAs`, `datafusion_physical_plan::execution_plan::ExecutionPlan`

**Derives**: Debug

**Methods** (8)

```rust
fn buffered(&self) -> &Arc<dyn ExecutionPlan>
fn compute_properties(buffered: &Arc<dyn ExecutionPlan>, streamed: &Arc<dyn ExecutionPlan>, schema: SchemaRef, join_type: JoinType, join_on: &(PhysicalExprRef, PhysicalExprRef)) -> Result<PlanProperties>
fn join_type(&self) -> JoinType
fn probe_side(join_type: &JoinType) -> JoinSide
fn sort_options(&self) -> &SortOptions
fn streamed(&self) -> &Arc<dyn ExecutionPlan>
fn swap_inputs(&self) -> Result<Arc<dyn ExecutionPlan>>
fn try_new(buffered: Arc<dyn ExecutionPlan>, streamed: Arc<dyn ExecutionPlan>, on: (Arc<dyn PhysicalExpr>, Arc<dyn PhysicalExpr>), operator: Operator, join_type: JoinType, num_partitions: usize) -> Result<Self>
```

**via `datafusion_physical_plan::display::DisplayAs`**

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `datafusion_physical_plan::execution_plan::ExecutionPlan`**

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
fn execute(&self, partition: usize, context: Arc<datafusion_execution::TaskContext>) -> Result<SendableRecordBatchStream>
fn input_distribution_requirements(&self) -> InputDistributionRequirements
fn metrics(&self) -> Option<MetricsSet>
fn name(&self) -> &str
fn properties(&self) -> &Arc<PlanProperties>
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, options: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
fn required_input_distribution(&self) -> Vec<Distribution>
fn required_input_ordering(&self) -> Vec<Option<OrderingRequirements>>
fn reset_state(Arc<self>) -> Result<Arc<dyn ExecutionPlan>>
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
fn with_new_children_and_same_properties(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

`PiecewiseMergeJoinExec` is a join execution plan that only evaluates single range filter and show much
better performance for these workloads than `NestedLoopJoin`

The physical planner will choose to evaluate this join when there is only one comparison filter. This
is a binary expression which contains [`Operator::Lt`], [`Operator::LtEq`], [`Operator::Gt`], and
[`Operator::GtEq`].:
Examples:
 - `col0` < `colb`, `col0` <= `colb`, `col0` > `colb`, `col0` >= `colb`

# Execution Plan Inputs
For `PiecewiseMergeJoin` we label all right inputs as the `streamed' side and the left outputs as the
'buffered' side.

`PiecewiseMergeJoin` takes a sorted input for the side to be buffered and is able to sort streamed record
batches during processing. Sorted input must specifically be ascending/descending based on the operator.

# Algorithms
Classic joins are processed differently compared to existence joins.

## Classic Joins (Inner, Full, Left, Right)
For classic joins we buffer the build side and stream the probe side (the "probe" side).
Both sides are sorted so that we can iterate from index 0 to the end on each side.  This ordering ensures
that when we find the first matching pair of rows, we can emit the current stream row joined with all remaining
probe rows from the match position onward, without rescanning earlier probe rows.

For `<` and `<=` operators, both inputs are sorted in **descending** order, while for `>` and `>=` operators
they are sorted in **ascending** order. This choice ensures that the pointer on the buffered side can advance
monotonically as we stream new batches from the stream side.

The streamed side may arrive unsorted, so this operator sorts each incoming batch in memory before
processing. The buffered side is required to be globally sorted; the plan declares this requirement
in `requires_input_order`, which allows the optimizer to automatically insert a `SortExec` on that side if needed.
By the time this operator runs, the buffered side is guaranteed to be in the proper order.

The pseudocode for the algorithm looks like this:

```text
for stream_row in stream_batch:
    for buffer_row in buffer_batch:
        if compare(stream_row, probe_row):
            output stream_row X buffer_batch[buffer_row:]
        else:
            continue
```

The algorithm uses the streamed side (larger) to drive the loop. This is due to every row on the stream side iterating
the buffered side to find every first match. By doing this, each match can output more result so that output
handling can be better vectorized for performance.

Here is an example:

We perform a `JoinType::Left` with these two batches and the operator being `Operator::Lt`(<). For each
row on the streamed side we move a pointer on the buffered until it matches the condition. Once we reach
the row which matches (in this case with row 1 on streamed will have its first match on row 2 on
buffered; 100 < 200 is true), we can emit all rows after that match. We can emit the rows like this because
if the batch is sorted in ascending order, every subsequent row will also satisfy the condition as they will
all be larger values.

```text
SQL statement:
SELECT *
FROM (VALUES (100), (200), (500)) AS streamed(a)
LEFT JOIN (VALUES (100), (200), (200), (300), (400)) AS buffered(b)
  ON streamed.a < buffered.b;

Processing Row 1:

      Sorted Buffered Side                                         Sorted Streamed Side
      ┌──────────────────┐                                         ┌──────────────────┐
    1 │       100        │                                       1 │       100        │
      ├──────────────────┤                                         ├──────────────────┤
    2 │       200        │ ─┐                                    2 │       200        │
      ├──────────────────┤  │  For row 1 on streamed side with     ├──────────────────┤
    3 │       200        │  │  value 100, we emit rows 2 - 5.    3 │       500        │
      ├──────────────────┤  │  as matches when the operator is     └──────────────────┘
    4 │       300        │  │  `Operator::Lt` (<) Emitting all
      ├──────────────────┤  │  rows after the first match (row
    5 │       400        │ ─┘  2 buffered side; 100 < 200)
      └──────────────────┘

Processing Row 2:
  By sorting the streamed side we know

      Sorted Buffered Side                                         Sorted Streamed Side
      ┌──────────────────┐                                         ┌──────────────────┐
    1 │       100        │                                       1 │       100        │
      ├──────────────────┤                                         ├──────────────────┤
    2 │       200        │ <- Start here when probing for the    2 │       200        │
      ├──────────────────┤    streamed side row 2.                 ├──────────────────┤
    3 │       200        │                                       3 │       500        │
      ├──────────────────┤                                         └──────────────────┘
    4 │       300        │
      ├──────────────────┤
    5 │       400        │
      └──────────────────┘
```

## Existence Joins (Semi, Anti, Mark)
Existence joins are made magnitudes of times faster with a `PiecewiseMergeJoin` as we only need to find
the min/max value of the streamed side to be able to emit all matches on the buffered side. By putting
the side we need to mark onto the sorted buffer side, we can emit all these matches at once.

For less than operations (`<`) both inputs are to be sorted in descending order and vice versa for greater
than (`>`) operations. `SortExec` is used to enforce sorting on the buffered side and streamed side does not
need to be sorted due to only needing to find the min/max.

For Left Semi, Anti, and Mark joins we swap the inputs so that the marked side is on the buffered side.

The pseudocode for the algorithm looks like this:

```text
// Using the example of a less than `<` operation
let max = max_batch(streamed_batch)

for buffer_row in buffer_batch:
    if buffer_row < max:
        output buffer_batch[buffer_row:]
```

Only need to find the min/max value and iterate through the buffered side once.

Here is an example:
We perform a `JoinType::LeftSemi` with these two batches and the operator being `Operator::Lt`(<). Because
the operator is `Operator::Lt` we can find the minimum value in the streamed side; in this case it is 200.
We can then advance a pointer from the start of the buffer side until we find the first value that satisfies
the predicate. All rows after that first matched value satisfy the condition 200 < x so we can mark all of
those rows as matched.

```text
SQL statement:
SELECT *
FROM (VALUES (500), (200), (300)) AS streamed(a)
LEFT SEMI JOIN (VALUES (100), (200), (200), (300), (400)) AS buffered(b)
  ON streamed.a < buffered.b;

         Sorted Buffered Side             Unsorted Streamed Side
           ┌──────────────────┐          ┌──────────────────┐
         1 │       100        │        1 │       500        │
           ├──────────────────┤          ├──────────────────┤
         2 │       200        │        2 │       200        │
           ├──────────────────┤          ├──────────────────┤
         3 │       200        │        3 │       300        │
           ├──────────────────┤          └──────────────────┘
         4 │       300        │ ─┐
           ├──────────────────┤  | We emit matches for row 4 - 5
         5 │       400        │ ─┘ on the buffered side.
           └──────────────────┘
            min value: 200
```

For both types of joins, the buffered side must be sorted ascending for `Operator::Lt` (<) or
`Operator::LtEq` (<=) and descending for `Operator::Gt` (>) or `Operator::GtEq` (>=).

# Partitioning Logic
Piecewise Merge Join requires one buffered side partition + round robin partitioned stream side. A counter
is used in the buffered side to coordinate when all streamed partitions are finished execution. This allows
for processing the rest of the unmatched rows for Left and Full joins. The last partition that finishes
execution will be responsible for outputting the unmatched rows.

# Performance Explanation (cost)
Piecewise Merge Join is used over Nested Loop Join due to its superior performance. Here is the breakdown:

R: Buffered Side
S: Streamed Side

## Piecewise Merge Join (PWMJ)

# Classic Join:
Requires sorting the probe side and, for each probe row, scanning the buffered side until the first match
is found.
    Complexity: `O(sort(S) + num_of_batches(|S|) * scan(R))`.

# Mark Join:
Sorts the probe side, then computes the min/max range of the probe keys and scans the buffered side only
within that range.
  Complexity: `O(|S| + scan(R[range]))`.

## Nested Loop Join
Compares every row from `S` with every row from `R`.
  Complexity: `O(|S| * |R|)`.

## Nested Loop Join
  Always going to be probe (O(S) * O(R)).

# Further Reference Material
DuckDB blog on Range Joins: [Range Joins in DuckDB](https://duckdb.org/2022/05/27/iejoin.html)

---
