# `datafusion_spark::function::array::expr_fn::array_repeat`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.array.expr_fn.array_repeat.json).

<a id="op-9f01e18b57b4921a7553a9ba"></a>
## array_repeat

`function` · `datafusion_spark::function::array::expr_fn::array_repeat` · datafusion-spark 55.1.0

```rust
fn array_repeat(element: datafusion_expr::Expr, count: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/array/mod.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

returns an array containing element count times.
