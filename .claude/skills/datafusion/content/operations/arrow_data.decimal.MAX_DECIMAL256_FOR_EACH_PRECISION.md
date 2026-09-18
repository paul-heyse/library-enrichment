# `arrow_data::decimal::MAX_DECIMAL256_FOR_EACH_PRECISION`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_data.decimal.MAX_DECIMAL256_FOR_EACH_PRECISION.json).

<a id="op-e06cab9df76bfc708a59b42b"></a>
## MAX_DECIMAL256_FOR_EACH_PRECISION

`constant` · `arrow_data::decimal::MAX_DECIMAL256_FOR_EACH_PRECISION` · arrow-data 59.3.0

```rust
const MAX_DECIMAL256_FOR_EACH_PRECISION: [arrow_buffer::i256; 77] = _
```

Source: `src/decimal.rs:56`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

`MAX_DECIMAL256_FOR_EACH_PRECISION[p]` holds the maximum [`i256`] value that can
be stored in a [`Decimal256`] value of precision `p`.

# Notes

Each element is the max value of signed 256-bit integer for the specified
precision which is encoded to the 32-byte width format of little-endian.

The first element is unused and is inserted so that we can look up using
precision as the index without the need to subtract 1 first.

# Example
```
# use arrow_buffer::i256;
# use arrow_data::decimal::MAX_DECIMAL256_FOR_EACH_PRECISION;
assert_eq!(MAX_DECIMAL256_FOR_EACH_PRECISION[3], i256::from(999));
```

[`Decimal256`]: arrow_schema::DataType::Decimal256
[`i256`]: arrow_buffer::i256
