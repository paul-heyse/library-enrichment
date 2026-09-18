# `datafusion_expr::expr_rewriter::order_by::rewrite_sort_cols_by_aggs`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr_rewriter.order_by.rewrite_sort_cols_by_aggs.json).

<a id="op-86650fe4791c1b00de050b2a"></a>
## rewrite_sort_cols_by_aggs

`function` · `datafusion_expr::expr_rewriter::order_by::rewrite_sort_cols_by_aggs` · datafusion-expr 55.1.0

```rust
fn rewrite_sort_cols_by_aggs(sorts: impl IntoIterator<Item = impl Into<expr::Sort>>, plan: &LogicalPlan) -> datafusion_common::Result<Vec<expr::Sort>>
```

Source: `src/expr_rewriter/order_by.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Rewrite sort on aggregate expressions to sort on the column of aggregate output
For example, `max(x)` is written to `col("max(x)")`
