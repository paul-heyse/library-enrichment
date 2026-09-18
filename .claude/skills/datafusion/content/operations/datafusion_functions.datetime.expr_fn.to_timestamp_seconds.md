# `datafusion_functions::datetime::expr_fn::to_timestamp_seconds`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.datetime.expr_fn.to_timestamp_seconds.json).

<a id="op-abe30f904270a6b592e14879"></a>
## to_timestamp_seconds

`function` · `datafusion_functions::datetime::expr_fn::to_timestamp_seconds` · datafusion-functions 55.1.0

```rust
fn to_timestamp_seconds(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

Source: `src/datetime/mod.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

converts a string and optional formats to a `Timestamp(Seconds, TimeZone)`
