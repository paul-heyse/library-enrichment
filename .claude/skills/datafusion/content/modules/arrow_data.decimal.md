# `arrow_data::decimal`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_data.decimal.json).

<a id="op-efdbbac9fed417e756f6a645"></a>
## decimal

`module` · `arrow_data::decimal` · arrow-data 59.3.0

```rust
mod decimal
```

Source: `src/decimal.rs:18`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Maximum and minimum values for [`Decimal256`], [`Decimal128`], [`Decimal64`] and [`Decimal32`].

Also provides functions to validate if a given decimal value is within
the valid range of the decimal type.

[`Decimal32`]: arrow_schema::DataType::Decimal32
[`Decimal64`]: arrow_schema::DataType::Decimal64
[`Decimal128`]: arrow_schema::DataType::Decimal128
[`Decimal256`]: arrow_schema::DataType::Decimal256
