# `arrow_data::decimal::is_validate_decimal256_precision`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_data.decimal.is_validate_decimal256_precision.json).

<a id="op-3c362edbec71190a7730f010"></a>
## is_validate_decimal256_precision

`function` · `arrow_data::decimal::is_validate_decimal256_precision` · arrow-data 59.3.0

```rust
fn is_validate_decimal256_precision(value: arrow_buffer::i256, precision: u8) -> bool
```

Source: `src/decimal.rs:1130`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Return true if the specified `i256` value can be properly
interpreted as a [`Decimal256`] number with precision `precision`

[`Decimal256`]: arrow_schema::DataType::Decimal256
