# `datafusion_spark::function::datetime::expr_fn::unix_millis`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.datetime.expr_fn.unix_millis.json).

<a id="op-92633299ef49a405dfbe2a9d"></a>
## unix_millis

`function` · `datafusion_spark::function::datetime::expr_fn::unix_millis` · datafusion-spark 55.1.0

```rust
fn unix_millis(ts: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/datetime/mod.rs:181`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Returns the number of milliseconds since epoch (1970-01-01 00:00:00 UTC) for the given timestamp `ts`.
