# `arrow_data::decimal::is_validate_decimal64_precision`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_data.decimal.is_validate_decimal64_precision.json).

<a id="op-9057e0ff1e5a505898daec7f"></a>
## is_validate_decimal64_precision

`function` · `arrow_data::decimal::is_validate_decimal64_precision` · arrow-data 59.3.0

```rust
fn is_validate_decimal64_precision(value: i64, precision: u8) -> bool
```

Source: `src/decimal.rs:1023`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Returns true if the specified `i64` value can be properly
interpreted as a [`Decimal64`] number with precision `precision`

[`Decimal64`]: arrow_schema::DataType::Decimal64
