# `datafusion_substrait::logical_plan::producer::rel`

Crate `datafusion-substrait` · 1 public items · structured records in [`model/datafusion_substrait.logical_plan.producer.rel.json`](../model/datafusion_substrait.logical_plan.producer.rel.json)

## to_substrait_rel

`function` · `datafusion_substrait::logical_plan::producer::rel::to_substrait_rel`

```rust
fn to_substrait_rel(producer: &mut impl SubstraitProducer, plan: &datafusion::logical_expr::LogicalPlan) -> datafusion::common::Result<Box<substrait::proto::Rel>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.logical_plan.producer.rel.to_substrait_rel.md).


---
