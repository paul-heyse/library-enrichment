# `datafusion_spark::function::conversion::spark_cast`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.conversion.spark_cast.json).

<a id="op-d6fa396e1a23e4bfed71158f"></a>
## spark_cast

`function` · `datafusion_spark::function::conversion::spark_cast` · datafusion-spark 55.1.0

```rust
fn spark_cast(config: &datafusion_common::config::ConfigOptions) -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

Source: `src/function/conversion/mod.rs:24`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of spark_cast
