# `datafusion_expr::logical_plan::builder::union_by_name`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.builder.union_by_name.json).

<a id="op-4cd6341e12a5c91fdc11dac4"></a>
## union_by_name

`function` · `datafusion_expr::logical_plan::builder::union_by_name` · datafusion-expr 55.1.0

```rust
fn union_by_name(left_plan: logical_plan::LogicalPlan, right_plan: logical_plan::LogicalPlan) -> datafusion_common::Result<logical_plan::LogicalPlan>
```

Source: `src/logical_plan/builder.rs:1928`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Like [`union`](../operations/datafusion_expr.logical_plan.builder.union.md#op-54e9bb17514175cbe14178f8), but combine rows from different tables by name, rather than
by position.
