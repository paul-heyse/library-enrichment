# `datafusion_common::scalar::ensure_timestamp_in_bounds`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.scalar.ensure_timestamp_in_bounds.json).

<a id="op-537bd6d93827fa644353983d"></a>
## ensure_timestamp_in_bounds

`function` · `datafusion_common::scalar::ensure_timestamp_in_bounds` · datafusion-common 55.1.0

```rust
fn ensure_timestamp_in_bounds(value: i64, multiplier: i64, source_type: &arrow::datatypes::DataType, target_type: &arrow::datatypes::DataType) -> error::Result<()>
```

Source: `src/scalar/mod.rs:186`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Ensures the provided value can be represented as a timestamp with the given
multiplier. Returns an [`DataFusionError::Execution`](../operations/datafusion_common.error.DataFusionError.md#op-03029cb87c98330b2e3da49c) when the converted
value would overflow the timestamp range.
