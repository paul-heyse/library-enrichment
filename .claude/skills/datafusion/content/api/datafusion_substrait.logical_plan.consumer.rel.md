# `datafusion_substrait::logical_plan::consumer::rel`

Crate `datafusion-substrait` · 1 public items · structured records in [`model/datafusion_substrait.logical_plan.consumer.rel.json`](../model/datafusion_substrait.logical_plan.consumer.rel.json)

## from_substrait_rel

`function` · `datafusion_substrait::logical_plan::consumer::rel::from_substrait_rel`

```rust
async fn from_substrait_rel<'async_recursion>(consumer: &impl SubstraitConsumer, relation: &substrait::proto::Rel) -> datafusion::common::Result<datafusion::logical_expr::LogicalPlan> where : 'async_recursion, : 'async_recursion
```

Convert Substrait Rel to DataFusion DataFrame

---
