# `datafusion_substrait::logical_plan::consumer::plan`

Crate `datafusion-substrait` · 2 public items · structured records in [`model/datafusion_substrait.logical_plan.consumer.plan.json`](../model/datafusion_substrait.logical_plan.consumer.plan.json)

## from_substrait_plan

`function` · `datafusion_substrait::logical_plan::consumer::plan::from_substrait_plan`

```rust
async fn from_substrait_plan(state: &datafusion::execution::SessionState, plan: &substrait::proto::Plan) -> datafusion::common::Result<datafusion::logical_expr::LogicalPlan>
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.logical_plan.consumer.plan.from_substrait_plan.md).


Convert Substrait Plan to DataFusion LogicalPlan

---

## from_substrait_plan_with_consumer

`function` · `datafusion_substrait::logical_plan::consumer::plan::from_substrait_plan_with_consumer`

```rust
async fn from_substrait_plan_with_consumer(consumer: &impl SubstraitConsumer, plan: &substrait::proto::Plan) -> datafusion::common::Result<datafusion::logical_expr::LogicalPlan>
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.logical_plan.consumer.plan.from_substrait_plan_with_consumer.md).


Convert Substrait Plan to DataFusion LogicalPlan using the given consumer

---
