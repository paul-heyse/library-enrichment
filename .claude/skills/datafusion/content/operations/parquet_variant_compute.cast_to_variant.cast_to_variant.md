# `parquet_variant_compute::cast_to_variant::cast_to_variant`

Full upstream contracts; raw type trees and source locators in [structured records](parquet_variant_compute.cast_to_variant.cast_to_variant.json).

<a id="op-c53402341f04a05bbb37a852"></a>
## cast_to_variant

`function` · `parquet_variant_compute::cast_to_variant::cast_to_variant` · parquet-variant-compute 59.3.0

```rust
fn cast_to_variant(input: &dyn Array) -> Result<VariantArray, arrow_schema::ArrowError>
```

Source: `src/cast_to_variant.rs:87`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

Convert an array to a [`VariantArray`](../operations/parquet_variant_compute.variant_array.VariantArray.md#op-0730efce8ca33e866d425d18) with strict mode enabled (returns errors on conversion
failures).

This function provides backward compatibility. For non-strict behavior,
use [`cast_to_variant_with_options`](../operations/parquet_variant_compute.cast_to_variant.cast_to_variant_with_options.md#op-5aa012f3cbaac6fa425da787) with `CastOptions { safe: true, ..Default::default() }`.
