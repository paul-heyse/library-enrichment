# `datafusion_substrait::logical_plan::consumer::utils::rename_fields_data_type`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.consumer.utils.rename_fields_data_type.json).

<a id="op-649806cd638c641f99a3dab5"></a>
## rename_fields_data_type

`function` · `datafusion_substrait::logical_plan::consumer::utils::rename_fields_data_type` · datafusion-substrait 55.1.0

```rust
fn rename_fields_data_type(field: datafusion::arrow::datatypes::Field, dfs_names: &Vec<String>, name_idx: &mut usize) -> datafusion::common::Result<datafusion::arrow::datatypes::Field>
```

Source: `src/logical_plan/consumer/utils.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

Rename the field's data type but not the field itself.
