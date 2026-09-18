# `arrow_data::decimal::MIN_DECIMAL256_FOR_EACH_PRECISION`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_data.decimal.MIN_DECIMAL256_FOR_EACH_PRECISION.json).

<a id="op-801b7e3a375accd0b838d348"></a>
## MIN_DECIMAL256_FOR_EACH_PRECISION

`constant` · `arrow_data::decimal::MIN_DECIMAL256_FOR_EACH_PRECISION` · arrow-data 59.3.0

```rust
const MIN_DECIMAL256_FOR_EACH_PRECISION: [arrow_buffer::i256; 77] = _
```

Source: `src/decimal.rs:383`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

`MIN_DECIMAL256_FOR_EACH_PRECISION[p]` holds the minimum [`i256`] value that can
be stored in a [`Decimal256`] value of precision `p`.

# Notes

Each element is the min value of signed 256-bit integer for the specified precision which
is encoded to the 76-byte width format of little-endian.

The first element is unused and is inserted so that we can look up using
precision as the index without the need to subtract 1 first.
# Example
```
# use arrow_buffer::i256;
# use arrow_data::decimal::MIN_DECIMAL256_FOR_EACH_PRECISION;
assert_eq!(MIN_DECIMAL256_FOR_EACH_PRECISION[3], i256::from(-999));
```

[`i256`]: arrow_buffer::i256
[`Decimal256`]: arrow_schema::DataType::Decimal256
