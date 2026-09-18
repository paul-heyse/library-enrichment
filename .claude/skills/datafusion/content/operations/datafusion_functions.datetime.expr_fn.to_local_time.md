# `datafusion_functions::datetime::expr_fn::to_local_time`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.datetime.expr_fn.to_local_time.json).

<a id="op-7ed94ba5397db3a8f49026d2"></a>
## to_local_time

`function` · `datafusion_functions::datetime::expr_fn::to_local_time` · datafusion-functions 55.1.0

```rust
fn to_local_time(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

Source: `src/datetime/mod.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

converts a timezone-aware timestamp to local time (with no offset or timezone information), i.e. strips off the timezone from the timestamp
