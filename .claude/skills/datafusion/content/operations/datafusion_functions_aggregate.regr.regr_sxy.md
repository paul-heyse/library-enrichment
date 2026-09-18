# `datafusion_functions_aggregate::regr::regr_sxy`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.regr.regr_sxy.json).

<a id="op-9e32cd50471b9f21b1deb796"></a>
## regr_sxy

`function` · `datafusion_functions_aggregate::regr::regr_sxy` · datafusion-functions-aggregate 55.1.0

```rust
fn regr_sxy(expr_y: datafusion_expr::Expr, expr_x: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/regr.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

Compute a linear regression of type [RegrType::SXY](../operations/datafusion_functions_aggregate.regr.RegrType.md#op-221d3c7954345cd778912e61)
