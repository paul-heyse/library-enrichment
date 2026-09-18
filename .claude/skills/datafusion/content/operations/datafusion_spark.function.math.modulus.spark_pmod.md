# `datafusion_spark::function::math::modulus::spark_pmod`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.math.modulus.spark_pmod.json).

<a id="op-b31a5e643e674220f4e980d7"></a>
## spark_pmod

`function` · `datafusion_spark::function::math::modulus::spark_pmod` · datafusion-spark 55.1.0

```rust
fn spark_pmod(args: &[datafusion_expr::ColumnarValue], enable_ansi_mode: bool) -> datafusion_common::Result<datafusion_expr::ColumnarValue>
```

Source: `src/function/math/modulus.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Spark-compatible `pmod` function
In ANSI mode, division by zero throws an error.
In legacy mode, division by zero returns NULL (Spark behavior).
