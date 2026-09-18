# `datafusion_common::nested_struct::validate_struct_compatibility`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.nested_struct.validate_struct_compatibility.json).

<a id="op-239266224e98a061c815b96d"></a>
## validate_struct_compatibility

`function` · `datafusion_common::nested_struct::validate_struct_compatibility` · datafusion-common 55.1.0

```rust
fn validate_struct_compatibility(source_fields: &[arrow::datatypes::FieldRef], target_fields: &[arrow::datatypes::FieldRef]) -> error::Result<()>
```

Source: `src/nested_struct.rs:492`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Validates compatibility between source and target struct fields for casting operations.

This function implements comprehensive struct compatibility checking by examining:
- Field name matching between source and target structs
- Type castability for each matching field (including recursive struct validation)
- Proper handling of missing fields (target fields not in source are allowed - filled with nulls)
- Proper handling of extra fields (source fields not in target are allowed - ignored)

# Compatibility Rules
- **Field Matching**: Fields are matched by name (case-sensitive)
- **Missing Target Fields**: Allowed - will be filled with null values during casting
- **Extra Source Fields**: Allowed - will be ignored during casting
- **Type Compatibility**: Each matching field must be castable using Arrow's type system
- **Nested Structs**: Recursively validates nested struct compatibility

# Arguments
* `source_fields` - Fields from the source struct type
* `target_fields` - Fields from the target struct type

# Returns
* `Ok(())` if the structs are compatible for casting
* `Err(DataFusionError)` with detailed error message if incompatible

# Examples
```text
// Compatible: source has extra field, target has missing field
// Source: {a: i32, b: string, c: f64}
// Target: {a: i64, d: bool}
// Result: Ok(()) - 'a' can cast i32->i64, 'b','c' ignored, 'd' filled with nulls

// Incompatible: matching field has incompatible types
// Source: {a: string}
// Target: {a: binary}
// Result: Err(...) - string cannot cast to binary
```

