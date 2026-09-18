# `parquet_variant::variant::decimal::VariantDecimal4`

Full upstream contracts; raw type trees and source locators in [structured records](parquet_variant.variant.decimal.VariantDecimal4.json).

<a id="op-4dc6c06316f04f09f72d4fea"></a>
## VariantDecimal4

`struct` · `parquet_variant::variant::decimal::VariantDecimal4` · parquet-variant 59.3.0

```rust
struct VariantDecimal4
```

Source: `src/variant/decimal.rs:219`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Represents a 4-byte decimal value in the Variant format.

This struct stores a decimal number using a 32-bit signed integer for the coefficient
and an 8-bit unsigned integer for the scale (number of decimal places). Its precision is limited to 9 digits.

For valid precision and scale values, see the Variant specification:
<https://github.com/apache/parquet-format/blob/87f2c8bf77eefb4c43d0ebaeea1778bd28ac3609/VariantEncoding.md?plain=1#L418-L420>

# Example: Create a VariantDecimal4
```
# use parquet_variant::VariantDecimal4;
// Create a value representing the decimal 123.4567
let decimal = VariantDecimal4::try_new(1234567, 4).expect("Failed to create decimal");
```

<a id="op-0979dafe0c3bb538cee53696"></a>
## Error

`assoc_type` · `parquet_variant::variant::decimal::VariantDecimal4::Error` · parquet-variant 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal4", "path": "VariantDecimal4"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [326, 1], "end": [326, 70], "filename": "src/variant/decimal.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal16", "path": "VariantDecimal16"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/variant/decimal.rs:326`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6768205d95bbcb54e7915085"></a>
## Error

`assoc_type` · `parquet_variant::variant::decimal::VariantDecimal4::Error` · parquet-variant 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal4", "path": "VariantDecimal4"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [342, 1], "end": [342, 53], "filename": "src/variant/decimal.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/variant/decimal.rs:342`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d37de219dd1efc1d5cbfbe0c"></a>
## Error

`assoc_type` · `parquet_variant::variant::decimal::VariantDecimal4::Error` · parquet-variant 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal4", "path": "VariantDecimal4"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [325, 1], "end": [325, 69], "filename": "src/variant/decimal.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal8", "path": "VariantDecimal8"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/variant/decimal.rs:325`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d2466d43e3cdd2f07f1668c2"></a>
## MAX_PRECISION

`assoc_const` · `parquet_variant::variant::decimal::VariantDecimal4::MAX_PRECISION` · parquet-variant 59.3.0

```rust
MAX_PRECISION
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal4", "path": "VariantDecimal4"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [224, 1], "end": [227, 2], "filename": "src/variant/decimal.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant/decimal.rs:226`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Maximum number of significant digits (9 for 4-byte decimals)

<a id="op-d68943daf0a317d5e67148c4"></a>
## MAX_PRECISION

`assoc_const` · `parquet_variant::variant::decimal::VariantDecimal4::MAX_PRECISION` · parquet-variant 59.3.0

```rust
MAX_PRECISION
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal4", "path": "VariantDecimal4"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [229, 1], "end": [229, 44], "filename": "src/variant/decimal.rs"}, "trait": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimalType", "path": "VariantDecimalType"}, "trait_path": "parquet_variant::variant::decimal::VariantDecimalType"}`

Source: `src/variant/decimal.rs:229`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-53a011aa08671d9a94f295a2"></a>
## MAX_UNSCALED_VALUE

`assoc_const` · `parquet_variant::variant::decimal::VariantDecimal4::MAX_UNSCALED_VALUE` · parquet-variant 59.3.0

```rust
MAX_UNSCALED_VALUE
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal4", "path": "VariantDecimal4"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [229, 1], "end": [229, 44], "filename": "src/variant/decimal.rs"}, "trait": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimalType", "path": "VariantDecimalType"}, "trait_path": "parquet_variant::variant::decimal::VariantDecimalType"}`

Source: `src/variant/decimal.rs:229`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-360c2cab285abaaa5c714c7a"></a>
## Native

`assoc_type` · `parquet_variant::variant::decimal::VariantDecimal4::Native` · parquet-variant 59.3.0

```rust
Native
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal4", "path": "VariantDecimal4"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [229, 1], "end": [229, 44], "filename": "src/variant/decimal.rs"}, "trait": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimalType", "path": "VariantDecimalType"}, "trait_path": "parquet_variant::variant::decimal::VariantDecimalType"}`

Source: `src/variant/decimal.rs:229`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-692cf7d507b8f012309f55ce"></a>
## clone

`function` · `parquet_variant::variant::decimal::VariantDecimal4::clone` · parquet-variant 59.3.0

```rust
fn clone(&self) -> VariantDecimal4
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal4", "path": "VariantDecimal4"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [218, 17], "end": [218, 22], "filename": "src/variant/decimal.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/variant/decimal.rs:218`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bfecf989b48b366d2aba41a0"></a>
## eq

`function` · `parquet_variant::variant::decimal::VariantDecimal4::eq` · parquet-variant 59.3.0

```rust
fn eq(&self, other: &VariantDecimal4) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal4", "path": "VariantDecimal4"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [218, 30], "end": [218, 39], "filename": "src/variant/decimal.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/variant/decimal.rs:218`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3d3505346193c89eb39f9283"></a>
## fmt

`function` · `parquet_variant::variant::decimal::VariantDecimal4::fmt` · parquet-variant 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal4", "path": "VariantDecimal4"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [218, 10], "end": [218, 15], "filename": "src/variant/decimal.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/variant/decimal.rs:218`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b634c63ae6cea040ffd56927"></a>
## fmt

`function` · `parquet_variant::variant::decimal::VariantDecimal4::fmt` · parquet-variant 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal4", "path": "VariantDecimal4"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [229, 1], "end": [229, 44], "filename": "src/variant/decimal.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/variant/decimal.rs:229`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1bcee25524b039670b890123"></a>
## integer

`function` · `parquet_variant::variant::decimal::VariantDecimal4::integer` · parquet-variant 59.3.0

```rust
fn integer(&self) -> i32
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal4", "path": "VariantDecimal4"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [229, 1], "end": [229, 44], "filename": "src/variant/decimal.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant/decimal.rs:229`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Returns the unscaled integer value of the decimal.

For example, if the decimal is `123.45`, this will return `12345`.

<a id="op-cc4f962946d1f9601b1d749c"></a>
## integer

`function` · `parquet_variant::variant::decimal::VariantDecimal4::integer` · parquet-variant 59.3.0

```rust
fn integer(&self) -> i32
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal4", "path": "VariantDecimal4"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [229, 1], "end": [229, 44], "filename": "src/variant/decimal.rs"}, "trait": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimalType", "path": "VariantDecimalType"}, "trait_path": "parquet_variant::variant::decimal::VariantDecimalType"}`

Source: `src/variant/decimal.rs:229`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0b50702617d2679e58586c2b"></a>
## scale

`function` · `parquet_variant::variant::decimal::VariantDecimal4::scale` · parquet-variant 59.3.0

```rust
fn scale(&self) -> u8
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal4", "path": "VariantDecimal4"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [229, 1], "end": [229, 44], "filename": "src/variant/decimal.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant/decimal.rs:229`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Returns the scale of the decimal (how many digits after the decimal point).

For example, if the decimal is `123.45`, this will return `2`.

<a id="op-6032e669d0c04a98ad9dbaa0"></a>
## scale

`function` · `parquet_variant::variant::decimal::VariantDecimal4::scale` · parquet-variant 59.3.0

```rust
fn scale(&self) -> u8
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal4", "path": "VariantDecimal4"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [229, 1], "end": [229, 44], "filename": "src/variant/decimal.rs"}, "trait": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimalType", "path": "VariantDecimalType"}, "trait_path": "parquet_variant::variant::decimal::VariantDecimalType"}`

Source: `src/variant/decimal.rs:229`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5bc66f7f08b0b5063cbfd8d4"></a>
## try_from

`function` · `parquet_variant::variant::decimal::VariantDecimal4::try_from` · parquet-variant 59.3.0

```rust
fn try_from(decimal: VariantDecimal8) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal4", "path": "VariantDecimal4"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [325, 1], "end": [325, 69], "filename": "src/variant/decimal.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal8", "path": "VariantDecimal8"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/variant/decimal.rs:325`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cb7adeb3cb999249f076260b"></a>
## try_from

`function` · `parquet_variant::variant::decimal::VariantDecimal4::try_from` · parquet-variant 59.3.0

```rust
fn try_from(integer: i32) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal4", "path": "VariantDecimal4"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [342, 1], "end": [342, 53], "filename": "src/variant/decimal.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/variant/decimal.rs:342`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dc4d7402e831026b5d403597"></a>
## try_from

`function` · `parquet_variant::variant::decimal::VariantDecimal4::try_from` · parquet-variant 59.3.0

```rust
fn try_from(decimal: VariantDecimal16) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal4", "path": "VariantDecimal4"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [326, 1], "end": [326, 70], "filename": "src/variant/decimal.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal16", "path": "VariantDecimal16"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/variant/decimal.rs:326`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0eae9a01164acfe753e7e89e"></a>
## try_new

`function` · `parquet_variant::variant::decimal::VariantDecimal4::try_new` · parquet-variant 59.3.0

```rust
fn try_new(integer: i32, scale: u8) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal4", "path": "VariantDecimal4"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [229, 1], "end": [229, 44], "filename": "src/variant/decimal.rs"}, "trait": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimalType", "path": "VariantDecimalType"}, "trait_path": "parquet_variant::variant::decimal::VariantDecimalType"}`

Source: `src/variant/decimal.rs:229`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-972d32a23295c4d96604fea5"></a>
## try_new

`function` · `parquet_variant::variant::decimal::VariantDecimal4::try_new` · parquet-variant 59.3.0

```rust
fn try_new(integer: i32, scale: u8) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal4", "path": "VariantDecimal4"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [229, 1], "end": [229, 44], "filename": "src/variant/decimal.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant/decimal.rs:229`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Attempts to create a new instance of this decimal type, failing if the value is too
wide or the scale is too large.

<a id="op-32fc307a55de29863c4fd128"></a>
## try_new_with_signed_scale

`function` · `parquet_variant::variant::decimal::VariantDecimal4::try_new_with_signed_scale` · parquet-variant 59.3.0

```rust
fn try_new_with_signed_scale(integer: i32, scale: i8) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal4", "path": "VariantDecimal4"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [229, 1], "end": [229, 44], "filename": "src/variant/decimal.rs"}, "trait": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimalType", "path": "VariantDecimalType"}, "trait_path": "parquet_variant::variant::decimal::VariantDecimalType"}`

Source: `src/variant/decimal.rs:229`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
