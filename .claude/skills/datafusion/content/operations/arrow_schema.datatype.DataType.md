# `arrow_schema::datatype::DataType`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_schema.datatype.DataType.json).

<a id="op-bf69df5b14436e006d3a531c"></a>
## DataType

`enum` · `arrow_schema::datatype::DataType` · arrow-schema 59.3.0

```rust
enum DataType
```

Source: `src/datatype.rs:96`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

 Datatypes supported by this implementation of Apache Arrow.

 The variants of this enum include primitive fixed size types as well as
 parametric or nested types. See [`Schema.fbs`] for Arrow's specification.

 # Examples

 Primitive types
 ```
 # use arrow_schema::DataType;
 // create a new 32-bit signed integer
 let data_type = DataType::Int32;
 ```

 Nested Types
 ```
 # use arrow_schema::{DataType, Field};
 # use std::sync::Arc;
 // create a new list of 32-bit signed integers directly
 let list_data_type = DataType::List(Arc::new(Field::new_list_field(DataType::Int32, true)));
 // Create the same list type with constructor
 let list_data_type2 = DataType::new_list(DataType::Int32, true);
 assert_eq!(list_data_type, list_data_type2);
 ```

 Dictionary Types
 ```
 # use arrow_schema::{DataType};
 // String Dictionary (key type Int32 and value type Utf8)
 let data_type = DataType::Dictionary(Box::new(DataType::Int32), Box::new(DataType::Utf8));
 ```

 Timestamp Types
 ```
 # use arrow_schema::{DataType, TimeUnit};
 // timestamp with millisecond precision without timezone specified
 let data_type = DataType::Timestamp(TimeUnit::Millisecond, None);
 // timestamp with nanosecond precision in UTC timezone
 let data_type = DataType::Timestamp(TimeUnit::Nanosecond, Some("UTC".into()));
```

 # Display and FromStr

 The `Display` and `FromStr` implementations for `DataType` are
 human-readable, parseable, and reversible.

 ```
 # use arrow_schema::DataType;
 let data_type = DataType::Dictionary(Box::new(DataType::Int32), Box::new(DataType::Utf8));
 let data_type_string = data_type.to_string();
 assert_eq!(data_type_string, "Dictionary(Int32, Utf8)");
 // display can be parsed back into the original type
 let parsed_data_type: DataType = data_type.to_string().parse().unwrap();
 assert_eq!(data_type, parsed_data_type);
 ```

 # Nested Support
 Currently, the Rust implementation supports the following nested types:
  - `List<T>`
  - `LargeList<T>`
  - `FixedSizeList<T>`
  - `Struct<T, U, V, ...>`
  - `Union<T, U, V, ...>`
  - `Map<K, V>`

 Nested types can themselves be nested within other arrays.
 For more information on these types please see
 [the physical memory layout of Apache Arrow]

 [`Schema.fbs`]: https://github.com/apache/arrow/blob/main/format/Schema.fbs
 [the physical memory layout of Apache Arrow]: https://arrow.apache.org/docs/format/Columnar.html#physical-memory-layout

<a id="op-582f9ed31b95febd680a4b5a"></a>
## Binary

`variant` · `arrow_schema::datatype::DataType::Binary` · arrow-schema 59.3.0

```rust
Binary
```

Source: `src/datatype.rs:274`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Opaque binary data of variable length.

A single Binary array can store up to [`i32::MAX`] bytes
of binary data in total.

Unresolved upstream links (retained, not inferred): ``i32::MAX``.

<a id="op-f6df51fbc9cfe5e3e638183b"></a>
## BinaryView

`variant` · `arrow_schema::datatype::DataType::BinaryView` · arrow-schema 59.3.0

```rust
BinaryView
```

Source: `src/datatype.rs:295`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Opaque binary data of variable length.

Logically the same as [`Binary`], but the internal representation uses a view
struct that contains the string length and either the string's entire data
inline (for small strings) or an inlined prefix, an index of another buffer,
and an offset pointing to a slice in that buffer (for non-small strings).

[`Binary`]: Self::Binary

<a id="op-72eeb159af4461aaf1b15a68"></a>
## Boolean

`variant` · `arrow_schema::datatype::DataType::Boolean` · arrow-schema 59.3.0

```rust
Boolean
```

Source: `src/datatype.rs:100`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

A boolean datatype representing the values `true` and `false`.

<a id="op-9e3071e3ccfadc14c99b045d"></a>
## Date32

`variant` · `arrow_schema::datatype::DataType::Date32` · arrow-schema 59.3.0

```rust
Date32
```

Source: `src/datatype.rs:220`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

A signed 32-bit date representing the elapsed time since UNIX epoch (1970-01-01)
in days.

<a id="op-4e02b24282692b3bcad20b8c"></a>
## Date64

`variant` · `arrow_schema::datatype::DataType::Date64` · arrow-schema 59.3.0

```rust
Date64
```

Source: `src/datatype.rs:257`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

A signed 64-bit date representing the elapsed time since UNIX epoch (1970-01-01)
in milliseconds.

# Valid Ranges

According to the Arrow specification ([Schema.fbs]), values of Date64
are treated as the number of *days*, in milliseconds, since the UNIX
epoch. Therefore, values of this type  must be evenly divisible by
`86_400_000`, the number of milliseconds in a standard day.

It is not valid to store milliseconds that do not represent an exact
day. The reason for this restriction is compatibility with other
language's native libraries (specifically Java), which historically
lacked a dedicated date type and only supported timestamps.

# Validation

This library does not validate or enforce that Date64 values are evenly
divisible by `86_400_000`  for performance and usability reasons. Date64
values are treated similarly to `Timestamp(TimeUnit::Millisecond,
None)`: values will be displayed with a time of day if the value does
not represent an exact day, and arithmetic will be done at the
millisecond granularity.

# Recommendation

Users should prefer [`Date32`] to cleanly represent the number
of days, or one of the Timestamp variants to include time as part of the
representation, depending on their use case.

# Further Reading

For more details, see [#5288](https://github.com/apache/arrow-rs/issues/5288).

[`Date32`]: Self::Date32
[Schema.fbs]: https://github.com/apache/arrow/blob/main/format/Schema.fbs

<a id="op-e54295a8e8bbe3cefae79bcd"></a>
## Decimal128

`variant` · `arrow_schema::datatype::DataType::Decimal128` · arrow-schema 59.3.0

```rust
Decimal128
```

Source: `src/datatype.rs:392`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Exact 128-bit width decimal value with precision and scale

* precision is the maximum number of digits in the unscaled value
* scale controls the position of the decimal point

The represented value is the unscaled integer multiplied by 10^{-scale}.
For example, the unscaled value 12345 with precision 5 and scale 2
represents 123.45.

Scale can also be negative. For example, the unscaled value 12 with
precision 2 and scale -3 represents 12000.

<a id="op-fe9990d44d3e962e1dadd391"></a>
## Decimal256

`variant` · `arrow_schema::datatype::DataType::Decimal256` · arrow-schema 59.3.0

```rust
Decimal256
```

Source: `src/datatype.rs:404`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Exact 256-bit width decimal value with precision and scale

* precision is the maximum number of digits in the unscaled value
* scale controls the position of the decimal point

The represented value is the unscaled integer multiplied by 10^{-scale}.
For example, the unscaled value 12345 with precision 5 and scale 2
represents 123.45.

Scale can also be negative. For example, the unscaled value 12 with
precision 2 and scale -3 represents 12000.

<a id="op-78b7678812b92ef892ce6b0f"></a>
## Decimal32

`variant` · `arrow_schema::datatype::DataType::Decimal32` · arrow-schema 59.3.0

```rust
Decimal32
```

Source: `src/datatype.rs:368`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Exact 32-bit width decimal value with precision and scale

* precision is the maximum number of digits in the unscaled value
* scale controls the position of the decimal point

The represented value is the unscaled integer multiplied by 10^{-scale}.
For example, the unscaled value 12345 with precision 5 and scale 2
represents 123.45.

Scale can also be negative. For example, the unscaled value 12 with
precision 2 and scale -3 represents 12000.

<a id="op-604a2f94cb7e1a898e3be66a"></a>
## Decimal64

`variant` · `arrow_schema::datatype::DataType::Decimal64` · arrow-schema 59.3.0

```rust
Decimal64
```

Source: `src/datatype.rs:380`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Exact 64-bit width decimal value with precision and scale

* precision is the maximum number of digits in the unscaled value
* scale controls the position of the decimal point

The represented value is the unscaled integer multiplied by 10^{-scale}.
For example, the unscaled value 12345 with precision 5 and scale 2
represents 123.45.

Scale can also be negative. For example, the unscaled value 12 with
precision 2 and scale -3 represents 12000.

<a id="op-d5eec6393c8334f2a08fe17b"></a>
## Dictionary

`variant` · `arrow_schema::datatype::DataType::Dictionary` · arrow-schema 59.3.0

```rust
Dictionary
```

Source: `src/datatype.rs:356`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

A dictionary encoded array (`key_type`, `value_type`), where
each array element is an index of `key_type` into an
associated dictionary of `value_type`.

Dictionary arrays are used to store columns of `value_type`
that contain many repeated values using less memory, but with
a higher CPU overhead for some operations.

This type mostly used to represent low cardinality string
arrays or a limited set of primitive types as integers.

<a id="op-4e357f478fbe89afebdf78c9"></a>
## Duration

`variant` · `arrow_schema::datatype::DataType::Duration` · arrow-schema 59.3.0

```rust
Duration
```

Source: `src/datatype.rs:265`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Measure of elapsed time in either seconds, milliseconds, microseconds or nanoseconds.

<a id="op-6b43f9fef6120cc1afe513d9"></a>
## Err

`assoc_type` · `arrow_schema::datatype::DataType::Err` · arrow-schema 59.3.0

```rust
Err
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::DataType", "path": "DataType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [500, 1], "end": [506, 2], "filename": "src/datatype.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/datatype.rs:501`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-63d7d508da956b6b7ea8dee7"></a>
## Error

`assoc_type` · `arrow_schema::datatype::DataType::Error` · arrow-schema 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::DataType", "path": "crate::DataType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [429, 1], "end": [645, 2], "filename": "src/ffi.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "arrow_schema::ffi::FFI_ArrowSchema", "path": "FFI_ArrowSchema"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/ffi.rs:430`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9216e238c4226e5f2346dd97"></a>
## Error

`assoc_type` · `arrow_schema::datatype::DataType::Error` · arrow-schema 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::DataType", "path": "DataType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [508, 1], "end": [514, 2], "filename": "src/datatype.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"primitive": "str"}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/datatype.rs:509`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-26999f56cfafc954001baa9c"></a>
## FixedSizeBinary

`variant` · `arrow_schema::datatype::DataType::FixedSizeBinary` · arrow-schema 59.3.0

```rust
FixedSizeBinary
```

Source: `src/datatype.rs:281`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Opaque binary data of fixed size.

Enum parameter specifies the number of bytes per value, defined by the
[`byteWidth` field] in the Arrow Spec

[`byteWidth` field]: https://github.com/apache/arrow/blob/2a89d03bbefd620b42126b8e00f8ae57e99cd638/format/Schema.fbs#L211

<a id="op-69f9b11fdd4d79bf23810b3a"></a>
## FixedSizeList

`variant` · `arrow_schema::datatype::DataType::FixedSizeList` · arrow-schema 59.3.0

```rust
FixedSizeList
```

Source: `src/datatype.rs:327`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

A list of some logical data type with fixed length.

<a id="op-04f79e5349eaf1d61ddaad51"></a>
## Float16

`variant` · `arrow_schema::datatype::DataType::Float16` · arrow-schema 59.3.0

```rust
Float16
```

Source: `src/datatype.rs:118`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

A 16-bit floating point number.

<a id="op-4c461badd8a0ab916e6a35c1"></a>
## Float32

`variant` · `arrow_schema::datatype::DataType::Float32` · arrow-schema 59.3.0

```rust
Float32
```

Source: `src/datatype.rs:120`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

A 32-bit floating point number.

<a id="op-c7ea9edacf7b91c7728828e8"></a>
## Float64

`variant` · `arrow_schema::datatype::DataType::Float64` · arrow-schema 59.3.0

```rust
Float64
```

Source: `src/datatype.rs:122`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

A 64-bit floating point number.

<a id="op-07d976bfcebba5cd2f1c24a0"></a>
## Int16

`variant` · `arrow_schema::datatype::DataType::Int16` · arrow-schema 59.3.0

```rust
Int16
```

Source: `src/datatype.rs:104`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

A signed 16-bit integer.

<a id="op-d541418442fe3889b69e1019"></a>
## Int32

`variant` · `arrow_schema::datatype::DataType::Int32` · arrow-schema 59.3.0

```rust
Int32
```

Source: `src/datatype.rs:106`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

A signed 32-bit integer.

<a id="op-b5e46464bd4cab7092efa1ed"></a>
## Int64

`variant` · `arrow_schema::datatype::DataType::Int64` · arrow-schema 59.3.0

```rust
Int64
```

Source: `src/datatype.rs:108`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

A signed 64-bit integer.

<a id="op-fb7865a9fb1164b7d5738475"></a>
## Int8

`variant` · `arrow_schema::datatype::DataType::Int8` · arrow-schema 59.3.0

```rust
Int8
```

Source: `src/datatype.rs:102`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

A signed 8-bit integer.

<a id="op-074ce27a00379965de563425"></a>
## Interval

`variant` · `arrow_schema::datatype::DataType::Interval` · arrow-schema 59.3.0

```rust
Interval
```

Source: `src/datatype.rs:269`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

A "calendar" interval which models types that don't necessarily
have a precise duration without the context of a base timestamp (e.g.
days can differ in length during day light savings time transitions).

<a id="op-adefdae5cd21cb2c38c0d4e1"></a>
## LargeBinary

`variant` · `arrow_schema::datatype::DataType::LargeBinary` · arrow-schema 59.3.0

```rust
LargeBinary
```

Source: `src/datatype.rs:286`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Opaque binary data of variable length and 64-bit offsets.

A single LargeBinary array can store up to [`i64::MAX`] bytes
of binary data in total.

Unresolved upstream links (retained, not inferred): ``i64::MAX``.

<a id="op-f0e628cee644a7235d5b4c1d"></a>
## LargeList

`variant` · `arrow_schema::datatype::DataType::LargeList` · arrow-schema 59.3.0

```rust
LargeList
```

Source: `src/datatype.rs:331`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

A list of some logical data type with variable length and 64-bit offsets.

A single LargeList array can store up to [`i64::MAX`] elements in total.

Unresolved upstream links (retained, not inferred): ``i64::MAX``.

<a id="op-999fe1ed1c045edb1f496432"></a>
## LargeListView

`variant` · `arrow_schema::datatype::DataType::LargeListView` · arrow-schema 59.3.0

```rust
LargeListView
```

Source: `src/datatype.rs:338`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

A list of some logical data type with variable length and 64-bit offsets.

Logically the same as [`LargeList`], but the internal representation differs in how child
data is referenced, allowing flexibility in how data is layed out.

[`LargeList`]: Self::LargeList

<a id="op-d897e1ec91191af958fa69ca"></a>
## LargeUtf8

`variant` · `arrow_schema::datatype::DataType::LargeUtf8` · arrow-schema 59.3.0

```rust
LargeUtf8
```

Source: `src/datatype.rs:305`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

A variable-length string in Unicode with UFT-8 encoding and 64-bit offsets.

A single LargeUtf8 array can store up to [`i64::MAX`] bytes
of string data in total.

Unresolved upstream links (retained, not inferred): ``i64::MAX``.

<a id="op-83ec578cb0e12f00905856b8"></a>
## List

`variant` · `arrow_schema::datatype::DataType::List` · arrow-schema 59.3.0

```rust
List
```

Source: `src/datatype.rs:318`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

A list of some logical data type with variable length.

A single List array can store up to [`i32::MAX`] elements in total.

Unresolved upstream links (retained, not inferred): ``i32::MAX``.

<a id="op-c7c132b56a4b0a554b58d90e"></a>
## ListView

`variant` · `arrow_schema::datatype::DataType::ListView` · arrow-schema 59.3.0

```rust
ListView
```

Source: `src/datatype.rs:325`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

A list of some logical data type with variable length.

Logically the same as [`List`], but the internal representation differs in how child
data is referenced, allowing flexibility in how data is layed out.

[`List`]: Self::List

<a id="op-e0392f37e4078beb085286d5"></a>
## Map

`variant` · `arrow_schema::datatype::DataType::Map` · arrow-schema 59.3.0

```rust
Map
```

Source: `src/datatype.rs:418`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

A Map is a logical nested type that is represented as

`List<entries: Struct<key: K, value: V>>`

The keys and values are each respectively contiguous.
The key and value types are not constrained, but keys should be
hashable and unique.
Whether the keys are sorted can be set in the `bool` after the `Field`.

In a field with Map type, the field has a child Struct field, which then
has two children: key type and the second the value type. The names of the
child fields may be respectively "entries", "key", and "value", but this is
not enforced.

<a id="op-0e6b89945b6e61141a2fa7ee"></a>
## Null

`variant` · `arrow_schema::datatype::DataType::Null` · arrow-schema 59.3.0

```rust
Null
```

Source: `src/datatype.rs:98`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Null type

<a id="op-3b6d5353249d9a9aee3579a4"></a>
## RunEndEncoded

`variant` · `arrow_schema::datatype::DataType::RunEndEncoded` · arrow-schema 59.3.0

```rust
RunEndEncoded
```

Source: `src/datatype.rs:430`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

A run-end encoding (REE) is a variation of run-length encoding (RLE). These
encodings are well-suited for representing data containing sequences of the
same value, called runs. Each run is represented as a value and an integer giving
the index in the array where the run ends.

A run-end encoded array has no buffers by itself, but has two child arrays. The
first child array, called the run ends array, holds either 16, 32, or 64-bit
signed integers. The actual values of each run are held in the second child array.

These child arrays are prescribed the standard names of "run_ends" and "values"
respectively.

<a id="op-6b87cda240d80cf3928ec575"></a>
## Struct

`variant` · `arrow_schema::datatype::DataType::Struct` · arrow-schema 59.3.0

```rust
Struct
```

Source: `src/datatype.rs:340`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

A nested datatype that contains a number of sub-fields.

<a id="op-6a017bf7a588fdb9294652ac"></a>
## Time32

`variant` · `arrow_schema::datatype::DataType::Time32` · arrow-schema 59.3.0

```rust
Time32
```

Source: `src/datatype.rs:260`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

A signed 32-bit time representing the elapsed time since midnight in the unit of `TimeUnit`.
Must be either seconds or milliseconds.

<a id="op-2a5f5e259eb3a58bc4a55d67"></a>
## Time64

`variant` · `arrow_schema::datatype::DataType::Time64` · arrow-schema 59.3.0

```rust
Time64
```

Source: `src/datatype.rs:263`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

A signed 64-bit time representing the elapsed time since midnight in the unit of `TimeUnit`.
Must be either microseconds or nanoseconds.

<a id="op-4311beb64d86f8afbc59cba2"></a>
## Timestamp

`variant` · `arrow_schema::datatype::DataType::Timestamp` · arrow-schema 59.3.0

```rust
Timestamp
```

Source: `src/datatype.rs:217`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

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
# use arrow_schema::{DataType, TimeUnit};
DataType::Timestamp(TimeUnit::Second, None);
DataType::Timestamp(TimeUnit::Second, Some("literal".into()));
DataType::Timestamp(TimeUnit::Second, Some("string".to_string().into()));
```

# Timezone representation
----------------------------
It is possible to use either the timezone string representation, such as "UTC", or the absolute time zone offset "+00:00".
For timezones with fixed offsets, such as "UTC" or "JST", the offset representation is recommended, as it is more explicit and less ambiguous.

Most arrow-rs functionalities use the absolute offset representation,
such as [`PrimitiveArray::with_timezone_utc`] that applies a
UTC timezone to timestamp arrays.

[`PrimitiveArray::with_timezone_utc`]: https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html#method.with_timezone_utc

Timezone string parsing
-----------------------
When feature `chrono-tz` is not enabled, allowed timezone strings are fixed offsets of the form "+09:00", "-09" or "+0930".

When feature `chrono-tz` is enabled, additional strings supported by [chrono_tz](https://docs.rs/chrono-tz/latest/chrono_tz/)
are also allowed, which include [IANA database](https://en.wikipedia.org/wiki/List_of_tz_database_time_zones)
timezones.

<a id="op-6370051c26be87c9b7d443ef"></a>
## UInt16

`variant` · `arrow_schema::datatype::DataType::UInt16` · arrow-schema 59.3.0

```rust
UInt16
```

Source: `src/datatype.rs:112`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

An unsigned 16-bit integer.

<a id="op-d9de57ad68dc6dd067d5e8b3"></a>
## UInt32

`variant` · `arrow_schema::datatype::DataType::UInt32` · arrow-schema 59.3.0

```rust
UInt32
```

Source: `src/datatype.rs:114`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

An unsigned 32-bit integer.

<a id="op-17e9bea2eea1c63fae20b4df"></a>
## UInt64

`variant` · `arrow_schema::datatype::DataType::UInt64` · arrow-schema 59.3.0

```rust
UInt64
```

Source: `src/datatype.rs:116`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

An unsigned 64-bit integer.

<a id="op-cb2c6427ae71a7e19fe384bf"></a>
## UInt8

`variant` · `arrow_schema::datatype::DataType::UInt8` · arrow-schema 59.3.0

```rust
UInt8
```

Source: `src/datatype.rs:110`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

An unsigned 8-bit integer.

<a id="op-77c858ff3881087e0207bcfc"></a>
## Union

`variant` · `arrow_schema::datatype::DataType::Union` · arrow-schema 59.3.0

```rust
Union
```

Source: `src/datatype.rs:345`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

A nested datatype that can represent slots of differing types. Components:

1. [`UnionFields`](../operations/arrow_schema.fields.UnionFields.md#op-daca7b2fe864f8176f562862)
2. The type of union (Sparse or Dense)

<a id="op-4e52d4ade5cb975c8f4c452a"></a>
## Utf8

`variant` · `arrow_schema::datatype::DataType::Utf8` · arrow-schema 59.3.0

```rust
Utf8
```

Source: `src/datatype.rs:300`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

A variable-length string in Unicode with UTF-8 encoding.

A single Utf8 array can store up to [`i32::MAX`] bytes
of string data in total.

Unresolved upstream links (retained, not inferred): ``i32::MAX``.

<a id="op-a3c8435fd0f9a132833a8fc0"></a>
## Utf8View

`variant` · `arrow_schema::datatype::DataType::Utf8View` · arrow-schema 59.3.0

```rust
Utf8View
```

Source: `src/datatype.rs:314`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

A variable-length string in Unicode with UTF-8 encoding

Logically the same as [`Utf8`], but the internal representation uses a view
struct that contains the string length and either the string's entire data
inline (for small strings) or an inlined prefix, an index of another buffer,
and an offset pointing to a slice in that buffer (for non-small strings).

[`Utf8`]: Self::Utf8

<a id="op-d9df911c79abf45087e8bf60"></a>
## clone

`function` · `arrow_schema::datatype::DataType::clone` · arrow-schema 59.3.0

```rust
fn clone(&self) -> DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::DataType", "path": "DataType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 10], "end": [94, 15], "filename": "src/datatype.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/datatype.rs:94`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-133b60e3285d36cd3e0fb24a"></a>
## cmp

`function` · `arrow_schema::datatype::DataType::cmp` · arrow-schema 59.3.0

```rust
fn cmp(&self, other: &DataType) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::DataType", "path": "DataType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 57], "end": [94, 60], "filename": "src/datatype.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/datatype.rs:94`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ac589cee0349a944d9acef85"></a>
## contains

`function` · `arrow_schema::datatype::DataType::contains` · arrow-schema 59.3.0

```rust
fn contains(&self, other: &DataType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::DataType", "path": "DataType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [516, 1], "end": [867, 2], "filename": "src/datatype.rs"}, "trait": null, "trait_path": null}`

Source: `src/datatype.rs:817`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Check to see if `self` is a superset of `other`

If DataType is a nested type, then it will check to see if the nested type is a superset of the other nested type
else it will check to see if the DataType is equal to the other DataType

<a id="op-b3627598a8e02e88bafb8dd8"></a>
## deserialize

`function` · `arrow_schema::datatype::DataType::deserialize` · arrow-schema 59.3.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::DataType", "path": "DataType"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 56], "end": [95, 74], "filename": "src/datatype.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/datatype.rs:95`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-39bb19b5afbe6c35b29265fa"></a>
## eq

`function` · `arrow_schema::datatype::DataType::eq` · arrow-schema 59.3.0

```rust
fn eq(&self, other: &DataType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::DataType", "path": "DataType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 24], "end": [94, 33], "filename": "src/datatype.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/datatype.rs:94`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-53c3524cc54a42bc910e6b05"></a>
## equals_datatype

`function` · `arrow_schema::datatype::DataType::equals_datatype` · arrow-schema 59.3.0

```rust
fn equals_datatype(&self, other: &DataType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::DataType", "path": "DataType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [516, 1], "end": [867, 2], "filename": "src/datatype.rs"}, "trait": null, "trait_path": null}`

Source: `src/datatype.rs:664`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Compares the datatype with another, ignoring nested field names
and metadata.

<a id="op-205a22b5fd3ccf887ab82d61"></a>
## fmt

`function` · `arrow_schema::datatype::DataType::fmt` · arrow-schema 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::DataType", "path": "crate::DataType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 1], "end": [184, 2], "filename": "src/datatype_display.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/datatype_display.rs:23`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b5f56bd7bc8a253e220c8d4c"></a>
## fmt

`function` · `arrow_schema::datatype::DataType::fmt` · arrow-schema 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::DataType", "path": "DataType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 17], "end": [94, 22], "filename": "src/datatype.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/datatype.rs:94`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-182d8f2675f354ee20f25bd2"></a>
## from_str

`function` · `arrow_schema::datatype::DataType::from_str` · arrow-schema 59.3.0

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::DataType", "path": "DataType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [500, 1], "end": [506, 2], "filename": "src/datatype.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/datatype.rs:503`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-65ae0e3bd0bdc0a5c90e8cf1"></a>
## hash

`function` · `arrow_schema::datatype::DataType::hash` · arrow-schema 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::DataType", "path": "DataType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 39], "end": [94, 43], "filename": "src/datatype.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/datatype.rs:94`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e7b71565a8644b9410144847"></a>
## is_binary

`function` · `arrow_schema::datatype::DataType::is_binary` · arrow-schema 59.3.0

```rust
fn is_binary(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::DataType", "path": "DataType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [516, 1], "end": [867, 2], "filename": "src/datatype.rs"}, "trait": null, "trait_path": null}`

Source: `src/datatype.rs:657`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns true if this type is a Binary type.

Binary types include Binary, LargeBinary, FixedSizeBinary and BinaryView.

<a id="op-569844a911b794a874189b52"></a>
## is_decimal

`function` · `arrow_schema::datatype::DataType::is_decimal` · arrow-schema 59.3.0

```rust
fn is_decimal(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::DataType", "path": "DataType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [516, 1], "end": [867, 2], "filename": "src/datatype.rs"}, "trait": null, "trait_path": null}`

Source: `src/datatype.rs:586`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns true if this type is decimal: (Decimal*).

<a id="op-d1cb4d0eb52fb20e596b412c"></a>
## is_dictionary_key_type

`function` · `arrow_schema::datatype::DataType::is_dictionary_key_type` · arrow-schema 59.3.0

```rust
fn is_dictionary_key_type(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::DataType", "path": "DataType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [516, 1], "end": [867, 2], "filename": "src/datatype.rs"}, "trait": null, "trait_path": null}`

Source: `src/datatype.rs:596`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns true if this type is valid as a dictionary key

<a id="op-ff138d0c717d49dacb346e82"></a>
## is_floating

`function` · `arrow_schema::datatype::DataType::is_floating` · arrow-schema 59.3.0

```rust
fn is_floating(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::DataType", "path": "DataType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [516, 1], "end": [867, 2], "filename": "src/datatype.rs"}, "trait": null, "trait_path": null}`

Source: `src/datatype.rs:559`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns true if this type is floating: (Float*).

<a id="op-15b03d3f51459acd7a976c1c"></a>
## is_integer

`function` · `arrow_schema::datatype::DataType::is_integer` · arrow-schema 59.3.0

```rust
fn is_integer(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::DataType", "path": "DataType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [516, 1], "end": [867, 2], "filename": "src/datatype.rs"}, "trait": null, "trait_path": null}`

Source: `src/datatype.rs:566`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns true if this type is integer: (Int*, UInt*).

<a id="op-78806c83558282b1c66847f7"></a>
## is_list

`function` · `arrow_schema::datatype::DataType::is_list` · arrow-schema 59.3.0

```rust
fn is_list(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::DataType", "path": "DataType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [516, 1], "end": [867, 2], "filename": "src/datatype.rs"}, "trait": null, "trait_path": null}`

Source: `src/datatype.rs:645`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns true if this type is a List type.

List types include List, LargeList, FixedSizeList, ListView, and LargeListView.

<a id="op-ffc3395d06a9be639b78b1d4"></a>
## is_nested

`function` · `arrow_schema::datatype::DataType::is_nested` · arrow-schema 59.3.0

```rust
fn is_nested(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::DataType", "path": "DataType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [516, 1], "end": [867, 2], "filename": "src/datatype.rs"}, "trait": null, "trait_path": null}`

Source: `src/datatype.rs:610`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns true if this type is nested (List, FixedSizeList, LargeList, ListView. LargeListView, Struct, Union,
or Map), or a dictionary of a nested type

<a id="op-0d23690d19759c4ebac0f8ac"></a>
## is_null

`function` · `arrow_schema::datatype::DataType::is_null` · arrow-schema 59.3.0

```rust
fn is_null(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::DataType", "path": "DataType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [516, 1], "end": [867, 2], "filename": "src/datatype.rs"}, "trait": null, "trait_path": null}`

Source: `src/datatype.rs:629`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns true if this type is DataType::Null.

<a id="op-0d9f26604dc295b9125ce1fb"></a>
## is_numeric

`function` · `arrow_schema::datatype::DataType::is_numeric` · arrow-schema 59.3.0

```rust
fn is_numeric(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::DataType", "path": "DataType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [516, 1], "end": [867, 2], "filename": "src/datatype.rs"}, "trait": null, "trait_path": null}`

Source: `src/datatype.rs:525`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns true if this type is numeric: (UInt*, Int*, Float*, Decimal*).

<a id="op-4a2ca4458c4443c7b93768ca"></a>
## is_primitive

`function` · `arrow_schema::datatype::DataType::is_primitive` · arrow-schema 59.3.0

```rust
fn is_primitive(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::DataType", "path": "DataType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [516, 1], "end": [867, 2], "filename": "src/datatype.rs"}, "trait": null, "trait_path": null}`

Source: `src/datatype.rs:519`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns true if the type is primitive: (numeric, temporal).

<a id="op-0cba154003aa94e7c43a6933"></a>
## is_run_ends_type

`function` · `arrow_schema::datatype::DataType::is_run_ends_type` · arrow-schema 59.3.0

```rust
fn is_run_ends_type(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::DataType", "path": "DataType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [516, 1], "end": [867, 2], "filename": "src/datatype.rs"}, "trait": null, "trait_path": null}`

Source: `src/datatype.rs:602`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns true if this type is valid for run-ends array in RunArray

<a id="op-685a3815d851c5a56f2842fb"></a>
## is_signed_integer

`function` · `arrow_schema::datatype::DataType::is_signed_integer` · arrow-schema 59.3.0

```rust
fn is_signed_integer(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::DataType", "path": "DataType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [516, 1], "end": [867, 2], "filename": "src/datatype.rs"}, "trait": null, "trait_path": null}`

Source: `src/datatype.rs:572`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns true if this type is signed integer: (Int*).

<a id="op-eb30eb5993e7e269fa541a13"></a>
## is_string

`function` · `arrow_schema::datatype::DataType::is_string` · arrow-schema 59.3.0

```rust
fn is_string(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::DataType", "path": "DataType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [516, 1], "end": [867, 2], "filename": "src/datatype.rs"}, "trait": null, "trait_path": null}`

Source: `src/datatype.rs:636`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns true if this type is a String type

<a id="op-abc9d1bed5329314317bff21"></a>
## is_temporal

`function` · `arrow_schema::datatype::DataType::is_temporal` · arrow-schema 59.3.0

```rust
fn is_temporal(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::DataType", "path": "DataType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [516, 1], "end": [867, 2], "filename": "src/datatype.rs"}, "trait": null, "trait_path": null}`

Source: `src/datatype.rs:549`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns true if this type is temporal: (Date*, Time*, Duration, or Interval).

<a id="op-8d182fa17b25415bf0814583"></a>
## is_unsigned_integer

`function` · `arrow_schema::datatype::DataType::is_unsigned_integer` · arrow-schema 59.3.0

```rust
fn is_unsigned_integer(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::DataType", "path": "DataType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [516, 1], "end": [867, 2], "filename": "src/datatype.rs"}, "trait": null, "trait_path": null}`

Source: `src/datatype.rs:579`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns true if this type is unsigned integer: (UInt*).

<a id="op-d1367f37ac72a629eef92b93"></a>
## new_fixed_size_list

`function` · `arrow_schema::datatype::DataType::new_fixed_size_list` · arrow-schema 59.3.0

```rust
fn new_fixed_size_list(data_type: DataType, size: i32, nullable: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::DataType", "path": "DataType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [516, 1], "end": [867, 2], "filename": "src/datatype.rs"}, "trait": null, "trait_path": null}`

Source: `src/datatype.rs:864`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Create a [`DataType::FixedSizeList`](../operations/arrow_schema.datatype.DataType.md#op-69f9b11fdd4d79bf23810b3a) with elements of the specified type, size
and nullability, and conventionally named inner [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf) (`"item"`).

To specify field level metadata, construct the inner [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf)
directly via [`Field::new`](../operations/arrow_schema.field.Field.md#op-269c6779855ec2f777e61a89) or [`Field::new_list_field`](../operations/arrow_schema.field.Field.md#op-6aa98c17b73325760850240d).

<a id="op-69ea80c19ccb47a481623873"></a>
## new_large_list

`function` · `arrow_schema::datatype::DataType::new_large_list` · arrow-schema 59.3.0

```rust
fn new_large_list(data_type: DataType, nullable: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::DataType", "path": "DataType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [516, 1], "end": [867, 2], "filename": "src/datatype.rs"}, "trait": null, "trait_path": null}`

Source: `src/datatype.rs:855`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Create a [`DataType::LargeList`](../operations/arrow_schema.datatype.DataType.md#op-f0e628cee644a7235d5b4c1d) with elements of the specified type
and nullability, and conventionally named inner [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf) (`"item"`).

To specify field level metadata, construct the inner [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf)
directly via [`Field::new`](../operations/arrow_schema.field.Field.md#op-269c6779855ec2f777e61a89) or [`Field::new_list_field`](../operations/arrow_schema.field.Field.md#op-6aa98c17b73325760850240d).

<a id="op-3868713d2218b77ba4bd77c1"></a>
## new_list

`function` · `arrow_schema::datatype::DataType::new_list` · arrow-schema 59.3.0

```rust
fn new_list(data_type: DataType, nullable: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::DataType", "path": "DataType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [516, 1], "end": [867, 2], "filename": "src/datatype.rs"}, "trait": null, "trait_path": null}`

Source: `src/datatype.rs:846`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Create a [`DataType::List`](../operations/arrow_schema.datatype.DataType.md#op-83ec578cb0e12f00905856b8) with elements of the specified type
and nullability, and conventionally named inner [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf) (`"item"`).

To specify field level metadata, construct the inner [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf)
directly via [`Field::new`](../operations/arrow_schema.field.Field.md#op-269c6779855ec2f777e61a89) or [`Field::new_list_field`](../operations/arrow_schema.field.Field.md#op-6aa98c17b73325760850240d).

<a id="op-c94654beb6daadb6b513dec2"></a>
## partial_cmp

`function` · `arrow_schema::datatype::DataType::partial_cmp` · arrow-schema 59.3.0

```rust
fn partial_cmp(&self, other: &DataType) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::DataType", "path": "DataType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 45], "end": [94, 55], "filename": "src/datatype.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/datatype.rs:94`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f00c758d0194cf544cffc527"></a>
## primitive_width

`function` · `arrow_schema::datatype::DataType::primitive_width` · arrow-schema 59.3.0

```rust
fn primitive_width(&self) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::DataType", "path": "DataType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [516, 1], "end": [867, 2], "filename": "src/datatype.rs"}, "trait": null, "trait_path": null}`

Source: `src/datatype.rs:725`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns the byte width of this type if it is a primitive type

Returns `None` if not a primitive type

<a id="op-61c54c27298385e665de40ba"></a>
## serialize

`function` · `arrow_schema::datatype::DataType::serialize` · arrow-schema 59.3.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::DataType", "path": "DataType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 38], "end": [95, 54], "filename": "src/datatype.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/datatype.rs:95`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-91afec408bc7d0efadc50555"></a>
## size

`function` · `arrow_schema::datatype::DataType::size` · arrow-schema 59.3.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::DataType", "path": "DataType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [516, 1], "end": [867, 2], "filename": "src/datatype.rs"}, "trait": null, "trait_path": null}`

Source: `src/datatype.rs:763`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Return size of this instance in bytes.

Includes the size of `Self`.

<a id="op-064c5b209ea991f4ee88d3f1"></a>
## try_from

`function` · `arrow_schema::datatype::DataType::try_from` · arrow-schema 59.3.0

```rust
fn try_from(value: &str) -> Result<Self, Self::Error>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::DataType", "path": "DataType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [508, 1], "end": [514, 2], "filename": "src/datatype.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"primitive": "str"}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/datatype.rs:511`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7442944bd7bd885ed66b7e4f"></a>
## try_from

`function` · `arrow_schema::datatype::DataType::try_from` · arrow-schema 59.3.0

```rust
fn try_from(c_schema: &FFI_ArrowSchema) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::DataType", "path": "crate::DataType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [429, 1], "end": [645, 2], "filename": "src/ffi.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "arrow_schema::ffi::FFI_ArrowSchema", "path": "FFI_ArrowSchema"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/ffi.rs:433`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

See [CDataInterface docs](https://arrow.apache.org/docs/format/CDataInterface.html#data-type-description-format-strings)
