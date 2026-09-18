# `datafusion_functions::core::expr_fn::arrow_try_cast`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.core.expr_fn.arrow_try_cast.json).

<a id="op-fdc06f37396e5857576f3dc3"></a>
## arrow_try_cast

`function` · `datafusion_functions::core::expr_fn::arrow_try_cast` · datafusion-functions 55.1.0

```rust
fn arrow_try_cast(arg1: datafusion_expr::Expr, arg2: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/core/mod.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Casts a value to a specific Arrow data type, returning NULL if the cast fails
