# `arrow_data::decimal::MIN_DECIMAL32_FOR_EACH_PRECISION`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_data.decimal.MIN_DECIMAL32_FOR_EACH_PRECISION.json).

<a id="op-b87397301f5ea9a50d7cee17"></a>
## MIN_DECIMAL32_FOR_EACH_PRECISION

`constant` · `arrow_data::decimal::MIN_DECIMAL32_FOR_EACH_PRECISION` · arrow-data 59.3.0

```rust
const MIN_DECIMAL32_FOR_EACH_PRECISION: [i32; 10] = _
```

Source: `src/decimal.rs:914`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

`MIN_DECIMAL32_FOR_EACH_PRECISION[p]` holds the minimum `ialue that can
be stored in a [`Decimal32`] value of precision `p`.

# Notes

The first element is unused and is inserted so that we can look up using
precision as the index without the need to subtract 1 first.

# Example
```
# use arrow_data::decimal::MIN_DECIMAL32_FOR_EACH_PRECISION;
assert_eq!(MIN_DECIMAL32_FOR_EACH_PRECISION[3], -999);
```

[`Decimal32`]: arrow_schema::DataType::Decimal32
