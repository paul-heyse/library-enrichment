# `datafusion_substrait::logical_plan::consumer::plan::from_substrait_plan`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.consumer.plan.from_substrait_plan.json).

<a id="op-4093698229f9655b9e4034f1"></a>
## from_substrait_plan

`function` · `datafusion_substrait::logical_plan::consumer::plan::from_substrait_plan` · datafusion-substrait 55.1.0

```rust
async fn from_substrait_plan(state: &datafusion::execution::SessionState, plan: &substrait::proto::Plan) -> datafusion::common::Result<datafusion::logical_expr::LogicalPlan>
```

Source: `src/logical_plan/consumer/plan.rs:28`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

Convert Substrait Plan to DataFusion LogicalPlan
