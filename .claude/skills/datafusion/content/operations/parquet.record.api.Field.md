# `parquet::record::api::Field`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.record.api.Field.json).

<a id="op-8c36302d82a7b4539ef6b575"></a>
## Field

`enum` · `parquet::record::api::Field` · parquet 59.3.0

```rust
enum Field
```

Source: `src/record/api.rs:585`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

API to represent a single field in a `Row`.

<a id="op-0be3cc86d3a093818d50d500"></a>
## Bool

`variant` · `parquet::record::api::Field::Bool` · parquet 59.3.0

```rust
Bool
```

Source: `src/record/api.rs:590`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Boolean value (`true`, `false`).

<a id="op-d7898016e0baa2863fc66e2f"></a>
## Byte

`variant` · `parquet::record::api::Field::Byte` · parquet 59.3.0

```rust
Byte
```

Source: `src/record/api.rs:592`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Signed integer INT_8.

<a id="op-3538272c5a91c6cb06f18c59"></a>
## Bytes

`variant` · `parquet::record::api::Field::Bytes` · parquet 59.3.0

```rust
Bytes
```

Source: `src/record/api.rs:618`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

General binary value.

<a id="op-b76dea4f725977c708f173c8"></a>
## Date

`variant` · `parquet::record::api::Field::Date` · parquet 59.3.0

```rust
Date
```

Source: `src/record/api.rs:621`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Date without a time of day, stores the number of days from the
Unix epoch, 1 January 1970.

<a id="op-0bb00c8a87e580cd67ed8fb6"></a>
## Decimal

`variant` · `parquet::record::api::Field::Decimal` · parquet 59.3.0

```rust
Decimal
```

Source: `src/record/api.rs:614`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Decimal value.

<a id="op-b82aa8656b04278ef18ce7eb"></a>
## Double

`variant` · `parquet::record::api::Field::Double` · parquet 59.3.0

```rust
Double
```

Source: `src/record/api.rs:612`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

IEEE 64-bit floating point value.

<a id="op-492597e6918913df7b6a7394"></a>
## Float

`variant` · `parquet::record::api::Field::Float` · parquet 59.3.0

```rust
Float
```

Source: `src/record/api.rs:610`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

IEEE 32-bit floating point value.

<a id="op-54d3c80ee55bc28e2e9df7d9"></a>
## Float16

`variant` · `parquet::record::api::Field::Float16` · parquet 59.3.0

```rust
Float16
```

Source: `src/record/api.rs:608`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

IEEE 16-bit floating point value.

<a id="op-c145133f5d436df4a4d517a6"></a>
## Group

`variant` · `parquet::record::api::Field::Group` · parquet 59.3.0

```rust
Group
```

Source: `src/record/api.rs:636`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Struct, child elements are tuples of field-value pairs.

<a id="op-7cf0431f982045b59e0d7fe7"></a>
## Int

`variant` · `parquet::record::api::Field::Int` · parquet 59.3.0

```rust
Int
```

Source: `src/record/api.rs:596`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Signed integer INT_32.

<a id="op-17ff3f93125599e491349731"></a>
## ListInternal

`variant` · `parquet::record::api::Field::ListInternal` · parquet 59.3.0

```rust
ListInternal
```

Source: `src/record/api.rs:638`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

List of elements.

<a id="op-da0401563e3889e1b4821f8a"></a>
## Long

`variant` · `parquet::record::api::Field::Long` · parquet 59.3.0

```rust
Long
```

Source: `src/record/api.rs:598`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Signed integer INT_64.

<a id="op-336811aa9f6cce669332a196"></a>
## MapInternal

`variant` · `parquet::record::api::Field::MapInternal` · parquet 59.3.0

```rust
MapInternal
```

Source: `src/record/api.rs:640`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

List of key-value pairs.

<a id="op-36ec4dc88fb0cb653ac28dae"></a>
## Null

`variant` · `parquet::record::api::Field::Null` · parquet 59.3.0

```rust
Null
```

Source: `src/record/api.rs:588`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Null value.

<a id="op-f0dc68b421a89cf7e2bd8048"></a>
## Short

`variant` · `parquet::record::api::Field::Short` · parquet 59.3.0

```rust
Short
```

Source: `src/record/api.rs:594`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Signed integer INT_16.

<a id="op-f2fd4151c68497de56ae53fb"></a>
## Str

`variant` · `parquet::record::api::Field::Str` · parquet 59.3.0

```rust
Str
```

Source: `src/record/api.rs:616`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

UTF-8 encoded character string.

<a id="op-d22177426ec0e0f4a248cd85"></a>
## TimeMicros

`variant` · `parquet::record::api::Field::TimeMicros` · parquet 59.3.0

```rust
TimeMicros
```

Source: `src/record/api.rs:626`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

The total number of microseconds since midnight.

<a id="op-89b04dc98ccadc67df7468d2"></a>
## TimeMillis

`variant` · `parquet::record::api::Field::TimeMillis` · parquet 59.3.0

```rust
TimeMillis
```

Source: `src/record/api.rs:624`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

The total number of milliseconds since midnight.

<a id="op-798bbd03e19336f511918c5f"></a>
## TimestampMicros

`variant` · `parquet::record::api::Field::TimestampMicros` · parquet 59.3.0

```rust
TimestampMicros
```

Source: `src/record/api.rs:631`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Microseconds from the Unix epoch, 1 January 1970.

<a id="op-8fefb79bb85ff9684cf6e3ba"></a>
## TimestampMillis

`variant` · `parquet::record::api::Field::TimestampMillis` · parquet 59.3.0

```rust
TimestampMillis
```

Source: `src/record/api.rs:629`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Milliseconds from the Unix epoch, 1 January 1970.

<a id="op-05b293d29e316b6f93e48e64"></a>
## UByte

`variant` · `parquet::record::api::Field::UByte` · parquet 59.3.0

```rust
UByte
```

Source: `src/record/api.rs:600`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Unsigned integer UINT_8.

<a id="op-14e0e35b75614266dd6b32a7"></a>
## UInt

`variant` · `parquet::record::api::Field::UInt` · parquet 59.3.0

```rust
UInt
```

Source: `src/record/api.rs:604`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Unsigned integer UINT_32.

<a id="op-66221ba173f8d84421e75507"></a>
## ULong

`variant` · `parquet::record::api::Field::ULong` · parquet 59.3.0

```rust
ULong
```

Source: `src/record/api.rs:606`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Unsigned integer UINT_64.

<a id="op-6c71e8d9e7e05af6e321f539"></a>
## UShort

`variant` · `parquet::record::api::Field::UShort` · parquet 59.3.0

```rust
UShort
```

Source: `src/record/api.rs:602`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Unsigned integer UINT_16.

<a id="op-bbbe23d553c5d05ef7e51555"></a>
## clone

`function` · `parquet::record::api::Field::clone` · parquet 59.3.0

```rust
fn clone(&self) -> Field
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [584, 10], "end": [584, 15], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/record/api.rs:584`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7a0e940447dfec2cb67c8c81"></a>
## convert_bool

`function` · `parquet::record::api::Field::convert_bool` · parquet 59.3.0

```rust
fn convert_bool(_descr: &ColumnDescPtr, value: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [643, 1], "end": [847, 2], "filename": "src/record/api.rs"}, "trait": null, "trait_path": null}`

Source: `src/record/api.rs:684`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Converts Parquet BOOLEAN type with logical type into `bool` value.

<a id="op-850152589f94abd7c18f4716"></a>
## convert_byte_array

`function` · `parquet::record::api::Field::convert_byte_array` · parquet 59.3.0

```rust
fn convert_byte_array(descr: &ColumnDescPtr, value: ByteArray) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [643, 1], "end": [847, 2], "filename": "src/record/api.rs"}, "trait": null, "trait_path": null}`

Source: `src/record/api.rs:749`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Converts Parquet BYTE_ARRAY type with converted type into a UTF8
string, decimal, float16, or an array of bytes.

<a id="op-d679738ce65b3c3b53a3b1ba"></a>
## convert_double

`function` · `parquet::record::api::Field::convert_double` · parquet 59.3.0

```rust
fn convert_double(_descr: &ColumnDescPtr, value: f64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [643, 1], "end": [847, 2], "filename": "src/record/api.rs"}, "trait": null, "trait_path": null}`

Source: `src/record/api.rs:742`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Converts Parquet DOUBLE type with converted type into `f64` value.

<a id="op-2be121a2a39eec386f69b663"></a>
## convert_float

`function` · `parquet::record::api::Field::convert_float` · parquet 59.3.0

```rust
fn convert_float(_descr: &ColumnDescPtr, value: f32) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [643, 1], "end": [847, 2], "filename": "src/record/api.rs"}, "trait": null, "trait_path": null}`

Source: `src/record/api.rs:736`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Converts Parquet FLOAT type with logical type into `f32` value.

<a id="op-df2f88219381d360e3b42994"></a>
## convert_int32

`function` · `parquet::record::api::Field::convert_int32` · parquet 59.3.0

```rust
fn convert_int32(descr: &ColumnDescPtr, value: i32) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [643, 1], "end": [847, 2], "filename": "src/record/api.rs"}, "trait": null, "trait_path": null}`

Source: `src/record/api.rs:690`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Converts Parquet INT32 type with converted type into `i32` value.

<a id="op-451963401ef34fd82a9d0818"></a>
## convert_int64

`function` · `parquet::record::api::Field::convert_int64` · parquet 59.3.0

```rust
fn convert_int64(descr: &ColumnDescPtr, value: i64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [643, 1], "end": [847, 2], "filename": "src/record/api.rs"}, "trait": null, "trait_path": null}`

Source: `src/record/api.rs:711`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Converts Parquet INT64 type with converted type into `i64` value.

<a id="op-d2a1e9f19e25cb41579db60b"></a>
## convert_int96

`function` · `parquet::record::api::Field::convert_int96` · parquet 59.3.0

```rust
fn convert_int96(_descr: &ColumnDescPtr, value: Int96) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [643, 1], "end": [847, 2], "filename": "src/record/api.rs"}, "trait": null, "trait_path": null}`

Source: `src/record/api.rs:730`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Converts Parquet INT96 (nanosecond timestamps) type and logical type into
`Timestamp` value.

<a id="op-e06ad92c52b8286e9a76e0f0"></a>
## eq

`function` · `parquet::record::api::Field::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &Field) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [584, 24], "end": [584, 33], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/record/api.rs:584`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-49d27abf038c5b63f098a4da"></a>
## fmt

`function` · `parquet::record::api::Field::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [584, 17], "end": [584, 22], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/record/api.rs:584`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-756d1832c300807d0cb1151e"></a>
## fmt

`function` · `parquet::record::api::Field::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [849, 1], "end": [934, 2], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/record/api.rs:850`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0d52d406369944247f9d0fa5"></a>
## is_primitive

`function` · `parquet::record::api::Field::is_primitive` · parquet 59.3.0

```rust
fn is_primitive(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [643, 1], "end": [847, 2], "filename": "src/record/api.rs"}, "trait": null, "trait_path": null}`

Source: `src/record/api.rs:675`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Determines if this Row represents a primitive value.

<a id="op-dd36fa404a21e51a19883ae6"></a>
## to_json_value

`function` · `parquet::record::api::Field::to_json_value` · parquet 59.3.0

```rust
fn to_json_value(&self) -> Value
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [643, 1], "end": [847, 2], "filename": "src/record/api.rs"}, "trait": null, "trait_path": null}`

Source: `src/record/api.rs:796`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Converts the Parquet field into a JSON [`Value`].

Unresolved upstream links (retained, not inferred): ``Value``.
