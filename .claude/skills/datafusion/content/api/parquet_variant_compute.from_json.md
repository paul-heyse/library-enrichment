# `parquet_variant_compute::from_json`

Crate `parquet-variant-compute` · 1 public items · structured records in [`model/parquet_variant_compute.from_json.json`](../model/parquet_variant_compute.from_json.json)

## json_to_variant

`function` · `parquet_variant_compute::from_json::json_to_variant`

Also reachable as `parquet::variant::json_to_variant`, `parquet_variant_compute::json_to_variant`

```rust
fn json_to_variant(input: &arrow::array::ArrayRef) -> Result<VariantArray, arrow_schema::ArrowError>
```

Parse a batch of JSON strings into a batch of Variants represented as
STRUCT<metadata: BINARY, value: BINARY> where nulls are preserved. The JSON strings in the input
must be valid.

Supports the following string array types:
- [`StringArray`]
- [`LargeStringArray`]
- [`StringViewArray`]

---
