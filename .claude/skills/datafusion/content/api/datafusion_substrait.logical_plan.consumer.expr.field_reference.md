# `datafusion_substrait::logical_plan::consumer::expr::field_reference`

Crate `datafusion-substrait` · 1 public items · structured records in [`model/datafusion_substrait.logical_plan.consumer.expr.field_reference.json`](../model/datafusion_substrait.logical_plan.consumer.expr.field_reference.json)

## from_field_reference

`function` · `datafusion_substrait::logical_plan::consumer::expr::field_reference::from_field_reference`

```rust
async fn from_field_reference(consumer: &impl SubstraitConsumer, field_ref: &substrait::proto::expression::FieldReference, input_schema: &datafusion::common::DFSchema) -> datafusion::common::Result<datafusion::logical_expr::Expr>
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.logical_plan.consumer.expr.field_reference.from_field_reference.md).


---
