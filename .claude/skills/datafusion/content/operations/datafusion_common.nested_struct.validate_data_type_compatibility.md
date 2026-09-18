# `datafusion_common::nested_struct::validate_data_type_compatibility`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.nested_struct.validate_data_type_compatibility.json).

<a id="op-e77d831bc012ad1b48d2ea85"></a>
## validate_data_type_compatibility

`function` · `datafusion_common::nested_struct::validate_data_type_compatibility` · datafusion-common 55.1.0

```rust
fn validate_data_type_compatibility(field_name: &str, source_type: &arrow::datatypes::DataType, target_type: &arrow::datatypes::DataType) -> error::Result<()>
```

Source: `src/nested_struct.rs:608`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Validates that `source_type` can be cast to `target_type`, recursively
handling container types that wrap structs.
