# `datafusion_functions_aggregate::regr::regr_avgx`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.regr.regr_avgx.json).

<a id="op-88c7e1f7f87e0b157783e722"></a>
## regr_avgx

`function` · `datafusion_functions_aggregate::regr::regr_avgx` · datafusion-functions-aggregate 55.1.0

```rust
fn regr_avgx(expr_y: datafusion_expr::Expr, expr_x: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/regr.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

Compute a linear regression of type [RegrType::AvgX](../operations/datafusion_functions_aggregate.regr.RegrType.md#op-837c02fbddf75943edc24450)
