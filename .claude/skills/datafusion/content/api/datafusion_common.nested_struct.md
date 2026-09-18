# `datafusion_common::nested_struct`

Crate `datafusion-common` · 6 public items · structured records in [`model/datafusion_common.nested_struct.json`](../model/datafusion_common.nested_struct.json)

## adapt_batch_to_schema

`function` · `datafusion_common::nested_struct::adapt_batch_to_schema`

```rust
fn adapt_batch_to_schema(batch: arrow::array::RecordBatch, target_schema: &arrow::datatypes::SchemaRef) -> error::Result<arrow::array::RecordBatch>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.nested_struct.adapt_batch_to_schema.md).


Adapts a `RecordBatch` to conform to `target_schema`, verifying that each target field
type contains the incoming column data type (as verified by [`arrow::datatypes::DataType::contains`])
and transforms the metadata/types of differing columns to match `target_schema`
without copying primitive buffer data.

If `batch` has an incompatible column count or incompatible column data types,
an error is returned.

---

## cast_column

`function` · `datafusion_common::nested_struct::cast_column`

Also reachable as `datafusion::common::cast_column`, `datafusion_common::cast_column`

```rust
fn cast_column(source_col: &arrow::array::ArrayRef, target_type: &arrow::datatypes::DataType, cast_options: &arrow::compute::CastOptions<'_>) -> error::Result<arrow::array::ArrayRef>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.nested_struct.cast_column.md).


Cast a column to match the target field type, with special handling for nested structs.

This function serves as the main entry point for column casting operations. For struct
types, it enforces that **only struct columns can be cast to struct types**.

## Casting Behavior
- **Struct Types**: Delegates to `cast_struct_column` for struct-to-struct casting only
- **Non-Struct Types**: Uses Arrow's standard `cast` function for primitive type conversions

## Cast Options
The `cast_options` argument controls how Arrow handles values that cannot be represented
in the target type. When `safe` is `false` (DataFusion's default) the cast will return an
error if such a value is encountered. Setting `safe` to `true` instead produces `NULL`
for out-of-range or otherwise invalid values. The options also allow customizing how
temporal values are formatted when cast to strings.

```
use arrow::array::{ArrayRef, Int64Array};
use arrow::compute::CastOptions;
use arrow::datatypes::DataType;
use datafusion_common::nested_struct::cast_column;
use std::sync::Arc;

let source: ArrayRef = Arc::new(Int64Array::from(vec![1, i64::MAX]));
// Permit lossy conversions by producing NULL on overflow instead of erroring
let options = CastOptions {
    safe: true,
    ..Default::default()
};
let result = cast_column(&source, &DataType::Int32, &options).unwrap();
assert!(result.is_null(1));
```

## Struct Casting Requirements
The struct casting logic requires that the source column must already be a struct type.
This makes the function useful for:
- Schema evolution scenarios where struct layouts change over time
- Data migration between different struct schemas
- Type-safe data processing pipelines that maintain struct type integrity

# Arguments
* `source_col` - The source array to cast
* `target_type` - The target data type to cast to
* `cast_options` - Options that govern strictness and formatting of the cast

# Returns
A `Result<ArrayRef>` containing the cast array

# Errors
Returns an error if:
- Attempting to cast a non-struct column to a struct type
- Arrow's cast function fails for non-struct types
- Memory allocation fails during struct construction
- Invalid data type combinations are encountered

---

## has_one_of_more_common_fields

`function` · `datafusion_common::nested_struct::has_one_of_more_common_fields`

```rust
fn has_one_of_more_common_fields(source_fields: &[arrow::datatypes::FieldRef], target_fields: &[arrow::datatypes::FieldRef]) -> bool
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.nested_struct.has_one_of_more_common_fields.md).


Check if two field lists have at least one common field by name.

This is useful for validating struct compatibility when casting between structs,
ensuring that source and target fields have overlapping names.

---

## requires_nested_struct_cast

`function` · `datafusion_common::nested_struct::requires_nested_struct_cast`

```rust
fn requires_nested_struct_cast(source_type: &arrow::datatypes::DataType, target_type: &arrow::datatypes::DataType) -> bool
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.nested_struct.requires_nested_struct_cast.md).


Returns true if casting from `source_type` to `target_type` requires
name-based nested struct casting logic, rather than Arrow's standard cast.

This is the case when both types are struct types, or both are the same
container type (List, LargeList, equal-width FixedSizeList, ListView,
LargeListView, Dictionary) wrapping types that recursively contain structs.

Use this predicate at both planning time (to decide whether to apply struct
compatibility validation) and execution time (to decide whether to route
through [`cast_column`] instead of Arrow's generic cast).

---

## validate_data_type_compatibility

`function` · `datafusion_common::nested_struct::validate_data_type_compatibility`

```rust
fn validate_data_type_compatibility(field_name: &str, source_type: &arrow::datatypes::DataType, target_type: &arrow::datatypes::DataType) -> error::Result<()>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.nested_struct.validate_data_type_compatibility.md).


Validates that `source_type` can be cast to `target_type`, recursively
handling container types that wrap structs.

---

## validate_struct_compatibility

`function` · `datafusion_common::nested_struct::validate_struct_compatibility`

```rust
fn validate_struct_compatibility(source_fields: &[arrow::datatypes::FieldRef], target_fields: &[arrow::datatypes::FieldRef]) -> error::Result<()>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.nested_struct.validate_struct_compatibility.md).


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

---
