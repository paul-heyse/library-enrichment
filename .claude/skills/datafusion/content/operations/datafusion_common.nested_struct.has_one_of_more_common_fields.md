# `datafusion_common::nested_struct::has_one_of_more_common_fields`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.nested_struct.has_one_of_more_common_fields.json).

<a id="op-6f771a3eaeddd0e082e5233f"></a>
## has_one_of_more_common_fields

`function` · `datafusion_common::nested_struct::has_one_of_more_common_fields` · datafusion-common 55.1.0

```rust
fn has_one_of_more_common_fields(source_fields: &[arrow::datatypes::FieldRef], target_fields: &[arrow::datatypes::FieldRef]) -> bool
```

Source: `src/nested_struct.rs:704`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Check if two field lists have at least one common field by name.

This is useful for validating struct compatibility when casting between structs,
ensuring that source and target fields have overlapping names.
