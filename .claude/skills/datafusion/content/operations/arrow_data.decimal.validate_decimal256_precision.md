# `arrow_data::decimal::validate_decimal256_precision`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_data.decimal.validate_decimal256_precision.json).

<a id="op-baefdb5d3925d6a7ec7d0d43"></a>
## validate_decimal256_precision

`function` · `arrow_data::decimal::validate_decimal256_precision` · arrow-data 59.3.0

```rust
fn validate_decimal256_precision(value: arrow_buffer::i256, precision: u8, scale: i8) -> Result<(), arrow_schema::ArrowError>
```

Source: `src/decimal.rs:1085`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Validates that the specified `i256` of value can be properly
interpreted as a [`Decimal256`] number with precision `precision`

[`Decimal256`]: arrow_schema::DataType::Decimal256
