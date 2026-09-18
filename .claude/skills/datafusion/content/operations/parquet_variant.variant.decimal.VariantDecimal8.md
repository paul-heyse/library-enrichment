# `parquet_variant::variant::decimal::VariantDecimal8`

Full upstream contracts; raw type trees and source locators in [structured records](parquet_variant.variant.decimal.VariantDecimal8.json).

<a id="op-692700eeedfd98510c88e160"></a>
## VariantDecimal8

`struct` · `parquet_variant::variant::decimal::VariantDecimal8` · parquet-variant 59.3.0

```rust
struct VariantDecimal8
```

Source: `src/variant/decimal.rs:247`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Represents an 8-byte decimal value in the Variant format.

This struct stores a decimal number using a 64-bit signed integer for the coefficient
and an 8-bit unsigned integer for the scale (number of decimal places). Its precision is between 10 and 18 digits.

For valid precision and scale values, see the Variant specification:

<https://github.com/apache/parquet-format/blob/87f2c8bf77eefb4c43d0ebaeea1778bd28ac3609/VariantEncoding.md?plain=1#L418-L420>

# Example: Create a VariantDecimal8
```
# use parquet_variant::VariantDecimal8;
// Create a value representing the decimal 123456.78
let decimal = VariantDecimal8::try_new(12345678, 2).expect("Failed to create decimal");
```

<a id="op-5710c32d40b0da5e7b9e1fcf"></a>
## Error

`assoc_type` · `parquet_variant::variant::decimal::VariantDecimal8::Error` · parquet-variant 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal8", "path": "VariantDecimal8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [327, 1], "end": [327, 70], "filename": "src/variant/decimal.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal16", "path": "VariantDecimal16"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/variant/decimal.rs:327`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e18ee2fbc800c298ead009ca"></a>
## Error

`assoc_type` · `parquet_variant::variant::decimal::VariantDecimal8::Error` · parquet-variant 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal8", "path": "VariantDecimal8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [343, 1], "end": [343, 53], "filename": "src/variant/decimal.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i64"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/variant/decimal.rs:343`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-254c7cf48067044360d86c3e"></a>
## MAX_PRECISION

`assoc_const` · `parquet_variant::variant::decimal::VariantDecimal8::MAX_PRECISION` · parquet-variant 59.3.0

```rust
MAX_PRECISION
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal8", "path": "VariantDecimal8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [257, 1], "end": [257, 44], "filename": "src/variant/decimal.rs"}, "trait": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimalType", "path": "VariantDecimalType"}, "trait_path": "parquet_variant::variant::decimal::VariantDecimalType"}`

Source: `src/variant/decimal.rs:257`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b6996fa6396b641e4746fb05"></a>
## MAX_PRECISION

`assoc_const` · `parquet_variant::variant::decimal::VariantDecimal8::MAX_PRECISION` · parquet-variant 59.3.0

```rust
MAX_PRECISION
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal8", "path": "VariantDecimal8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [252, 1], "end": [255, 2], "filename": "src/variant/decimal.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant/decimal.rs:254`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Maximum number of significant digits (18 for 8-byte decimals)

<a id="op-14cb71e5b2394b8c5abdf8ac"></a>
## MAX_UNSCALED_VALUE

`assoc_const` · `parquet_variant::variant::decimal::VariantDecimal8::MAX_UNSCALED_VALUE` · parquet-variant 59.3.0

```rust
MAX_UNSCALED_VALUE
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal8", "path": "VariantDecimal8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [257, 1], "end": [257, 44], "filename": "src/variant/decimal.rs"}, "trait": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimalType", "path": "VariantDecimalType"}, "trait_path": "parquet_variant::variant::decimal::VariantDecimalType"}`

Source: `src/variant/decimal.rs:257`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6eb1b84ed21ce3ab282b536b"></a>
## Native

`assoc_type` · `parquet_variant::variant::decimal::VariantDecimal8::Native` · parquet-variant 59.3.0

```rust
Native
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal8", "path": "VariantDecimal8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [257, 1], "end": [257, 44], "filename": "src/variant/decimal.rs"}, "trait": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimalType", "path": "VariantDecimalType"}, "trait_path": "parquet_variant::variant::decimal::VariantDecimalType"}`

Source: `src/variant/decimal.rs:257`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ff7c0951e054119790a8ef90"></a>
## clone

`function` · `parquet_variant::variant::decimal::VariantDecimal8::clone` · parquet-variant 59.3.0

```rust
fn clone(&self) -> VariantDecimal8
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal8", "path": "VariantDecimal8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [246, 17], "end": [246, 22], "filename": "src/variant/decimal.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/variant/decimal.rs:246`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b55fbc9b714772ee47e4f47a"></a>
## eq

`function` · `parquet_variant::variant::decimal::VariantDecimal8::eq` · parquet-variant 59.3.0

```rust
fn eq(&self, other: &VariantDecimal8) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal8", "path": "VariantDecimal8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [246, 30], "end": [246, 39], "filename": "src/variant/decimal.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/variant/decimal.rs:246`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cc1831e72cf5f8f619c45b6b"></a>
## fmt

`function` · `parquet_variant::variant::decimal::VariantDecimal8::fmt` · parquet-variant 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal8", "path": "VariantDecimal8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [257, 1], "end": [257, 44], "filename": "src/variant/decimal.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/variant/decimal.rs:257`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e853266539513981962297c2"></a>
## fmt

`function` · `parquet_variant::variant::decimal::VariantDecimal8::fmt` · parquet-variant 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal8", "path": "VariantDecimal8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [246, 10], "end": [246, 15], "filename": "src/variant/decimal.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/variant/decimal.rs:246`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3556854a2fe63aed1fbf4e4e"></a>
## from

`function` · `parquet_variant::variant::decimal::VariantDecimal8::from` · parquet-variant 59.3.0

```rust
fn from(decimal: VariantDecimal4) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal8", "path": "VariantDecimal8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [301, 1], "end": [301, 65], "filename": "src/variant/decimal.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal4", "path": "VariantDecimal4"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/variant/decimal.rs:301`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aa16bd3b06a5838098e67f4f"></a>
## integer

`function` · `parquet_variant::variant::decimal::VariantDecimal8::integer` · parquet-variant 59.3.0

```rust
fn integer(&self) -> i64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal8", "path": "VariantDecimal8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [257, 1], "end": [257, 44], "filename": "src/variant/decimal.rs"}, "trait": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimalType", "path": "VariantDecimalType"}, "trait_path": "parquet_variant::variant::decimal::VariantDecimalType"}`

Source: `src/variant/decimal.rs:257`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e14bc1027b840caae3ae2c47"></a>
## integer

`function` · `parquet_variant::variant::decimal::VariantDecimal8::integer` · parquet-variant 59.3.0

```rust
fn integer(&self) -> i64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal8", "path": "VariantDecimal8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [257, 1], "end": [257, 44], "filename": "src/variant/decimal.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant/decimal.rs:257`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Returns the unscaled integer value of the decimal.

For example, if the decimal is `123.45`, this will return `12345`.

<a id="op-7571b9cab375e0611577f76f"></a>
## scale

`function` · `parquet_variant::variant::decimal::VariantDecimal8::scale` · parquet-variant 59.3.0

```rust
fn scale(&self) -> u8
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal8", "path": "VariantDecimal8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [257, 1], "end": [257, 44], "filename": "src/variant/decimal.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant/decimal.rs:257`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Returns the scale of the decimal (how many digits after the decimal point).

For example, if the decimal is `123.45`, this will return `2`.

<a id="op-db146c26900ff859995500f8"></a>
## scale

`function` · `parquet_variant::variant::decimal::VariantDecimal8::scale` · parquet-variant 59.3.0

```rust
fn scale(&self) -> u8
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal8", "path": "VariantDecimal8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [257, 1], "end": [257, 44], "filename": "src/variant/decimal.rs"}, "trait": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimalType", "path": "VariantDecimalType"}, "trait_path": "parquet_variant::variant::decimal::VariantDecimalType"}`

Source: `src/variant/decimal.rs:257`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-772baa046e86772a991f625f"></a>
## try_from

`function` · `parquet_variant::variant::decimal::VariantDecimal8::try_from` · parquet-variant 59.3.0

```rust
fn try_from(decimal: VariantDecimal16) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal8", "path": "VariantDecimal8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [327, 1], "end": [327, 70], "filename": "src/variant/decimal.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal16", "path": "VariantDecimal16"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/variant/decimal.rs:327`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bd215ab7f35c3ed415ddc97e"></a>
## try_from

`function` · `parquet_variant::variant::decimal::VariantDecimal8::try_from` · parquet-variant 59.3.0

```rust
fn try_from(integer: i64) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal8", "path": "VariantDecimal8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [343, 1], "end": [343, 53], "filename": "src/variant/decimal.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i64"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/variant/decimal.rs:343`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cda22a342f85c6181c64c088"></a>
## try_new

`function` · `parquet_variant::variant::decimal::VariantDecimal8::try_new` · parquet-variant 59.3.0

```rust
fn try_new(integer: i64, scale: u8) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal8", "path": "VariantDecimal8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [257, 1], "end": [257, 44], "filename": "src/variant/decimal.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant/decimal.rs:257`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Attempts to create a new instance of this decimal type, failing if the value is too
wide or the scale is too large.

<a id="op-d5f22eb91d944b16fd25c646"></a>
## try_new

`function` · `parquet_variant::variant::decimal::VariantDecimal8::try_new` · parquet-variant 59.3.0

```rust
fn try_new(integer: i64, scale: u8) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal8", "path": "VariantDecimal8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [257, 1], "end": [257, 44], "filename": "src/variant/decimal.rs"}, "trait": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimalType", "path": "VariantDecimalType"}, "trait_path": "parquet_variant::variant::decimal::VariantDecimalType"}`

Source: `src/variant/decimal.rs:257`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f7f94a638208e164d949bb25"></a>
## try_new_with_signed_scale

`function` · `parquet_variant::variant::decimal::VariantDecimal8::try_new_with_signed_scale` · parquet-variant 59.3.0

```rust
fn try_new_with_signed_scale(integer: i64, scale: i8) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal8", "path": "VariantDecimal8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [257, 1], "end": [257, 44], "filename": "src/variant/decimal.rs"}, "trait": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimalType", "path": "VariantDecimalType"}, "trait_path": "parquet_variant::variant::decimal::VariantDecimalType"}`

Source: `src/variant/decimal.rs:257`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
