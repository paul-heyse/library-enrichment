# `datafusion_spark::function::string::ilike::spark_ilike`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.string.ilike.spark_ilike.json).

<a id="op-0bdd030bd42d6c4210b696e6"></a>
## spark_ilike

`function` · `datafusion_spark::function::string::ilike::spark_ilike` · datafusion-spark 55.1.0

```rust
fn spark_ilike(args: &[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef>
```

Source: `src/function/string/ilike.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Returns true if str matches pattern (case insensitive).
