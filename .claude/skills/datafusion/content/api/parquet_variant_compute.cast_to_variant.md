# `parquet_variant_compute::cast_to_variant`

Crate `parquet-variant-compute` · 2 public items · structured records in [`model/parquet_variant_compute.cast_to_variant.json`](../model/parquet_variant_compute.cast_to_variant.json)

## cast_to_variant

`function` · `parquet_variant_compute::cast_to_variant::cast_to_variant`

Also reachable as `parquet::variant::cast_to_variant`, `parquet_variant_compute::cast_to_variant`

```rust
fn cast_to_variant(input: &dyn Array) -> Result<VariantArray, arrow_schema::ArrowError>
```

Convert an array to a [`VariantArray`] with strict mode enabled (returns errors on conversion
failures).

This function provides backward compatibility. For non-strict behavior,
use [`cast_to_variant_with_options`] with `CastOptions { safe: true, ..Default::default() }`.

---

## cast_to_variant_with_options

`function` · `parquet_variant_compute::cast_to_variant::cast_to_variant_with_options`

Also reachable as `parquet::variant::cast_to_variant_with_options`, `parquet_variant_compute::cast_to_variant_with_options`

```rust
fn cast_to_variant_with_options(input: &dyn Array, options: &arrow::compute::CastOptions<'_>) -> Result<VariantArray, arrow_schema::ArrowError>
```

Casts a typed arrow [`Array`] to a [`VariantArray`]. This is useful when you
need to convert a specific data type

# Arguments
* `input` - A reference to the input [`Array`] to cast

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

---
