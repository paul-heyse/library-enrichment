# `datafusion_substrait::logical_plan::consumer::types`

Crate `datafusion-substrait` · 3 public items · structured records in [`model/datafusion_substrait.logical_plan.consumer.types.json`](../model/datafusion_substrait.logical_plan.consumer.types.json)

## field_from_substrait_type

`function` · `datafusion_substrait::logical_plan::consumer::types::field_from_substrait_type`

```rust
fn field_from_substrait_type(consumer: &impl SubstraitConsumer, dt: &substrait::proto::Type, dfs_names: &[String], name_idx: &mut usize) -> datafusion::common::Result<datafusion::arrow::datatypes::FieldRef>
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.logical_plan.consumer.types.field_from_substrait_type.md).


---

## from_substrait_named_struct

`function` · `datafusion_substrait::logical_plan::consumer::types::from_substrait_named_struct`

```rust
fn from_substrait_named_struct(consumer: &impl SubstraitConsumer, base_schema: &substrait::proto::NamedStruct) -> datafusion::common::Result<datafusion::common::DFSchema>
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.logical_plan.consumer.types.from_substrait_named_struct.md).


Convert Substrait NamedStruct to DataFusion DFSchemaRef

---

## from_substrait_type

`function` · `datafusion_substrait::logical_plan::consumer::types::from_substrait_type`

```rust
fn from_substrait_type(consumer: &impl SubstraitConsumer, dt: &substrait::proto::Type, dfs_names: &[String], name_idx: &mut usize) -> datafusion::common::Result<datafusion::arrow::datatypes::DataType>
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.logical_plan.consumer.types.from_substrait_type.md).


---
