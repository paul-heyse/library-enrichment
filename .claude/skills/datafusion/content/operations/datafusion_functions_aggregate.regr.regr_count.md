# `datafusion_functions_aggregate::regr::regr_count`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.regr.regr_count.json).

<a id="op-a13ea0381fa706dc9ec8c62f"></a>
## regr_count

`function` · `datafusion_functions_aggregate::regr::regr_count` · datafusion-functions-aggregate 55.1.0

```rust
fn regr_count(expr_y: datafusion_expr::Expr, expr_x: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/regr.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

Compute a linear regression of type [RegrType::Count](../operations/datafusion_functions_aggregate.regr.RegrType.md#op-db7df5a11835c2447cb7e058)
