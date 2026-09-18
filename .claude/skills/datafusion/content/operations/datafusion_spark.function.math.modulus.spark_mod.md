# `datafusion_spark::function::math::modulus::spark_mod`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.math.modulus.spark_mod.json).

<a id="op-fa0a066fa194401fcb47cbb7"></a>
## spark_mod

`function` · `datafusion_spark::function::math::modulus::spark_mod` · datafusion-spark 55.1.0

```rust
fn spark_mod(args: &[datafusion_expr::ColumnarValue], enable_ansi_mode: bool) -> datafusion_common::Result<datafusion_expr::ColumnarValue>
```

Source: `src/function/math/modulus.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Spark-compatible `mod` function
In ANSI mode, division by zero throws an error.
In legacy mode, division by zero returns NULL (Spark behavior).
