# `parquet_variant_compute::from_json::json_to_variant`

Full upstream contracts; raw type trees and source locators in [structured records](parquet_variant_compute.from_json.json_to_variant.json).

<a id="op-11703d156275cc10c9cb614e"></a>
## json_to_variant

`function` · `parquet_variant_compute::from_json::json_to_variant` · parquet-variant-compute 59.3.0

```rust
fn json_to_variant(input: &arrow::array::ArrayRef) -> Result<VariantArray, arrow_schema::ArrowError>
```

Source: `src/from_json.rs:47`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

Parse a batch of JSON strings into a batch of Variants represented as
STRUCT<metadata: BINARY, value: BINARY> where nulls are preserved. The JSON strings in the input
must be valid.

Supports the following string array types:
- [`StringArray`](../operations/arrow_array.array.string_array.StringArray.md#op-5d32f770159d415652a55945)
- [`LargeStringArray`](../operations/arrow_array.array.string_array.LargeStringArray.md#op-829f12b9a45fbfe752a7ee52)
- [`StringViewArray`](../operations/arrow_array.array.byte_view_array.StringViewArray.md#op-f468d0a8f2aaecdb58e1f8c0)
