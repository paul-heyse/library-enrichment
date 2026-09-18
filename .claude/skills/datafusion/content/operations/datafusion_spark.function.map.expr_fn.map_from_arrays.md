# `datafusion_spark::function::map::expr_fn::map_from_arrays`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.map.expr_fn.map_from_arrays.json).

<a id="op-96ff2551a56eaf6d9a77749b"></a>
## map_from_arrays

`function` · `datafusion_spark::function::map::expr_fn::map_from_arrays` · datafusion-spark 55.1.0

```rust
fn map_from_arrays(keys: datafusion_expr::Expr, values: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/map/mod.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Creates a map from arrays of keys and values.
