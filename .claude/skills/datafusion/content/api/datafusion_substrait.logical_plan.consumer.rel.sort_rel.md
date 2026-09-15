# `datafusion_substrait::logical_plan::consumer::rel::sort_rel`

Crate `datafusion-substrait` · 1 public items · structured records in [`model/datafusion_substrait.logical_plan.consumer.rel.sort_rel.json`](../model/datafusion_substrait.logical_plan.consumer.rel.sort_rel.json)

## from_sort_rel

`function` · `datafusion_substrait::logical_plan::consumer::rel::sort_rel::from_sort_rel`

```rust
async fn from_sort_rel(consumer: &impl SubstraitConsumer, sort: &substrait::proto::SortRel) -> datafusion::common::Result<datafusion::logical_expr::LogicalPlan>
```

---
