# `datafusion_substrait::logical_plan::consumer::rel::join_rel`

Crate `datafusion-substrait` · 1 public items · structured records in [`model/datafusion_substrait.logical_plan.consumer.rel.join_rel.json`](../model/datafusion_substrait.logical_plan.consumer.rel.join_rel.json)

## from_join_rel

`function` · `datafusion_substrait::logical_plan::consumer::rel::join_rel::from_join_rel`

```rust
async fn from_join_rel(consumer: &impl SubstraitConsumer, join: &substrait::proto::JoinRel) -> datafusion::common::Result<datafusion::logical_expr::LogicalPlan>
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.logical_plan.consumer.rel.join_rel.from_join_rel.md).


---
