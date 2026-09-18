# `datafusion_spark::function::datetime::expr_fn::unix_date`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.datetime.expr_fn.unix_date.json).

<a id="op-3c6bc4c2e4da41e677853bdd"></a>
## unix_date

`function` · `datafusion_spark::function::datetime::expr_fn::unix_date` · datafusion-spark 55.1.0

```rust
fn unix_date(dt: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/datetime/mod.rs:171`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Returns the number of days since epoch (1970-01-01) for the given date `dt`.
