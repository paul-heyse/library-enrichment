# `parquet::basic::LogicalType`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.basic.LogicalType.json).

<a id="op-20280a63f013d6b86b4da0d5"></a>
## LogicalType

`enum` · `parquet::basic::LogicalType` · parquet 59.3.0

```rust
enum LogicalType
```

Source: `src/basic.rs:256`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Logical types used by version 2.4.0+ of the Parquet format.

This is an *entirely new* struct as of version
4.0.0. The struct previously named `LogicalType` was renamed to
[`ConvertedType`](../operations/parquet.basic.ConvertedType.md#op-89a0d32ad627c75d0804c1a4). Please see the README.md for more details.

<a id="op-bd4082aa4fc1b981d7bfbb1d"></a>
## Bson

`variant` · `parquet::basic::LogicalType::Bson` · parquet 59.3.0

```rust
Bson
```

Source: `src/basic.rs:256`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A BSON document.

<a id="op-17898aaa1bcad3853c19d7b9"></a>
## Date

`variant` · `parquet::basic::LogicalType::Date` · parquet 59.3.0

```rust
Date
```

Source: `src/basic.rs:256`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A date stored as days since Unix epoch.

<a id="op-7abe1af3518b631c02ea366f"></a>
## Decimal

`variant` · `parquet::basic::LogicalType::Decimal` · parquet 59.3.0

```rust
Decimal
```

Source: `src/basic.rs:256`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A decimal value with a specified scale and precision.

<a id="op-a1f2e007a4b5850434c08a45"></a>
## Enum

`variant` · `parquet::basic::LogicalType::Enum` · parquet 59.3.0

```rust
Enum
```

Source: `src/basic.rs:256`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A set of predefined values.

<a id="op-5c5675700e0adeee24840fd0"></a>
## Err

`assoc_type` · `parquet::basic::LogicalType::Err` · parquet 59.3.0

```rust
Err
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::LogicalType", "path": "LogicalType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1320, 1], "end": [1352, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/basic.rs:1321`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9f97e5a010f5fa7a2c9ae09a"></a>
## Float16

`variant` · `parquet::basic::LogicalType::Float16` · parquet 59.3.0

```rust
Float16
```

Source: `src/basic.rs:256`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A 16-bit floating point number.

<a id="op-d8f7db3ce93041ddf6ddf923"></a>
## Geography

`variant` · `parquet::basic::LogicalType::Geography` · parquet 59.3.0

```rust
Geography
```

Source: `src/basic.rs:256`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A geospatial feature in the WKB format with an explicit (non-linear/non-planar) edges interpolation.

<a id="op-2b6536285d9ee92393941578"></a>
## Geometry

`variant` · `parquet::basic::LogicalType::Geometry` · parquet 59.3.0

```rust
Geometry
```

Source: `src/basic.rs:256`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A geospatial feature in the Well-Known Binary (WKB) format with linear/planar edges interpolation.

<a id="op-72ffa2951526b9cd21969168"></a>
## Integer

`variant` · `parquet::basic::LogicalType::Integer` · parquet 59.3.0

```rust
Integer
```

Source: `src/basic.rs:256`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

An integer with a specified bit width and signedness.

<a id="op-7ee4884bc4672ab95aebb650"></a>
## Json

`variant` · `parquet::basic::LogicalType::Json` · parquet 59.3.0

```rust
Json
```

Source: `src/basic.rs:256`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A JSON document.

<a id="op-41801309199209a71bdf7a8b"></a>
## List

`variant` · `parquet::basic::LogicalType::List` · parquet 59.3.0

```rust
List
```

Source: `src/basic.rs:256`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A list of elements.

<a id="op-f03e83f9ad324f74e4991dd9"></a>
## Map

`variant` · `parquet::basic::LogicalType::Map` · parquet 59.3.0

```rust
Map
```

Source: `src/basic.rs:256`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A map of key-value pairs.

<a id="op-343aca54338786e94cf56418"></a>
## String

`variant` · `parquet::basic::LogicalType::String` · parquet 59.3.0

```rust
String
```

Source: `src/basic.rs:256`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A UTF8 encoded string.

<a id="op-bc301f08c88c95d399002fa6"></a>
## Time

`variant` · `parquet::basic::LogicalType::Time` · parquet 59.3.0

```rust
Time
```

Source: `src/basic.rs:256`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A time stored as [`TimeUnit`](../operations/parquet.basic.TimeUnit.md#op-1e7a48b4288c785dfda1d22d) since midnight.

<a id="op-fa4c74e698ef30fc30113993"></a>
## Timestamp

`variant` · `parquet::basic::LogicalType::Timestamp` · parquet 59.3.0

```rust
Timestamp
```

Source: `src/basic.rs:256`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A timestamp stored as [`TimeUnit`](../operations/parquet.basic.TimeUnit.md#op-1e7a48b4288c785dfda1d22d) since Unix epoch.

<a id="op-c673826eabc7f1efd1dc48b7"></a>
## Unknown

`variant` · `parquet::basic::LogicalType::Unknown` · parquet 59.3.0

```rust
Unknown
```

Source: `src/basic.rs:256`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

An unknown logical type.

<a id="op-e383f54ae5815756a4efe897"></a>
## Uuid

`variant` · `parquet::basic::LogicalType::Uuid` · parquet 59.3.0

```rust
Uuid
```

Source: `src/basic.rs:256`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A UUID.

<a id="op-9ffad32eecdf62b793cc9b15"></a>
## Variant

`variant` · `parquet::basic::LogicalType::Variant` · parquet 59.3.0

```rust
Variant
```

Source: `src/basic.rs:256`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A Variant value.

<a id="op-09e63ec6cc8208b65cdc9909"></a>
## _Unknown

`variant` · `parquet::basic::LogicalType::_Unknown` · parquet 59.3.0

```rust
_Unknown
```

Source: `src/basic.rs:256`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e17b22e7393a2bd3bad3689f"></a>
## clone

`function` · `parquet::basic::LogicalType::clone` · parquet 59.3.0

```rust
fn clone(&self) -> LogicalType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::LogicalType", "path": "LogicalType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [256, 1], "end": [299, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/basic.rs:256`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5a1e26b6c9313421625fbe40"></a>
## decimal

`function` · `parquet::basic::LogicalType::decimal` · parquet 59.3.0

```rust
fn decimal(scale: i32, precision: i32) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::LogicalType", "path": "LogicalType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [301, 1], "end": [347, 2], "filename": "src/basic.rs"}, "trait": null, "trait_path": null}`

Source: `src/basic.rs:311`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a [`LogicalType::Decimal`](../operations/parquet.basic.LogicalType.md#op-7abe1af3518b631c02ea366f) variant with the given `scale` and `precision`

<a id="op-915e5900ee9e9367474bcbf2"></a>
## eq

`function` · `parquet::basic::LogicalType::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &LogicalType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::LogicalType", "path": "LogicalType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [256, 1], "end": [299, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/basic.rs:256`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c5c65bad49a869c1bbe77cf9"></a>
## fmt

`function` · `parquet::basic::LogicalType::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::LogicalType", "path": "LogicalType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [256, 1], "end": [299, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/basic.rs:256`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7027c6405f0a691381eee8f0"></a>
## from_str

`function` · `parquet::basic::LogicalType::from_str` · parquet 59.3.0

```rust
fn from_str(s: &str) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::LogicalType", "path": "LogicalType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1320, 1], "end": [1352, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/basic.rs:1323`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1bc2099143be22bd38cf3b0c"></a>
## geography

`function` · `parquet::basic::LogicalType::geography` · parquet 59.3.0

```rust
fn geography(crs: Option<String>, algorithm: Option<EdgeInterpolationAlgorithm>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::LogicalType", "path": "LogicalType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [301, 1], "end": [347, 2], "filename": "src/basic.rs"}, "trait": null, "trait_path": null}`

Source: `src/basic.rs:344`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a [`LogicalType::Geography`](../operations/parquet.basic.LogicalType.md#op-d8f7db3ce93041ddf6ddf923) variant with the given `crs` and `algorithm`

<a id="op-ed43db239261991127da19c1"></a>
## geometry

`function` · `parquet::basic::LogicalType::geometry` · parquet 59.3.0

```rust
fn geometry(crs: Option<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::LogicalType", "path": "LogicalType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [301, 1], "end": [347, 2], "filename": "src/basic.rs"}, "trait": null, "trait_path": null}`

Source: `src/basic.rs:339`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a [`LogicalType::Geometry`](../operations/parquet.basic.LogicalType.md#op-2b6536285d9ee92393941578) variant with the given `crs`

<a id="op-cc1d2776db46d4cf45850484"></a>
## integer

`function` · `parquet::basic::LogicalType::integer` · parquet 59.3.0

```rust
fn integer(bit_width: i8, is_signed: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::LogicalType", "path": "LogicalType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [301, 1], "end": [347, 2], "filename": "src/basic.rs"}, "trait": null, "trait_path": null}`

Source: `src/basic.rs:303`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a [`LogicalType::Integer`](../operations/parquet.basic.LogicalType.md#op-72ffa2951526b9cd21969168) variant with the given `bit_width` and `is_signed`

<a id="op-94eaed4bd52a081b240ac8da"></a>
## time

`function` · `parquet::basic::LogicalType::time` · parquet 59.3.0

```rust
fn time(is_adjusted_to_u_t_c: bool, unit: TimeUnit) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::LogicalType", "path": "LogicalType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [301, 1], "end": [347, 2], "filename": "src/basic.rs"}, "trait": null, "trait_path": null}`

Source: `src/basic.rs:316`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a [`LogicalType::Time`](../operations/parquet.basic.LogicalType.md#op-bc301f08c88c95d399002fa6) variant with the given `is_adjusted_to_u_t_c` and `unit`

<a id="op-767ea7a9fd40207a44617e7e"></a>
## timestamp

`function` · `parquet::basic::LogicalType::timestamp` · parquet 59.3.0

```rust
fn timestamp(is_adjusted_to_u_t_c: bool, unit: TimeUnit) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::LogicalType", "path": "LogicalType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [301, 1], "end": [347, 2], "filename": "src/basic.rs"}, "trait": null, "trait_path": null}`

Source: `src/basic.rs:324`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a [`LogicalType::Timestamp`](../operations/parquet.basic.LogicalType.md#op-fa4c74e698ef30fc30113993) variant with the given `is_adjusted_to_u_t_c` and `unit`

<a id="op-1c70a3e6e97551af71d92ab5"></a>
## variant

`function` · `parquet::basic::LogicalType::variant` · parquet 59.3.0

```rust
fn variant(specification_version: Option<i8>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::LogicalType", "path": "LogicalType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [301, 1], "end": [347, 2], "filename": "src/basic.rs"}, "trait": null, "trait_path": null}`

Source: `src/basic.rs:332`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a [`LogicalType::Variant`](../operations/parquet.basic.LogicalType.md#op-9ffad32eecdf62b793cc9b15) variant with the given `specification_version`
