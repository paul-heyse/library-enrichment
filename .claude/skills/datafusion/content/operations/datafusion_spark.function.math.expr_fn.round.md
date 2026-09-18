# `datafusion_spark::function::math::expr_fn::round`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.math.expr_fn.round.json).

<a id="op-810a189ea650959f035d0241"></a>
## round

`function` · `datafusion_spark::function::math::expr_fn::round` · datafusion-spark 55.1.0

```rust
fn round(arg1: datafusion_expr::Expr, arg2: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/math/mod.rs:87`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Rounds the value of expr to scale decimal places using HALF_UP rounding mode.
