# `datafusion_functions::datetime::expr_fn::to_timestamp_nanos`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.datetime.expr_fn.to_timestamp_nanos.json).

<a id="op-0be8fe23c0712fe3ee4731a8"></a>
## to_timestamp_nanos

`function` · `datafusion_functions::datetime::expr_fn::to_timestamp_nanos` · datafusion-functions 55.1.0

```rust
fn to_timestamp_nanos(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

Source: `src/datetime/mod.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

converts a string and optional formats to a `Timestamp(Nanoseconds, TimeZone)`
