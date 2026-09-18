# `parquet_variant_compute::unshred_variant::unshred_variant`

Full upstream contracts; raw type trees and source locators in [structured records](parquet_variant_compute.unshred_variant.unshred_variant.json).

<a id="op-9b16a68379d75ed39823f170"></a>
## unshred_variant

`function` · `parquet_variant_compute::unshred_variant::unshred_variant` · parquet-variant-compute 59.3.0

```rust
fn unshred_variant(array: &VariantArray) -> arrow::error::Result<VariantArray>
```

Source: `src/unshred_variant.rs:60`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

Removes all (nested) typed_value columns from a VariantArray by converting them back to binary
variant and merging the resulting values back into the value column.

This function efficiently converts a shredded VariantArray back to an unshredded form where all
data resides in the value column.

# Arguments
* `array` - The VariantArray to unshred

# Returns
A new VariantArray with all data in the value column and no typed_value column

# Errors
- If the shredded data contains spec violations (e.g., field name conflicts)
- If unsupported data types are encountered in typed_value columns
