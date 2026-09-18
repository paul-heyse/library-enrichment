# `datafusion_spark::function::datetime::expr_fn::to_utc_timestamp`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.datetime.expr_fn.to_utc_timestamp.json).

<a id="op-effe23b02095669046080ab8"></a>
## to_utc_timestamp

`function` · `datafusion_spark::function::datetime::expr_fn::to_utc_timestamp` · datafusion-spark 55.1.0

```rust
fn to_utc_timestamp(ts: datafusion_expr::Expr, tz: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/datetime/mod.rs:166`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Interpret a given timestamp `ts` in timezone `tz` and then convert it to UTC timezone.
