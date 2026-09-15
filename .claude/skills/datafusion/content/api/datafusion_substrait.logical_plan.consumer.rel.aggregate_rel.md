# `datafusion_substrait::logical_plan::consumer::rel::aggregate_rel`

Crate `datafusion-substrait` · 1 public items · structured records in [`model/datafusion_substrait.logical_plan.consumer.rel.aggregate_rel.json`](../model/datafusion_substrait.logical_plan.consumer.rel.aggregate_rel.json)

## from_aggregate_rel

`function` · `datafusion_substrait::logical_plan::consumer::rel::aggregate_rel::from_aggregate_rel`

```rust
async fn from_aggregate_rel(consumer: &impl SubstraitConsumer, agg: &substrait::proto::AggregateRel) -> datafusion::common::Result<datafusion::logical_expr::LogicalPlan>
```

---
