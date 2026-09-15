# `parquet_variant::variant`

Crate `parquet-variant` · 2 public items · structured records in [`model/parquet_variant.variant.json`](../model/parquet_variant.variant.json)

## Variant

`enum` · `parquet_variant::variant::Variant`

```rust
enum Variant<'m, 'v>
```

**Variants**: `Null`, `Int8`, `Int16`, `Int32`, `Int64`, `Date`, `TimestampMicros`, `TimestampNtzMicros`, `TimestampNanos`, `TimestampNtzNanos`, `Decimal4`, `Decimal8`, `Decimal16`, `Float`, `Double`, `BooleanTrue`, `BooleanFalse`, `Binary`, `String`, `Time`, `Uuid`, `ShortString`, `Object`, `List`

**Implements**: `core::convert::From`, `core::convert::TryFrom`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (37)

```rust
fn as_boolean(&self) -> Option<bool>
fn as_decimal16(&self) -> Option<VariantDecimal16>
fn as_decimal4(&self) -> Option<VariantDecimal4>
fn as_decimal8(&self) -> Option<VariantDecimal8>
fn as_f16(&self) -> Option<f16>
fn as_f32(&self) -> Option<f32>
fn as_f64(&self) -> Option<f64>
fn as_int16(&self) -> Option<i16>
fn as_int32(&self) -> Option<i32>
fn as_int64(&self) -> Option<i64>
fn as_int8(&self) -> Option<i8>
fn as_list(&'m self) -> Option<&'m VariantList<'m, 'v>>
fn as_naive_date(&self) -> Option<NaiveDate>
fn as_null(&self) -> Option<()>
fn as_object(&'m self) -> Option<&'m VariantObject<'m, 'v>>
fn as_string(&'v self) -> Option<&'v str>
fn as_time_utc(&'m self) -> Option<NaiveTime>
fn as_timestamp_micros(&self) -> Option<DateTime<Utc>>
fn as_timestamp_nanos(&self) -> Option<DateTime<Utc>>
fn as_timestamp_ntz_micros(&self) -> Option<NaiveDateTime>
fn as_timestamp_ntz_nanos(&self) -> Option<NaiveDateTime>
fn as_u16(&self) -> Option<u16>
fn as_u32(&self) -> Option<u32>
fn as_u64(&self) -> Option<u64>
fn as_u8(&self) -> Option<u8>
fn as_u8_slice(&'v self) -> Option<&'v [u8]>
fn as_uuid(&self) -> Option<Uuid>
fn get_list_element(&self, index: usize) -> Option<Self>
fn get_object_field(&self, field_name: &str) -> Option<Self>
fn get_path(&self, path: &VariantPath<'_>) -> Option<Variant<'_, '_>>
fn is_fully_validated(&self) -> bool
fn metadata(&self) -> &VariantMetadata<'m>
fn new(metadata: &'m [u8], value: &'v [u8]) -> Self
fn new_with_metadata(metadata: VariantMetadata<'m>, value: &'v [u8]) -> Self
fn try_new(metadata: &'m [u8], value: &'v [u8]) -> Result<Self, ArrowError>
fn try_new_with_metadata(metadata: VariantMetadata<'m>, value: &'v [u8]) -> Result<Self, ArrowError>
fn with_full_validation(self) -> Result<Self, ArrowError>
```

**via `core::convert::From`**

```rust
fn from(value: f32) -> Self
fn from(value: u64) -> Self
fn from(value: u16) -> Self
fn from(value: i64) -> Self
fn from(value: i16) -> Self
fn from(value: bool) -> Self
fn from(value: VariantDecimal16) -> Self
fn from(value: Uuid) -> Self
fn from(value: &'v [u8]) -> Self
fn from(value: DateTime<Utc>) -> Self
fn from(value: f64) -> Self
fn from(value: half::f16) -> Self
fn from(value: u32) -> Self
fn from(value: u8) -> Self
fn from(value: i32) -> Self
fn from(value: i8) -> Self
fn from((): ()) -> Self
fn from(value: &'v str) -> Self
fn from(value: VariantDecimal4) -> Self
fn from(value: NaiveTime) -> Self
fn from(value: VariantDecimal8) -> Self
fn from(value: NaiveDateTime) -> Self
fn from(value: NaiveDate) -> Self
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: (i32, u8)) -> Result<Self, Self::Error>
fn try_from(value: (i64, u8)) -> Result<Self, Self::Error>
fn try_from(value: (i128, u8)) -> Result<Self, Self::Error>
```

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
[`num_cast`], [`cast_num_to_bool`], [`single_bool_to_numeric`], and
[`cast_single_string_to_boolean_default`].

- [`Self::as_boolean`] accepts boolean, numeric, and string variants.
  Numeric zero maps to `false`; non-zero maps to `true`. String parsing follows
  Arrow UTF8-to-boolean cast rules.
- Numeric accessors such as [`Self::as_int8`], [`Self::as_int64`], [`Self::as_u8`],
  [`Self::as_u64`], [`Self::as_f16`], [`Self::as_f32`], and [`Self::as_f64`] accept
  boolean and numeric variants (integers, floating-point, and decimals).
  They return `None` when conversion is not possible.
- Decimal accessors such as [`Self::as_decimal4`], [`Self::as_decimal8`], and
  [`Self::as_decimal16`] accept compatible decimal variants, integer variants,
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

Instances produced by [`Self::try_new`], [`Self::try_new_with_metadata`], or [`Self::with_full_validation`]
are fully _validated_. They always contain _valid_ data, and infallible accesses such as
iteration and indexing are panic-free. The validation cost is `O(m + v)` where `m` and
`v` are the number of bytes in the metadata and value buffers, respectively.

Instances produced by [`Self::new`] and [`Self::new_with_metadata`] are _unvalidated_ and so
they may contain either _valid_ or _invalid_ data. Infallible accesses to variant objects and
arrays, such as iteration and indexing will panic if the underlying bytes are _invalid_, and
fallible alternatives are provided as panic-free alternatives. [`Self::with_full_validation`] can also be
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

---

## ShortString

`struct` · `parquet_variant::variant::ShortString`

```rust
struct ShortString<'a>
```

**Implements**: `core::convert::AsRef`, `core::convert::TryFrom`, `core::ops::deref::Deref`

**Derives**: Clone, Copy, Debug, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn as_str(&self) -> &'a str
fn try_new(value: &'a str) -> Result<Self, ArrowError>
```

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: &'a str) -> Result<Self, Self::Error>
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

A Variant [`ShortString`]

This implementation is a zero cost wrapper over `&str` that ensures
the length of the underlying string is a valid Variant short string (63 bytes or less)

---
