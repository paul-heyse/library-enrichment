# `arrow_data::decimal::is_validate_decimal_precision`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_data.decimal.is_validate_decimal_precision.json).

<a id="op-f3444df287ba44a640ff1077"></a>
## is_validate_decimal_precision

`function` · `arrow_data::decimal::is_validate_decimal_precision` · arrow-data 59.3.0

```rust
fn is_validate_decimal_precision(value: i128, precision: u8) -> bool
```

Source: `src/decimal.rs:1074`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Returns true if the specified `i128` value can be properly
interpreted as a [`Decimal128`] number with precision `precision`

[`Decimal128`]: arrow_schema::DataType::Decimal128
