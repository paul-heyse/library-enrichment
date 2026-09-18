# `datafusion_spark::function::datetime::expr_fn::from_utc_timestamp`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.datetime.expr_fn.from_utc_timestamp.json).

<a id="op-26b43539a835a32880ef7e33"></a>
## from_utc_timestamp

`function` · `datafusion_spark::function::datetime::expr_fn::from_utc_timestamp` · datafusion-spark 55.1.0

```rust
fn from_utc_timestamp(ts: datafusion_expr::Expr, tz: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/datetime/mod.rs:161`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Interpret a given timestamp `ts` in UTC timezone and then convert it to timezone `tz`.
