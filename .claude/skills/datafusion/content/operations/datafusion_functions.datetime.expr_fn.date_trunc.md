# `datafusion_functions::datetime::expr_fn::date_trunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.datetime.expr_fn.date_trunc.json).

<a id="op-48ede03a721f4342ec77be75"></a>
## date_trunc

`function` · `datafusion_functions::datetime::expr_fn::date_trunc` · datafusion-functions 55.1.0

```rust
fn date_trunc(part: datafusion_expr::Expr, date: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/datetime/mod.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

truncates the date to a specified level of precision
