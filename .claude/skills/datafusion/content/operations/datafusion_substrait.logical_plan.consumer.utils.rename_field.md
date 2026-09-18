# `datafusion_substrait::logical_plan::consumer::utils::rename_field`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.consumer.utils.rename_field.json).

<a id="op-10def79294885e7bbf45fe36"></a>
## rename_field

`function` · `datafusion_substrait::logical_plan::consumer::utils::rename_field` · datafusion-substrait 55.1.0

```rust
fn rename_field(field: &datafusion::arrow::datatypes::Field, dfs_names: &Vec<String>, unnamed_field_suffix: usize, name_idx: &mut usize) -> datafusion::common::Result<datafusion::arrow::datatypes::Field>
```

Source: `src/logical_plan/consumer/utils.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

Traverse through the field, renaming the provided field itself and all its inner struct fields.
