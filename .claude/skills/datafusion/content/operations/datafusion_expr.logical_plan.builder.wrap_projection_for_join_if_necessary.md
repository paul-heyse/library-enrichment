# `datafusion_expr::logical_plan::builder::wrap_projection_for_join_if_necessary`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.builder.wrap_projection_for_join_if_necessary.json).

<a id="op-1bfee971152b03b162f2411a"></a>
## wrap_projection_for_join_if_necessary

`function` · `datafusion_expr::logical_plan::builder::wrap_projection_for_join_if_necessary` · datafusion-expr 55.1.0

```rust
fn wrap_projection_for_join_if_necessary(join_keys: &[Expr], input: logical_plan::LogicalPlan) -> datafusion_common::Result<(logical_plan::LogicalPlan, Vec<datafusion_common::Column>, bool)>
```

Source: `src/logical_plan/builder.rs:2143`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Wrap projection for a plan, if the join keys contains normal expression.
