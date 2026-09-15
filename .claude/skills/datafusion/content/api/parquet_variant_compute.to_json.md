# `parquet_variant_compute::to_json`

Crate `parquet-variant-compute` · 1 public items · structured records in [`model/parquet_variant_compute.to_json.json`](../model/parquet_variant_compute.to_json.json)

## variant_to_json

`function` · `parquet_variant_compute::to_json::variant_to_json`

Also reachable as `parquet::variant::variant_to_json`, `parquet_variant_compute::variant_to_json`

```rust
fn variant_to_json(input: &arrow::array::ArrayRef) -> Result<arrow::array::StringArray, arrow_schema::ArrowError>
```

Transform a batch of Variant represented as STRUCT<metadata: BINARY, value: BINARY> to a batch
of JSON strings where nulls are preserved. The JSON strings in the input must be valid.

---
