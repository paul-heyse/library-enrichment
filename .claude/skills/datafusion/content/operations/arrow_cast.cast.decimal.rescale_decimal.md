# `arrow_cast::cast::decimal::rescale_decimal`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_cast.cast.decimal.rescale_decimal.json).

<a id="op-5c55697ac52bd273aa5725b8"></a>
## rescale_decimal

`function` · `arrow_cast::cast::decimal::rescale_decimal` · arrow-cast 59.3.0

```rust
fn rescale_decimal<I: DecimalType, O: DecimalType>(value: I::Native, input_precision: u8, input_scale: i8, output_precision: u8, output_scale: i8) -> Option<O::Native> where I::Native: DecimalCast + ArrowNativeTypeOp, O::Native: DecimalCast + ArrowNativeTypeOp
```

Source: `src/cast/decimal.rs:301`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Rescales a decimal value from `(input_precision, input_scale)` to
`(output_precision, output_scale)` and returns the converted number when it fits
within the output precision.

The function first validates that the requested precision and scale are supported for
both the source and destination decimal types. It then either upscales (multiplying
by an appropriate power of ten) or downscales (dividing with rounding) the input value.
When the scaling factor exceeds the precision table of the destination type, the value
is treated as an overflow for upscaling, or rounded to zero for downscaling (as any
possible result would be zero at the requested scale).

This mirrors the column-oriented helpers of decimal casting but operates on a single value
(row-level) instead of an entire array.

Returns `None` if the value cannot be represented with the requested precision.
