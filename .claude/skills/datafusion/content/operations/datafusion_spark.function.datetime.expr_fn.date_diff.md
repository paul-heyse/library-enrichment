# `datafusion_spark::function::datetime::expr_fn::date_diff`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.datetime.expr_fn.date_diff.json).

<a id="op-b4f88cd471cf4a0d6dd87b24"></a>
## date_diff

`function` · `datafusion_spark::function::datetime::expr_fn::date_diff` · datafusion-spark 55.1.0

```rust
fn date_diff(end: datafusion_expr::Expr, start: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/datetime/mod.rs:136`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Returns the number of days from start `start` to end `end`.
