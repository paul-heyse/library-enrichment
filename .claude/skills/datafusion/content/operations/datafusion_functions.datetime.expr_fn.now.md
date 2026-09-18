# `datafusion_functions::datetime::expr_fn::now`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.datetime.expr_fn.now.json).

<a id="op-31000e76fc086aedff300f4d"></a>
## now

`function` · `datafusion_functions::datetime::expr_fn::now` · datafusion-functions 55.1.0

```rust
fn now() -> datafusion_expr::Expr
```

Source: `src/datetime/mod.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

returns the current timestamp in nanoseconds, using the same value for all instances of now() in same statement
