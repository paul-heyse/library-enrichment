# `datafusion_spark::function::conditional::expr_fn::if`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.conditional.expr_fn.if.json).

<a id="op-ceeb7be701c45a7f6e9f6bda"></a>
## if

`function` · `datafusion_spark::function::conditional::expr_fn::if` · datafusion-spark 55.1.0

```rust
fn if(arg1: datafusion_expr::Expr, arg2: datafusion_expr::Expr, arg3: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/conditional/mod.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

If arg1 evaluates to true, then returns arg2; otherwise returns arg3
