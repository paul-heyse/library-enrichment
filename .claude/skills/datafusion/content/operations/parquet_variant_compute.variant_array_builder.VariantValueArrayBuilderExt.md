# `parquet_variant_compute::variant_array_builder::VariantValueArrayBuilderExt`

Full upstream contracts; raw type trees and source locators in [structured records](parquet_variant_compute.variant_array_builder.VariantValueArrayBuilderExt.json).

<a id="op-6960419281b164f2296e04b3"></a>
## VariantValueArrayBuilderExt

`struct` · `parquet_variant_compute::variant_array_builder::VariantValueArrayBuilderExt` · parquet-variant-compute 59.3.0

```rust
struct VariantValueArrayBuilderExt<'a>
```

Source: `src/variant_array_builder.rs:416`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

A thin [`VariantBuilderExt`] wrapper that hides the short-lived (per-row)
[`ReadOnlyMetadataBuilder`] instances that [`VariantValueArrayBuilder`] requires.
