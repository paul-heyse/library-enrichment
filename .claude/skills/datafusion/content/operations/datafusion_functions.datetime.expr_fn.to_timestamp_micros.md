# `datafusion_functions::datetime::expr_fn::to_timestamp_micros`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.datetime.expr_fn.to_timestamp_micros.json).

<a id="op-5d09f12c395dc54fe743c94e"></a>
## to_timestamp_micros

`function` · `datafusion_functions::datetime::expr_fn::to_timestamp_micros` · datafusion-functions 55.1.0

```rust
fn to_timestamp_micros(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

Source: `src/datetime/mod.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

converts a string and optional formats to a `Timestamp(Microseconds, TimeZone)`
