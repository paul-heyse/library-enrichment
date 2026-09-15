# `datafusion_physical_plan::sorts::partitioned_topk`

Crate `datafusion-physical-plan` · 2 public items · structured records in [`model/datafusion_physical_plan.sorts.partitioned_topk.json`](../model/datafusion_physical_plan.sorts.partitioned_topk.json)

## WindowFnKind

`enum` · `datafusion_physical_plan::sorts::partitioned_topk::WindowFnKind`

```rust
enum WindowFnKind
```

**Variants**: `RowNumber`, `Rank`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

Which window function `PartitionedTopKExec` is optimizing.

Different ranking functions have different per-partition retention rules:
- [`RowNumber`](Self::RowNumber): exactly K rows per partition.
- [`Rank`](Self::Rank): K rows plus any rows tied at the boundary
  ORDER BY value (RANK semantics — `WHERE rk <= K` may keep more
  than K rows when ties straddle the boundary).

---

## PartitionedTopKExec

`struct` · `datafusion_physical_plan::sorts::partitioned_topk::PartitionedTopKExec`

```rust
struct PartitionedTopKExec
```

**Implements**: `datafusion_physical_plan::display::DisplayAs`, `datafusion_physical_plan::execution_plan::ExecutionPlan`

**Derives**: Clone, Debug

**Methods** (6)

```rust
fn expr(&self) -> &LexOrdering
fn fetch(&self) -> usize
fn fn_kind(&self) -> WindowFnKind
fn input(&self) -> &Arc<dyn ExecutionPlan>
fn partition_prefix_len(&self) -> usize
fn try_new(input: Arc<dyn ExecutionPlan>, expr: LexOrdering, partition_prefix_len: usize, fetch: usize, fn_kind: WindowFnKind) -> Result<Self>
```

**via `datafusion_physical_plan::display::DisplayAs`**

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut Formatter<'_>) -> fmt::Result
```

**via `datafusion_physical_plan::execution_plan::ExecutionPlan`**

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
fn input_distribution_requirements(&self) -> InputDistributionRequirements
fn maintains_input_order(&self) -> Vec<bool>
fn name(&self) -> &'static str
fn properties(&self) -> &Arc<PlanProperties>
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, _: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
fn required_input_distribution(&self) -> Vec<Distribution>
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

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
[`arrow::row::RowConverter`] /
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

---
