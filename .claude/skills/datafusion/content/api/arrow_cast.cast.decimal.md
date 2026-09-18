# `arrow_cast::cast::decimal`

Crate `arrow-cast` · 4 public items · structured records in [`model/arrow_cast.cast.decimal.json`](../model/arrow_cast.cast.decimal.json)

## parse_string_to_decimal_native

`function` · `arrow_cast::cast::decimal::parse_string_to_decimal_native`

Also reachable as `arrow::compute::parse_string_to_decimal_native`, `arrow_cast::cast::parse_string_to_decimal_native`, `arrow_cast::parse_string_to_decimal_native`

```rust
fn parse_string_to_decimal_native<T: DecimalType>(value_str: &str, scale: usize) -> Result<T::Native, ArrowError> where T::Native: DecimalCast + ArrowNativeTypeOp
```

[Full member, field, variant and typed contracts](../operations/arrow_cast.cast.decimal.parse_string_to_decimal_native.md).


Parses given string to specified decimal native (i128/i256) based on given
scale. Returns an `Err` if it cannot parse given string.

---

## rescale_decimal

`function` · `arrow_cast::cast::decimal::rescale_decimal`

Also reachable as `arrow::compute::rescale_decimal`, `arrow_cast::cast::rescale_decimal`, `arrow_cast::rescale_decimal`

```rust
fn rescale_decimal<I: DecimalType, O: DecimalType>(value: I::Native, input_precision: u8, input_scale: i8, output_precision: u8, output_scale: i8) -> Option<O::Native> where I::Native: DecimalCast + ArrowNativeTypeOp, O::Native: DecimalCast + ArrowNativeTypeOp
```

[Full member, field, variant and typed contracts](../operations/arrow_cast.cast.decimal.rescale_decimal.md).


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

---

## single_float_to_decimal

`function` · `arrow_cast::cast::decimal::single_float_to_decimal`

Also reachable as `arrow::compute::single_float_to_decimal`, `arrow_cast::cast::single_float_to_decimal`, `arrow_cast::single_float_to_decimal`

```rust
fn single_float_to_decimal<D>(input: f64, mul: f64) -> Option<D::Native> where D: DecimalType + ArrowPrimitiveType, <D as ArrowPrimitiveType>::Native: DecimalCast
```

[Full member, field, variant and typed contracts](../operations/arrow_cast.cast.decimal.single_float_to_decimal.md).


Cast a single floating point value to a decimal native with the given multiple.
Returns `None` if the value cannot be represented with the requested precision.

---

## DecimalCast

`trait` · `arrow_cast::cast::decimal::DecimalCast`

Also reachable as `arrow::compute::DecimalCast`, `arrow_cast::DecimalCast`, `arrow_cast::cast::DecimalCast`

```rust
trait DecimalCast: Sized
```

**Implementors** (1)

- `arrow_buffer::bigint::i256`

**Methods** (6)

```rust
fn from_decimal<T: DecimalCast>(n: T) -> Option<Self>
fn from_f64(n: f64) -> Option<Self>
fn to_i128(self) -> Option<i128>
fn to_i256(self) -> Option<i256>
fn to_i32(self) -> Option<i32>
fn to_i64(self) -> Option<i64>
```

[Full member, field, variant and typed contracts](../operations/arrow_cast.cast.decimal.DecimalCast.md).


A utility trait that provides checked conversions between
decimal types inspired by [`NumCast`]

---
