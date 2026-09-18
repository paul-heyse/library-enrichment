# `datafusion_spark::function::datetime::expr_fn::weekday`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.datetime.expr_fn.weekday.json).

<a id="op-ac071e62d9131ad7168358a5"></a>
## weekday

`function` · `datafusion_spark::function::datetime::expr_fn::weekday` · datafusion-spark 55.1.0

```rust
fn weekday(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/datetime/mod.rs:191`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Returns the day of the week for date/timestamp as an integer where Monday = 0, Tuesday = 1, ..., Sunday = 6.
