# `datafusion_spark::function::math::expr_fn::width_bucket`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.math.expr_fn.width_bucket.json).

<a id="op-620e53eca0cf87aad30d484c"></a>
## width_bucket

`function` · `datafusion_spark::function::math::expr_fn::width_bucket` · datafusion-spark 55.1.0

```rust
fn width_bucket(arg1: datafusion_expr::Expr, arg2: datafusion_expr::Expr, arg3: datafusion_expr::Expr, arg4: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/math/mod.rs:93`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Returns the bucket number into which the value of this expression would fall after being evaluated.
