# `arrow_cast::parse::parse_decimal`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_cast.parse.parse_decimal.json).

<a id="op-759e79341f11d494cb8b8025"></a>
## parse_decimal

`function` · `arrow_cast::parse::parse_decimal` · arrow-cast 59.3.0

```rust
fn parse_decimal<T: DecimalType>(s: &str, precision: u8, scale: i8) -> Result<T::Native, arrow_schema::ArrowError>
```

Source: `src/parse.rs:910`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Parse the string format decimal value to i128/i256 format and checking the precision and scale.
Expected behavior:
- The result value can't be out of bounds.
- When parsing a decimal with scale 0, all fractional digits will be discarded. The final
  fractional digits may be a subset or a superset of the digits after the decimal point when
  e-notation is used.
