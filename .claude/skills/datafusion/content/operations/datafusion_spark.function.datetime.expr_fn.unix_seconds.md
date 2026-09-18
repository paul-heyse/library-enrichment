# `datafusion_spark::function::datetime::expr_fn::unix_seconds`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.datetime.expr_fn.unix_seconds.json).

<a id="op-1037048b825f9c4dc1287ea5"></a>
## unix_seconds

`function` · `datafusion_spark::function::datetime::expr_fn::unix_seconds` · datafusion-spark 55.1.0

```rust
fn unix_seconds(ts: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/datetime/mod.rs:186`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Returns the number of seconds since epoch (1970-01-01 00:00:00 UTC) for the given timestamp `ts`.
