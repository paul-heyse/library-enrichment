# `datafusion_spark::function::math::expr_fn::pow`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.math.expr_fn.pow.json).

<a id="op-fc7a1d98cc2fd9097b3e7027"></a>
## pow

`function` · `datafusion_spark::function::math::expr_fn::pow` · datafusion-spark 55.1.0

```rust
fn pow(arg1: datafusion_expr::Expr, arg2: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/math/mod.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Returns base raised to the power of exponent. Returns Infinity for pow(0, negative).
