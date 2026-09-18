# `datafusion_spark::function::array::spark_array::make_array_inner`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.array.spark_array.make_array_inner.json).

<a id="op-0eca6354739485bc820664b7"></a>
## make_array_inner

`function` · `datafusion_spark::function::array::spark_array::make_array_inner` · datafusion-spark 55.1.0

```rust
fn make_array_inner(arrays: &[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef>
```

Source: `src/function/array/spark_array.rs:112`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

`make_array_inner` is the implementation of the `make_array` function.
Constructs an array using the input `data` as `ArrayRef`.
Returns a reference-counted `Array` instance result.
