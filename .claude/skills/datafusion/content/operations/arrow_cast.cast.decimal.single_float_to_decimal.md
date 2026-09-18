# `arrow_cast::cast::decimal::single_float_to_decimal`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_cast.cast.decimal.single_float_to_decimal.json).

<a id="op-0a33a9565f7d3c895d065439"></a>
## single_float_to_decimal

`function` · `arrow_cast::cast::decimal::single_float_to_decimal` · arrow-cast 59.3.0

```rust
fn single_float_to_decimal<D>(input: f64, mul: f64) -> Option<D::Native> where D: DecimalType + ArrowPrimitiveType, <D as ArrowPrimitiveType>::Native: DecimalCast
```

Source: `src/cast/decimal.rs:814`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Cast a single floating point value to a decimal native with the given multiple.
Returns `None` if the value cannot be represented with the requested precision.
