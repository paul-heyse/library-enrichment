# `datafusion_spark::function::datetime::expr_fn::time_trunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.datetime.expr_fn.time_trunc.json).

<a id="op-b1b71b71ed6756cb8c2ae6b6"></a>
## time_trunc

`function` · `datafusion_spark::function::datetime::expr_fn::time_trunc` · datafusion-spark 55.1.0

```rust
fn time_trunc(fmt: datafusion_expr::Expr, t: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/datetime/mod.rs:146`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Truncates a time `t` to the unit specified by the format `fmt`.
