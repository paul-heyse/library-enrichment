# `datafusion_substrait::logical_plan::consumer::utils`

Crate `datafusion-substrait` · 4 public items · structured records in [`model/datafusion_substrait.logical_plan.consumer.utils.json`](../model/datafusion_substrait.logical_plan.consumer.utils.json)

## from_substrait_sorts

`function` · `datafusion_substrait::logical_plan::consumer::utils::from_substrait_sorts`

```rust
async fn from_substrait_sorts(consumer: &impl SubstraitConsumer, substrait_sorts: &Vec<substrait::proto::SortField>, input_schema: &datafusion::common::DFSchema) -> datafusion::common::Result<Vec<datafusion::logical_expr::expr::Sort>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.logical_plan.consumer.utils.from_substrait_sorts.md).


Convert Substrait Sorts to DataFusion Exprs

---

## rename_data_type

`function` · `datafusion_substrait::logical_plan::consumer::utils::rename_data_type`

```rust
fn rename_data_type(data_type: &datafusion::arrow::datatypes::DataType, dfs_names: &Vec<String>, name_idx: &mut usize) -> datafusion::common::Result<datafusion::arrow::datatypes::DataType>
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.logical_plan.consumer.utils.rename_data_type.md).


Traverse through the data type (incl. lists/maps/etc), renaming all inner struct fields.

---

## rename_field

`function` · `datafusion_substrait::logical_plan::consumer::utils::rename_field`

```rust
fn rename_field(field: &datafusion::arrow::datatypes::Field, dfs_names: &Vec<String>, unnamed_field_suffix: usize, name_idx: &mut usize) -> datafusion::common::Result<datafusion::arrow::datatypes::Field>
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.logical_plan.consumer.utils.rename_field.md).


Traverse through the field, renaming the provided field itself and all its inner struct fields.

---

## rename_fields_data_type

`function` · `datafusion_substrait::logical_plan::consumer::utils::rename_fields_data_type`

```rust
fn rename_fields_data_type(field: datafusion::arrow::datatypes::Field, dfs_names: &Vec<String>, name_idx: &mut usize) -> datafusion::common::Result<datafusion::arrow::datatypes::Field>
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.logical_plan.consumer.utils.rename_fields_data_type.md).


Rename the field's data type but not the field itself.

---
