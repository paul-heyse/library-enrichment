# `datafusion_spark::function::datetime::expr_fn::date_add`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.datetime.expr_fn.date_add.json).

<a id="op-e53b32c4fffc6c18a5e9ef88"></a>
## date_add

`function` · `datafusion_spark::function::datetime::expr_fn::date_add` · datafusion-spark 55.1.0

```rust
fn date_add(arg1: datafusion_expr::Expr, arg2: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/datetime/mod.rs:88`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Returns the date that is days days after start. The function returns NULL if at least one of the input parameters is NULL.
