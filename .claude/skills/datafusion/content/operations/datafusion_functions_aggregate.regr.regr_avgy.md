# `datafusion_functions_aggregate::regr::regr_avgy`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.regr.regr_avgy.json).

<a id="op-2abe02223457f24efa772f8f"></a>
## regr_avgy

`function` · `datafusion_functions_aggregate::regr::regr_avgy` · datafusion-functions-aggregate 55.1.0

```rust
fn regr_avgy(expr_y: datafusion_expr::Expr, expr_x: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/regr.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

Compute a linear regression of type [RegrType::AvgY](../operations/datafusion_functions_aggregate.regr.RegrType.md#op-7a4643ff3521ad328f27a290)
