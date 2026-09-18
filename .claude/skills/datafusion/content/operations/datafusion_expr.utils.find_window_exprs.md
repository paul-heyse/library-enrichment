# `datafusion_expr::utils::find_window_exprs`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.utils.find_window_exprs.json).

<a id="op-78317ba7370116c608585b54"></a>
## find_window_exprs

`function` · `datafusion_expr::utils::find_window_exprs` · datafusion-expr 55.1.0

```rust
fn find_window_exprs<'a>(exprs: impl IntoIterator<Item = &'a Expr>) -> Vec<Expr>
```

Source: `src/utils.rs:757`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Collect all deeply nested `Expr::WindowFunction`. They are returned in order of occurrence
(depth first), with duplicates omitted.
