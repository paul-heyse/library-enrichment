# `datafusion_physical_plan::sorts::partitioned_topk`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.sorts.partitioned_topk.json).

<a id="op-8b0659d5994bf05040a1a5bc"></a>
## partitioned_topk

`module` · `datafusion_physical_plan::sorts::partitioned_topk` · datafusion-physical-plan 55.1.0

```rust
mod partitioned_topk
```

Source: `src/sorts/partitioned_topk.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

[`PartitionedTopKExec`](../operations/datafusion_physical_plan.sorts.partitioned_topk.PartitionedTopKExec.md#op-665d9b8c9f50ce696eacb602): Top-K per partition operator

For queries like:
```sql
SELECT *, ROW_NUMBER() OVER (PARTITION BY pk ORDER BY val) as rn
FROM t WHERE rn <= N
```

Instead of sorting the entire dataset, this operator delegates to a
per-partition heap-of-K implementation (one variant for `ROW_NUMBER`
and a sibling variant for `RANK`), both of which maintain one heap per
distinct partition key while sharing a single [`arrow::row::RowConverter`](../operations/arrow_row.RowConverter.md#op-3289371bf6ccaf9946ba91e3),
[`MemoryReservation`](datafusion_execution::memory_pool::MemoryReservation),
and metrics set across all partitions, and emit only the top-K rows
per partition in sorted order `(partition_keys, order_keys)`.
