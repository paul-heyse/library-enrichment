# `datafusion_functions_aggregate::regr::regr_r2`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.regr.regr_r2.json).

<a id="op-86de8d350006330e100f548e"></a>
## regr_r2

`function` · `datafusion_functions_aggregate::regr::regr_r2` · datafusion-functions-aggregate 55.1.0

```rust
fn regr_r2(expr_y: datafusion_expr::Expr, expr_x: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/regr.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

Compute a linear regression of type [RegrType::R2](../operations/datafusion_functions_aggregate.regr.RegrType.md#op-8adef3daf50807b624623e2b)
