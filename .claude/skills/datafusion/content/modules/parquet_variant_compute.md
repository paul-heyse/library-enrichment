# `parquet_variant_compute`

Full upstream contracts; raw type trees and source locators in [structured records](parquet_variant_compute.json).

<a id="op-72154b4e4751c8223991ae4d"></a>
## parquet_variant_compute

`module` · `parquet_variant_compute` · parquet-variant-compute 59.3.0

```rust
mod parquet_variant_compute
```

Source: `src/lib.rs:18`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

[`VariantArray`](../operations/parquet_variant_compute.variant_array.VariantArray.md#op-0730efce8ca33e866d425d18) and compute kernels for the [Variant Binary Encoding] from [Apache Parquet].

## Main APIs
- [`VariantArray`](../operations/parquet_variant_compute.variant_array.VariantArray.md#op-0730efce8ca33e866d425d18) : Represents an array of `Variant` values.
- [`VariantArrayBuilder`](../operations/parquet_variant_compute.variant_array_builder.VariantArrayBuilder.md#op-76813e1b9441f7d6a4a08d37): For building [`VariantArray`](../operations/parquet_variant_compute.variant_array.VariantArray.md#op-0730efce8ca33e866d425d18)

# Compute Kernels
- [`json_to_variant()`](../operations/parquet_variant_compute.from_json.json_to_variant.md#op-11703d156275cc10c9cb614e): Function to convert Arrays of JSON strings to a `VariantArray`.
- [`variant_to_json()`](../operations/parquet_variant_compute.to_json.variant_to_json.md#op-1d04d07f3578f3d8b1f79b33): Function to convert a `VariantArray` to arrays of JSON strings.
- [`cast_to_variant()`](../operations/parquet_variant_compute.cast_to_variant.cast_to_variant.md#op-c53402341f04a05bbb37a852): Cast Arrow arrays to `VariantArray`.
- [`variant_get()`](../operations/parquet_variant_compute.variant_get.variant_get.md#op-cde4ae0b2b5f47169d5edbe6): Convert `VariantArray` (or an inner path) to a strongly-typed Arrow array.
- [`shred_variant()`](../operations/parquet_variant_compute.shred_variant.shred_variant.md#op-8b1e4649ad8625a60395c4f7): Shred a `VariantArray` according to the provided shredding schema
- [`unshred_variant()`](../operations/parquet_variant_compute.unshred_variant.unshred_variant.md#op-9b16a68379d75ed39823f170): Unshred a `VariantArray` to pure binary variant.

## 🚧 Work In Progress

This crate is under active development and is not yet ready for production use.
If you are interested in helping, you can find more information on the GitHub [Variant issue]

[Variant Binary Encoding]: https://github.com/apache/parquet-format/blob/master/VariantEncoding.md
[Apache Parquet]: https://parquet.apache.org/
[`VariantPath`]: parquet_variant::VariantPath
[Variant issue]: https://github.com/apache/arrow-rs/issues/6736
