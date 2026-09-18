# `parquet_variant_compute::cast_to_variant::cast_to_variant_with_options`

Full upstream contracts; raw type trees and source locators in [structured records](parquet_variant_compute.cast_to_variant.cast_to_variant_with_options.json).

<a id="op-5aa012f3cbaac6fa425da787"></a>
## cast_to_variant_with_options

`function` · `parquet_variant_compute::cast_to_variant::cast_to_variant_with_options` · parquet-variant-compute 59.3.0

```rust
fn cast_to_variant_with_options(input: &dyn Array, options: &arrow::compute::CastOptions<'_>) -> Result<VariantArray, arrow_schema::ArrowError>
```

Source: `src/cast_to_variant.rs:57`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

Casts a typed arrow [`Array`](../operations/arrow_array.array.Array.md#op-cabb9fba5e3e968cee823d21) to a [`VariantArray`](../operations/parquet_variant_compute.variant_array.VariantArray.md#op-0730efce8ca33e866d425d18). This is useful when you
need to convert a specific data type

# Arguments
* `input` - A reference to the input [`Array`](../operations/arrow_array.array.Array.md#op-cabb9fba5e3e968cee823d21) to cast

# Notes
If the input array element is null, the corresponding element in the
output `VariantArray` will also be null (not `Variant::Null`).

# Example
```
# use arrow::array::{Array, ArrayRef, Int64Array};
# use parquet_variant::Variant;
# use parquet_variant_compute::cast_to_variant;
// input is an Int64Array, which will be cast to a VariantArray
let input = Int64Array::from(vec![Some(1), None, Some(3)]);
let result = cast_to_variant(&input).unwrap();
assert_eq!(result.len(), 3);
assert_eq!(result.value(0), Variant::Int64(1));
assert!(result.is_null(1)); // note null, not Variant::Null
assert_eq!(result.value(2), Variant::Int64(3));
```

For `DataType::Timestamp`s: if the timestamp has any level of precision
greater than a microsecond, it will be truncated. For example
`1970-01-01T00:00:01.234567890Z`
will be truncated to
`1970-01-01T00:00:01.234567Z`

# Arguments
* `input` - The array to convert to VariantArray
* `options` - Options controlling conversion behavior
