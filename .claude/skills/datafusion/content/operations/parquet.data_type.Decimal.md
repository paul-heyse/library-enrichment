# `parquet::data_type::Decimal`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.data_type.Decimal.json).

<a id="op-c76cde535e5f3f268048b168"></a>
## Decimal

`enum` · `parquet::data_type::Decimal` · parquet 59.3.0

```rust
enum Decimal
```

Source: `src/data_type.rs:422`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Rust representation for Decimal values.

This is not a representation of Parquet physical type, but rather a wrapper for
DECIMAL logical type, and serves as container for raw parts of decimal values:
unscaled value in bytes, precision and scale.

<a id="op-0aa60d0a717d62b02d409ada"></a>
## Bytes

`variant` · `parquet::data_type::Decimal::Bytes` · parquet 59.3.0

```rust
Bytes
```

Source: `src/data_type.rs:442`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Decimal backed by byte array.

<a id="op-398e59f5e5e0416ffa01e0a9"></a>
## Int32

`variant` · `parquet::data_type::Decimal::Int32` · parquet 59.3.0

```rust
Int32
```

Source: `src/data_type.rs:424`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Decimal backed by `i32`.

<a id="op-45fc93d0cc4ab2b31093bf14"></a>
## Int64

`variant` · `parquet::data_type::Decimal::Int64` · parquet 59.3.0

```rust
Int64
```

Source: `src/data_type.rs:433`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Decimal backed by `i64`.

<a id="op-b7c1b3cfc10d0f96f34e77e1"></a>
## as_bytes

`function` · `parquet::data_type::Decimal::as_bytes` · parquet 59.3.0

```rust
fn as_bytes(&self) -> &[u8]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::Decimal", "path": "Decimal"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [653, 1], "end": [657, 2], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::AsBytes", "path": "AsBytes"}, "trait_path": "parquet::data_type::AsBytes"}`

Source: `src/data_type.rs:654`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-025b2d1704b576354d6df33a"></a>
## clone

`function` · `parquet::data_type::Decimal::clone` · parquet 59.3.0

```rust
fn clone(&self) -> Decimal
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::Decimal", "path": "Decimal"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [421, 10], "end": [421, 15], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/data_type.rs:421`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2b53c965d95a39964c1db79c"></a>
## data

`function` · `parquet::data_type::Decimal::data` · parquet 59.3.0

```rust
fn data(&self) -> &[u8]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::Decimal", "path": "Decimal"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [452, 1], "end": [508, 2], "filename": "src/data_type.rs"}, "trait": null, "trait_path": null}`

Source: `src/data_type.rs:483`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns bytes of unscaled value.

<a id="op-79314afcdef0db7a42d24372"></a>
## default

`function` · `parquet::data_type::Decimal::default` · parquet 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::Decimal", "path": "Decimal"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [510, 1], "end": [514, 2], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/data_type.rs:511`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d566cbb11787162a4ec23464"></a>
## eq

`function` · `parquet::data_type::Decimal::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &Decimal) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::Decimal", "path": "Decimal"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [516, 1], "end": [522, 2], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/data_type.rs:517`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-299f98c85c9609a8406bb20d"></a>
## fmt

`function` · `parquet::data_type::Decimal::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::Decimal", "path": "Decimal"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [421, 17], "end": [421, 22], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/data_type.rs:421`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-59f0eb8843e10f499ec751a2"></a>
## from_bytes

`function` · `parquet::data_type::Decimal::from_bytes` · parquet 59.3.0

```rust
fn from_bytes(value: ByteArray, precision: i32, scale: i32) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::Decimal", "path": "Decimal"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [452, 1], "end": [508, 2], "filename": "src/data_type.rs"}, "trait": null, "trait_path": null}`

Source: `src/data_type.rs:474`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates new decimal value from `ByteArray`.

<a id="op-ab050bb814ba02ef4f741f84"></a>
## from_i32

`function` · `parquet::data_type::Decimal::from_i32` · parquet 59.3.0

```rust
fn from_i32(value: i32, precision: i32, scale: i32) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::Decimal", "path": "Decimal"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [452, 1], "end": [508, 2], "filename": "src/data_type.rs"}, "trait": null, "trait_path": null}`

Source: `src/data_type.rs:454`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates new decimal value from `i32`.

<a id="op-0c5f1eb90f8cc92c4ee07eba"></a>
## from_i64

`function` · `parquet::data_type::Decimal::from_i64` · parquet 59.3.0

```rust
fn from_i64(value: i64, precision: i32, scale: i32) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::Decimal", "path": "Decimal"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [452, 1], "end": [508, 2], "filename": "src/data_type.rs"}, "trait": null, "trait_path": null}`

Source: `src/data_type.rs:464`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates new decimal value from `i64`.

<a id="op-7d3d260cb2b424945fce9606"></a>
## precision

`function` · `parquet::data_type::Decimal::precision` · parquet 59.3.0

```rust
fn precision(&self) -> i32
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::Decimal", "path": "Decimal"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [452, 1], "end": [508, 2], "filename": "src/data_type.rs"}, "trait": null, "trait_path": null}`

Source: `src/data_type.rs:492`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns decimal precision.

<a id="op-8cb2406eb8c2aadbf9647665"></a>
## scale

`function` · `parquet::data_type::Decimal::scale` · parquet 59.3.0

```rust
fn scale(&self) -> i32
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::Decimal", "path": "Decimal"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [452, 1], "end": [508, 2], "filename": "src/data_type.rs"}, "trait": null, "trait_path": null}`

Source: `src/data_type.rs:501`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns decimal scale.
