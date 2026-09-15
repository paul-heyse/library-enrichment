# `datafusion_physical_plan::sort_pushdown`

Crate `datafusion-physical-plan` · 1 public items · structured records in [`model/datafusion_physical_plan.sort_pushdown.json`](../model/datafusion_physical_plan.sort_pushdown.json)

## SortOrderPushdownResult

`enum` · `datafusion_physical_plan::sort_pushdown::SortOrderPushdownResult`

Also reachable as `datafusion::physical_plan::SortOrderPushdownResult`, `datafusion_physical_plan::SortOrderPushdownResult`

```rust
enum SortOrderPushdownResult<T>
```

**Variants**: `Exact`, `Inexact`, `Unsupported`

**Derives**: Clone, Debug

**Methods** (4)

```rust
fn into_inexact(self) -> Self
fn into_inner(self) -> Option<T>
fn map<U, F: FnOnce(T) -> U>(self, f: F) -> SortOrderPushdownResult<U>
fn try_map<U, E, F: FnOnce(T) -> Result<U, E>>(self, f: F) -> Result<SortOrderPushdownResult<U>, E>
```

Result of attempting to push down sort ordering to a node.

Used by [`ExecutionPlan::try_pushdown_sort`] to communicate
whether and how sort ordering was successfully pushed down.

[`ExecutionPlan::try_pushdown_sort`]: crate::ExecutionPlan::try_pushdown_sort

---
