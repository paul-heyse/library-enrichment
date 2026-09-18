# `datafusion_functions::datetime::expr_fn::date_bin`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.datetime.expr_fn.date_bin.json).

<a id="op-6a298df28d1758ff7cac66b4"></a>
## date_bin

`function` · `datafusion_functions::datetime::expr_fn::date_bin` · datafusion-functions 55.1.0

```rust
fn date_bin(stride: datafusion_expr::Expr, source: datafusion_expr::Expr, origin: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/datetime/mod.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

coerces an arbitrary timestamp to the start of the nearest specified interval
