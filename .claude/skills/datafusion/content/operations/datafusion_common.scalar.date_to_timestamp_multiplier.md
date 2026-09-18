# `datafusion_common::scalar::date_to_timestamp_multiplier`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.scalar.date_to_timestamp_multiplier.json).

<a id="op-127318e140d4c4cba862425f"></a>
## date_to_timestamp_multiplier

`function` · `datafusion_common::scalar::date_to_timestamp_multiplier` · datafusion-common 55.1.0

```rust
fn date_to_timestamp_multiplier(source_type: &arrow::datatypes::DataType, target_type: &arrow::datatypes::DataType) -> Option<i64>
```

Source: `src/scalar/mod.rs:115`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns the multiplier that converts the input date representation into the
desired timestamp unit, if the conversion requires a multiplication that can
overflow an `i64`.
