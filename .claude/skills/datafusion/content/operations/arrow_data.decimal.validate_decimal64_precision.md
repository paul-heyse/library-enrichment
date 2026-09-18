# `arrow_data::decimal::validate_decimal64_precision`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_data.decimal.validate_decimal64_precision.json).

<a id="op-6b6e2753c6e1f3c9ef49d25e"></a>
## validate_decimal64_precision

`function` · `arrow_data::decimal::validate_decimal64_precision` · arrow-data 59.3.0

```rust
fn validate_decimal64_precision(value: i64, precision: u8, scale: i8) -> Result<(), arrow_schema::ArrowError>
```

Source: `src/decimal.rs:979`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Validates that the specified `i64` value can be properly
interpreted as a [`Decimal64`] number with precision `precision`

[`Decimal64`]: arrow_schema::DataType::Decimal64
