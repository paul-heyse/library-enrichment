# `parquet::basic::ConvertedType`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.basic.ConvertedType.json).

<a id="op-89a0d32ad627c75d0804c1a4"></a>
## ConvertedType

`enum` · `parquet::basic::ConvertedType` · parquet 59.3.0

```rust
enum ConvertedType
```

Source: `src/basic.rs:70`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Common types (converted types) used by frameworks when using Parquet.

This helps map between types in those frameworks to the base types in Parquet.
This is only metadata and not needed to read or write the data.

This struct was renamed from `LogicalType` in version 4.0.0.
If targeting Parquet format 2.4.0 or above, please use [LogicalType](../operations/parquet.basic.LogicalType.md#op-20280a63f013d6b86b4da0d5) instead.

<a id="op-2ca0f9e39fab7cb77cfc1f03"></a>
## BSON

`variant` · `parquet::basic::ConvertedType::BSON` · parquet 59.3.0

```rust
BSON
```

Source: `src/basic.rs:70`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A BSON document embedded within a single BINARY column.

<a id="op-6437b9b594f58c87e8d52ad7"></a>
## DATE

`variant` · `parquet::basic::ConvertedType::DATE` · parquet 59.3.0

```rust
DATE
```

Source: `src/basic.rs:70`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A date stored as days since Unix epoch, encoded as the INT32 physical type.

<a id="op-d5162a4c7c47bd978a137b16"></a>
## DECIMAL

`variant` · `parquet::basic::ConvertedType::DECIMAL` · parquet 59.3.0

```rust
DECIMAL
```

Source: `src/basic.rs:70`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A decimal value.

This may be used to annotate BYTE_ARRAY or FIXED_LEN_BYTE_ARRAY primitive
types. The underlying byte array stores the unscaled value encoded as two's
complement using big-endian byte order (the most significant byte is the
zeroth element). The value of the decimal is the value * 10^{-scale}.

This must be accompanied by a (maximum) precision and a scale in the
SchemaElement. The precision specifies the number of digits in the decimal
and the scale stores the location of the decimal point. For example 1.23
would have precision 3 (3 total digits) and scale 2 (the decimal point is
2 digits over).

<a id="op-94b0336dbe4051018aa0b2a8"></a>
## ENUM

`variant` · `parquet::basic::ConvertedType::ENUM` · parquet 59.3.0

```rust
ENUM
```

Source: `src/basic.rs:70`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

An enum is converted into a BYTE_ARRAY field

<a id="op-dbfd5dbb8b6b5e4d123b6c28"></a>
## Err

`assoc_type` · `parquet::basic::ConvertedType::Err` · parquet 59.3.0

```rust
Err
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::ConvertedType", "path": "ConvertedType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1287, 1], "end": [1318, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/basic.rs:1288`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb43ad9f9fae82027e41956d"></a>
## INTERVAL

`variant` · `parquet::basic::ConvertedType::INTERVAL` · parquet 59.3.0

```rust
INTERVAL
```

Source: `src/basic.rs:70`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

An interval of time

This type annotates data stored as a FIXED_LEN_BYTE_ARRAY of length 12.
This data is composed of three separate little endian unsigned integers.
Each stores a component of a duration of time. The first integer identifies
the number of months associated with the duration, the second identifies
the number of days associated with the duration and the third identifies
the number of milliseconds associated with the provided duration.
This duration of time is independent of any particular timezone or date.

<a id="op-ef3113dd92f4b11ccd6e3d0d"></a>
## INT_16

`variant` · `parquet::basic::ConvertedType::INT_16` · parquet 59.3.0

```rust
INT_16
```

Source: `src/basic.rs:70`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A signed 16 bit integer value stored as INT32 physical type.

<a id="op-ee7c538961bd6382e5d70a6c"></a>
## INT_32

`variant` · `parquet::basic::ConvertedType::INT_32` · parquet 59.3.0

```rust
INT_32
```

Source: `src/basic.rs:70`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A signed 32 bit integer value stored as INT32 physical type.

<a id="op-af001a1e9d5b1b0dcefd8dc2"></a>
## INT_64

`variant` · `parquet::basic::ConvertedType::INT_64` · parquet 59.3.0

```rust
INT_64
```

Source: `src/basic.rs:70`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A signed 64 bit integer value stored as INT64 physical type.

<a id="op-b9d69268285541ec8d5c683a"></a>
## INT_8

`variant` · `parquet::basic::ConvertedType::INT_8` · parquet 59.3.0

```rust
INT_8
```

Source: `src/basic.rs:70`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A signed 8 bit integer value stored as INT32 physical type.

<a id="op-71a72baabc24b951940b7f70"></a>
## JSON

`variant` · `parquet::basic::ConvertedType::JSON` · parquet 59.3.0

```rust
JSON
```

Source: `src/basic.rs:70`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A JSON document embedded within a single UTF8 column.

<a id="op-25f3095aa83006ecabba1335"></a>
## LIST

`variant` · `parquet::basic::ConvertedType::LIST` · parquet 59.3.0

```rust
LIST
```

Source: `src/basic.rs:70`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A list is converted into an optional field containing a repeated field for its
values.

<a id="op-38c87ff699b423fc8a1ccdb4"></a>
## MAP

`variant` · `parquet::basic::ConvertedType::MAP` · parquet 59.3.0

```rust
MAP
```

Source: `src/basic.rs:70`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A map is converted as an optional field containing a repeated key/value pair.

<a id="op-3df51afacded52e38ebfe666"></a>
## MAP_KEY_VALUE

`variant` · `parquet::basic::ConvertedType::MAP_KEY_VALUE` · parquet 59.3.0

```rust
MAP_KEY_VALUE
```

Source: `src/basic.rs:70`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A key/value pair is converted into a group of two fields.

<a id="op-3e04259246293bde01177829"></a>
## MAX_DISCRIMINANT

`assoc_const` · `parquet::basic::ConvertedType::MAX_DISCRIMINANT` · parquet 59.3.0

```rust
MAX_DISCRIMINANT
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::ConvertedType", "path": "ConvertedType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [172, 2], "filename": "src/basic.rs"}, "trait": null, "trait_path": null}`

Source: `src/basic.rs:70`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the largest discriminant value defined for this enum.

<a id="op-d426f95490be07d91a7795f6"></a>
## NONE

`variant` · `parquet::basic::ConvertedType::NONE` · parquet 59.3.0

```rust
NONE
```

Source: `src/basic.rs:70`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Not defined in the spec, used internally to indicate no type conversion

<a id="op-2b11581e258f8c37619f6d25"></a>
## TIMESTAMP_MICROS

`variant` · `parquet::basic::ConvertedType::TIMESTAMP_MICROS` · parquet 59.3.0

```rust
TIMESTAMP_MICROS
```

Source: `src/basic.rs:70`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Date and time recorded as microseconds since the Unix epoch.
The value is stored as an INT64 physical type.

<a id="op-b09732907c0bbe4147bdc3d8"></a>
## TIMESTAMP_MILLIS

`variant` · `parquet::basic::ConvertedType::TIMESTAMP_MILLIS` · parquet 59.3.0

```rust
TIMESTAMP_MILLIS
```

Source: `src/basic.rs:70`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Date and time recorded as milliseconds since the Unix epoch.
Recorded as a physical type of INT64.

<a id="op-b7bb1f621ed32ac271856309"></a>
## TIME_MICROS

`variant` · `parquet::basic::ConvertedType::TIME_MICROS` · parquet 59.3.0

```rust
TIME_MICROS
```

Source: `src/basic.rs:70`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

The total number of microseconds since midnight. The value is stored as an INT64
physical type.

<a id="op-e544b28e4dd4540f2e3e8956"></a>
## TIME_MILLIS

`variant` · `parquet::basic::ConvertedType::TIME_MILLIS` · parquet 59.3.0

```rust
TIME_MILLIS
```

Source: `src/basic.rs:70`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

The total number of milliseconds since midnight. The value is stored as an INT32
physical type.

<a id="op-c75fa17d852cfe5cc174fea0"></a>
## UINT_16

`variant` · `parquet::basic::ConvertedType::UINT_16` · parquet 59.3.0

```rust
UINT_16
```

Source: `src/basic.rs:70`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

An unsigned 16 bit integer value stored as INT32 physical type.

<a id="op-6029540d81b985e24f2a3283"></a>
## UINT_32

`variant` · `parquet::basic::ConvertedType::UINT_32` · parquet 59.3.0

```rust
UINT_32
```

Source: `src/basic.rs:70`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

An unsigned 32 bit integer value stored as INT32 physical type.

<a id="op-ca83dfb9e7fa97d22d187592"></a>
## UINT_64

`variant` · `parquet::basic::ConvertedType::UINT_64` · parquet 59.3.0

```rust
UINT_64
```

Source: `src/basic.rs:70`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

An unsigned 64 bit integer value stored as INT64 physical type.

<a id="op-4577dabb128cf673b0836e4a"></a>
## UINT_8

`variant` · `parquet::basic::ConvertedType::UINT_8` · parquet 59.3.0

```rust
UINT_8
```

Source: `src/basic.rs:70`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

An unsigned 8 bit integer value stored as INT32 physical type.

<a id="op-c02c45b322f78769c8e69a7f"></a>
## UTF8

`variant` · `parquet::basic::ConvertedType::UTF8` · parquet 59.3.0

```rust
UTF8
```

Source: `src/basic.rs:70`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A BYTE_ARRAY actually contains UTF8 encoded chars.

<a id="op-b04695b4aacbd45dd061b428"></a>
## VARIANTS

`assoc_const` · `parquet::basic::ConvertedType::VARIANTS` · parquet 59.3.0

```rust
VARIANTS
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::ConvertedType", "path": "ConvertedType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [172, 2], "filename": "src/basic.rs"}, "trait": null, "trait_path": null}`

Source: `src/basic.rs:70`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns a slice containing every variant of this enum.

<a id="op-3dc3aef4ba24ced592d20d11"></a>
## clone

`function` · `parquet::basic::ConvertedType::clone` · parquet 59.3.0

```rust
fn clone(&self) -> ConvertedType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::ConvertedType", "path": "ConvertedType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [172, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/basic.rs:70`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-292bdc539f0086779f515f36"></a>
## cmp

`function` · `parquet::basic::ConvertedType::cmp` · parquet 59.3.0

```rust
fn cmp(&self, other: &ConvertedType) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::ConvertedType", "path": "ConvertedType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [172, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/basic.rs:70`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-de9366e24a4a46a30051bb4c"></a>
## eq

`function` · `parquet::basic::ConvertedType::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &ConvertedType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::ConvertedType", "path": "ConvertedType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [172, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/basic.rs:70`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b2a512645717b2c885c9b5d6"></a>
## fmt

`function` · `parquet::basic::ConvertedType::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::ConvertedType", "path": "ConvertedType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [172, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/basic.rs:70`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b6b8899b13ab5810d956d36f"></a>
## fmt

`function` · `parquet::basic::ConvertedType::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::ConvertedType", "path": "ConvertedType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [172, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/basic.rs:70`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6cd81b0db7c509d4cb7da165"></a>
## from

`function` · `parquet::basic::ConvertedType::from` · parquet 59.3.0

```rust
fn from(value: Option<LogicalType>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::ConvertedType", "path": "ConvertedType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1205, 1], "end": [1251, 2], "filename": "src/basic.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "parquet::basic::LogicalType", "path": "LogicalType"}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/basic.rs:1206`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4a5916a0eecf561a3d9523be"></a>
## from_str

`function` · `parquet::basic::ConvertedType::from_str` · parquet 59.3.0

```rust
fn from_str(s: &str) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::ConvertedType", "path": "ConvertedType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1287, 1], "end": [1318, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/basic.rs:1290`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c98977d06fd46afa43ee5c9"></a>
## hash

`function` · `parquet::basic::ConvertedType::hash` · parquet 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::ConvertedType", "path": "ConvertedType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [172, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/basic.rs:70`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ef708b88e0c9c59864a282f4"></a>
## partial_cmp

`function` · `parquet::basic::ConvertedType::partial_cmp` · parquet 59.3.0

```rust
fn partial_cmp(&self, other: &ConvertedType) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::ConvertedType", "path": "ConvertedType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [172, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/basic.rs:70`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
