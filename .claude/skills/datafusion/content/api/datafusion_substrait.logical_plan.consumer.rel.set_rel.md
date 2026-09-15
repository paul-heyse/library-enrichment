# `datafusion_substrait::logical_plan::consumer::rel::set_rel`

Crate `datafusion-substrait` · 1 public items · structured records in [`model/datafusion_substrait.logical_plan.consumer.rel.set_rel.json`](../model/datafusion_substrait.logical_plan.consumer.rel.set_rel.json)

## from_set_rel

`function` · `datafusion_substrait::logical_plan::consumer::rel::set_rel::from_set_rel`

```rust
async fn from_set_rel(consumer: &impl SubstraitConsumer, set: &substrait::proto::SetRel) -> datafusion::common::Result<datafusion::logical_expr::LogicalPlan>
```

---
