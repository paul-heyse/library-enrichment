# `datafusion_spark::function::datetime::expr_fn::add_months`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.datetime.expr_fn.add_months.json).

<a id="op-a7d90da49caa0c0539d00e04"></a>
## add_months

`function` · `datafusion_spark::function::datetime::expr_fn::add_months` · datafusion-spark 55.1.0

```rust
fn add_months(arg1: datafusion_expr::Expr, arg2: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/datetime/mod.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Returns the date that is months months after start. The function returns NULL if at least one of the input parameters is NULL.
