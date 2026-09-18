# `datafusion_expr::utils::find_column_exprs`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.utils.find_column_exprs.json).

<a id="op-0da5f46e05c99cec87b59d0f"></a>
## find_column_exprs

`function` · `datafusion_expr::utils::find_column_exprs` · datafusion-expr 55.1.0

```rust
fn find_column_exprs(exprs: &[Expr]) -> Vec<Expr>
```

Source: `src/utils.rs:901`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Collect all deeply nested `Expr::Column`'s. They are returned in order of
appearance (depth first), and may contain duplicates.
