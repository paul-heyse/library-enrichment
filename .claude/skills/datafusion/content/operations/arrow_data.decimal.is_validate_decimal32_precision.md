# `arrow_data::decimal::is_validate_decimal32_precision`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_data.decimal.is_validate_decimal32_precision.json).

<a id="op-dc4e8e8a722c5ab07a442901"></a>
## is_validate_decimal32_precision

`function` · `arrow_data::decimal::is_validate_decimal32_precision` · arrow-data 59.3.0

```rust
fn is_validate_decimal32_precision(value: i32, precision: u8) -> bool
```

Source: `src/decimal.rs:968`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Returns true if the specified `i32` value can be properly
interpreted as a [`Decimal32`] number with precision `precision`

[`Decimal32`]: arrow_schema::DataType::Decimal32
