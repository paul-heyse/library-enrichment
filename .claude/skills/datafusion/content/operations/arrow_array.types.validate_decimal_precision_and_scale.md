# `arrow_array::types::validate_decimal_precision_and_scale`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.types.validate_decimal_precision_and_scale.json).

<a id="op-19e591802948571420363d19"></a>
## validate_decimal_precision_and_scale

`function` · `arrow_array::types::validate_decimal_precision_and_scale` · arrow-array 59.3.0

```rust
fn validate_decimal_precision_and_scale<T: DecimalType>(precision: u8, scale: i8) -> Result<(), arrow_schema::ArrowError>
```

Source: `src/types.rs:1412`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Validate that `precision` and `scale` are valid for `T`

Returns an Error if:
- `precision` is zero
- `precision` is larger than `T:MAX_PRECISION`
- `scale` is larger than `T::MAX_SCALE`
- `scale` is > `precision`
