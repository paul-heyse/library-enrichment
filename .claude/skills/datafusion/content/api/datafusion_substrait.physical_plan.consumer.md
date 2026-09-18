# `datafusion_substrait::physical_plan::consumer`

Crate `datafusion-substrait` · 1 public items · structured records in [`model/datafusion_substrait.physical_plan.consumer.json`](../model/datafusion_substrait.physical_plan.consumer.json)

## from_substrait_rel

`function` · `datafusion_substrait::physical_plan::consumer::from_substrait_rel`

```rust
async fn from_substrait_rel<'async_recursion>(_ctx: &datafusion::prelude::SessionContext, rel: &substrait::proto::Rel, _extensions: &std::collections::HashMap<u32, &String>) -> datafusion::error::Result<std::sync::Arc<dyn ExecutionPlan>> where : 'async_recursion, : 'async_recursion, : 'async_recursion
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.physical_plan.consumer.from_substrait_rel.md).


Convert Substrait Rel to DataFusion ExecutionPlan

---
