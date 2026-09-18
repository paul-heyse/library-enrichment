# `parquet_variant_compute::to_json::variant_to_json`

Full upstream contracts; raw type trees and source locators in [structured records](parquet_variant_compute.to_json.variant_to_json.json).

<a id="op-1d04d07f3578f3d8b1f79b33"></a>
## variant_to_json

`function` · `parquet_variant_compute::to_json::variant_to_json` · parquet-variant-compute 59.3.0

```rust
fn variant_to_json(input: &arrow::array::ArrayRef) -> Result<arrow::array::StringArray, arrow_schema::ArrowError>
```

Source: `src/to_json.rs:30`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

Transform a batch of Variant represented as STRUCT<metadata: BINARY, value: BINARY> to a batch
of JSON strings where nulls are preserved. The JSON strings in the input must be valid.
