# `datafusion_functions_aggregate::regr::regr_intercept`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.regr.regr_intercept.json).

<a id="op-f19b1e443a62b26dd5b1da32"></a>
## regr_intercept

`function` · `datafusion_functions_aggregate::regr::regr_intercept` · datafusion-functions-aggregate 55.1.0

```rust
fn regr_intercept(expr_y: datafusion_expr::Expr, expr_x: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/regr.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

Compute a linear regression of type [RegrType::Intercept](../operations/datafusion_functions_aggregate.regr.RegrType.md#op-df845dfd12d2720c187616ad)
