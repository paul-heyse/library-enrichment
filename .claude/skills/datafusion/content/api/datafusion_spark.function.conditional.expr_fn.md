# `datafusion_spark::function::conditional::expr_fn`

Crate `datafusion-spark` · 1 public items · structured records in [`model/datafusion_spark.function.conditional.expr_fn.json`](../model/datafusion_spark.function.conditional.expr_fn.json)

## if

`function` · `datafusion_spark::function::conditional::expr_fn::if`

Also reachable as `datafusion_spark::expr_fn::if`

```rust
fn if(arg1: datafusion_expr::Expr, arg2: datafusion_expr::Expr, arg3: datafusion_expr::Expr) -> datafusion_expr::Expr
```

If arg1 evaluates to true, then returns arg2; otherwise returns arg3

---
