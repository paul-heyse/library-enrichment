# `datafusion_substrait::logical_plan::consumer::rel::fetch_rel`

Crate `datafusion-substrait` · 1 public items · structured records in [`model/datafusion_substrait.logical_plan.consumer.rel.fetch_rel.json`](../model/datafusion_substrait.logical_plan.consumer.rel.fetch_rel.json)

## from_fetch_rel

`function` · `datafusion_substrait::logical_plan::consumer::rel::fetch_rel::from_fetch_rel`

```rust
async fn from_fetch_rel<'async_recursion>(consumer: &impl SubstraitConsumer, fetch: &substrait::proto::FetchRel) -> datafusion::common::Result<datafusion::logical_expr::LogicalPlan> where : 'async_recursion, : 'async_recursion
```

---
