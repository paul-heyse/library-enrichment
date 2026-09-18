# `arrow_data::decimal::validate_decimal32_precision`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_data.decimal.validate_decimal32_precision.json).

<a id="op-481370756807a678c6bdbc1b"></a>
## validate_decimal32_precision

`function` · `arrow_data::decimal::validate_decimal32_precision` · arrow-data 59.3.0

```rust
fn validate_decimal32_precision(value: i32, precision: u8, scale: i8) -> Result<(), arrow_schema::ArrowError>
```

Source: `src/decimal.rs:924`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Validates that the specified `i32` value can be properly
interpreted as a [`Decimal32`] number with precision `precision`

[`Decimal32`]: arrow_schema::DataType::Decimal32
