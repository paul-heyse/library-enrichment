# `datafusion_functions::datetime::expr_fn::to_timestamp_millis`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.datetime.expr_fn.to_timestamp_millis.json).

<a id="op-4fc0b04cdaf0ae81927c06fa"></a>
## to_timestamp_millis

`function` · `datafusion_functions::datetime::expr_fn::to_timestamp_millis` · datafusion-functions 55.1.0

```rust
fn to_timestamp_millis(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

Source: `src/datetime/mod.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

converts a string and optional formats to a `Timestamp(Milliseconds, TimeZone)`
