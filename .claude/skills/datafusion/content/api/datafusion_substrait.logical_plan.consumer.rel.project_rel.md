# `datafusion_substrait::logical_plan::consumer::rel::project_rel`

Crate `datafusion-substrait` · 1 public items · structured records in [`model/datafusion_substrait.logical_plan.consumer.rel.project_rel.json`](../model/datafusion_substrait.logical_plan.consumer.rel.project_rel.json)

## from_project_rel

`function` · `datafusion_substrait::logical_plan::consumer::rel::project_rel::from_project_rel`

```rust
async fn from_project_rel<'async_recursion>(consumer: &impl SubstraitConsumer, p: &substrait::proto::ProjectRel) -> datafusion::common::Result<datafusion::logical_expr::LogicalPlan> where : 'async_recursion, : 'async_recursion
```

---
