# `datafusion_expr::logical_plan::builder::project`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.builder.project.json).

<a id="op-15b0eab893fc5aa0fc53e824"></a>
## project

`function` · `datafusion_expr::logical_plan::builder::project` · datafusion-expr 55.1.0

```rust
fn project(plan: logical_plan::LogicalPlan, expr: impl IntoIterator<Item = impl Into<select_expr::SelectExpr>>) -> datafusion_common::Result<logical_plan::LogicalPlan>
```

Source: `src/logical_plan/builder.rs:1943`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create Projection
# Errors
This function errors under any of the following conditions:
* Two or more expressions have the same name
* An invalid expression is used (e.g. a `sort` expression)
