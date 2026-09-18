# `datafusion_physical_plan::aggregates::topk_types_supported`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.aggregates.topk_types_supported.json).

<a id="op-3ae84ceefb0b7a0e723321e3"></a>
## topk_types_supported

`function` · `datafusion_physical_plan::aggregates::topk_types_supported` · datafusion-physical-plan 55.1.0

```rust
fn topk_types_supported(key_type: &arrow::datatypes::DataType, value_type: &arrow::datatypes::DataType) -> bool
```

Source: `src/aggregates/mod.rs:228`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns true if TopK aggregation data structures support the provided key and value types.

This function checks whether both the key type (used for grouping) and value type
(used in min/max aggregation) can be handled by the TopK aggregation heap and hash table.
Supported types include Arrow primitives (integers, floats, decimals, intervals) and
UTF-8 strings (`Utf8`, `LargeUtf8`, `Utf8View`).
```text
