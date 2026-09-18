# `datafusion_substrait::logical_plan::consumer::utils::rename_data_type`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.consumer.utils.rename_data_type.json).

<a id="op-1166800e3c154615ee1c7a69"></a>
## rename_data_type

`function` · `datafusion_substrait::logical_plan::consumer::utils::rename_data_type` · datafusion-substrait 55.1.0

```rust
fn rename_data_type(data_type: &datafusion::arrow::datatypes::DataType, dfs_names: &Vec<String>, name_idx: &mut usize) -> datafusion::common::Result<datafusion::arrow::datatypes::DataType>
```

Source: `src/logical_plan/consumer/utils.rs:79`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

Traverse through the data type (incl. lists/maps/etc), renaming all inner struct fields.
