# `datafusion_physical_optimizer::window_topn`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.window_topn.json).

<a id="op-6f5f2b3b713a833eca107471"></a>
## window_topn

`module` · `datafusion_physical_optimizer::window_topn` · datafusion-physical-optimizer 55.1.0

```rust
mod window_topn
```

Source: `src/window_topn.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

[`WindowTopN`](../operations/datafusion_physical_optimizer.window_topn.WindowTopN.md#op-d3e707f45dfad8feac4ca8f0) optimizer rule for per-partition top-K window queries.

Detects queries of the form:

```sql
SELECT * FROM (
    SELECT *, ROW_NUMBER() OVER (PARTITION BY pk ORDER BY val) as rn
    FROM t
) WHERE rn <= K;
```

or with `RANK()` in place of `ROW_NUMBER()`:

```sql
SELECT * FROM (
    SELECT *, RANK() OVER (PARTITION BY pk ORDER BY val) as rk
    FROM t
) WHERE rk <= K;
```

And replaces the `FilterExec → BoundedWindowAggExec` pipeline with
`BoundedWindowAggExec → PartitionedTopKExec(fetch=K)`, removing the
`FilterExec` and inserting `PartitionedTopKExec` under the window.

The appropriate [`WindowFnKind`] is forwarded to `PartitionedTopKExec`.
RANK requires a non-empty `ORDER BY` clause (otherwise all rows tie at
rank 1 and the optimization is degenerate).

See [`PartitionedTopKExec`] for details on the replacement operator.

[`PartitionedTopKExec`]: datafusion_physical_plan::sorts::partitioned_topk::PartitionedTopKExec
[`WindowFnKind`]: datafusion_physical_plan::sorts::partitioned_topk::WindowFnKind
