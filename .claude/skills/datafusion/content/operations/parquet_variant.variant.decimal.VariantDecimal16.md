# `parquet_variant::variant::decimal::VariantDecimal16`

Full upstream contracts; raw type trees and source locators in [structured records](parquet_variant.variant.decimal.VariantDecimal16.json).

<a id="op-14fe75b2b1af0d1e4b3afdaa"></a>
## VariantDecimal16

`struct` · `parquet_variant::variant::decimal::VariantDecimal16` · parquet-variant 59.3.0

```rust
struct VariantDecimal16
```

Source: `src/variant/decimal.rs:275`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Represents an 16-byte decimal value in the Variant format.

This struct stores a decimal number using a 128-bit signed integer for the coefficient
and an 8-bit unsigned integer for the scale (number of decimal places). Its precision is between 19 and 38 digits.

For valid precision and scale values, see the Variant specification:

<https://github.com/apache/parquet-format/blob/87f2c8bf77eefb4c43d0ebaeea1778bd28ac3609/VariantEncoding.md?plain=1#L418-L420>

# Example: Create a VariantDecimal16
```
# use parquet_variant::VariantDecimal16;
// Create a value representing the decimal 12345678901234567.890
let decimal = VariantDecimal16::try_new(12345678901234567890, 3).unwrap();
```

<a id="op-74997e92a57b1c023bf6689a"></a>
## Error

`assoc_type` · `parquet_variant::variant::decimal::VariantDecimal16::Error` · parquet-variant 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal16", "path": "VariantDecimal16"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [344, 1], "end": [344, 55], "filename": "src/variant/decimal.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i128"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/variant/decimal.rs:344`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4505594edcfc3a56518159a0"></a>
## MAX_PRECISION

`assoc_const` · `parquet_variant::variant::decimal::VariantDecimal16::MAX_PRECISION` · parquet-variant 59.3.0

```rust
MAX_PRECISION
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal16", "path": "VariantDecimal16"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [285, 1], "end": [285, 46], "filename": "src/variant/decimal.rs"}, "trait": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimalType", "path": "VariantDecimalType"}, "trait_path": "parquet_variant::variant::decimal::VariantDecimalType"}`

Source: `src/variant/decimal.rs:285`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-caa6f0bac85fd1cadc40b6fc"></a>
## MAX_PRECISION

`assoc_const` · `parquet_variant::variant::decimal::VariantDecimal16::MAX_PRECISION` · parquet-variant 59.3.0

```rust
MAX_PRECISION
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal16", "path": "VariantDecimal16"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [280, 1], "end": [283, 2], "filename": "src/variant/decimal.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant/decimal.rs:282`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Maximum number of significant digits (38 for 16-byte decimals)

<a id="op-4976839a5e320150518c1b0e"></a>
## MAX_UNSCALED_VALUE

`assoc_const` · `parquet_variant::variant::decimal::VariantDecimal16::MAX_UNSCALED_VALUE` · parquet-variant 59.3.0

```rust
MAX_UNSCALED_VALUE
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal16", "path": "VariantDecimal16"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [285, 1], "end": [285, 46], "filename": "src/variant/decimal.rs"}, "trait": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimalType", "path": "VariantDecimalType"}, "trait_path": "parquet_variant::variant::decimal::VariantDecimalType"}`

Source: `src/variant/decimal.rs:285`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-21e3d695d7b76ab6b25c7adb"></a>
## Native

`assoc_type` · `parquet_variant::variant::decimal::VariantDecimal16::Native` · parquet-variant 59.3.0

```rust
Native
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal16", "path": "VariantDecimal16"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [285, 1], "end": [285, 46], "filename": "src/variant/decimal.rs"}, "trait": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimalType", "path": "VariantDecimalType"}, "trait_path": "parquet_variant::variant::decimal::VariantDecimalType"}`

Source: `src/variant/decimal.rs:285`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e12e634a63def7f6d85c29bd"></a>
## clone

`function` · `parquet_variant::variant::decimal::VariantDecimal16::clone` · parquet-variant 59.3.0

```rust
fn clone(&self) -> VariantDecimal16
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal16", "path": "VariantDecimal16"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [274, 17], "end": [274, 22], "filename": "src/variant/decimal.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/variant/decimal.rs:274`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-23b278f6acbb2d01f7c6a6f3"></a>
## eq

`function` · `parquet_variant::variant::decimal::VariantDecimal16::eq` · parquet-variant 59.3.0

```rust
fn eq(&self, other: &VariantDecimal16) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal16", "path": "VariantDecimal16"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [274, 30], "end": [274, 39], "filename": "src/variant/decimal.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/variant/decimal.rs:274`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d499ffc89c284f93753c298"></a>
## fmt

`function` · `parquet_variant::variant::decimal::VariantDecimal16::fmt` · parquet-variant 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal16", "path": "VariantDecimal16"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [274, 10], "end": [274, 15], "filename": "src/variant/decimal.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/variant/decimal.rs:274`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e5d355fc9f868af77510d0a8"></a>
## fmt

`function` · `parquet_variant::variant::decimal::VariantDecimal16::fmt` · parquet-variant 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal16", "path": "VariantDecimal16"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [285, 1], "end": [285, 46], "filename": "src/variant/decimal.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/variant/decimal.rs:285`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-424f165ab39b6a3c92013332"></a>
## from

`function` · `parquet_variant::variant::decimal::VariantDecimal16::from` · parquet-variant 59.3.0

```rust
fn from(decimal: VariantDecimal8) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal16", "path": "VariantDecimal16"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [303, 1], "end": [303, 66], "filename": "src/variant/decimal.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal8", "path": "VariantDecimal8"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/variant/decimal.rs:303`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-585d36186cfa68809e5fb650"></a>
## from

`function` · `parquet_variant::variant::decimal::VariantDecimal16::from` · parquet-variant 59.3.0

```rust
fn from(decimal: VariantDecimal4) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal16", "path": "VariantDecimal16"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [302, 1], "end": [302, 66], "filename": "src/variant/decimal.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal4", "path": "VariantDecimal4"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/variant/decimal.rs:302`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aaf586d51298e72b5d576a52"></a>
## integer

`function` · `parquet_variant::variant::decimal::VariantDecimal16::integer` · parquet-variant 59.3.0

```rust
fn integer(&self) -> i128
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal16", "path": "VariantDecimal16"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [285, 1], "end": [285, 46], "filename": "src/variant/decimal.rs"}, "trait": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimalType", "path": "VariantDecimalType"}, "trait_path": "parquet_variant::variant::decimal::VariantDecimalType"}`

Source: `src/variant/decimal.rs:285`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f6731eab0d96a8d5af86f87c"></a>
## integer

`function` · `parquet_variant::variant::decimal::VariantDecimal16::integer` · parquet-variant 59.3.0

```rust
fn integer(&self) -> i128
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal16", "path": "VariantDecimal16"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [285, 1], "end": [285, 46], "filename": "src/variant/decimal.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant/decimal.rs:285`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Returns the unscaled integer value of the decimal.

For example, if the decimal is `123.45`, this will return `12345`.

<a id="op-372459635e839616ecb5d8cf"></a>
## scale

`function` · `parquet_variant::variant::decimal::VariantDecimal16::scale` · parquet-variant 59.3.0

```rust
fn scale(&self) -> u8
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal16", "path": "VariantDecimal16"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [285, 1], "end": [285, 46], "filename": "src/variant/decimal.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant/decimal.rs:285`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Returns the scale of the decimal (how many digits after the decimal point).

For example, if the decimal is `123.45`, this will return `2`.

<a id="op-c8b5dd6968bdb23949eb8303"></a>
## scale

`function` · `parquet_variant::variant::decimal::VariantDecimal16::scale` · parquet-variant 59.3.0

```rust
fn scale(&self) -> u8
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal16", "path": "VariantDecimal16"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [285, 1], "end": [285, 46], "filename": "src/variant/decimal.rs"}, "trait": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimalType", "path": "VariantDecimalType"}, "trait_path": "parquet_variant::variant::decimal::VariantDecimalType"}`

Source: `src/variant/decimal.rs:285`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0bfc556d5d72d0098ff66e28"></a>
## try_from

`function` · `parquet_variant::variant::decimal::VariantDecimal16::try_from` · parquet-variant 59.3.0

```rust
fn try_from(integer: i128) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal16", "path": "VariantDecimal16"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [344, 1], "end": [344, 55], "filename": "src/variant/decimal.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i128"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/variant/decimal.rs:344`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7f8546d814d4f6413f32077f"></a>
## try_new

`function` · `parquet_variant::variant::decimal::VariantDecimal16::try_new` · parquet-variant 59.3.0

```rust
fn try_new(integer: i128, scale: u8) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal16", "path": "VariantDecimal16"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [285, 1], "end": [285, 46], "filename": "src/variant/decimal.rs"}, "trait": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimalType", "path": "VariantDecimalType"}, "trait_path": "parquet_variant::variant::decimal::VariantDecimalType"}`

Source: `src/variant/decimal.rs:285`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d30b51169df81162c4852913"></a>
## try_new

`function` · `parquet_variant::variant::decimal::VariantDecimal16::try_new` · parquet-variant 59.3.0

```rust
fn try_new(integer: i128, scale: u8) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal16", "path": "VariantDecimal16"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [285, 1], "end": [285, 46], "filename": "src/variant/decimal.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant/decimal.rs:285`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Attempts to create a new instance of this decimal type, failing if the value is too
wide or the scale is too large.

<a id="op-5f11bd756c32c23c690cd8e7"></a>
## try_new_with_signed_scale

`function` · `parquet_variant::variant::decimal::VariantDecimal16::try_new_with_signed_scale` · parquet-variant 59.3.0

```rust
fn try_new_with_signed_scale(integer: i128, scale: i8) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal16", "path": "VariantDecimal16"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [285, 1], "end": [285, 46], "filename": "src/variant/decimal.rs"}, "trait": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimalType", "path": "VariantDecimalType"}, "trait_path": "parquet_variant::variant::decimal::VariantDecimalType"}`

Source: `src/variant/decimal.rs:285`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
