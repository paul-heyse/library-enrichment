# `datafusion_spark::function::datetime::expr_fn::date_trunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.datetime.expr_fn.date_trunc.json).

<a id="op-b98db6f491ff27dea10248bb"></a>
## date_trunc

`function` · `datafusion_spark::function::datetime::expr_fn::date_trunc` · datafusion-spark 55.1.0

```rust
fn date_trunc(fmt: datafusion_expr::Expr, ts: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/datetime/mod.rs:141`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Truncates a timestamp `ts` to the unit specified by the format `fmt`.
