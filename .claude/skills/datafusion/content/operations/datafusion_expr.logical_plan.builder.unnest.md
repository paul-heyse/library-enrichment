# `datafusion_expr::logical_plan::builder::unnest`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.builder.unnest.json).

<a id="op-4622d5e490829e44b3ee5107"></a>
## unnest

`function` · `datafusion_expr::logical_plan::builder::unnest` · datafusion-expr 55.1.0

```rust
fn unnest(input: logical_plan::LogicalPlan, columns: Vec<datafusion_common::Column>) -> datafusion_common::Result<logical_plan::LogicalPlan>
```

Source: `src/logical_plan/builder.rs:2248`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a [`LogicalPlan::Unnest`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-1cdf6ed5deb59cb55472f8c2) plan
