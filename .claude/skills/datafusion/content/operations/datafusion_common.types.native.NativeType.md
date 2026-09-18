# `datafusion_common::types::native::NativeType`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.types.native.NativeType.json).

<a id="op-73a442cf6fa53632ee0536d1"></a>
## NativeType

`enum` · `datafusion_common::types::native::NativeType` · datafusion-common 55.1.0

```rust
enum NativeType
```

Source: `src/types/native.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Representation of a type that DataFusion can handle natively. It is a subset
of the physical variants in Arrow's native [`DataType`](../operations/arrow_schema.datatype.DataType.md#op-bf69df5b14436e006d3a531c).

<a id="op-6b00816863efd6c1b29a52c4"></a>
## Binary

`variant` · `datafusion_common::types::native::NativeType::Binary` · datafusion-common 55.1.0

```rust
Binary
```

Source: `src/types/native.rs:148`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Opaque binary data of variable length.

<a id="op-1bbd9123f7edf22d1e196139"></a>
## Boolean

`variant` · `datafusion_common::types::native::NativeType::Boolean` · datafusion-common 55.1.0

```rust
Boolean
```

Source: `src/types/native.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A boolean type representing the values `true` and `false`.

<a id="op-2dd6b45d402b1a2c72ac6bab"></a>
## Date

`variant` · `datafusion_common::types::native::NativeType::Date` · datafusion-common 55.1.0

```rust
Date
```

Source: `src/types/native.rs:138`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A signed date representing the elapsed time since UNIX epoch (1970-01-01)
in days.

<a id="op-c6a5f35119870c74f5fa15f2"></a>
## Decimal

`variant` · `datafusion_common::types::native::NativeType::Decimal` · datafusion-common 55.1.0

```rust
Decimal
```

Source: `src/types/native.rs:175`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Decimal value with precision and scale

* precision is the total number of digits
* scale is the number of digits past the decimal

For example the number 123.45 has precision 5 and scale 2.

In certain situations, scale could be negative number. For
negative scale, it is the number of padding 0 to the right
of the digits.

For example the number 12300 could be treated as a decimal
has precision 3 and scale -2.

<a id="op-c5c20ec6ffdc711108f49cb2"></a>
## Duration

`variant` · `datafusion_common::types::native::NativeType::Duration` · datafusion-common 55.1.0

```rust
Duration
```

Source: `src/types/native.rs:142`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Measure of elapsed time in either seconds, milliseconds, microseconds or nanoseconds.

<a id="op-a7b3e3c30609acaced60fffd"></a>
## FixedSizeBinary

`variant` · `datafusion_common::types::native::NativeType::FixedSizeBinary` · datafusion-common 55.1.0

```rust
FixedSizeBinary
```

Source: `src/types/native.rs:151`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Opaque binary data of fixed size.
Enum parameter specifies the number of bytes per value.

<a id="op-7a524e7425b92198753e6ac0"></a>
## FixedSizeList

`variant` · `datafusion_common::types::native::NativeType::FixedSizeList` · datafusion-common 55.1.0

```rust
FixedSizeList
```

Source: `src/types/native.rs:157`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A list of some logical data type with fixed length.

<a id="op-6f783165105642973cd01336"></a>
## Float16

`variant` · `datafusion_common::types::native::NativeType::Float16` · datafusion-common 55.1.0

```rust
Float16
```

Source: `src/types/native.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A 16-bit floating point number.

<a id="op-e4dae58fc2531a4e9be15ce1"></a>
## Float32

`variant` · `datafusion_common::types::native::NativeType::Float32` · datafusion-common 55.1.0

```rust
Float32
```

Source: `src/types/native.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A 32-bit floating point number.

<a id="op-08fbba85c278777a4dfd04ff"></a>
## Float64

`variant` · `datafusion_common::types::native::NativeType::Float64` · datafusion-common 55.1.0

```rust
Float64
```

Source: `src/types/native.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A 64-bit floating point number.

<a id="op-759ee03e3863f95733b0dd86"></a>
## Int16

`variant` · `datafusion_common::types::native::NativeType::Int16` · datafusion-common 55.1.0

```rust
Int16
```

Source: `src/types/native.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A signed 16-bit integer.

<a id="op-d57aba57624a9da0f43149ac"></a>
## Int32

`variant` · `datafusion_common::types::native::NativeType::Int32` · datafusion-common 55.1.0

```rust
Int32
```

Source: `src/types/native.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A signed 32-bit integer.

<a id="op-0e82dc1ccbd6cc69140781b5"></a>
## Int64

`variant` · `datafusion_common::types::native::NativeType::Int64` · datafusion-common 55.1.0

```rust
Int64
```

Source: `src/types/native.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A signed 64-bit integer.

<a id="op-543fe9025a2aa6b20f80815b"></a>
## Int8

`variant` · `datafusion_common::types::native::NativeType::Int8` · datafusion-common 55.1.0

```rust
Int8
```

Source: `src/types/native.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A signed 8-bit integer.

<a id="op-5c407dec8bb1e4341cafd9f0"></a>
## Interval

`variant` · `datafusion_common::types::native::NativeType::Interval` · datafusion-common 55.1.0

```rust
Interval
```

Source: `src/types/native.rs:146`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A "calendar" interval which models types that don't necessarily
have a precise duration without the context of a base timestamp (e.g.
days can differ in length during day light savings time transitions).

<a id="op-6658235c7d61b0f0b211535f"></a>
## List

`variant` · `datafusion_common::types::native::NativeType::List` · datafusion-common 55.1.0

```rust
List
```

Source: `src/types/native.rs:155`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A list of some logical data type with variable length.

<a id="op-fc7483f9fe89555d385f8a7a"></a>
## Map

`variant` · `datafusion_common::types::native::NativeType::Map` · datafusion-common 55.1.0

```rust
Map
```

Source: `src/types/native.rs:184`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A Map is a type that an association between a key and a value.

The key and value types are not constrained, but keys should be
hashable and unique.

In a field with Map type, key type and the second the value type. The names of the
child fields may be respectively "entries", "key", and "value", but this is
not enforced.

<a id="op-059f41f8f036776f7e21cb33"></a>
## Null

`variant` · `datafusion_common::types::native::NativeType::Null` · datafusion-common 55.1.0

```rust
Null
```

Source: `src/types/native.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Null type

<a id="op-fae489522cf5fe03b2d94805"></a>
## String

`variant` · `datafusion_common::types::native::NativeType::String` · datafusion-common 55.1.0

```rust
String
```

Source: `src/types/native.rs:153`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A variable-length string in Unicode with UTF-8 encoding.

<a id="op-1d38f3be424615255a07a036"></a>
## Struct

`variant` · `datafusion_common::types::native::NativeType::Struct` · datafusion-common 55.1.0

```rust
Struct
```

Source: `src/types/native.rs:159`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A nested type that contains a number of sub-fields.

<a id="op-a05765288f8307710d763222"></a>
## Time

`variant` · `datafusion_common::types::native::NativeType::Time` · datafusion-common 55.1.0

```rust
Time
```

Source: `src/types/native.rs:140`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A signed time representing the elapsed time since midnight in the unit of `TimeUnit`.

<a id="op-b2707c710aa01a14261953c3"></a>
## Timestamp

`variant` · `datafusion_common::types::native::NativeType::Timestamp` · datafusion-common 55.1.0

```rust
Timestamp
```

Source: `src/types/native.rs:135`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A timestamp with an optional timezone.

Time is measured as a Unix epoch, counting the seconds from
00:00:00.000 on 1 January 1970, excluding leap seconds,
as a signed 64-bit integer.

The time zone is a string indicating the name of a time zone, one of:

* As used in the Olson time zone database (the "tz database" or
  "tzdata"), such as "America/New_York"
* An absolute time zone offset of the form +XX:XX or -XX:XX, such as +07:30

Timestamps with a non-empty timezone
------------------------------------

If a Timestamp column has a non-empty timezone value, its epoch is
1970-01-01 00:00:00 (January 1st 1970, midnight) in the *UTC* timezone
(the Unix epoch), regardless of the Timestamp's own timezone.

Therefore, timestamp values with a non-empty timezone correspond to
physical points in time together with some additional information about
how the data was obtained and/or how to display it (the timezone).

  For example, the timestamp value 0 with the timezone string "Europe/Paris"
  corresponds to "January 1st 1970, 00h00" in the UTC timezone, but the
  application may prefer to display it as "January 1st 1970, 01h00" in
  the Europe/Paris timezone (which is the same physical point in time).

One consequence is that timestamp values with a non-empty timezone
can be compared and ordered directly, since they all share the same
well-known point of reference (the Unix epoch).

Timestamps with an unset / empty timezone
-----------------------------------------

If a Timestamp column has no timezone value, its epoch is
1970-01-01 00:00:00 (January 1st 1970, midnight) in an *unknown* timezone.

Therefore, timestamp values without a timezone cannot be meaningfully
interpreted as physical points in time, but only as calendar / clock
indications ("wall clock time") in an unspecified timezone.

  For example, the timestamp value 0 with an empty timezone string
  corresponds to "January 1st 1970, 00h00" in an unknown timezone: there
  is not enough information to interpret it as a well-defined physical
  point in time.

One consequence is that timestamp values without a timezone cannot
be reliably compared or ordered, since they may have different points of
reference.  In particular, it is *not* possible to interpret an unset
or empty timezone as the same as "UTC".

Conversion between timezones
----------------------------

If a Timestamp column has a non-empty timezone, changing the timezone
to a different non-empty value is a metadata-only operation:
the timestamp values need not change as their point of reference remains
the same (the Unix epoch).

However, if a Timestamp column has no timezone value, changing it to a
non-empty value requires to think about the desired semantics.
One possibility is to assume that the original timestamp values are
relative to the epoch of the timezone being set; timestamp values should
then adjusted to the Unix epoch (for example, changing the timezone from
empty to "Europe/Paris" would require converting the timestamp values
from "Europe/Paris" to "UTC", which seems counter-intuitive but is
nevertheless correct).

```
# use arrow::datatypes::{DataType, TimeUnit};
DataType::Timestamp(TimeUnit::Second, None);
DataType::Timestamp(TimeUnit::Second, Some("literal".into()));
DataType::Timestamp(TimeUnit::Second, Some("string".to_string().into()));
```

<a id="op-f65652cfb765e83c7d7a3f35"></a>
## UInt16

`variant` · `datafusion_common::types::native::NativeType::UInt16` · datafusion-common 55.1.0

```rust
UInt16
```

Source: `src/types/native.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

An unsigned 16-bit integer.

<a id="op-3e9ca410bd65f0e00f1e8b0e"></a>
## UInt32

`variant` · `datafusion_common::types::native::NativeType::UInt32` · datafusion-common 55.1.0

```rust
UInt32
```

Source: `src/types/native.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

An unsigned 32-bit integer.

<a id="op-2adf1871d4e21642d80dfe6c"></a>
## UInt64

`variant` · `datafusion_common::types::native::NativeType::UInt64` · datafusion-common 55.1.0

```rust
UInt64
```

Source: `src/types/native.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

An unsigned 64-bit integer.

<a id="op-8a4d034cb1b749a69b64a2b5"></a>
## UInt8

`variant` · `datafusion_common::types::native::NativeType::UInt8` · datafusion-common 55.1.0

```rust
UInt8
```

Source: `src/types/native.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

An unsigned 8-bit integer.

<a id="op-0a02dbd668428b8a6ad4ffb8"></a>
## Union

`variant` · `datafusion_common::types::native::NativeType::Union` · datafusion-common 55.1.0

```rust
Union
```

Source: `src/types/native.rs:161`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A nested type that can represent slots of differing types.

<a id="op-df9fd503ac0e8bfbc55d76a0"></a>
## clone

`function` · `datafusion_common::types::native::NativeType::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> NativeType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::native::NativeType", "path": "NativeType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 17], "end": [32, 22], "filename": "src/types/native.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/types/native.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8f32d821bed4725f45b70f52"></a>
## cmp

`function` · `datafusion_common::types::native::NativeType::cmp` · datafusion-common 55.1.0

```rust
fn cmp(&self, other: &NativeType) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::native::NativeType", "path": "NativeType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 57], "end": [32, 60], "filename": "src/types/native.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/types/native.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aa2eb471cff6aaca11c2aeda"></a>
## default_cast_for

`function` · `datafusion_common::types::native::NativeType::default_cast_for` · datafusion-common 55.1.0

```rust
fn default_cast_for(&self, origin: &DataType) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::native::NativeType", "path": "NativeType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [266, 1], "end": [444, 2], "filename": "src/types/native.rs"}, "trait": {"args": null, "id": "datafusion_common::types::logical::LogicalType", "path": "LogicalType"}, "trait_path": "datafusion_common::types::logical::LogicalType"}`

Source: `src/types/native.rs:280`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns the default casted type for the given arrow type

For types like String or Date, multiple arrow types mapped to the same logical type
If the given arrow type is one of them, we return the same type
Otherwise, we define the default casted type for the given arrow type

<a id="op-caa1734ed0d67aa88f81fb67"></a>
## eq

`function` · `datafusion_common::types::native::NativeType::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &NativeType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::native::NativeType", "path": "NativeType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 24], "end": [32, 33], "filename": "src/types/native.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/types/native.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3997d9ed841d41659076bf6e"></a>
## fmt

`function` · `datafusion_common::types::native::NativeType::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::native::NativeType", "path": "NativeType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 10], "end": [32, 15], "filename": "src/types/native.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/types/native.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c3bf29a500ddf5035084a31"></a>
## fmt

`function` · `datafusion_common::types::native::NativeType::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::native::NativeType", "path": "NativeType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [197, 1], "end": [264, 2], "filename": "src/types/native.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/types/native.rs:198`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4df83ccb28f1fe45b3f4e2aa"></a>
## from

`function` · `datafusion_common::types::native::NativeType::from` · datafusion-common 55.1.0

```rust
fn from(value: DataType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::native::NativeType", "path": "NativeType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [456, 1], "end": [501, 2], "filename": "src/types/native.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::DataType", "path": "DataType"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/types/native.rs:457`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e145f214672db658fc7e69b4"></a>
## from

`function` · `datafusion_common::types::native::NativeType::from` · datafusion-common 55.1.0

```rust
fn from(value: &DataType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::native::NativeType", "path": "NativeType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [450, 1], "end": [454, 2], "filename": "src/types/native.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::DataType", "path": "DataType"}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/types/native.rs:451`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c57ade68d07d3d41874a720"></a>
## hash

`function` · `datafusion_common::types::native::NativeType::hash` · datafusion-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::native::NativeType", "path": "NativeType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 39], "end": [32, 43], "filename": "src/types/native.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/types/native.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-393ed543c94ba0df9c9f6831"></a>
## is_binary

`function` · `datafusion_common::types::native::NativeType::is_binary` · datafusion-common 55.1.0

```rust
fn is_binary(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::native::NativeType", "path": "NativeType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [503, 1], "end": [562, 2], "filename": "src/types/native.rs"}, "trait": null, "trait_path": null}`

Source: `src/types/native.rs:544`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-05530fed2b1f472c9cdefc90"></a>
## is_date

`function` · `datafusion_common::types::native::NativeType::is_date` · datafusion-common 55.1.0

```rust
fn is_date(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::native::NativeType", "path": "NativeType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [503, 1], "end": [562, 2], "filename": "src/types/native.rs"}, "trait": null, "trait_path": null}`

Source: `src/types/native.rs:524`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-da625abee3f9a2be217248e8"></a>
## is_decimal

`function` · `datafusion_common::types::native::NativeType::is_decimal` · datafusion-common 55.1.0

```rust
fn is_decimal(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::native::NativeType", "path": "NativeType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [503, 1], "end": [562, 2], "filename": "src/types/native.rs"}, "trait": null, "trait_path": null}`

Source: `src/types/native.rs:554`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2f49d7250861d375fe30fcae"></a>
## is_duration

`function` · `datafusion_common::types::native::NativeType::is_duration` · datafusion-common 55.1.0

```rust
fn is_duration(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::native::NativeType", "path": "NativeType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [503, 1], "end": [562, 2], "filename": "src/types/native.rs"}, "trait": null, "trait_path": null}`

Source: `src/types/native.rs:539`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6698151721d059fd37d00dcc"></a>
## is_float

`function` · `datafusion_common::types::native::NativeType::is_float` · datafusion-common 55.1.0

```rust
fn is_float(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::native::NativeType", "path": "NativeType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [503, 1], "end": [562, 2], "filename": "src/types/native.rs"}, "trait": null, "trait_path": null}`

Source: `src/types/native.rs:559`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5cee9ab7791c52b448bc854f"></a>
## is_integer

`function` · `datafusion_common::types::native::NativeType::is_integer` · datafusion-common 55.1.0

```rust
fn is_integer(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::native::NativeType", "path": "NativeType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [503, 1], "end": [562, 2], "filename": "src/types/native.rs"}, "trait": null, "trait_path": null}`

Source: `src/types/native.rs:510`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-30c4be2ceea8f82bbc67d252"></a>
## is_interval

`function` · `datafusion_common::types::native::NativeType::is_interval` · datafusion-common 55.1.0

```rust
fn is_interval(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::native::NativeType", "path": "NativeType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [503, 1], "end": [562, 2], "filename": "src/types/native.rs"}, "trait": null, "trait_path": null}`

Source: `src/types/native.rs:534`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d39c30997b80be600c39d699"></a>
## is_null

`function` · `datafusion_common::types::native::NativeType::is_null` · datafusion-common 55.1.0

```rust
fn is_null(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::native::NativeType", "path": "NativeType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [503, 1], "end": [562, 2], "filename": "src/types/native.rs"}, "trait": null, "trait_path": null}`

Source: `src/types/native.rs:549`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b6f876ef8b048b1b1002a939"></a>
## is_numeric

`function` · `datafusion_common::types::native::NativeType::is_numeric` · datafusion-common 55.1.0

```rust
fn is_numeric(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::native::NativeType", "path": "NativeType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [503, 1], "end": [562, 2], "filename": "src/types/native.rs"}, "trait": null, "trait_path": null}`

Source: `src/types/native.rs:505`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cc320f1b04b0f4ae1197f9b1"></a>
## is_time

`function` · `datafusion_common::types::native::NativeType::is_time` · datafusion-common 55.1.0

```rust
fn is_time(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::native::NativeType", "path": "NativeType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [503, 1], "end": [562, 2], "filename": "src/types/native.rs"}, "trait": null, "trait_path": null}`

Source: `src/types/native.rs:529`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3022a1192b08416ecd318382"></a>
## is_timestamp

`function` · `datafusion_common::types::native::NativeType::is_timestamp` · datafusion-common 55.1.0

```rust
fn is_timestamp(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::native::NativeType", "path": "NativeType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [503, 1], "end": [562, 2], "filename": "src/types/native.rs"}, "trait": null, "trait_path": null}`

Source: `src/types/native.rs:519`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4908720ec30320fa2be307a8"></a>
## native

`function` · `datafusion_common::types::native::NativeType::native` · datafusion-common 55.1.0

```rust
fn native(&self) -> &NativeType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::native::NativeType", "path": "NativeType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [266, 1], "end": [444, 2], "filename": "src/types/native.rs"}, "trait": {"args": null, "id": "datafusion_common::types::logical::LogicalType", "path": "LogicalType"}, "trait_path": "datafusion_common::types::logical::LogicalType"}`

Source: `src/types/native.rs:267`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ac50f300cd01e75546542abc"></a>
## partial_cmp

`function` · `datafusion_common::types::native::NativeType::partial_cmp` · datafusion-common 55.1.0

```rust
fn partial_cmp(&self, other: &NativeType) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::native::NativeType", "path": "NativeType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 45], "end": [32, 55], "filename": "src/types/native.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/types/native.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fc7f89fe27a61c174a54a380"></a>
## signature

`function` · `datafusion_common::types::native::NativeType::signature` · datafusion-common 55.1.0

```rust
fn signature(&self) -> TypeSignature<'_>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::native::NativeType", "path": "NativeType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [266, 1], "end": [444, 2], "filename": "src/types/native.rs"}, "trait": {"args": null, "id": "datafusion_common::types::logical::LogicalType", "path": "LogicalType"}, "trait_path": "datafusion_common::types::logical::LogicalType"}`

Source: `src/types/native.rs:271`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
