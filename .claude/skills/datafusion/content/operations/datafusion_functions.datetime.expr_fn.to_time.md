# `datafusion_functions::datetime::expr_fn::to_time`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.datetime.expr_fn.to_time.json).

<a id="op-332f59f4c62b6612a4b1f3bb"></a>
## to_time

`function` · `datafusion_functions::datetime::expr_fn::to_time` · datafusion-functions 55.1.0

```rust
fn to_time(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

Source: `src/datetime/mod.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

converts a string and optional formats to a `Time64(Nanoseconds)`
