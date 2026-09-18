# `datafusion_spark::function::datetime::expr_fn::next_day`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.datetime.expr_fn.next_day.json).

<a id="op-2dd9696af967b00186a86ff2"></a>
## next_day

`function` · `datafusion_spark::function::datetime::expr_fn::next_day` · datafusion-spark 55.1.0

```rust
fn next_day(arg1: datafusion_expr::Expr, arg2: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/datetime/mod.rs:131`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Returns the first date which is later than start_date and named as indicated. The function returns NULL if at least one of the input parameters is NULL.
