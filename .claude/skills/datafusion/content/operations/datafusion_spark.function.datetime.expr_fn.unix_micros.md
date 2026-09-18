# `datafusion_spark::function::datetime::expr_fn::unix_micros`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.datetime.expr_fn.unix_micros.json).

<a id="op-2b917b6319efb3274a54b58f"></a>
## unix_micros

`function` · `datafusion_spark::function::datetime::expr_fn::unix_micros` · datafusion-spark 55.1.0

```rust
fn unix_micros(ts: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/datetime/mod.rs:176`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Returns the number of microseconds since epoch (1970-01-01 00:00:00 UTC) for the given timestamp `ts`.
