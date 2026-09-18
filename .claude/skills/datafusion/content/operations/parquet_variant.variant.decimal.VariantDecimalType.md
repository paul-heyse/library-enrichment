# `parquet_variant::variant::decimal::VariantDecimalType`

Full upstream contracts; raw type trees and source locators in [structured records](parquet_variant.variant.decimal.VariantDecimalType.json).

<a id="op-be16698bbe38ec54b0a4c3a0"></a>
## VariantDecimalType

`trait` · `parquet_variant::variant::decimal::VariantDecimalType` · parquet-variant 59.3.0

```rust
trait VariantDecimalType: Into<super::Variant<'static, 'static>>
```

Source: `src/variant/decimal.rs:41`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Trait for variant decimal types, enabling generic code across Decimal4/8/16

This trait provides a common interface for the three variant decimal types,
allowing generic functions and data structures to work with any decimal width.
It is modeled after Arrow's `DecimalType` trait but adapted for variant semantics.

# Example

```
# use parquet_variant::{VariantDecimal4, VariantDecimal8, VariantDecimalType};
#
fn extract_scale<D: VariantDecimalType>(decimal: D) -> u8 {
    decimal.scale()
}

let dec4 = VariantDecimal4::try_new(12345, 2).unwrap();
let dec8 = VariantDecimal8::try_new(67890, 3).unwrap();

assert_eq!(extract_scale(dec4), 2);
assert_eq!(extract_scale(dec8), 3);
```

<a id="op-47340f0c5fb6b00abecfb59e"></a>
## MAX_PRECISION

`assoc_const` · `parquet_variant::variant::decimal::VariantDecimalType::MAX_PRECISION` · parquet-variant 59.3.0

```rust
MAX_PRECISION
```

Source: `src/variant/decimal.rs:46`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Maximum number of significant digits this decimal type can represent (9, 18, or 38)

<a id="op-bfcf44409d0154346dd168b3"></a>
## MAX_UNSCALED_VALUE

`assoc_const` · `parquet_variant::variant::decimal::VariantDecimalType::MAX_UNSCALED_VALUE` · parquet-variant 59.3.0

```rust
MAX_UNSCALED_VALUE
```

Source: `src/variant/decimal.rs:48`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

The largest positive unscaled value that fits in [`Self::MAX_PRECISION`](../operations/parquet_variant.variant.decimal.VariantDecimalType.md#op-47340f0c5fb6b00abecfb59e) digits.

<a id="op-ab016523e9f6bcff9c036f9c"></a>
## Native

`assoc_type` · `parquet_variant::variant::decimal::VariantDecimalType::Native` · parquet-variant 59.3.0

```rust
Native
```

Source: `src/variant/decimal.rs:43`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

The underlying signed integer type (i32, i64, or i128)

<a id="op-5a050c07108abae74972d303"></a>
## integer

`function` · `parquet_variant::variant::decimal::VariantDecimalType::integer` · parquet-variant 59.3.0

```rust
fn integer(&self) -> Self::Native
```

Source: `src/variant/decimal.rs:103`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Returns the unscaled integer value

<a id="op-b553625aabfbb69f7d18e1fd"></a>
## is_valid_precision_and_scale

`function` · `parquet_variant::variant::decimal::VariantDecimalType::is_valid_precision_and_scale` · parquet-variant 59.3.0

```rust
fn is_valid_precision_and_scale(precision: &u8, scale: &i8) -> bool
```

Source: `src/variant/decimal.rs:68`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

True if the given precision and scale are valid for this variant decimal type.

NOTE: By a strict reading of the "decimal table" in the [variant spec], one might conclude that
each decimal type has both lower and upper bounds on precision (i.e. Decimal16 with precision 5
is invalid because Decimal4 "covers" it). But the variant shredding integration tests
specifically expect such cases to succeed, so we only enforce the upper bound here.

[shredding spec]: https://github.com/apache/parquet-format/blob/master/VariantEncoding.md#encoding-types

# Example
```
# use parquet_variant::{VariantDecimal4, VariantDecimalType};
#
assert!(VariantDecimal4::is_valid_precision_and_scale(&5, &2));
assert!(!VariantDecimal4::is_valid_precision_and_scale(&10, &2)); // too wide
assert!(!VariantDecimal4::is_valid_precision_and_scale(&5, &-1)); // negative scale
assert!(!VariantDecimal4::is_valid_precision_and_scale(&5, &7)); // scale too big
```

<a id="op-128d15d0aa9b347047508f3c"></a>
## scale

`function` · `parquet_variant::variant::decimal::VariantDecimalType::scale` · parquet-variant 59.3.0

```rust
fn scale(&self) -> u8
```

Source: `src/variant/decimal.rs:106`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Returns the scale (number of digits after the decimal point)

<a id="op-a2eeb35377b6b871e47dc0a5"></a>
## try_new

`function` · `parquet_variant::variant::decimal::VariantDecimalType::try_new` · parquet-variant 59.3.0

```rust
fn try_new(integer: Self::Native, scale: u8) -> Result<Self, ArrowError>
```

Source: `src/variant/decimal.rs:91`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Creates a new decimal value from the given unscaled integer and scale, failing if the
integer's width, or the requested scale, exceeds `MAX_PRECISION`.

NOTE: For compatibility with arrow decimal types, negative scale is allowed as long
as the rescaled value fits in the available precision.

# Example

```
# use parquet_variant::{VariantDecimal4, VariantDecimalType};
#
// Valid: 123.45 (5 digits, scale 2)
let d = VariantDecimal4::try_new(12345, 2).unwrap();
assert_eq!(d.integer(), 12345);
assert_eq!(d.scale(), 2);

VariantDecimal4::try_new(123, 10).expect_err("scale exceeds MAX_PRECISION");
VariantDecimal4::try_new(1234567890, 10).expect_err("value's width exceeds MAX_PRECISION");
```

<a id="op-fbf09968f8b29df88e4f0a60"></a>
## try_new_with_signed_scale

`function` · `parquet_variant::variant::decimal::VariantDecimalType::try_new_with_signed_scale` · parquet-variant 59.3.0

```rust
fn try_new_with_signed_scale(integer: Self::Native, scale: i8) -> Result<Self, ArrowError>
```

Source: `src/variant/decimal.rs:100`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Attempts to convert an unscaled arrow decimal value to the indicated variant decimal type.

Unlike [`Self::try_new`](../operations/parquet_variant.variant.decimal.VariantDecimalType.md#op-a2eeb35377b6b871e47dc0a5), this function accepts a signed scale, and attempts to rescale
negative-scale values to their equivalent (larger) scale-0 values. For example, a decimal
value of 123 with scale -2 becomes 12300 with scale 0.

Fails if rescaling fails, or for any of the reasons [`Self::try_new`](../operations/parquet_variant.variant.decimal.VariantDecimalType.md#op-a2eeb35377b6b871e47dc0a5) could fail.
