# `parquet_variant_compute::variant_get::variant_get`

Full upstream contracts; raw type trees and source locators in [structured records](parquet_variant_compute.variant_get.variant_get.json).

<a id="op-cde4ae0b2b5f47169d5edbe6"></a>
## variant_get

`function` · `parquet_variant_compute::variant_get::variant_get` · parquet-variant-compute 59.3.0

```rust
fn variant_get(input: &arrow::array::ArrayRef, options: GetOptions<'_>) -> arrow::error::Result<arrow::array::ArrayRef>
```

Source: `src/variant_get.rs:431`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

Returns an array with the specified path extracted from the variant values.

The return array type depends on the `as_type` field of the options parameter
1. `as_type: None`: a VariantArray is returned. The values in this new VariantArray will point
   to the specified path.
2. `as_type: Some(<specific field>)`: an array of the specified type is returned.

TODO: How would a caller request a struct or list type where the fields/elements can be any
variant? Caller can pass None as the requested type to fetch a specific path, but it would
quickly become annoying (and inefficient) to call `variant_get` for each leaf value in a struct or
list and then try to assemble the results.
