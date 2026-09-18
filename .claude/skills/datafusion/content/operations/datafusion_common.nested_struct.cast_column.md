# `datafusion_common::nested_struct::cast_column`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.nested_struct.cast_column.json).

<a id="op-e3a60b7a33b761444e0681be"></a>
## cast_column

`function` · `datafusion_common::nested_struct::cast_column` · datafusion-common 55.1.0

```rust
fn cast_column(source_col: &arrow::array::ArrayRef, target_type: &arrow::datatypes::DataType, cast_options: &arrow::compute::CastOptions<'_>) -> error::Result<arrow::array::ArrayRef>
```

Source: `src/nested_struct.rs:238`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

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
