# `datafusion_expr::expr_fn::lambda_var`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr_fn.lambda_var.json).

<a id="op-fb76d81134f885cbc54c8e2f"></a>
## lambda_var

`function` · `datafusion_expr::expr_fn::lambda_var` · datafusion-expr 55.1.0

```rust
fn lambda_var(name: impl Into<String>) -> Expr
```

Source: `src/expr_fn.rs:739`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create an unresolved lambda variable expression

The expression tree or [`LogicalPlan`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-2f2092c4f87ff1cc0b33c3da) which
owns this variable must be resolved before usage with either
[`Expr::resolve_lambda_variables`](../operations/datafusion_expr.expr.Expr.md#op-a84a19ab1b8cbaf85e0d80f0) or [`LogicalPlan::resolve_lambda_variables`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-d1d663c1430a1eb0e6c976f3).

[LogicalPlan::resolve_lambda_variables]: crate::LogicalPlan::resolve_lambda_variables
