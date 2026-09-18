# `arrow_data::decimal::validate_decimal_precision`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_data.decimal.validate_decimal_precision.json).

<a id="op-a3cfaf90faafdba3cb59e48f"></a>
## validate_decimal_precision

`function` · `arrow_data::decimal::validate_decimal_precision` · arrow-data 59.3.0

```rust
fn validate_decimal_precision(value: i128, precision: u8, scale: i8) -> Result<(), arrow_schema::ArrowError>
```

Source: `src/decimal.rs:1034`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Validates that the specified `i128` value can be properly
interpreted as a [`Decimal128`] number with precision `precision`

[`Decimal128`]: arrow_schema::DataType::Decimal128
