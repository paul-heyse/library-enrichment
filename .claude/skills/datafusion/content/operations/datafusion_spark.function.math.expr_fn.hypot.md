# `datafusion_spark::function::math::expr_fn::hypot`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.math.expr_fn.hypot.json).

<a id="op-304486bcf8f4986db58f4d85"></a>
## hypot

`function` · `datafusion_spark::function::math::expr_fn::hypot` · datafusion-spark 55.1.0

```rust
fn hypot(arg1: datafusion_expr::Expr, arg2: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/math/mod.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Returns sqrt(a^2 + b^2) without intermediate overflow or underflow.
