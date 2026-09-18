# `datafusion_spark::function::string::like::spark_like`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.string.like.spark_like.json).

<a id="op-e7ace7b88de3c1ef77ec3200"></a>
## spark_like

`function` · `datafusion_spark::function::string::like::spark_like` · datafusion-spark 55.1.0

```rust
fn spark_like(args: &[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef>
```

Source: `src/function/string/like.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Returns true if str matches pattern (case sensitive).
