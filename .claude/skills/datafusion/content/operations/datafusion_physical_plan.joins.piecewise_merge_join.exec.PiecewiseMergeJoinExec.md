# `datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.joins.piecewise_merge_join.exec.PiecewiseMergeJoinExec.json).

<a id="op-71e08f549599efb697877878"></a>
## PiecewiseMergeJoinExec

`struct` · `datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec` · datafusion-physical-plan 55.1.0

```rust
struct PiecewiseMergeJoinExec
```

Source: `src/joins/piecewise_merge_join/exec.rs:256`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

`PiecewiseMergeJoinExec` is a join execution plan that only evaluates single range filter and show much
better performance for these workloads than `NestedLoopJoin`

The physical planner will choose to evaluate this join when there is only one comparison filter. This
is a binary expression which contains [`Operator::Lt`](../operations/datafusion_expr_common.operator.Operator.md#op-cd6acf014049368f8f713cfc), [`Operator::LtEq`](../operations/datafusion_expr_common.operator.Operator.md#op-fbf3b4422ce6720aa8c06a26), [`Operator::Gt`](../operations/datafusion_expr_common.operator.Operator.md#op-ba09457bd455640ac8a900ad), and
[`Operator::GtEq`](../operations/datafusion_expr_common.operator.Operator.md#op-fce00c00fae5e2faefc21a89).:
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

<a id="op-56f19e6bcf613eb8a6ad1f1e"></a>
## apply_expressions

`function` · `datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec::apply_expressions` · datafusion-physical-plan 55.1.0

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec", "path": "PiecewiseMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [474, 1], "end": [647, 2], "filename": "src/joins/piecewise_merge_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/piecewise_merge_join/exec.rs:487`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-13728b698a6145d52b213e96"></a>
## buffered

`struct_field` · `datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec::buffered` · datafusion-physical-plan 55.1.0

```rust
buffered: std::sync::Arc<dyn ExecutionPlan>
```

Source: `src/joins/piecewise_merge_join/exec.rs:258`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Left buffered execution plan

<a id="op-3d1bcaab534b3e7acd8477b3"></a>
## buffered

`function` · `datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec::buffered` · datafusion-physical-plan 55.1.0

```rust
fn buffered(&self) -> &Arc<dyn ExecutionPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec", "path": "PiecewiseMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [290, 1], "end": [472, 2], "filename": "src/joins/piecewise_merge_join/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/piecewise_merge_join/exec.rs:385`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Reference to buffered side execution plan

<a id="op-da66cb77ff580f089a4a118e"></a>
## children

`function` · `datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec::children` · datafusion-physical-plan 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec", "path": "PiecewiseMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [474, 1], "end": [647, 2], "filename": "src/joins/piecewise_merge_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/piecewise_merge_join/exec.rs:483`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-87bba3e6e3a3a9d6cd4197ae"></a>
## compute_properties

`function` · `datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec::compute_properties` · datafusion-physical-plan 55.1.0

```rust
fn compute_properties(buffered: &Arc<dyn ExecutionPlan>, streamed: &Arc<dyn ExecutionPlan>, schema: SchemaRef, join_type: JoinType, join_on: &(PhysicalExprRef, PhysicalExprRef)) -> Result<PlanProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec", "path": "PiecewiseMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [290, 1], "end": [472, 2], "filename": "src/joins/piecewise_merge_join/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/piecewise_merge_join/exec.rs:421`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-53f418551f21998a1f2b9bd5"></a>
## execute

`function` · `datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec::execute` · datafusion-physical-plan 55.1.0

```rust
fn execute(&self, partition: usize, context: Arc<datafusion_execution::TaskContext>) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec", "path": "PiecewiseMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [474, 1], "end": [647, 2], "filename": "src/joins/piecewise_merge_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/piecewise_merge_join/exec.rs:597`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-42de22cbbf67579258c24312"></a>
## fmt

`function` · `datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec", "path": "PiecewiseMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 10], "end": [255, 15], "filename": "src/joins/piecewise_merge_join/exec.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/joins/piecewise_merge_join/exec.rs:255`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fab2d2c240957dd4f4fec3e3"></a>
## fmt_as

`function` · `datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec::fmt_as` · datafusion-physical-plan 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec", "path": "PiecewiseMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [649, 1], "end": [676, 2], "filename": "src/joins/piecewise_merge_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}, "trait_path": "datafusion_physical_plan::display::DisplayAs"}`

Source: `src/joins/piecewise_merge_join/exec.rs:650`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5217ad887d839545f91190ce"></a>
## input_distribution_requirements

`function` · `datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec::input_distribution_requirements` · datafusion-physical-plan 55.1.0

```rust
fn input_distribution_requirements(&self) -> InputDistributionRequirements
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec", "path": "PiecewiseMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [474, 1], "end": [647, 2], "filename": "src/joins/piecewise_merge_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/piecewise_merge_join/exec.rs:499`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-44afc0b0b00c45c6261215fa"></a>
## join_type

`function` · `datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec::join_type` · datafusion-physical-plan 55.1.0

```rust
fn join_type(&self) -> JoinType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec", "path": "PiecewiseMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [290, 1], "end": [472, 2], "filename": "src/joins/piecewise_merge_join/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/piecewise_merge_join/exec.rs:395`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Join type

<a id="op-a0def740bbfbe55894eb6fad"></a>
## join_type

`struct_field` · `datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec::join_type` · datafusion-physical-plan 55.1.0

```rust
join_type: datafusion_expr::JoinType
```

Source: `src/joins/piecewise_merge_join/exec.rs:266`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

How the join is performed

<a id="op-47a21dcb39ed1f3b1a6d9a29"></a>
## metrics

`function` · `datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec::metrics` · datafusion-physical-plan 55.1.0

```rust
fn metrics(&self) -> Option<MetricsSet>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec", "path": "PiecewiseMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [474, 1], "end": [647, 2], "filename": "src/joins/piecewise_merge_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/piecewise_merge_join/exec.rs:644`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-19802b81fc278b68de576e87"></a>
## name

`function` · `datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec::name` · datafusion-physical-plan 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec", "path": "PiecewiseMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [474, 1], "end": [647, 2], "filename": "src/joins/piecewise_merge_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/piecewise_merge_join/exec.rs:475`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d38fe7896a36f164c6c77b06"></a>
## on

`struct_field` · `datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec::on` · datafusion-physical-plan 55.1.0

```rust
on: (std::sync::Arc<dyn PhysicalExpr>, std::sync::Arc<dyn PhysicalExpr>)
```

Source: `src/joins/piecewise_merge_join/exec.rs:262`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The two expressions being compared

<a id="op-84cc692e65dc12233397380c"></a>
## operator

`struct_field` · `datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec::operator` · datafusion-physical-plan 55.1.0

```rust
operator: datafusion_expr::Operator
```

Source: `src/joins/piecewise_merge_join/exec.rs:264`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Comparison operator in the range predicate

<a id="op-abe9d80effed6a38dd7bf0f8"></a>
## probe_side

`function` · `datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec::probe_side` · datafusion-physical-plan 55.1.0

```rust
fn probe_side(join_type: &JoinType) -> JoinSide
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec", "path": "PiecewiseMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [290, 1], "end": [472, 2], "filename": "src/joins/piecewise_merge_join/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/piecewise_merge_join/exec.rs:406`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Get probe side (streamed side) for the PiecewiseMergeJoin
In current implementation, probe side is determined according to join type.

<a id="op-208108b8902a86845a462b53"></a>
## properties

`function` · `datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec::properties` · datafusion-physical-plan 55.1.0

```rust
fn properties(&self) -> &Arc<PlanProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec", "path": "PiecewiseMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [474, 1], "end": [647, 2], "filename": "src/joins/piecewise_merge_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/piecewise_merge_join/exec.rs:479`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c806be8e538cfe4b74afb40d"></a>
## replace_children

`function` · `datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec::replace_children` · datafusion-physical-plan 55.1.0

```rust
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, options: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec", "path": "PiecewiseMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [474, 1], "end": [647, 2], "filename": "src/joins/piecewise_merge_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/piecewise_merge_join/exec.rs:521`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-62122842145cda8ca678b85c"></a>
## required_input_distribution

`function` · `datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec::required_input_distribution` · datafusion-physical-plan 55.1.0

```rust
fn required_input_distribution(&self) -> Vec<Distribution>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec", "path": "PiecewiseMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [474, 1], "end": [647, 2], "filename": "src/joins/piecewise_merge_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/piecewise_merge_join/exec.rs:495`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1124e68058ff49e6acfde437"></a>
## required_input_ordering

`function` · `datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec::required_input_ordering` · datafusion-physical-plan 55.1.0

```rust
fn required_input_ordering(&self) -> Vec<Option<OrderingRequirements>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec", "path": "PiecewiseMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [474, 1], "end": [647, 2], "filename": "src/joins/piecewise_merge_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/piecewise_merge_join/exec.rs:506`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c7a756b4c01161908d2de856"></a>
## reset_state

`function` · `datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec::reset_state` · datafusion-physical-plan 55.1.0

```rust
fn reset_state(Arc<self>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec", "path": "PiecewiseMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [474, 1], "end": [647, 2], "filename": "src/joins/piecewise_merge_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/piecewise_merge_join/exec.rs:588`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-24edcee7fceeab7b34667d95"></a>
## sort_options

`function` · `datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec::sort_options` · datafusion-physical-plan 55.1.0

```rust
fn sort_options(&self) -> &SortOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec", "path": "PiecewiseMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [290, 1], "end": [472, 2], "filename": "src/joins/piecewise_merge_join/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/piecewise_merge_join/exec.rs:400`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Reference to sort options

<a id="op-309d60bcdd756fcb7a5c7f00"></a>
## streamed

`struct_field` · `datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec::streamed` · datafusion-physical-plan 55.1.0

```rust
streamed: std::sync::Arc<dyn ExecutionPlan>
```

Source: `src/joins/piecewise_merge_join/exec.rs:260`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Right streamed execution plan

<a id="op-cc354371f6482620737c6a2a"></a>
## streamed

`function` · `datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec::streamed` · datafusion-physical-plan 55.1.0

```rust
fn streamed(&self) -> &Arc<dyn ExecutionPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec", "path": "PiecewiseMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [290, 1], "end": [472, 2], "filename": "src/joins/piecewise_merge_join/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/piecewise_merge_join/exec.rs:390`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Reference to streamed side execution plan

<a id="op-fa3e679eb754da98e2928ac1"></a>
## swap_inputs

`function` · `datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec::swap_inputs` · datafusion-physical-plan 55.1.0

```rust
fn swap_inputs(&self) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec", "path": "PiecewiseMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [290, 1], "end": [472, 2], "filename": "src/joins/piecewise_merge_join/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/piecewise_merge_join/exec.rs:469`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9124f568305ef825ee1029cd"></a>
## try_new

`function` · `datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec::try_new` · datafusion-physical-plan 55.1.0

```rust
fn try_new(buffered: Arc<dyn ExecutionPlan>, streamed: Arc<dyn ExecutionPlan>, on: (Arc<dyn PhysicalExpr>, Arc<dyn PhysicalExpr>), operator: Operator, join_type: JoinType, num_partitions: usize) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec", "path": "PiecewiseMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [290, 1], "end": [472, 2], "filename": "src/joins/piecewise_merge_join/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/piecewise_merge_join/exec.rs:291`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-684f8bcaa917e35949ba655b"></a>
## with_new_children

`function` · `datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec::with_new_children` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec", "path": "PiecewiseMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [474, 1], "end": [647, 2], "filename": "src/joins/piecewise_merge_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/piecewise_merge_join/exec.rs:568`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4ecf5e018c05f682159c7049"></a>
## with_new_children_and_same_properties

`function` · `datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec::with_new_children_and_same_properties` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children_and_same_properties(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec", "path": "PiecewiseMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [474, 1], "end": [647, 2], "filename": "src/joins/piecewise_merge_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/piecewise_merge_join/exec.rs:578`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
