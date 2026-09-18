# `datafusion_substrait::physical_plan::producer::to_substrait_rel`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.physical_plan.producer.to_substrait_rel.json).

<a id="op-cda26ea8bd44bc13b81fd36c"></a>
## to_substrait_rel

`function` · `datafusion_substrait::physical_plan::producer::to_substrait_rel` · datafusion-substrait 55.1.0

```rust
fn to_substrait_rel(plan: &dyn ExecutionPlan, _extension_info: &mut (Vec<extensions::SimpleExtensionDeclaration>, std::collections::HashMap<String, u32>)) -> datafusion::error::Result<Box<substrait::proto::Rel>>
```

Source: `src/physical_plan/producer.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

Convert DataFusion ExecutionPlan to Substrait Rel
