# `datafusion_functions_aggregate::regr::regr_sxx`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.regr.regr_sxx.json).

<a id="op-a1d3c1f878d6a1f5f437e49e"></a>
## regr_sxx

`function` · `datafusion_functions_aggregate::regr::regr_sxx` · datafusion-functions-aggregate 55.1.0

```rust
fn regr_sxx(expr_y: datafusion_expr::Expr, expr_x: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/regr.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

Compute a linear regression of type [RegrType::SXX](../operations/datafusion_functions_aggregate.regr.RegrType.md#op-9269a85f54e590948afeac5b)
