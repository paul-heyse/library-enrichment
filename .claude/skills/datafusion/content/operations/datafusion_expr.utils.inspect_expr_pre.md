# `datafusion_expr::utils::inspect_expr_pre`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.utils.inspect_expr_pre.json).

<a id="op-5a8f11d8bccd57e9c950139b"></a>
## inspect_expr_pre

`function` · `datafusion_expr::utils::inspect_expr_pre` · datafusion-expr 55.1.0

```rust
fn inspect_expr_pre<F, E>(expr: &Expr, f: F) -> datafusion_common::Result<(), E> where F: FnMut(&Expr) -> datafusion_common::Result<(), E>
```

Source: `src/utils.rs:817`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Recursively inspect an [`Expr`](../operations/datafusion_expr.expr.Expr.md#op-230499d6f244cf7372db53bc) and all its children.
