# `datafusion_physical_optimizer::window_topn`

Crate `datafusion-physical-optimizer` · 1 public items · structured records in [`model/datafusion_physical_optimizer.window_topn.json`](../model/datafusion_physical_optimizer.window_topn.json)

## WindowTopN

`struct` · `datafusion_physical_optimizer::window_topn::WindowTopN`

```rust
struct WindowTopN
```

**Implements**: `datafusion_session::physical_optimizer::PhysicalOptimizerRule`

**Derives**: Clone, Debug, Default

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_session::physical_optimizer::PhysicalOptimizerRule`**

```rust
fn name(&self) -> &str
fn optimize(&self, plan: Arc<dyn ExecutionPlan>, config: &ConfigOptions) -> Result<Arc<dyn ExecutionPlan>>
fn schema_check(&self) -> bool
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_optimizer.window_topn.WindowTopN.md).


Physical optimizer rule that converts per-partition `ROW_NUMBER` and
`RANK` top-K queries into a more efficient plan using
[`PartitionedTopKExec`].

# Pattern Detected

```text
FilterExec(<ranking fn output> <= K)
  [optional ProjectionExec]
    BoundedWindowAggExec(<ranking fn> PARTITION BY ... ORDER BY ...)
```

# Replacement

```text
[optional ProjectionExec]
  BoundedWindowAggExec(<ranking fn> PARTITION BY ... ORDER BY ...)
    PartitionedTopKExec(fn=<row_number|rank>, partition_keys, order_keys, fetch=K)
```

The `FilterExec` is removed entirely. The child of `BoundedWindowAggExec` is now
`PartitionedTopKExec`, which maintains a per-partition top-K heap (and,
for `RANK`, a sibling ties `Vec`) instead of sorting the whole dataset.

# Supported Predicates

- `rn <= K` → fetch = K
- `rn < K` → fetch = K - 1
- `K >= rn` (flipped) → fetch = K
- `K > rn` (flipped) → fetch = K - 1

# When the Rule Fires

All of the following must be true:
- Config flag `enable_window_topn` is `true`
- The plan matches `FilterExec → [ProjectionExec] → BoundedWindowAggExec`
- The window function is `ROW_NUMBER` or `RANK` (not `DENSE_RANK`)
- The window function has a `PARTITION BY` clause (global top-K is
  already handled by `SortExec` with `fetch`)
- For `RANK`: a non-empty `ORDER BY` clause (otherwise all rows tie
  at rank 1 — the optimization is useless and the boundary-tie storage
  would be unbounded)
- The filter predicate compares the window output column to an integer
  literal using `<=`, `<`, `>=`, or `>`

[`PartitionedTopKExec`]: datafusion_physical_plan::sorts::partitioned_topk::PartitionedTopKExec

---
