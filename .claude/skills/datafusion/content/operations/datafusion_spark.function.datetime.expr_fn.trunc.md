# `datafusion_spark::function::datetime::expr_fn::trunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.datetime.expr_fn.trunc.json).

<a id="op-689165e50accda377f6addad"></a>
## trunc

`function` · `datafusion_spark::function::datetime::expr_fn::trunc` · datafusion-spark 55.1.0

```rust
fn trunc(dt: datafusion_expr::Expr, fmt: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/datetime/mod.rs:151`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Truncates a date `dt` to the unit specified by the format `fmt`.
