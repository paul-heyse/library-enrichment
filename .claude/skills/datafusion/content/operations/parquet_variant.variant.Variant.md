# `parquet_variant::variant::Variant`

Full upstream contracts; raw type trees and source locators in [structured records](parquet_variant.variant.Variant.json).

<a id="op-56d8e2cab45de055ef39f43d"></a>
## Variant

`enum` · `parquet_variant::variant::Variant` · parquet-variant 59.3.0

```rust
enum Variant<'m, 'v>
```

Source: `src/variant.rs:247`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Represents a [Parquet Variant]

The lifetimes `'m` and `'v` are for metadata and value buffers, respectively.

# Background

The [specification] says:

The Variant Binary Encoding allows representation of semi-structured data
(e.g. JSON) in a form that can be efficiently queried by path. The design is
intended to allow efficient access to nested data even in the presence of
very wide or deep structures.

Another motivation for the representation is that (aside from metadata) each
nested Variant value is contiguous and self-contained. For example, in a
Variant containing an Array of Variant values, the representation of an
inner Variant value, when paired with the metadata of the full variant, is
itself a valid Variant.

When stored in Parquet files, Variant fields can also be *shredded*. Shredding
refers to extracting some elements of the variant into separate columns for
more efficient extraction/filter pushdown. The [Variant Shredding
specification] describes the details of shredding Variant values as typed
Parquet columns.

A Variant represents a type that contains one of:

* Primitive: A type and corresponding value (e.g. INT, STRING)

* Array: An ordered list of Variant values

* Object: An unordered collection of string/Variant pairs (i.e. key/value
  pairs). An object may not contain duplicate keys.

# Encoding

A Variant is encoded with 2 binary values, the value and the metadata. The
metadata stores a header and an optional dictionary of field names which are
referred to by offset in the value. The value is a binary representation of
the actual data, and varies depending on the type.

# Design Goals

The design goals of the Rust API are as follows:
1. Speed / Zero copy access (no `clone`ing is required)
2. Safety
3. Follow standard Rust conventions

[Parquet Variant]: https://github.com/apache/parquet-format/blob/master/VariantEncoding.md
[specification]: https://github.com/apache/parquet-format/blob/master/VariantEncoding.md
[Variant Shredding specification]: https://github.com/apache/parquet-format/blob/master/VariantShredding.md

# Casting Semantics

Scalar conversion semantics intentionally follow Arrow cast behavior where applicable.
Conversions in this module delegate to Arrow compute cast helpers such as
[`num_cast`](../operations/arrow_cast.cast.num_cast.md#op-0e92fdea3be20c6f49abfe49), [`cast_num_to_bool`](../operations/arrow_cast.cast.cast_num_to_bool.md#op-16860439f843ad790d37a510), [`single_bool_to_numeric`](../operations/arrow_cast.cast.single_bool_to_numeric.md#op-a7b1aae3cef1c29d484ac0e9), and
[`cast_single_string_to_boolean_default`](../operations/arrow_cast.cast.string.cast_single_string_to_boolean_default.md#op-553fa47b77607f7890b9d39c).

- [`Self::as_boolean`](../operations/parquet_variant.variant.Variant.md#op-37acc19f0bf5e38fedccbafa) accepts boolean, numeric, and string variants.
  Numeric zero maps to `false`; non-zero maps to `true`. String parsing follows
  Arrow UTF8-to-boolean cast rules.
- Numeric accessors such as [`Self::as_int8`](../operations/parquet_variant.variant.Variant.md#op-05d7bf64ff0b436379b004c1), [`Self::as_int64`](../operations/parquet_variant.variant.Variant.md#op-c5553d7ea41cbebd7cfd9e54), [`Self::as_u8`](../operations/parquet_variant.variant.Variant.md#op-a51652df3035f9e6b5563b22),
  [`Self::as_u64`](../operations/parquet_variant.variant.Variant.md#op-21279685d3d7efd35e876d0a), [`Self::as_f16`](../operations/parquet_variant.variant.Variant.md#op-0d3dd23695705c5024c38b93), [`Self::as_f32`](../operations/parquet_variant.variant.Variant.md#op-e9e17d20455ec72be4ec6952), and [`Self::as_f64`](../operations/parquet_variant.variant.Variant.md#op-aa0bd1a22bfec77b976136f5) accept
  boolean and numeric variants (integers, floating-point, and decimals).
  They return `None` when conversion is not possible.
- Decimal accessors such as [`Self::as_decimal4`](../operations/parquet_variant.variant.Variant.md#op-6d322c33f1d31ef811b020ac), [`Self::as_decimal8`](../operations/parquet_variant.variant.Variant.md#op-dcbc2ef05ddf1bb17e78370b), and
  [`Self::as_decimal16`](../operations/parquet_variant.variant.Variant.md#op-74dd74708d44b87135a22ef6) accept compatible decimal variants, integer variants,
  float variants and string variants.
  They return `None` when conversion is not possible.

# Examples:

## Creating `Variant` from Rust Types
```
use parquet_variant::Variant;
// variants can be directly constructed
let variant = Variant::Int32(123);
// or constructed via `From` impls
assert_eq!(variant, Variant::from(123i32));
```
## Creating `Variant` from metadata and value
```
# use parquet_variant::{Variant, VariantMetadata};
let metadata = [0x01, 0x00, 0x00];
let value = [0x09, 0x48, 0x49];
// parse the header metadata
assert_eq!(
  Variant::from("HI"),
  Variant::new(&metadata, &value)
);
```

## Using `Variant` values
```
# use parquet_variant::Variant;
# let variant = Variant::Int32(123);
// variants can be used in match statements like normal enums
match variant {
  Variant::Int32(i) => println!("Integer: {}", i),
  Variant::String(s) => println!("String: {}", s),
  _ => println!("Other variant"),
}
```

# Validation

Every instance of variant is either _valid_ or _invalid_. depending on whether the
underlying bytes are a valid encoding of a variant value (see below).

Instances produced by [`Self::try_new`](../operations/parquet_variant.variant.Variant.md#op-db76763b87893e42f4db1c97), [`Self::try_new_with_metadata`](../operations/parquet_variant.variant.Variant.md#op-4ad50695a7b49b85d98239ed), or [`Self::with_full_validation`](../operations/parquet_variant.variant.Variant.md#op-353568855fd0414171c82a71)
are fully _validated_. They always contain _valid_ data, and infallible accesses such as
iteration and indexing are panic-free. The validation cost is `O(m + v)` where `m` and
`v` are the number of bytes in the metadata and value buffers, respectively.

Instances produced by [`Self::new`](../operations/parquet_variant.variant.Variant.md#op-69dc8ecd853e71e63a220c10) and [`Self::new_with_metadata`](../operations/parquet_variant.variant.Variant.md#op-5031210bdfee520209cb447b) are _unvalidated_ and so
they may contain either _valid_ or _invalid_ data. Infallible accesses to variant objects and
arrays, such as iteration and indexing will panic if the underlying bytes are _invalid_, and
fallible alternatives are provided as panic-free alternatives. [`Self::with_full_validation`](../operations/parquet_variant.variant.Variant.md#op-353568855fd0414171c82a71) can also be
used to _validate_ an _unvalidated_ instance, if desired.

_Unvalidated_ instances can be constructed in constant time. This can be useful if the caller
knows the underlying bytes were already validated previously, or if the caller intends to
perform a small number of (fallible) accesses to a large variant value.

A _validated_ variant value guarantees that the associated [metadata] and all nested [object]
and [array] values are _valid_. Primitive variant subtypes are always _valid_ by construction.

# Safety

Even an _invalid_ variant value is still _safe_ to use in the Rust sense. Accessing it with
infallible methods may cause panics but will never lead to undefined behavior.

[metadata]: VariantMetadata#Validation
[object]: VariantObject#Validation
[array]: VariantList#Validation

<a id="op-5e2ce44fe114522703688918"></a>
## Binary

`variant` · `parquet_variant::variant::Variant::Binary` · parquet-variant 59.3.0

```rust
Binary
```

Source: `src/variant.rs:284`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Primitive (type_id=1): BINARY

<a id="op-7cc41da669d6d2fd532b9464"></a>
## BooleanFalse

`variant` · `parquet_variant::variant::Variant::BooleanFalse` · parquet-variant 59.3.0

```rust
BooleanFalse
```

Source: `src/variant.rs:281`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Primitive (type_id=1): BOOLEAN (false)

<a id="op-113df4ae6d56369b00f53db5"></a>
## BooleanTrue

`variant` · `parquet_variant::variant::Variant::BooleanTrue` · parquet-variant 59.3.0

```rust
BooleanTrue
```

Source: `src/variant.rs:279`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Primitive (type_id=1): BOOLEAN (true)

<a id="op-5587e2ee16f741fc148108fa"></a>
## Date

`variant` · `parquet_variant::variant::Variant::Date` · parquet-variant 59.3.0

```rust
Date
```

Source: `src/variant.rs:259`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Primitive (type_id=1): DATE

<a id="op-e7175e64995ea15704967785"></a>
## Decimal16

`variant` · `parquet_variant::variant::Variant::Decimal16` · parquet-variant 59.3.0

```rust
Decimal16
```

Source: `src/variant.rs:273`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Primitive (type_id=1): DECIMAL(precision, scale) 128-bits

<a id="op-4dc30501e502dfd09d810fa5"></a>
## Decimal4

`variant` · `parquet_variant::variant::Variant::Decimal4` · parquet-variant 59.3.0

```rust
Decimal4
```

Source: `src/variant.rs:269`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Primitive (type_id=1): DECIMAL(precision, scale) 32-bits

<a id="op-a85b97f88af29093cd0c598c"></a>
## Decimal8

`variant` · `parquet_variant::variant::Variant::Decimal8` · parquet-variant 59.3.0

```rust
Decimal8
```

Source: `src/variant.rs:271`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Primitive (type_id=1): DECIMAL(precision, scale) 64-bits

<a id="op-943dd8f3f7608e1cbd9bcce7"></a>
## Double

`variant` · `parquet_variant::variant::Variant::Double` · parquet-variant 59.3.0

```rust
Double
```

Source: `src/variant.rs:277`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Primitive (type_id=1): DOUBLE

<a id="op-3abcab4164d7a7f8c481ff5d"></a>
## Error

`assoc_type` · `parquet_variant::variant::Variant::Error` · parquet-variant 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"lifetime": "'_"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1821, 1], "end": [1829, 2], "filename": "src/variant.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"tuple": [{"primitive": "i32"}, {"primitive": "u8"}]}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/variant.rs:1822`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-830b47c693fb6647dd134cdb"></a>
## Error

`assoc_type` · `parquet_variant::variant::Variant::Error` · parquet-variant 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"lifetime": "'_"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1841, 1], "end": [1849, 2], "filename": "src/variant.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"tuple": [{"primitive": "i128"}, {"primitive": "u8"}]}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/variant.rs:1842`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c6a22a29b4c0b48e04d27f73"></a>
## Error

`assoc_type` · `parquet_variant::variant::Variant::Error` · parquet-variant 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"lifetime": "'_"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1831, 1], "end": [1839, 2], "filename": "src/variant.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"tuple": [{"primitive": "i64"}, {"primitive": "u8"}]}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/variant.rs:1832`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-05e7f8c81a06808e2fd4d5fa"></a>
## Float

`variant` · `parquet_variant::variant::Variant::Float` · parquet-variant 59.3.0

```rust
Float
```

Source: `src/variant.rs:275`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Primitive (type_id=1): FLOAT

<a id="op-ca1dfe1fd4035282902234da"></a>
## Int16

`variant` · `parquet_variant::variant::Variant::Int16` · parquet-variant 59.3.0

```rust
Int16
```

Source: `src/variant.rs:253`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Primitive (type_id=1): INT(16, SIGNED)

<a id="op-f3c48c884545417cc27bd456"></a>
## Int32

`variant` · `parquet_variant::variant::Variant::Int32` · parquet-variant 59.3.0

```rust
Int32
```

Source: `src/variant.rs:255`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Primitive (type_id=1): INT(32, SIGNED)

<a id="op-5cbedb7787256cca8f4e9755"></a>
## Int64

`variant` · `parquet_variant::variant::Variant::Int64` · parquet-variant 59.3.0

```rust
Int64
```

Source: `src/variant.rs:257`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Primitive (type_id=1): INT(64, SIGNED)

<a id="op-07cae483ad4a6f0bad4618ad"></a>
## Int8

`variant` · `parquet_variant::variant::Variant::Int8` · parquet-variant 59.3.0

```rust
Int8
```

Source: `src/variant.rs:251`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Primitive (type_id=1): INT(8, SIGNED)

<a id="op-9ee3bf99ad7b6967fe1dd651"></a>
## List

`variant` · `parquet_variant::variant::Variant::List` · parquet-variant 59.3.0

```rust
List
```

Source: `src/variant.rs:297`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Array (type_id=4): N/A

<a id="op-a77e1395c0a1609bb5be4d06"></a>
## Null

`variant` · `parquet_variant::variant::Variant::Null` · parquet-variant 59.3.0

```rust
Null
```

Source: `src/variant.rs:249`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Primitive type: Null

<a id="op-f448b5728cc2baae6632c68e"></a>
## Object

`variant` · `parquet_variant::variant::Variant::Object` · parquet-variant 59.3.0

```rust
Object
```

Source: `src/variant.rs:295`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Object (type_id=3): N/A

<a id="op-1a842997df5d618141102ea0"></a>
## ShortString

`variant` · `parquet_variant::variant::Variant::ShortString` · parquet-variant 59.3.0

```rust
ShortString
```

Source: `src/variant.rs:292`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Short String (type_id=2): STRING

<a id="op-0aba1fcff29b9f92db023de7"></a>
## String

`variant` · `parquet_variant::variant::Variant::String` · parquet-variant 59.3.0

```rust
String
```

Source: `src/variant.rs:286`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Primitive (type_id=1): STRING

<a id="op-99b5047e4470e6f922f29cf1"></a>
## Time

`variant` · `parquet_variant::variant::Variant::Time` · parquet-variant 59.3.0

```rust
Time
```

Source: `src/variant.rs:288`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Primitive (type_id=1): TIME(isAdjustedToUTC=false, MICROS)

<a id="op-3aa28f8ffd1710c804e45b6d"></a>
## TimestampMicros

`variant` · `parquet_variant::variant::Variant::TimestampMicros` · parquet-variant 59.3.0

```rust
TimestampMicros
```

Source: `src/variant.rs:261`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Primitive (type_id=1): TIMESTAMP(isAdjustedToUTC=true, MICROS)

<a id="op-d1677b651f47f2107b36edfb"></a>
## TimestampNanos

`variant` · `parquet_variant::variant::Variant::TimestampNanos` · parquet-variant 59.3.0

```rust
TimestampNanos
```

Source: `src/variant.rs:265`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Primitive (type_id=1): TIMESTAMP(isAdjustedToUTC=true, NANOS)

<a id="op-5c9def4bc3a62ab643cac433"></a>
## TimestampNtzMicros

`variant` · `parquet_variant::variant::Variant::TimestampNtzMicros` · parquet-variant 59.3.0

```rust
TimestampNtzMicros
```

Source: `src/variant.rs:263`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Primitive (type_id=1): TIMESTAMP(isAdjustedToUTC=false, MICROS)

<a id="op-4d794b9637c51157932fd949"></a>
## TimestampNtzNanos

`variant` · `parquet_variant::variant::Variant::TimestampNtzNanos` · parquet-variant 59.3.0

```rust
TimestampNtzNanos
```

Source: `src/variant.rs:267`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Primitive (type_id=1): TIMESTAMP(isAdjustedToUTC=false, NANOS)

<a id="op-5f02b62d956577458568a92d"></a>
## Uuid

`variant` · `parquet_variant::variant::Variant::Uuid` · parquet-variant 59.3.0

```rust
Uuid
```

Source: `src/variant.rs:290`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Primitive (type_id=1): UUID

<a id="op-37acc19f0bf5e38fedccbafa"></a>
## as_boolean

`function` · `parquet_variant::variant::Variant::as_boolean` · parquet-variant 59.3.0

```rust
fn as_boolean(&self) -> Option<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [337, 1], "end": [1646, 2], "filename": "src/variant.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant.rs:567`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Converts this variant to a `bool` if possible.

Returns `Some(bool)` for boolean, numeric and string variants,
`None` for non-boolean variants.

# Examples

```
use parquet_variant::Variant;

// you can extract a bool from the true variant
let v1 = Variant::from(true);
assert_eq!(v1.as_boolean(), Some(true));

// and the false variant
let v2 = Variant::from(false);
assert_eq!(v2.as_boolean(), Some(false));

// and a numeric variant
let v3 = Variant::from(3);
assert_eq!(v3.as_boolean(), Some(true));

// and a string variant
let v4 = Variant::from("true");
assert_eq!(v4.as_boolean(), Some(true));

// but not from other variants
let v5 = Variant::from("hello!");
assert_eq!(v5.as_boolean(), None);
```

<a id="op-74dd74708d44b87135a22ef6"></a>
## as_decimal16

`function` · `parquet_variant::variant::Variant::as_decimal16` · parquet-variant 59.3.0

```rust
fn as_decimal16(&self) -> Option<VariantDecimal16>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [337, 1], "end": [1646, 2], "filename": "src/variant.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant.rs:1324`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Converts this variant to tuple with a 16-byte unscaled value if possible.

Returns `Some((i128, u8))` for decimal variants where the unscaled value
fits in `i128` range, the scale will be 0 if the input is string variants.
`None` for non-decimal variants or decimal values that would overflow.

# Examples

```
use parquet_variant::{Variant, VariantDecimal16, VariantDecimal4};

// you can extract decimal parts from smaller or equally-sized decimal variants
let v1 = Variant::from(VariantDecimal4::try_new(1234_i32, 2).unwrap());
assert_eq!(v1.as_decimal16(), VariantDecimal16::try_new(1234_i128, 2).ok());

// or from a string variant if it can be parsed as decimal
let v2 = Variant::from("123.45");
assert_eq!(v2.as_decimal16(), VariantDecimal16::try_new(12345, 2).ok());

// but not if the variant is not a decimal
let v3 = Variant::from("hello!");
assert_eq!(v3.as_decimal16(), None);
```

<a id="op-6d322c33f1d31ef811b020ac"></a>
## as_decimal4

`function` · `parquet_variant::variant::Variant::as_decimal4` · parquet-variant 59.3.0

```rust
fn as_decimal4(&self) -> Option<VariantDecimal4>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [337, 1], "end": [1646, 2], "filename": "src/variant.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant.rs:1230`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Converts this variant to tuple with a 4-byte unscaled value if possible.

Returns `Some((i32, u8))` for decimal variants where the unscaled value
fits in `i32` range,
`None` for non-decimal variants or decimal values that would overflow.

# Examples

```
use parquet_variant::{Variant, VariantDecimal4, VariantDecimal8};

// you can extract decimal parts from smaller or equally-sized decimal variants
let v1 = Variant::from(VariantDecimal4::try_new(1234_i32, 2).unwrap());
assert_eq!(v1.as_decimal4(), VariantDecimal4::try_new(1234_i32, 2).ok());

// and from larger decimal variants if they fit
let v2 = Variant::from(VariantDecimal8::try_new(1234_i64, 2).unwrap());
assert_eq!(v2.as_decimal4(), VariantDecimal4::try_new(1234_i32, 2).ok());

// or from string variants if they can be parsed as decimals
let v3 = Variant::from("123.45");
assert_eq!(v3.as_decimal4(), VariantDecimal4::try_new(12345, 2).ok());

// but not if the value would overflow i32
let v4 = Variant::from(VariantDecimal8::try_new(12345678901i64, 2).unwrap());
assert_eq!(v4.as_decimal4(), None);

// or if the variant is not a decimal
let v5 = Variant::from("hello!");
assert_eq!(v5.as_decimal4(), None);
```

<a id="op-dcbc2ef05ddf1bb17e78370b"></a>
## as_decimal8

`function` · `parquet_variant::variant::Variant::as_decimal8` · parquet-variant 59.3.0

```rust
fn as_decimal8(&self) -> Option<VariantDecimal8>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [337, 1], "end": [1646, 2], "filename": "src/variant.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant.rs:1281`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Converts this variant to tuple with an 8-byte unscaled value if possible.

Returns `Some((i64, u8))` for decimal variants where the unscaled value
fits in `i64` range, the scale will be 0 if the input is string variants.
`None` for non-decimal variants or decimal values that would overflow.

# Examples

```
use parquet_variant::{Variant, VariantDecimal4, VariantDecimal8, VariantDecimal16};

// you can extract decimal parts from smaller or equally-sized decimal variants
let v1 = Variant::from(VariantDecimal4::try_new(1234_i32, 2).unwrap());
assert_eq!(v1.as_decimal8(), VariantDecimal8::try_new(1234_i64, 2).ok());

// and from larger decimal variants if they fit
let v2 = Variant::from(VariantDecimal16::try_new(1234_i128, 2).unwrap());
assert_eq!(v2.as_decimal8(), VariantDecimal8::try_new(1234_i64, 2).ok());

// or from string variants if they can be parsed as decimals
let v3 = Variant::from("123.45");
assert_eq!(v3.as_decimal8(), VariantDecimal8::try_new(12345, 2).ok());

// but not if the value would overflow i64
let v4 = Variant::from(VariantDecimal16::try_new(2e19 as i128, 2).unwrap());
assert_eq!(v4.as_decimal8(), None);

// or if the variant is not a decimal
let v5 = Variant::from("hello!");
assert_eq!(v5.as_decimal8(), None);
```

<a id="op-0d3dd23695705c5024c38b93"></a>
## as_f16

`function` · `parquet_variant::variant::Variant::as_f16` · parquet-variant 59.3.0

```rust
fn as_f16(&self) -> Option<f16>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [337, 1], "end": [1646, 2], "filename": "src/variant.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant.rs:1379`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Converts this variant to an `f16` if possible.

Returns `Some(f16)` for boolean and numeric variants(integers, floating-point,
and decimals with scale 0) that fit in `f16` range
`None` otherwise.

# Example

```
use parquet_variant::Variant;
use half::f16;

// you can extract an f16 from a float variant
let v1 = Variant::from(std::f32::consts::PI);
assert_eq!(v1.as_f16(), Some(f16::from_f32(std::f32::consts::PI)));

// and from a double variant (with loss of precision to nearest f16)
let v2 = Variant::from(std::f64::consts::PI);
assert_eq!(v2.as_f16(), Some(f16::from_f64(std::f64::consts::PI)));

// and from boolean
let v3 = Variant::BooleanTrue;
assert_eq!(v3.as_f16(), Some(f16::from_f32(1.0)));

// return inf if overflow
let v4 = Variant::from(123456);
assert_eq!(v4.as_f16(), Some(f16::INFINITY));

// but not from other variants
let v5 = Variant::from("hello!");
assert_eq!(v5.as_f16(), None);

<a id="op-e9e17d20455ec72be4ec6952"></a>
## as_f32

`function` · `parquet_variant::variant::Variant::as_f32` · parquet-variant 59.3.0

```rust
fn as_f32(&self) -> Option<f32>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [337, 1], "end": [1646, 2], "filename": "src/variant.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant.rs:1414`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Converts this variant to an `f32` if possible.

Returns `Some(f32)` for boolean and numeric variants(integers, floating-point,
and decimals with scale 0) that fit in `f32` range
`None` otherwise.

# Examples

```
use parquet_variant::Variant;

// you can extract an f32 from a float variant
let v1 = Variant::from(std::f32::consts::PI);
assert_eq!(v1.as_f32(), Some(std::f32::consts::PI));

// and from a double variant (with loss of precision to nearest f32)
let v2 = Variant::from(std::f64::consts::PI);
assert_eq!(v2.as_f32(), Some(std::f32::consts::PI));

// and from boolean variant
let v3 = Variant::BooleanTrue;
assert_eq!(v3.as_f32(), Some(1.0));

// and return inf if overflow
let v4 = Variant::from(f64::MAX);
assert_eq!(v4.as_f32(), Some(f32::INFINITY));

// but not from other variants
let v5 = Variant::from("hello!");
assert_eq!(v5.as_f32(), None);
```

<a id="op-aa0bd1a22bfec77b976136f5"></a>
## as_f64

`function` · `parquet_variant::variant::Variant::as_f64` · parquet-variant 59.3.0

```rust
fn as_f64(&self) -> Option<f64>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [337, 1], "end": [1646, 2], "filename": "src/variant.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant.rs:1445`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Converts this variant to an `f64` if possible.

Returns `Some(f64)` for boolean and numeric variants(integers, floating-point,
and decimals with scale 0) that fit in `f64` range
`None` for other variants or can't be represented by an f64.

# Examples

```
use parquet_variant::Variant;

// you can extract an f64 from a float variant
let v1 = Variant::from(std::f32::consts::PI);
assert_eq!(v1.as_f64(), Some(std::f32::consts::PI as f64));

// and from a double variant
let v2 = Variant::from(std::f64::consts::PI);
assert_eq!(v2.as_f64(), Some(std::f64::consts::PI));

// and from boolean variant
let v3 = Variant::BooleanTrue;
assert_eq!(v3.as_f64(), Some(1.0f64));

// but not from other variants
let v5 = Variant::from("hello!");
assert_eq!(v5.as_f64(), None);
```

<a id="op-dbd9a6e52aefb6d24fa01ee6"></a>
## as_int16

`function` · `parquet_variant::variant::Variant::as_int16` · parquet-variant 59.3.0

```rust
fn as_int16(&self) -> Option<i16>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [337, 1], "end": [1646, 2], "filename": "src/variant.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant.rs:958`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Converts this variant to an `i16` if possible.

Returns `Some(i16)` for boolean and numeric variants(integers, floating-point,
and decimals with scale 0) that fit in `i16` range
`None` for other variants or values that would overflow.

# Examples

```
use parquet_variant::Variant;

// you can read an int64 variant into an i16 if it fits
let v1 = Variant::from(123i64);
assert_eq!(v1.as_int16(), Some(123i16));

// or from boolean variant
let v2 = Variant::BooleanFalse;
assert_eq!(v2.as_int16(), Some(0));

// but not if it would overflow
let v3 = Variant::from(123456i64);
assert_eq!(v3.as_int16(), None);

// or if the variant cannot be cast into an integer
let v4 = Variant::from("hello!");
assert_eq!(v4.as_int16(), None);
```

<a id="op-b363f3430b17cf1fc55bfcf1"></a>
## as_int32

`function` · `parquet_variant::variant::Variant::as_int32` · parquet-variant 59.3.0

```rust
fn as_int32(&self) -> Option<i32>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [337, 1], "end": [1646, 2], "filename": "src/variant.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant.rs:989`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Converts this variant to an `i32` if possible.

Returns `Some(i32)` for boolean and numeric variants(integers, floating-point,
and decimals with scale 0) that fit in `i32` range
`None` for other variants or values that would overflow.

# Examples

```
use parquet_variant::Variant;

// you can read an int64 variant into an i32 if it fits
let v1 = Variant::from(123i64);
assert_eq!(v1.as_int32(), Some(123i32));

// or from boolean variant
let v2 = Variant::BooleanFalse;
assert_eq!(v2.as_int32(), Some(0));

// but not if it would overflow
let v3 = Variant::from(12345678901i64);
assert_eq!(v3.as_int32(), None);

// or if the variant cannot be cast into an integer
let v4 = Variant::from("hello!");
assert_eq!(v4.as_int32(), None);
```

<a id="op-c5553d7ea41cbebd7cfd9e54"></a>
## as_int64

`function` · `parquet_variant::variant::Variant::as_int64` · parquet-variant 59.3.0

```rust
fn as_int64(&self) -> Option<i64>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [337, 1], "end": [1646, 2], "filename": "src/variant.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant.rs:1016`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Converts this variant to an `i64` if possible.

Returns `Some(i64)` for boolean and numeric variants(integers, floating-point,
and decimals with scale 0) that fit in `i64` range
`None` for other variants or values that would overflow.

# Examples

```
use parquet_variant::Variant;

// you can read an int64 variant into an i64
let v1 = Variant::from(123i64);
assert_eq!(v1.as_int64(), Some(123i64));

// or from boolean variant
let v2 = Variant::BooleanFalse;
assert_eq!(v2.as_int64(), Some(0));

// but not a variant that cannot be cast into an integer
let v3 = Variant::from("hello!");
assert_eq!(v3.as_int64(), None);
```

<a id="op-05d7bf64ff0b436379b004c1"></a>
## as_int8

`function` · `parquet_variant::variant::Variant::as_int8` · parquet-variant 59.3.0

```rust
fn as_int8(&self) -> Option<i8>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [337, 1], "end": [1646, 2], "filename": "src/variant.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant.rs:927`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Converts this variant to an `i8` if possible.

Returns `Some(i8)` for boolean and numeric variants(integers, floating-point,
and decimals with scale 0) that fit in `i8` range,
`None` for other variants or values that would overflow.

# Examples

```
use parquet_variant::Variant;

// you can read an int64 variant into an i8 if it fits
let v1 = Variant::from(123i64);
assert_eq!(v1.as_int8(), Some(123i8));

// or from boolean variant
let v2 = Variant::BooleanFalse;
assert_eq!(v2.as_int8(), Some(0));

// but not if it would overflow
let v3 = Variant::from(1234i64);
assert_eq!(v3.as_int8(), None);

// or if the variant cannot be cast into an integer
let v4 = Variant::from("hello!");
assert_eq!(v4.as_int8(), None);
```

<a id="op-dd7d161e5c10ce915e5b58cf"></a>
## as_list

`function` · `parquet_variant::variant::Variant::as_list` · parquet-variant 59.3.0

```rust
fn as_list(&'m self) -> Option<&'m VariantList<'m, 'v>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [337, 1], "end": [1646, 2], "filename": "src/variant.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant.rs:1534`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Converts this variant to a `List` if it is a [`VariantList`](../operations/parquet_variant.variant.list.VariantList.md#op-50198737fce518dd29afb54a).

Returns `Some(&VariantList)` for list variants,
`None` for non-list variants.

See [`Self::get_path`](../operations/parquet_variant.variant.Variant.md#op-c50232f473c847799463d621) to dynamically traverse lists

# Examples
```
# use parquet_variant::{Variant, VariantBuilder, VariantList};
# let (metadata, value) = {
# let mut builder = VariantBuilder::new();
#   let mut list = builder.new_list();
#   list.append_value("John");
#   list.append_value("Doe");
#   list.finish();
#   builder.finish()
# };
// list that is ["John", "Doe"]
let variant = Variant::new(&metadata, &value);
// use the `as_list` method to access the list
let list = variant.as_list().expect("variant should be a list");
assert_eq!(list.len(), 2);
assert_eq!(list.get(0).unwrap(), Variant::from("John"));
assert_eq!(list.get(1).unwrap(), Variant::from("Doe"));
```

<a id="op-c0bf6644ea34dfd2ae0cc5c0"></a>
## as_naive_date

`function` · `parquet_variant::variant::Variant::as_naive_date` · parquet-variant 59.3.0

```rust
fn as_naive_date(&self) -> Option<NaiveDate>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [337, 1], "end": [1646, 2], "filename": "src/variant.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant.rs:603`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Converts this variant to a `NaiveDate` if possible.

Returns `Some(NaiveDate)` for date variants,
`None` for non-date variants.

# Examples

```
use parquet_variant::Variant;
use chrono::NaiveDate;

// you can extract a NaiveDate from a date variant
let date = NaiveDate::from_ymd_opt(2025, 4, 12).unwrap();
let v1 = Variant::from(date);
assert_eq!(v1.as_naive_date(), Some(date));

// but not from other variants
let v2 = Variant::from("hello!");
assert_eq!(v2.as_naive_date(), None);
```

<a id="op-4a868d951e18bb2b6fdd867e"></a>
## as_null

`function` · `parquet_variant::variant::Variant::as_null` · parquet-variant 59.3.0

```rust
fn as_null(&self) -> Option<()>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [337, 1], "end": [1646, 2], "filename": "src/variant.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant.rs:533`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Converts this variant to `()` if it is null.

Returns `Some(())` for null variants,
`None` for non-null variants.

# Examples

```
use parquet_variant::Variant;

// you can extract `()` from a null variant
let v1 = Variant::from(());
assert_eq!(v1.as_null(), Some(()));

// but not from other variants
let v2 = Variant::from("hello!");
assert_eq!(v2.as_null(), None);
```

<a id="op-49a9fec07832505931bda83c"></a>
## as_object

`function` · `parquet_variant::variant::Variant::as_object` · parquet-variant 59.3.0

```rust
fn as_object(&'m self) -> Option<&'m VariantObject<'m, 'v>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [337, 1], "end": [1646, 2], "filename": "src/variant.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant.rs:1472`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Converts this variant to an `Object` if it is an [`VariantObject`](../operations/parquet_variant.variant.object.VariantObject.md#op-4002021fea3ee4472f856a19).

Returns `Some(&VariantObject)` for object variants,
`None` for non-object variants.

See [`Self::get_path`](../operations/parquet_variant.variant.Variant.md#op-c50232f473c847799463d621) to dynamically traverse objects

# Examples
```
# use parquet_variant::{Variant, VariantBuilder, VariantObject};
# let (metadata, value) = {
# let mut builder = VariantBuilder::new();
#   let mut obj = builder.new_object();
#   obj.insert("name", "John");
#   obj.finish();
#   builder.finish()
# };
// object that is {"name": "John"}
 let variant = Variant::new(&metadata, &value);
// use the `as_object` method to access the object
let obj = variant.as_object().expect("variant should be an object");
assert_eq!(obj.get("name"), Some(Variant::from("John")));
```

<a id="op-46349e00b98b97a4eefc9562"></a>
## as_string

`function` · `parquet_variant::variant::Variant::as_string` · parquet-variant 59.3.0

```rust
fn as_string(&'v self) -> Option<&'v str>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [337, 1], "end": [1646, 2], "filename": "src/variant.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant.rs:807`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Converts this variant to a `&str` if possible.

Returns `Some(&str)` for string variants (both regular and short strings),
`None` for non-string variants.

# Examples

```
use parquet_variant::Variant;

// you can extract a string from string variants
let s = "hello!";
let v1 = Variant::from(s);
assert_eq!(v1.as_string(), Some(s));

// but not from other variants
let v2 = Variant::from(123i64);
assert_eq!(v2.as_string(), None);
```

<a id="op-cb688a80bafec456e9623fe8"></a>
## as_time_utc

`function` · `parquet_variant::variant::Variant::as_time_utc` · parquet-variant 59.3.0

```rust
fn as_time_utc(&'m self) -> Option<NaiveTime>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [337, 1], "end": [1646, 2], "filename": "src/variant.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant.rs:1562`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Converts this variant to a `NaiveTime` if possible.

Returns `Some(NaiveTime)` for `Variant::Time`,
`None` for non-Time variants.

# Example

```
use chrono::NaiveTime;
use parquet_variant::Variant;

// you can extract a `NaiveTime` from a `Variant::Time`
let time = NaiveTime::from_hms_micro_opt(1, 2, 3, 4).unwrap();
let v1 = Variant::from(time);
assert_eq!(Some(time), v1.as_time_utc());

// but not from other variants.
let v2 = Variant::from("Hello");
assert_eq!(None, v2.as_time_utc());
```

<a id="op-544a84eaabe4bde5c4242760"></a>
## as_timestamp_micros

`function` · `parquet_variant::variant::Variant::as_timestamp_micros` · parquet-variant 59.3.0

```rust
fn as_timestamp_micros(&self) -> Option<DateTime<Utc>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [337, 1], "end": [1646, 2], "filename": "src/variant.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant.rs:640`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Converts this variant to a `DateTime<Utc>` if possible.

Returns `Some(DateTime<Utc>)` for [`Variant::TimestampMicros`](../operations/parquet_variant.variant.Variant.md#op-3aa28f8ffd1710c804e45b6d) variants,
`None` for other variants.

# Examples

```
use parquet_variant::Variant;
use chrono::NaiveDate;

// you can extract a DateTime<Utc> from a UTC-adjusted variant
let datetime = NaiveDate::from_ymd_opt(2025, 4, 16)
    .unwrap()
    .and_hms_milli_opt(12, 34, 56, 780)
    .unwrap()
    .and_utc();
let v1 = Variant::from(datetime);
assert_eq!(v1.as_timestamp_micros(), Some(datetime));

// but not for other variants.
let datetime_nanos = NaiveDate::from_ymd_opt(2025, 8, 14)
    .unwrap()
    .and_hms_nano_opt(12, 33, 54, 123456789)
    .unwrap()
    .and_utc();
let v2 = Variant::from(datetime_nanos);
assert_eq!(v2.as_timestamp_micros(), None);
```

<a id="op-7efec2603f4fc09b08b34258"></a>
## as_timestamp_nanos

`function` · `parquet_variant::variant::Variant::as_timestamp_nanos` · parquet-variant 59.3.0

```rust
fn as_timestamp_nanos(&self) -> Option<DateTime<Utc>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [337, 1], "end": [1646, 2], "filename": "src/variant.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant.rs:715`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Converts this variant to a `DateTime<Utc>` if possible.

Returns `Some(DateTime<Utc>)` for timestamp variants,
`None` for other variants.

# Examples

```
use parquet_variant::Variant;
use chrono::NaiveDate;

// you can extract a DateTime<Utc> from a UTC-adjusted nanosecond-precision variant
let datetime = NaiveDate::from_ymd_opt(2025, 4, 16)
    .unwrap()
    .and_hms_nano_opt(12, 34, 56, 789123456)
    .unwrap()
    .and_utc();
let v1 = Variant::from(datetime);
assert_eq!(v1.as_timestamp_nanos(), Some(datetime));

// or from UTC-adjusted microsecond-precision variant
let datetime_micros = NaiveDate::from_ymd_opt(2025, 8, 14)
    .unwrap()
    .and_hms_milli_opt(12, 33, 54, 123)
    .unwrap()
    .and_utc();
// this will convert to `Variant::TimestampMicros`.
let v2 = Variant::from(datetime_micros);
assert_eq!(v2.as_timestamp_nanos(), Some(datetime_micros));

// but not for other variants.
let v3 = Variant::from("hello!");
assert_eq!(v3.as_timestamp_nanos(), None);
```

<a id="op-1efad007a102f853d4e37b5d"></a>
## as_timestamp_ntz_micros

`function` · `parquet_variant::variant::Variant::as_timestamp_ntz_micros` · parquet-variant 59.3.0

```rust
fn as_timestamp_ntz_micros(&self) -> Option<NaiveDateTime>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [337, 1], "end": [1646, 2], "filename": "src/variant.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant.rs:674`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Converts this variant to a `NaiveDateTime` if possible.

Returns `Some(NaiveDateTime)` for [`Variant::TimestampNtzMicros`](../operations/parquet_variant.variant.Variant.md#op-5c9def4bc3a62ab643cac433) variants,
`None` for other variants.

# Examples

```
use parquet_variant::Variant;
use chrono::NaiveDate;

// you can extract a NaiveDateTime from a non-UTC-adjusted variant
let datetime = NaiveDate::from_ymd_opt(2025, 4, 16)
    .unwrap()
    .and_hms_milli_opt(12, 34, 56, 780)
    .unwrap();
let v1 = Variant::from(datetime);
assert_eq!(v1.as_timestamp_ntz_micros(), Some(datetime));

// but not for other variants.
let datetime_nanos = NaiveDate::from_ymd_opt(2025, 8, 14)
    .unwrap()
    .and_hms_nano_opt(12, 33, 54, 123456789)
    .unwrap();
let v2 = Variant::from(datetime_nanos);
assert_eq!(v2.as_timestamp_micros(), None);
```

<a id="op-b043fdd97282b6012c0b6ab6"></a>
## as_timestamp_ntz_nanos

`function` · `parquet_variant::variant::Variant::as_timestamp_ntz_nanos` · parquet-variant 59.3.0

```rust
fn as_timestamp_ntz_nanos(&self) -> Option<NaiveDateTime>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [337, 1], "end": [1646, 2], "filename": "src/variant.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant.rs:754`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Converts this variant to a `NaiveDateTime` if possible.

Returns `Some(NaiveDateTime)` for timestamp variants,
`None` for other variants.

# Examples

```
use parquet_variant::Variant;
use chrono::NaiveDate;

// you can extract a NaiveDateTime from a non-UTC-adjusted variant
let datetime = NaiveDate::from_ymd_opt(2025, 4, 16)
    .unwrap()
    .and_hms_nano_opt(12, 34, 56, 789123456)
    .unwrap();
let v1 = Variant::from(datetime);
assert_eq!(v1.as_timestamp_ntz_nanos(), Some(datetime));

// or from a microsecond-precision non-UTC-adjusted variant
let datetime_micros = NaiveDate::from_ymd_opt(2025, 8, 14)
    .unwrap()
    .and_hms_milli_opt(12, 33, 54, 123)
    .unwrap();
// this will convert to `Variant::TimestampMicros`.
let v2 = Variant::from(datetime_micros);
assert_eq!(v2.as_timestamp_ntz_nanos(), Some(datetime_micros));

// but not for other variants.
let v3 = Variant::from("hello!");
assert_eq!(v3.as_timestamp_ntz_nanos(), None);
```

<a id="op-ab630bad0b38f50cde85494c"></a>
## as_u16

`function` · `parquet_variant::variant::Variant::as_u16` · parquet-variant 59.3.0

```rust
fn as_u16(&self) -> Option<u16>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [337, 1], "end": [1646, 2], "filename": "src/variant.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant.rs:1098`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Converts this variant to an `u16` if possible.

Returns `Some(u16)` for boolean and numeric variants(integers, floating-point,
and decimals with scale 0) that fit in `u16` range
`None` for other variants or values that would overflow.

# Examples

```
 use parquet_variant::{Variant, VariantDecimal4};

 // you can read an int64 variant into an u16
 let v1 = Variant::from(123i64);
 assert_eq!(v1.as_u16(), Some(123u16));

 // or a Decimal4 with scale 0 into u8
 let d = VariantDecimal4::try_new(u16::MAX as i32, 0).unwrap();
 let v2 = Variant::from(d);
 assert_eq!(v2.as_u16(), Some(u16::MAX));

 // or a variant that decimal with scale not equal to zero
 let d = VariantDecimal4::try_new(123, 2).unwrap();
 let v3 = Variant::from(d);
 assert_eq!(v3.as_u16(), Some(1));

// or from boolean variant
let v4= Variant::BooleanFalse;
assert_eq!(v4.as_u16(), Some(0));

 // but not a variant that can't fit into the range
 let v5 = Variant::from(-1);
 assert_eq!(v5.as_u16(), None);

 // or not a variant that cannot be cast into an integer
 let v6 = Variant::from("hello!");
 assert_eq!(v6.as_u16(), None);
```

<a id="op-abdafb9650b7bced2cc2b241"></a>
## as_u32

`function` · `parquet_variant::variant::Variant::as_u32` · parquet-variant 59.3.0

```rust
fn as_u32(&self) -> Option<u32>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [337, 1], "end": [1646, 2], "filename": "src/variant.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant.rs:1139`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Converts this variant to an `u32` if possible.

Returns `Some(u32)` for boolean and numeric variants(integers, floating-point,
and decimals with scale 0) that fit in `u32` range
`None` for other variants or values that would overflow.

# Examples

```
 use parquet_variant::{Variant, VariantDecimal8};

 // you can read an int64 variant into an u32
 let v1 = Variant::from(123i64);
 assert_eq!(v1.as_u32(), Some(123u32));

 // or a Decimal4 with scale 0 into u8
 let d = VariantDecimal8::try_new(u32::MAX as i64, 0).unwrap();
 let v2 = Variant::from(d);
 assert_eq!(v2.as_u32(), Some(u32::MAX));

 // or a variant that decimal with scale not equal to zero
 let d = VariantDecimal8::try_new(123, 2).unwrap();
 let v3 = Variant::from(d);
 assert_eq!(v3.as_u32(), Some(1));

// or from boolean variant
let v4 = Variant::BooleanFalse;
assert_eq!(v4.as_u32(), Some(0));

 // but not a variant that can't fit into the range
 let v5 = Variant::from(-1);
 assert_eq!(v5.as_u32(), None);

 // or not a variant that cannot be cast into an integer
 let v6 = Variant::from("hello!");
 assert_eq!(v6.as_u32(), None);
```

<a id="op-21279685d3d7efd35e876d0a"></a>
## as_u64

`function` · `parquet_variant::variant::Variant::as_u64` · parquet-variant 59.3.0

```rust
fn as_u64(&self) -> Option<u64>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [337, 1], "end": [1646, 2], "filename": "src/variant.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant.rs:1180`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Converts this variant to an `u64` if possible.

Returns `Some(u64)` for boolean and numeric variants(integers, floating-point,
and decimals with scale 0) that fit in `u64` range
`None` for other variants or values that would overflow.

# Examples

```
 use parquet_variant::{Variant, VariantDecimal16};

 // you can read an int64 variant into an u64
 let v1 = Variant::from(123i64);
 assert_eq!(v1.as_u64(), Some(123u64));

 // or a Decimal16 with scale 0 into u8
 let d = VariantDecimal16::try_new(u64::MAX as i128, 0).unwrap();
 let v2 = Variant::from(d);
 assert_eq!(v2.as_u64(), Some(u64::MAX));

 // or a variant that decimal with scale not equal to zero
let d = VariantDecimal16::try_new(123, 2).unwrap();
 let v3 = Variant::from(d);
 assert_eq!(v3.as_u64(), Some(1));

// or from boolean variant
let v4 = Variant::BooleanFalse;
assert_eq!(v4.as_u64(), Some(0));

 // but not a variant that can't fit into the range
 let v5 = Variant::from(-1);
 assert_eq!(v5.as_u64(), None);

 // or not a variant that cannot be cast into an integer
 let v6 = Variant::from("hello!");
 assert_eq!(v6.as_u64(), None);
```

<a id="op-a51652df3035f9e6b5563b22"></a>
## as_u8

`function` · `parquet_variant::variant::Variant::as_u8` · parquet-variant 59.3.0

```rust
fn as_u8(&self) -> Option<u8>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [337, 1], "end": [1646, 2], "filename": "src/variant.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant.rs:1057`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Converts this variant to a `u8` if possible.

Returns `Some(u8)` for boolean and numeric variants(integers, floating-point,
and decimals with scale 0) that fit in `u8` range
`None` for other variants or values that would overflow.

# Examples

```
 use parquet_variant::{Variant, VariantDecimal4};

 // you can read an int64 variant into an u8
 let v1 = Variant::from(123i64);
 assert_eq!(v1.as_u8(), Some(123u8));

 // or a Decimal4 with scale 0 into u8
 let d = VariantDecimal4::try_new(26, 0).unwrap();
 let v2 = Variant::from(d);
 assert_eq!(v2.as_u8(), Some(26u8));

 // or a variant that decimal with scale not equal to zero
 let d = VariantDecimal4::try_new(123, 2).unwrap();
 let v3 = Variant::from(d);
 assert_eq!(v3.as_u8(), Some(1));

// or from boolean variant
let v4 = Variant::BooleanFalse;
assert_eq!(v4.as_u8(), Some(0));

 // but not a variant that can't fit into the range
 let v5 = Variant::from(-1);
 assert_eq!(v5.as_u8(), None);

 // or not a variant that cannot be cast into an integer
 let v6 = Variant::from("hello!");
 assert_eq!(v6.as_u8(), None);
```

<a id="op-7d823435cd40e4134fad629c"></a>
## as_u8_slice

`function` · `parquet_variant::variant::Variant::as_u8_slice` · parquet-variant 59.3.0

```rust
fn as_u8_slice(&'v self) -> Option<&'v [u8]>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [337, 1], "end": [1646, 2], "filename": "src/variant.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant.rs:780`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Converts this variant to a `&[u8]` if possible.

Returns `Some(&[u8])` for binary variants,
`None` for non-binary variants.

# Examples

```
use parquet_variant::Variant;

// you can extract a byte slice from a binary variant
let data = b"hello!";
let v1 = Variant::Binary(data);
assert_eq!(v1.as_u8_slice(), Some(data.as_slice()));

// but not from other variant types
let v2 = Variant::from(123i64);
assert_eq!(v2.as_u8_slice(), None);
```

<a id="op-11ab58ae7b12661d3e336c6a"></a>
## as_uuid

`function` · `parquet_variant::variant::Variant::as_uuid` · parquet-variant 59.3.0

```rust
fn as_uuid(&self) -> Option<Uuid>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [337, 1], "end": [1646, 2], "filename": "src/variant.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant.rs:833`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Converts this variant to a `uuid hyphenated string` if possible.

Returns `Some(String)` for UUID variants, `None` for non-UUID variants.

# Examples

```
use parquet_variant::Variant;

// You can extract a UUID from a UUID variant
let s = uuid::Uuid::parse_str("67e55044-10b1-426f-9247-bb680e5fe0c8").unwrap();
let v1 = Variant::Uuid(s);
assert_eq!(s, v1.as_uuid().unwrap());
assert_eq!("67e55044-10b1-426f-9247-bb680e5fe0c8", v1.as_uuid().unwrap().to_string());

//but not from other variants
let v2 = Variant::from(1234);
assert_eq!(None, v2.as_uuid())
```

<a id="op-9bfaa77db6a3f84ee759b687"></a>
## clone

`function` · `parquet_variant::variant::Variant::clone` · parquet-variant 59.3.0

```rust
fn clone(&self) -> Variant<'m, 'v>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [246, 10], "end": [246, 15], "filename": "src/variant.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/variant.rs:246`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7623ee8f5237fc54178a2dca"></a>
## eq

`function` · `parquet_variant::variant::Variant::eq` · parquet-variant 59.3.0

```rust
fn eq(&self, other: &Variant<'m, 'v>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [246, 17], "end": [246, 26], "filename": "src/variant.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/variant.rs:246`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6a794faf9a463f9fa7f08e55"></a>
## fmt

`function` · `parquet_variant::variant::Variant::fmt` · parquet-variant 59.3.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"lifetime": "'_"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1875, 1], "end": [1924, 2], "filename": "src/variant.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/variant.rs:1876`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0102b40b90e2de9767cd943c"></a>
## from

`function` · `parquet_variant::variant::Variant::from` · parquet-variant 59.3.0

```rust
fn from(value: Uuid) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"lifetime": "'_"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1805, 1], "end": [1809, 2], "filename": "src/variant.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "uuid::Uuid", "path": "Uuid"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/variant.rs:1806`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-03ca66b10aade2c24ce821e5"></a>
## from

`function` · `parquet_variant::variant::Variant::from` · parquet-variant 59.3.0

```rust
fn from(value: half::f16) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"lifetime": "'_"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1749, 1], "end": [1753, 2], "filename": "src/variant.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "half::binary16::f16", "path": "f16"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/variant.rs:1750`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-044f15d3b32535f189dac9b6"></a>
## from

`function` · `parquet_variant::variant::Variant::from` · parquet-variant 59.3.0

```rust
fn from(value: &'v [u8]) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1793, 1], "end": [1797, 2], "filename": "src/variant.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'v", "type": {"slice": {"primitive": "u8"}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/variant.rs:1794`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06fab6f10e212d4c6db47734"></a>
## from

`function` · `parquet_variant::variant::Variant::from` · parquet-variant 59.3.0

```rust
fn from(value: f64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"lifetime": "'_"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1761, 1], "end": [1765, 2], "filename": "src/variant.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "f64"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/variant.rs:1762`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-13e23956f02df0ee3127715c"></a>
## from

`function` · `parquet_variant::variant::Variant::from` · parquet-variant 59.3.0

```rust
fn from(value: VariantDecimal16) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"lifetime": "'_"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1743, 1], "end": [1747, 2], "filename": "src/variant.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal16", "path": "VariantDecimal16"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/variant.rs:1744`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-144f2b455c00e68e31010972"></a>
## from

`function` · `parquet_variant::variant::Variant::from` · parquet-variant 59.3.0

```rust
fn from(value: f32) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"lifetime": "'_"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1755, 1], "end": [1759, 2], "filename": "src/variant.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "f32"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/variant.rs:1756`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-150ffb42854f00c633ca3446"></a>
## from

`function` · `parquet_variant::variant::Variant::from` · parquet-variant 59.3.0

```rust
fn from(value: i64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"lifetime": "'_"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1681, 1], "end": [1685, 2], "filename": "src/variant.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i64"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/variant.rs:1682`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1a66bda0e5f27deb8e3f82ce"></a>
## from

`function` · `parquet_variant::variant::Variant::from` · parquet-variant 59.3.0

```rust
fn from(value: i8) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"lifetime": "'_"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1663, 1], "end": [1667, 2], "filename": "src/variant.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i8"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/variant.rs:1664`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-30680d7fb52cbd4627eca81b"></a>
## from

`function` · `parquet_variant::variant::Variant::from` · parquet-variant 59.3.0

```rust
fn from(value: VariantDecimal4) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"lifetime": "'_"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1731, 1], "end": [1735, 2], "filename": "src/variant.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal4", "path": "VariantDecimal4"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/variant.rs:1732`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3b0a3581a9afd79e07990fc1"></a>
## from

`function` · `parquet_variant::variant::Variant::from` · parquet-variant 59.3.0

```rust
fn from(value: DateTime<Utc>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"lifetime": "'_"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1773, 1], "end": [1781, 2], "filename": "src/variant.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "chrono::offset::utc::Utc", "path": "Utc"}}}], "constraints": []}}, "id": "chrono::datetime::DateTime", "path": "DateTime"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/variant.rs:1774`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3c69032db372399d465d2b55"></a>
## from

`function` · `parquet_variant::variant::Variant::from` · parquet-variant 59.3.0

```rust
fn from(value: i32) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"lifetime": "'_"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1675, 1], "end": [1679, 2], "filename": "src/variant.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/variant.rs:1676`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4221dcb30ff4f453f4376573"></a>
## from

`function` · `parquet_variant::variant::Variant::from` · parquet-variant 59.3.0

```rust
fn from(value: NaiveDate) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"lifetime": "'_"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1767, 1], "end": [1771, 2], "filename": "src/variant.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "chrono::naive::date::NaiveDate", "path": "NaiveDate"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/variant.rs:1768`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6157dd2d2f23aa0df43be999"></a>
## from

`function` · `parquet_variant::variant::Variant::from` · parquet-variant 59.3.0

```rust
fn from(value: u8) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"lifetime": "'_"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1687, 1], "end": [1696, 2], "filename": "src/variant.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "u8"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/variant.rs:1688`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c96b007581364a0de83c570"></a>
## from

`function` · `parquet_variant::variant::Variant::from` · parquet-variant 59.3.0

```rust
fn from(value: &'v str) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1811, 1], "end": [1819, 2], "filename": "src/variant.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'v", "type": {"primitive": "str"}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/variant.rs:1812`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-79e5c1134def2d29dfb40f12"></a>
## from

`function` · `parquet_variant::variant::Variant::from` · parquet-variant 59.3.0

```rust
fn from(value: u32) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"lifetime": "'_"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1708, 1], "end": [1717, 2], "filename": "src/variant.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "u32"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/variant.rs:1709`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-85559d902a5e383a5480f3f9"></a>
## from

`function` · `parquet_variant::variant::Variant::from` · parquet-variant 59.3.0

```rust
fn from(value: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"lifetime": "'_"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1654, 1], "end": [1661, 2], "filename": "src/variant.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "bool"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/variant.rs:1655`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-913d27da3ec0328326c281e2"></a>
## from

`function` · `parquet_variant::variant::Variant::from` · parquet-variant 59.3.0

```rust
fn from((): ()) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"lifetime": "'_"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1648, 1], "end": [1652, 2], "filename": "src/variant.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"tuple": []}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/variant.rs:1649`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d6cca8d5dc90db093175c10"></a>
## from

`function` · `parquet_variant::variant::Variant::from` · parquet-variant 59.3.0

```rust
fn from(value: i16) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"lifetime": "'_"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1669, 1], "end": [1673, 2], "filename": "src/variant.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i16"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/variant.rs:1670`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aba4f6e1a55fb2d2556abbfd"></a>
## from

`function` · `parquet_variant::variant::Variant::from` · parquet-variant 59.3.0

```rust
fn from(value: VariantDecimal8) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"lifetime": "'_"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1737, 1], "end": [1741, 2], "filename": "src/variant.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "parquet_variant::variant::decimal::VariantDecimal8", "path": "VariantDecimal8"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/variant.rs:1738`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c1506e90d130979619c9e6fd"></a>
## from

`function` · `parquet_variant::variant::Variant::from` · parquet-variant 59.3.0

```rust
fn from(value: u16) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"lifetime": "'_"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1698, 1], "end": [1707, 2], "filename": "src/variant.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "u16"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/variant.rs:1699`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d349fe71422996cf6bfa137c"></a>
## from

`function` · `parquet_variant::variant::Variant::from` · parquet-variant 59.3.0

```rust
fn from(value: NaiveTime) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"lifetime": "'_"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1799, 1], "end": [1803, 2], "filename": "src/variant.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "chrono::naive::time::NaiveTime", "path": "NaiveTime"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/variant.rs:1800`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea5f26e5734f2933a6b8cc44"></a>
## from

`function` · `parquet_variant::variant::Variant::from` · parquet-variant 59.3.0

```rust
fn from(value: NaiveDateTime) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"lifetime": "'_"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1783, 1], "end": [1791, 2], "filename": "src/variant.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "chrono::naive::datetime::NaiveDateTime", "path": "NaiveDateTime"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/variant.rs:1784`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f4ee1e68e97555b24fc78461"></a>
## from

`function` · `parquet_variant::variant::Variant::from` · parquet-variant 59.3.0

```rust
fn from(value: u64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"lifetime": "'_"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1719, 1], "end": [1729, 2], "filename": "src/variant.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "u64"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/variant.rs:1720`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e68dab604bfbb363de2d1814"></a>
## get_list_element

`function` · `parquet_variant::variant::Variant::get_list_element` · parquet-variant 59.3.0

```rust
fn get_list_element(&self, index: usize) -> Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [337, 1], "end": [1646, 2], "filename": "src/variant.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant.rs:1591`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

If this is a list and the requested index is in bounds, retrieves the corresponding
element. Otherwise, returns None.

This is shorthand for [`Self::as_list`](../operations/parquet_variant.variant.Variant.md#op-dd7d161e5c10ce915e5b58cf) followed by [`VariantList::get`](../operations/parquet_variant.variant.list.VariantList.md#op-25d78e3beb6960aa905df29d).

# Examples
```
# use parquet_variant::{Variant, VariantBuilder, VariantList};
# let mut builder = VariantBuilder::new();
# let mut list = builder.new_list();
# list.append_value("John");
# list.append_value("Doe");
# list.finish();
# let (metadata, value) = builder.finish();
// list that is ["John", "Doe"]
let variant = Variant::new(&metadata, &value);
// use the `get_list_element` method to access the list
assert_eq!(variant.get_list_element(0), Some(Variant::from("John")));
assert_eq!(variant.get_list_element(1), Some(Variant::from("Doe")));
assert!(variant.get_list_element(2).is_none());
```

<a id="op-cfdce4303d24c3a54825861b"></a>
## get_object_field

`function` · `parquet_variant::variant::Variant::get_object_field` · parquet-variant 59.3.0

```rust
fn get_object_field(&self, field_name: &str) -> Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [337, 1], "end": [1646, 2], "filename": "src/variant.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant.rs:1501`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

If this is an object and the requested field name exists, retrieves the corresponding field
value. Otherwise, returns None.

This is shorthand for [`Self::as_object`](../operations/parquet_variant.variant.Variant.md#op-49a9fec07832505931bda83c) followed by [`VariantObject::get`](../operations/parquet_variant.variant.object.VariantObject.md#op-750e05c8a1b9ba9029bcb186).

# Examples
```
# use parquet_variant::{Variant, VariantBuilder, VariantObject};
# let mut builder = VariantBuilder::new();
# let mut obj = builder.new_object();
# obj.insert("name", "John");
# obj.finish();
# let (metadata, value) = builder.finish();
// object that is {"name": "John"}
 let variant = Variant::new(&metadata, &value);
// use the `get_object_field` method to access the object
let obj = variant.get_object_field("name");
assert_eq!(obj, Some(Variant::from("John")));
let obj = variant.get_object_field("foo");
assert!(obj.is_none());
```

<a id="op-c50232f473c847799463d621"></a>
## get_path

`function` · `parquet_variant::variant::Variant::get_path` · parquet-variant 59.3.0

```rust
fn get_path(&self, path: &VariantPath<'_>) -> Option<Variant<'_, '_>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [337, 1], "end": [1646, 2], "filename": "src/variant.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant.rs:1639`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Return a new Variant with the path followed.

If the path is not found, `None` is returned.

# Example
```
# use parquet_variant::{Variant, VariantBuilder, VariantObject, VariantPath};
# let mut builder = VariantBuilder::new();
# let mut obj = builder.new_object();
# let mut list = obj.new_list("foo");
# list.append_value("bar");
# list.append_value("baz");
# list.finish();
# obj.finish();
# let (metadata, value) = builder.finish();
// given a variant like `{"foo": ["bar", "baz"]}`
let variant = Variant::new(&metadata, &value);
// Accessing a non existent path returns None
assert_eq!(variant.get_path(&VariantPath::try_from("non_existent").unwrap()), None);
// Access obj["foo"]
let path = VariantPath::try_from("foo").unwrap();
let foo = variant.get_path(&path).expect("field `foo` should exist");
assert!(foo.as_list().is_some(), "field `foo` should be a list");
// Access foo[0]
let path = VariantPath::from(0);
let bar = foo.get_path(&path).expect("element 0 should exist");
// bar is a string
assert_eq!(bar.as_string(), Some("bar"));
// You can also access nested paths
let path = VariantPath::try_from("foo").unwrap().join(0);
assert_eq!(variant.get_path(&path).unwrap(), bar);
```

<a id="op-a4462df771cbb728d0ba530c"></a>
## is_fully_validated

`function` · `parquet_variant::variant::Variant::is_fully_validated` · parquet-variant 59.3.0

```rust
fn is_fully_validated(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [337, 1], "end": [1646, 2], "filename": "src/variant.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant.rs:487`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

True if this variant instance has already been [validated].

[validated]: Self#Validation

<a id="op-492b23db38b3b872affde77f"></a>
## metadata

`function` · `parquet_variant::variant::Variant::metadata` · parquet-variant 59.3.0

```rust
fn metadata(&self) -> &VariantMetadata<'m>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [337, 1], "end": [1646, 2], "filename": "src/variant.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant.rs:1599`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Return the metadata dictionary associated with this variant value.

<a id="op-69dc8ecd853e71e63a220c10"></a>
## new

`function` · `parquet_variant::variant::Variant::new` · parquet-variant 59.3.0

```rust
fn new(metadata: &'m [u8], value: &'v [u8]) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [337, 1], "end": [1646, 2], "filename": "src/variant.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant.rs:377`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Attempts to interpret a metadata and value buffer pair as a new `Variant`.

The instance is [unvalidated].

# Example
```
use parquet_variant::{Variant, VariantMetadata};
let metadata = [0x01, 0x00, 0x00];
let value = [0x09, 0x48, 0x49];
// parse the header metadata
assert_eq!(
  Variant::from("HI"),
  Variant::new(&metadata, &value)
);
```

[unvalidated]: Self#Validation

<a id="op-5031210bdfee520209cb447b"></a>
## new_with_metadata

`function` · `parquet_variant::variant::Variant::new_with_metadata` · parquet-variant 59.3.0

```rust
fn new_with_metadata(metadata: VariantMetadata<'m>, value: &'v [u8]) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [337, 1], "end": [1646, 2], "filename": "src/variant.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant.rs:412`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Similar to [`Self::try_new_with_metadata`](../operations/parquet_variant.variant.Variant.md#op-4ad50695a7b49b85d98239ed), but [unvalidated].

[unvalidated]: Self#Validation

<a id="op-226554b5ee767c11796e37d1"></a>
## try_from

`function` · `parquet_variant::variant::Variant::try_from` · parquet-variant 59.3.0

```rust
fn try_from(value: (i64, u8)) -> Result<Self, Self::Error>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"lifetime": "'_"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1831, 1], "end": [1839, 2], "filename": "src/variant.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"tuple": [{"primitive": "i64"}, {"primitive": "u8"}]}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/variant.rs:1834`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3fb7957dac71eff5812a8d7f"></a>
## try_from

`function` · `parquet_variant::variant::Variant::try_from` · parquet-variant 59.3.0

```rust
fn try_from(value: (i32, u8)) -> Result<Self, Self::Error>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"lifetime": "'_"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1821, 1], "end": [1829, 2], "filename": "src/variant.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"tuple": [{"primitive": "i32"}, {"primitive": "u8"}]}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/variant.rs:1824`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7ffb4339e29ac9e860e5e3b9"></a>
## try_from

`function` · `parquet_variant::variant::Variant::try_from` · parquet-variant 59.3.0

```rust
fn try_from(value: (i128, u8)) -> Result<Self, Self::Error>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"lifetime": "'_"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1841, 1], "end": [1849, 2], "filename": "src/variant.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"tuple": [{"primitive": "i128"}, {"primitive": "u8"}]}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/variant.rs:1844`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-db76763b87893e42f4db1c97"></a>
## try_new

`function` · `parquet_variant::variant::Variant::try_new` · parquet-variant 59.3.0

```rust
fn try_new(metadata: &'m [u8], value: &'v [u8]) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [337, 1], "end": [1646, 2], "filename": "src/variant.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant.rs:355`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Attempts to interpret a metadata and value buffer pair as a new `Variant`.

The instance is fully [validated].

# Example
```
use parquet_variant::{Variant, VariantMetadata};
let metadata = [0x01, 0x00, 0x00];
let value = [0x09, 0x48, 0x49];
// parse the header metadata
assert_eq!(
  Variant::from("HI"),
  Variant::try_new(&metadata, &value).unwrap()
);
```

[validated]: Self#Validation

<a id="op-4ad50695a7b49b85d98239ed"></a>
## try_new_with_metadata

`function` · `parquet_variant::variant::Variant::try_new_with_metadata` · parquet-variant 59.3.0

```rust
fn try_new_with_metadata(metadata: VariantMetadata<'m>, value: &'v [u8]) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [337, 1], "end": [1646, 2], "filename": "src/variant.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant.rs:402`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Create a new variant with existing metadata.

The instance is fully [validated].

# Example
```
# use parquet_variant::{Variant, VariantMetadata};
let metadata = [0x01, 0x00, 0x00];
let value = [0x09, 0x48, 0x49];
// parse the header metadata first
let metadata = VariantMetadata::new(&metadata);
assert_eq!(
  Variant::from("HI"),
  Variant::try_new_with_metadata(metadata, &value).unwrap()
);
```

[validated]: Self#Validation

<a id="op-353568855fd0414171c82a71"></a>
## with_full_validation

`function` · `parquet_variant::variant::Variant::with_full_validation` · parquet-variant 59.3.0

```rust
fn with_full_validation(self) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [337, 1], "end": [1646, 2], "filename": "src/variant.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant.rs:506`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Recursively validates this variant value, ensuring that infallible access will not panic due
to invalid bytes.

Variant leaf values are always valid by construction, but [objects] and [arrays] can be
constructed in unvalidated (and potentially invalid) state.

If [`Self::is_fully_validated`](../operations/parquet_variant.variant.Variant.md#op-a4462df771cbb728d0ba530c) is true, validation is a no-op. Otherwise, the cost is `O(m + v)`
where `m` and `v` are the sizes of metadata and value buffers, respectively.

[objects]: VariantObject#Validation
[arrays]: VariantList#Validation
