# `datafusion_physical_plan::sorts::partitioned_topk::PartitionedTopKExec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.sorts.partitioned_topk.PartitionedTopKExec.json).

<a id="op-665d9b8c9f50ce696eacb602"></a>
## PartitionedTopKExec

`struct` · `datafusion_physical_plan::sorts::partitioned_topk::PartitionedTopKExec` · datafusion-physical-plan 55.1.0

```rust
struct PartitionedTopKExec
```

Source: `src/sorts/partitioned_topk.rs:177`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Per-partition Top-K operator for window function queries.

# Background

"Top K per partition" is a common analytics pattern used for queries such as
"find the top 3 products by revenue for each store". The (simplified) SQL
for such a query might be:

```sql
SELECT * FROM (
    SELECT *, ROW_NUMBER() OVER (PARTITION BY store ORDER BY revenue DESC) as rn
    FROM sales
) WHERE rn <= 3;
```

The unoptimized physical plan would be:

```text
FilterExec: rn <= 3
  BoundedWindowAggExec: ROW_NUMBER() PARTITION BY [store] ORDER BY [revenue DESC]
    SortExec: expr=[store ASC, revenue DESC]
      DataSourceExec
```

This plan sorts the **entire** dataset (O(N log N)), computes `ROW_NUMBER`
for **all** rows, and then filters to keep only the top K per partition.
With 10M rows, 1K partitions, and K=3, it sorts all 10M rows but only
keeps 3K.

# Optimization

`PartitionedTopKExec` replaces the `SortExec` and the `FilterExec` is
removed. The optimized plan becomes:

```text
BoundedWindowAggExec: ROW_NUMBER() PARTITION BY [store] ORDER BY [revenue DESC]
  PartitionedTopKExec: fetch=3, partition=[store], order=[revenue DESC]
    DataSourceExec
```

Instead of sorting the entire dataset, this operator reads unsorted input
and delegates to a per-partition heap-of-K implementation (`PartitionedTopK`
for `ROW_NUMBER` and `PartitionedTopKRank` for `RANK`), each maintaining
one heap per distinct partition key while sharing a single
[`arrow::row::RowConverter`](../operations/arrow_row.RowConverter.md#op-3289371bf6ccaf9946ba91e3) /
[`MemoryReservation`](datafusion_execution::memory_pool::MemoryReservation)
across all partitions, and emits only the top-K rows per partition in
sorted order `(partition_keys, order_keys)`.

Cost: O(N log K) time instead of O(N log N), and O(K × P × row_size)
memory where K = fetch, P = number of distinct partitions.
## Why maintaining partition key order in output
Window functions do not require partition keys to be globally sorted, and
enforcing such ordering in the output can introduce unnecessary overhead.
However, the physical optimizer framework currently cannot express an
ordering that is only grouped by some keys while ordered by others. For
example:


# Example

For the query above with `fetch=3` and input:

```text
store | revenue
------|--------
  A   |  100
  B   |   50
  A   |  200
  B   |  150
  A   |  300
  A   |  400
```

The operator maintains two heaps:
- **store=A**: keeps top-3 by revenue DESC → {400, 300, 200}, evicts 100
- **store=B**: keeps top-3 by revenue DESC → {150, 50} (only 2 rows)

Output (sorted by store ASC, revenue DESC):

```text
store | revenue
------|--------
  A   |  400
  A   |  300
  A   |  200
  B   |  150
  B   |   50
```

This is then passed to `BoundedWindowAggExec` which assigns
`ROW_NUMBER` 1, 2, 3 to each partition — all of which satisfy `rn <= 3`.

# Limitations

- Only activated when the window function is `ROW_NUMBER` or `RANK` with
  a `PARTITION BY` clause. `RANK` additionally requires a non-empty
  `ORDER BY` (with an empty `ORDER BY`, every row ties at rank 1 and the
  heap-of-K rewrite doesn't apply). Global top-K (no `PARTITION BY`) is
  already handled efficiently by `SortExec` with `fetch`.
- For very high cardinality partition keys (millions of distinct values),
  both memory usage and runtime overhead can become significant. In such
  cases, the sort-based plan is more robust. Therefore, this optimization
  is currently controlled by a configuration flag.

<a id="op-50d728b544944f91b19f03d9"></a>
## apply_expressions

`function` · `datafusion_physical_plan::sorts::partitioned_topk::PartitionedTopKExec::apply_expressions` · datafusion-physical-plan 55.1.0

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::partitioned_topk::PartitionedTopKExec", "path": "PartitionedTopKExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [338, 1], "end": [453, 2], "filename": "src/sorts/partitioned_topk.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/partitioned_topk.rs:385`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-41c23209183b51873c08112b"></a>
## children

`function` · `datafusion_physical_plan::sorts::partitioned_topk::PartitionedTopKExec::children` · datafusion-physical-plan 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::partitioned_topk::PartitionedTopKExec", "path": "PartitionedTopKExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [338, 1], "end": [453, 2], "filename": "src/sorts/partitioned_topk.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/partitioned_topk.rs:366`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6d344867fe089c6d64c83456"></a>
## clone

`function` · `datafusion_physical_plan::sorts::partitioned_topk::PartitionedTopKExec::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> PartitionedTopKExec
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::partitioned_topk::PartitionedTopKExec", "path": "PartitionedTopKExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [176, 17], "end": [176, 22], "filename": "src/sorts/partitioned_topk.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sorts/partitioned_topk.rs:176`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-97cc23436d099ede54710e2c"></a>
## execute

`function` · `datafusion_physical_plan::sorts::partitioned_topk::PartitionedTopKExec::execute` · datafusion-physical-plan 55.1.0

```rust
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::partitioned_topk::PartitionedTopKExec", "path": "PartitionedTopKExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [338, 1], "end": [453, 2], "filename": "src/sorts/partitioned_topk.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/partitioned_topk.rs:405`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-01bb39e46d21ae0830f321ba"></a>
## expr

`function` · `datafusion_physical_plan::sorts::partitioned_topk::PartitionedTopKExec::expr` · datafusion-physical-plan 55.1.0

```rust
fn expr(&self) -> &LexOrdering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::partitioned_topk::PartitionedTopKExec", "path": "PartitionedTopKExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [203, 1], "end": [293, 2], "filename": "src/sorts/partitioned_topk.rs"}, "trait": null, "trait_path": null}`

Source: `src/sorts/partitioned_topk.rs:254`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns the full sort ordering `[partition_keys..., order_keys...]`.

<a id="op-4313c02ed0cd5d6d55875a6b"></a>
## fetch

`function` · `datafusion_physical_plan::sorts::partitioned_topk::PartitionedTopKExec::fetch` · datafusion-physical-plan 55.1.0

```rust
fn fetch(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::partitioned_topk::PartitionedTopKExec", "path": "PartitionedTopKExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [203, 1], "end": [293, 2], "filename": "src/sorts/partitioned_topk.rs"}, "trait": null, "trait_path": null}`

Source: `src/sorts/partitioned_topk.rs:265`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns the maximum number of rows retained per partition.

<a id="op-075e163cdc1212ea0164dc6a"></a>
## fmt

`function` · `datafusion_physical_plan::sorts::partitioned_topk::PartitionedTopKExec::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::partitioned_topk::PartitionedTopKExec", "path": "PartitionedTopKExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [176, 10], "end": [176, 15], "filename": "src/sorts/partitioned_topk.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sorts/partitioned_topk.rs:176`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d2506e2af0209d365f2790e5"></a>
## fmt_as

`function` · `datafusion_physical_plan::sorts::partitioned_topk::PartitionedTopKExec::fmt_as` · datafusion-physical-plan 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::partitioned_topk::PartitionedTopKExec", "path": "PartitionedTopKExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [295, 1], "end": [336, 2], "filename": "src/sorts/partitioned_topk.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}, "trait_path": "datafusion_physical_plan::display::DisplayAs"}`

Source: `src/sorts/partitioned_topk.rs:296`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eb8f9e17dc757005f88ebaca"></a>
## fn_kind

`function` · `datafusion_physical_plan::sorts::partitioned_topk::PartitionedTopKExec::fn_kind` · datafusion-physical-plan 55.1.0

```rust
fn fn_kind(&self) -> WindowFnKind
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::partitioned_topk::PartitionedTopKExec", "path": "PartitionedTopKExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [203, 1], "end": [293, 2], "filename": "src/sorts/partitioned_topk.rs"}, "trait": null, "trait_path": null}`

Source: `src/sorts/partitioned_topk.rs:270`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns which window function this operator is optimizing.

<a id="op-ed40cbb844579947c1e2b1b9"></a>
## input

`function` · `datafusion_physical_plan::sorts::partitioned_topk::PartitionedTopKExec::input` · datafusion-physical-plan 55.1.0

```rust
fn input(&self) -> &Arc<dyn ExecutionPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::partitioned_topk::PartitionedTopKExec", "path": "PartitionedTopKExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [203, 1], "end": [293, 2], "filename": "src/sorts/partitioned_topk.rs"}, "trait": null, "trait_path": null}`

Source: `src/sorts/partitioned_topk.rs:249`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns the child execution plan.

<a id="op-972613dcb656ce0a0b0dd4ef"></a>
## input_distribution_requirements

`function` · `datafusion_physical_plan::sorts::partitioned_topk::PartitionedTopKExec::input_distribution_requirements` · datafusion-physical-plan 55.1.0

```rust
fn input_distribution_requirements(&self) -> InputDistributionRequirements
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::partitioned_topk::PartitionedTopKExec", "path": "PartitionedTopKExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [338, 1], "end": [453, 2], "filename": "src/sorts/partitioned_topk.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/partitioned_topk.rs:351`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3d82f3937700b3a693373486"></a>
## maintains_input_order

`function` · `datafusion_physical_plan::sorts::partitioned_topk::PartitionedTopKExec::maintains_input_order` · datafusion-physical-plan 55.1.0

```rust
fn maintains_input_order(&self) -> Vec<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::partitioned_topk::PartitionedTopKExec", "path": "PartitionedTopKExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [338, 1], "end": [453, 2], "filename": "src/sorts/partitioned_topk.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/partitioned_topk.rs:362`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6592ebbce27ea86adabbeaee"></a>
## name

`function` · `datafusion_physical_plan::sorts::partitioned_topk::PartitionedTopKExec::name` · datafusion-physical-plan 55.1.0

```rust
fn name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::partitioned_topk::PartitionedTopKExec", "path": "PartitionedTopKExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [338, 1], "end": [453, 2], "filename": "src/sorts/partitioned_topk.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/partitioned_topk.rs:339`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f9816124c1c496d5a25dfe0e"></a>
## partition_prefix_len

`function` · `datafusion_physical_plan::sorts::partitioned_topk::PartitionedTopKExec::partition_prefix_len` · datafusion-physical-plan 55.1.0

```rust
fn partition_prefix_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::partitioned_topk::PartitionedTopKExec", "path": "PartitionedTopKExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [203, 1], "end": [293, 2], "filename": "src/sorts/partitioned_topk.rs"}, "trait": null, "trait_path": null}`

Source: `src/sorts/partitioned_topk.rs:260`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns the number of leading expressions in [`Self::expr`](../operations/datafusion_physical_plan.sorts.partitioned_topk.PartitionedTopKExec.md#op-01bb39e46d21ae0830f321ba) that
define the partition key.

<a id="op-0dea512579e16d16566b4bde"></a>
## properties

`function` · `datafusion_physical_plan::sorts::partitioned_topk::PartitionedTopKExec::properties` · datafusion-physical-plan 55.1.0

```rust
fn properties(&self) -> &Arc<PlanProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::partitioned_topk::PartitionedTopKExec", "path": "PartitionedTopKExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [338, 1], "end": [453, 2], "filename": "src/sorts/partitioned_topk.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/partitioned_topk.rs:343`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-64f41fbb5b1bd59120126035"></a>
## replace_children

`function` · `datafusion_physical_plan::sorts::partitioned_topk::PartitionedTopKExec::replace_children` · datafusion-physical-plan 55.1.0

```rust
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, _: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::partitioned_topk::PartitionedTopKExec", "path": "PartitionedTopKExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [338, 1], "end": [453, 2], "filename": "src/sorts/partitioned_topk.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/partitioned_topk.rs:370`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-07721c489ae3fbea95fd05c3"></a>
## required_input_distribution

`function` · `datafusion_physical_plan::sorts::partitioned_topk::PartitionedTopKExec::required_input_distribution` · datafusion-physical-plan 55.1.0

```rust
fn required_input_distribution(&self) -> Vec<Distribution>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::partitioned_topk::PartitionedTopKExec", "path": "PartitionedTopKExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [338, 1], "end": [453, 2], "filename": "src/sorts/partitioned_topk.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/partitioned_topk.rs:347`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a304ce539e7a36e229a89d03"></a>
## try_new

`function` · `datafusion_physical_plan::sorts::partitioned_topk::PartitionedTopKExec::try_new` · datafusion-physical-plan 55.1.0

```rust
fn try_new(input: Arc<dyn ExecutionPlan>, expr: LexOrdering, partition_prefix_len: usize, fetch: usize, fn_kind: WindowFnKind) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::partitioned_topk::PartitionedTopKExec", "path": "PartitionedTopKExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [203, 1], "end": [293, 2], "filename": "src/sorts/partitioned_topk.rs"}, "trait": null, "trait_path": null}`

Source: `src/sorts/partitioned_topk.rs:229`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a new `PartitionedTopKExec`.

# Arguments

* `input` - The child execution plan providing unsorted input rows.
* `expr` - Full sort ordering `[partition_keys..., order_keys...]`.
  For `PARTITION BY pk ORDER BY val ASC`, this would be `[pk ASC, val ASC]`.
* `partition_prefix_len` - Number of leading expressions in `expr`
  that form the partition key. Must be >= 1.
* `fetch` - Maximum rows to retain per partition (the K in "top-K").
* `fn_kind` - Which ranking window function this operator optimizes
  ([`WindowFnKind::RowNumber`](../operations/datafusion_physical_plan.sorts.partitioned_topk.WindowFnKind.md#op-99bfffea8644338484d99182) or [`WindowFnKind::Rank`](../operations/datafusion_physical_plan.sorts.partitioned_topk.WindowFnKind.md#op-61ea1931e400e25a7990c3d4)).

# Example

```text
// For: ROW_NUMBER() OVER (PARTITION BY store ORDER BY revenue DESC) ... WHERE rn <= 5
PartitionedTopKExec::try_new(
    data_source,
    LexOrdering([store ASC, revenue DESC]),
    1,    // partition_prefix_len: 1 partition column (store)
    5,    // fetch: keep top 5 per partition
    WindowFnKind::RowNumber,
)
```

<a id="op-5e7e3809af6e242d0a3ad05f"></a>
## with_new_children

`function` · `datafusion_physical_plan::sorts::partitioned_topk::PartitionedTopKExec::with_new_children` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::partitioned_topk::PartitionedTopKExec", "path": "PartitionedTopKExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [338, 1], "end": [453, 2], "filename": "src/sorts/partitioned_topk.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/partitioned_topk.rs:395`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
