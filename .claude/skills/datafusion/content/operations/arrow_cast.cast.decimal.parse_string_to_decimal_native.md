# `arrow_cast::cast::decimal::parse_string_to_decimal_native`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_cast.cast.decimal.parse_string_to_decimal_native.json).

<a id="op-20079590ad7b8391e9cf4814"></a>
## parse_string_to_decimal_native

`function` · `arrow_cast::cast::decimal::parse_string_to_decimal_native` · arrow-cast 59.3.0

```rust
fn parse_string_to_decimal_native<T: DecimalType>(value_str: &str, scale: usize) -> Result<T::Native, ArrowError> where T::Native: DecimalCast + ArrowNativeTypeOp
```

Source: `src/cast/decimal.rs:534`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Parses given string to specified decimal native (i128/i256) based on given
scale. Returns an `Err` if it cannot parse given string.
