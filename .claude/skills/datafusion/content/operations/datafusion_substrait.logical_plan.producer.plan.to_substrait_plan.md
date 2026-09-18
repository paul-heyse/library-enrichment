# `datafusion_substrait::logical_plan::producer::plan::to_substrait_plan`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.producer.plan.to_substrait_plan.json).

<a id="op-cfb4705f4a6372605b49e42d"></a>
## to_substrait_plan

`function` · `datafusion_substrait::logical_plan::producer::plan::to_substrait_plan` · datafusion-substrait 55.1.0

```rust
fn to_substrait_plan(plan: &datafusion::logical_expr::LogicalPlan, state: &datafusion::execution::SessionState) -> datafusion::common::Result<Box<substrait::proto::Plan>>
```

Source: `src/logical_plan/producer/plan.rs:27`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

Convert DataFusion LogicalPlan to Substrait Plan
