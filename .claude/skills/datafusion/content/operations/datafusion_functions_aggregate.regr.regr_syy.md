# `datafusion_functions_aggregate::regr::regr_syy`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.regr.regr_syy.json).

<a id="op-5bb84742ec1b4a6590884b32"></a>
## regr_syy

`function` · `datafusion_functions_aggregate::regr::regr_syy` · datafusion-functions-aggregate 55.1.0

```rust
fn regr_syy(expr_y: datafusion_expr::Expr, expr_x: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/regr.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

Compute a linear regression of type [RegrType::SYY](../operations/datafusion_functions_aggregate.regr.RegrType.md#op-dbe3b3110b9876d68289fd19)
