# `datafusion_spark::function::array::expr_fn::spark_array_contains`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.array.expr_fn.spark_array_contains.json).

<a id="op-366b9cbcca33e734d6a98cfd"></a>
## spark_array_contains

`function` · `datafusion_spark::function::array::expr_fn::spark_array_contains` · datafusion-spark 55.1.0

```rust
fn spark_array_contains(array: datafusion_expr::Expr, element: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/array/mod.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Returns true if the array contains the element (Spark semantics).
