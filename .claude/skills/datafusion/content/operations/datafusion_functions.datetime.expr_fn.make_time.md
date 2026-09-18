# `datafusion_functions::datetime::expr_fn::make_time`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.datetime.expr_fn.make_time.json).

<a id="op-1356b3e6ee7f61501965f030"></a>
## make_time

`function` · `datafusion_functions::datetime::expr_fn::make_time` · datafusion-functions 55.1.0

```rust
fn make_time(hour: datafusion_expr::Expr, minute: datafusion_expr::Expr, second: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/datetime/mod.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

make a time from hour, minute and second component parts
