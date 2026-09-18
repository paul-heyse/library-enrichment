# `datafusion_physical_optimizer::pushdown_sort`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.pushdown_sort.json).

<a id="op-b45baf02cb7114994535e760"></a>
## pushdown_sort

`module` · `datafusion_physical_optimizer::pushdown_sort` · datafusion-physical-optimizer 55.1.0

```rust
mod pushdown_sort
```

Source: `src/pushdown_sort.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

Sort Pushdown Optimization

This optimizer attempts to push sort requirements down through the execution plan
tree to data sources that can natively handle them (e.g., by scanning files in
reverse order).

## How it works

1. Detects `SortExec` nodes in the plan
2. Calls `try_pushdown_sort()` on the input to recursively push the sort requirement
3. Each node type defines its own pushdown behavior:
   - **Transparent nodes** (CoalesceBatchesExec, RepartitionExec, etc.) delegate to
     their children and wrap the result
   - **Data sources** (DataSourceExec) check if they can optimize for the ordering
   - **Blocking nodes** return `Unsupported` to stop pushdown
4. Based on the result:
   - `Exact`: Remove the Sort operator (data source guarantees perfect ordering)
   - `Inexact`: Keep Sort but use optimized input (enables early termination for TopK)
   - `Unsupported`: No change

## Capabilities

- **Sort elimination**: when a data source's natural ordering satisfies the
  request, return `Exact` and remove the `SortExec` entirely. Preserves
  `fetch` (LIMIT) from the eliminated `SortExec` for early termination.
- **Statistics-based file sorting**: sort files within each partition by
  min/max statistics. When files are non-overlapping but listed in wrong
  order (e.g., alphabetical order ≠ sort key order), this fixes the ordering
  and enables sort elimination. Works for both single-partition and
  multi-partition plans with multi-file groups.
- **Reverse scan optimization**: when required sort is the reverse of the data source's
  natural ordering, enable reverse scanning (reading row groups in reverse order)
- **Prefix matching**: if data has ordering [A DESC, B ASC] and query needs
  [A DESC], the existing ordering satisfies the requirement (`Exact`).
  If the query needs [A ASC] (reverse of the prefix), a reverse scan is
  used (`Inexact`, `SortExec` retained)

Related issue: <https://github.com/apache/datafusion/issues/17348>
