# `datafusion_spark::function::datetime::expr_fn::make_interval`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.datetime.expr_fn.make_interval.json).

<a id="op-01ce71c346c682af0fc9634b"></a>
## make_interval

`function` · `datafusion_spark::function::datetime::expr_fn::make_interval` · datafusion-spark 55.1.0

```rust
fn make_interval(years: datafusion_expr::Expr, months: datafusion_expr::Expr, weeks: datafusion_expr::Expr, days: datafusion_expr::Expr, hours: datafusion_expr::Expr, mins: datafusion_expr::Expr, secs: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/datetime/mod.rs:119`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Make interval from years, months, weeks, days, hours, mins and secs.
