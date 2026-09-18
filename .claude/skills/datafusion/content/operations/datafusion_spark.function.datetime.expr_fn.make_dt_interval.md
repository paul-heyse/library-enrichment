# `datafusion_spark::function::datetime::expr_fn::make_dt_interval`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.datetime.expr_fn.make_dt_interval.json).

<a id="op-2cb246c852d1951f79de2009"></a>
## make_dt_interval

`function` · `datafusion_spark::function::datetime::expr_fn::make_dt_interval` · datafusion-spark 55.1.0

```rust
fn make_dt_interval(days: datafusion_expr::Expr, hours: datafusion_expr::Expr, mins: datafusion_expr::Expr, secs: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/datetime/mod.rs:114`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Make a day time interval from given days, hours, mins and secs (return type is actually a Duration(Microsecond))
