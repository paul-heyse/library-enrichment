# `datafusion_expr::utils::group_window_expr_by_sort_keys`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.utils.group_window_expr_by_sort_keys.json).

<a id="op-7181b449279c9bcaf4cc13c7"></a>
## group_window_expr_by_sort_keys

`function` · `datafusion_expr::utils::group_window_expr_by_sort_keys` · datafusion-expr 55.1.0

```rust
fn group_window_expr_by_sort_keys(window_expr: impl IntoIterator<Item = Expr>) -> datafusion_common::Result<Vec<(Vec<(expr::Sort, bool)>, Vec<Expr>)>>
```

Source: `src/utils.rs:622`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Group a slice of window expression expr by their order by expressions
