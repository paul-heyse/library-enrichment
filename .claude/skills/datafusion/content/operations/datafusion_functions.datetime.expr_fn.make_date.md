# `datafusion_functions::datetime::expr_fn::make_date`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.datetime.expr_fn.make_date.json).

<a id="op-05df1981b6d99def4250041d"></a>
## make_date

`function` · `datafusion_functions::datetime::expr_fn::make_date` · datafusion-functions 55.1.0

```rust
fn make_date(year: datafusion_expr::Expr, month: datafusion_expr::Expr, day: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/datetime/mod.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

make a date from year, month and day component parts
