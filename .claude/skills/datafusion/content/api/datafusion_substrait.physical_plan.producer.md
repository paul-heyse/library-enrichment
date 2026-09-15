# `datafusion_substrait::physical_plan::producer`

Crate `datafusion-substrait` · 1 public items · structured records in [`model/datafusion_substrait.physical_plan.producer.json`](../model/datafusion_substrait.physical_plan.producer.json)

## to_substrait_rel

`function` · `datafusion_substrait::physical_plan::producer::to_substrait_rel`

```rust
fn to_substrait_rel(plan: &dyn ExecutionPlan, _extension_info: &mut (Vec<extensions::SimpleExtensionDeclaration>, std::collections::HashMap<String, u32>)) -> datafusion::error::Result<Box<substrait::proto::Rel>>
```

Convert DataFusion ExecutionPlan to Substrait Rel

---
