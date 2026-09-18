# `datafusion_functions_aggregate::regr::regr_slope`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.regr.regr_slope.json).

<a id="op-f3723f0595923b67f775eae8"></a>
## regr_slope

`function` · `datafusion_functions_aggregate::regr::regr_slope` · datafusion-functions-aggregate 55.1.0

```rust
fn regr_slope(expr_y: datafusion_expr::Expr, expr_x: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/regr.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

Compute a linear regression of type [RegrType::Slope](../operations/datafusion_functions_aggregate.regr.RegrType.md#op-1506aa38eba54280b7ededeb)
