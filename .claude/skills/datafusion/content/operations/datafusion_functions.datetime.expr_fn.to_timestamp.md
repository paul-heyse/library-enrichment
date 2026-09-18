# `datafusion_functions::datetime::expr_fn::to_timestamp`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.datetime.expr_fn.to_timestamp.json).

<a id="op-4014389c1312de50b47f3f42"></a>
## to_timestamp

`function` · `datafusion_functions::datetime::expr_fn::to_timestamp` · datafusion-functions 55.1.0

```rust
fn to_timestamp(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

Source: `src/datetime/mod.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

converts a string and optional formats to a `Timestamp(Nanoseconds, TimeZone)`
