# `parquet_variant_compute::unshred_variant`

Crate `parquet-variant-compute` · 1 public items · structured records in [`model/parquet_variant_compute.unshred_variant.json`](../model/parquet_variant_compute.unshred_variant.json)

## unshred_variant

`function` · `parquet_variant_compute::unshred_variant::unshred_variant`

Also reachable as `parquet::variant::unshred_variant`, `parquet_variant_compute::unshred_variant`

```rust
fn unshred_variant(array: &VariantArray) -> arrow::error::Result<VariantArray>
```

[Full member, field, variant and typed contracts](../operations/parquet_variant_compute.unshred_variant.unshred_variant.md).


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

---
