# `datafusion_substrait::logical_plan::consumer::rel::read_rel`

Crate `datafusion-substrait` · 2 public items · structured records in [`model/datafusion_substrait.logical_plan.consumer.rel.read_rel.json`](../model/datafusion_substrait.logical_plan.consumer.rel.read_rel.json)

## apply_masking

`function` · `datafusion_substrait::logical_plan::consumer::rel::read_rel::apply_masking`

```rust
fn apply_masking(schema: datafusion::common::DFSchema, mask_expression: &::core::option::Option<substrait::proto::expression::MaskExpression>) -> datafusion::common::Result<datafusion::common::DFSchema>
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.logical_plan.consumer.rel.read_rel.apply_masking.md).


---

## from_read_rel

`function` · `datafusion_substrait::logical_plan::consumer::rel::read_rel::from_read_rel`

```rust
async fn from_read_rel(consumer: &impl SubstraitConsumer, read: &substrait::proto::ReadRel) -> datafusion::common::Result<datafusion::logical_expr::LogicalPlan>
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.logical_plan.consumer.rel.read_rel.from_read_rel.md).


---
