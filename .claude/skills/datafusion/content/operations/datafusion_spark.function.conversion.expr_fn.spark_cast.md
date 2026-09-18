# `datafusion_spark::function::conversion::expr_fn::spark_cast`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.conversion.expr_fn.spark_cast.json).

<a id="op-19193433cf07b20f37cfb890"></a>
## spark_cast

`function` · `datafusion_spark::function::conversion::expr_fn::spark_cast` · datafusion-spark 55.1.0

```rust
fn spark_cast(arg1: datafusion_expr::Expr, arg2: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/conversion/mod.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Casts given value to the specified type following Spark-compatible semantics
