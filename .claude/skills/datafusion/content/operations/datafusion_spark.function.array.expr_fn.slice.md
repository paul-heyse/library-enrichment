# `datafusion_spark::function::array::expr_fn::slice`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.array.expr_fn.slice.json).

<a id="op-98ae301e9f61d84574f5c529"></a>
## slice

`function` · `datafusion_spark::function::array::expr_fn::slice` · datafusion-spark 55.1.0

```rust
fn slice(array: datafusion_expr::Expr, start: datafusion_expr::Expr, length: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/array/mod.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Returns a slice of the array from the start index with the given length.
