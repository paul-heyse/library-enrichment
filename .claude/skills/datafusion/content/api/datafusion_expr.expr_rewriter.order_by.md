# `datafusion_expr::expr_rewriter::order_by`

Crate `datafusion-expr` · 1 public items · structured records in [`model/datafusion_expr.expr_rewriter.order_by.json`](../model/datafusion_expr.expr_rewriter.order_by.json)

## rewrite_sort_cols_by_aggs

`function` · `datafusion_expr::expr_rewriter::order_by::rewrite_sort_cols_by_aggs`

Also reachable as `datafusion_expr::expr_rewriter::rewrite_sort_cols_by_aggs`

```rust
fn rewrite_sort_cols_by_aggs(sorts: impl IntoIterator<Item = impl Into<expr::Sort>>, plan: &LogicalPlan) -> datafusion_common::Result<Vec<expr::Sort>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_rewriter.order_by.rewrite_sort_cols_by_aggs.md).


Rewrite sort on aggregate expressions to sort on the column of aggregate output
For example, `max(x)` is written to `col("max(x)")`

---
