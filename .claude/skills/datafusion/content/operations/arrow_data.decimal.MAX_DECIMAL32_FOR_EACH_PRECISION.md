# `arrow_data::decimal::MAX_DECIMAL32_FOR_EACH_PRECISION`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_data.decimal.MAX_DECIMAL32_FOR_EACH_PRECISION.json).

<a id="op-38cd249ef04d88d4174edca6"></a>
## MAX_DECIMAL32_FOR_EACH_PRECISION

`constant` · `arrow_data::decimal::MAX_DECIMAL32_FOR_EACH_PRECISION` · arrow-data 59.3.0

```rust
const MAX_DECIMAL32_FOR_EACH_PRECISION: [i32; 10] = _
```

Source: `src/decimal.rs:894`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

`MAX_DECIMAL32_FOR_EACH_PRECISION[p]` holds the maximum `i32` value that can
be stored in [`Decimal32`] value of precision `p`.

# Notes

The first element is unused and is inserted so that we can look up using
precision as the index without the need to subtract 1 first.

# Example
```
# use arrow_data::decimal::MAX_DECIMAL32_FOR_EACH_PRECISION;
assert_eq!(MAX_DECIMAL32_FOR_EACH_PRECISION[3], 999);
```

[`Decimal32`]: arrow_schema::DataType::Decimal32
