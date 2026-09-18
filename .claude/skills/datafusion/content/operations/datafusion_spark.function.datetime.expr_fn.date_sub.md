# `datafusion_spark::function::datetime::expr_fn::date_sub`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.datetime.expr_fn.date_sub.json).

<a id="op-41b0a5833f641e0a98a283e3"></a>
## date_sub

`function` · `datafusion_spark::function::datetime::expr_fn::date_sub` · datafusion-spark 55.1.0

```rust
fn date_sub(arg1: datafusion_expr::Expr, arg2: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/datetime/mod.rs:93`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Returns the date that is days days before start. The function returns NULL if at least one of the input parameters is NULL.
