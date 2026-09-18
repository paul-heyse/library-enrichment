# `datafusion_spark::function::aggregate::expr_fn::try_sum`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.aggregate.expr_fn.try_sum.json).

<a id="op-49b6c2d708da67862ee1ab3f"></a>
## try_sum

`function` · `datafusion_spark::function::aggregate::expr_fn::try_sum` · datafusion-spark 55.1.0

```rust
fn try_sum(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/aggregate/mod.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Returns the sum of values for a column, or NULL if overflow occurs
