# `datafusion_substrait::physical_plan::consumer::from_substrait_rel`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.physical_plan.consumer.from_substrait_rel.json).

<a id="op-e537e147795ea4a0dba39673"></a>
## from_substrait_rel

`function` · `datafusion_substrait::physical_plan::consumer::from_substrait_rel` · datafusion-substrait 55.1.0

```rust
async fn from_substrait_rel<'async_recursion>(_ctx: &datafusion::prelude::SessionContext, rel: &substrait::proto::Rel, _extensions: &std::collections::HashMap<u32, &String>) -> datafusion::error::Result<std::sync::Arc<dyn ExecutionPlan>> where : 'async_recursion, : 'async_recursion, : 'async_recursion
```

Source: `src/physical_plan/consumer.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

Convert Substrait Rel to DataFusion ExecutionPlan
