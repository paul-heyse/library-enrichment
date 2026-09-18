# `arrow_data::decimal::MAX_DECIMAL128_FOR_EACH_PRECISION`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_data.decimal.MAX_DECIMAL128_FOR_EACH_PRECISION.json).

<a id="op-827ff8980de5127ba0248d08"></a>
## MAX_DECIMAL128_FOR_EACH_PRECISION

`constant` · `arrow_data::decimal::MAX_DECIMAL128_FOR_EACH_PRECISION` · arrow-data 59.3.0

```rust
const MAX_DECIMAL128_FOR_EACH_PRECISION: [i128; 39] = _
```

Source: `src/decimal.rs:706`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

`MAX_DECIMAL128_FOR_EACH_PRECISION[p]` holds the maximum `i128` value that can
be stored in [`Decimal128`] value of precision `p`.

# Notes

The first element is unused and is inserted so that we can look up using
precision as the index without the need to subtract 1 first.

# Example
```
# use arrow_data::decimal::MAX_DECIMAL128_FOR_EACH_PRECISION;
assert_eq!(MAX_DECIMAL128_FOR_EACH_PRECISION[3], 999);
```

[`Decimal128`]: arrow_schema::DataType::Decimal128
