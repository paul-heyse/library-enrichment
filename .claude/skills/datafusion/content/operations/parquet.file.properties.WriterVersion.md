# `parquet::file::properties::WriterVersion`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.properties.WriterVersion.json).

<a id="op-1c547ba99b6da23becd1e5d2"></a>
## WriterVersion

`enum` · `parquet::file::properties::WriterVersion` · parquet 59.3.0

```rust
enum WriterVersion
```

Source: `src/file/properties.rs:139`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Parquet writer version.

Basic constant, which is not part of the Thrift definition.

<a id="op-c0a03114aa856c162f8759b9"></a>
## Err

`assoc_type` · `parquet::file::properties::WriterVersion::Err` · parquet 59.3.0

```rust
Err
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterVersion", "path": "WriterVersion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [156, 1], "end": [166, 2], "filename": "src/file/properties.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/file/properties.rs:157`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3b930e02e781632fc7a684e3"></a>
## PARQUET_1_0

`variant` · `parquet::file::properties::WriterVersion::PARQUET_1_0` · parquet 59.3.0

```rust
PARQUET_1_0
```

Source: `src/file/properties.rs:141`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Parquet format version 1.0

<a id="op-4bd082fe2665b85b088af443"></a>
## PARQUET_2_0

`variant` · `parquet::file::properties::WriterVersion::PARQUET_2_0` · parquet 59.3.0

```rust
PARQUET_2_0
```

Source: `src/file/properties.rs:143`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Parquet format version 2.0

<a id="op-237dace63a64eec4284a387a"></a>
## as_num

`function` · `parquet::file::properties::WriterVersion::as_num` · parquet 59.3.0

```rust
fn as_num(&self) -> i32
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterVersion", "path": "WriterVersion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [146, 1], "end": [154, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:148`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns writer version as `i32`.

<a id="op-f491c1696d97c47da48be536"></a>
## clone

`function` · `parquet::file::properties::WriterVersion::clone` · parquet 59.3.0

```rust
fn clone(&self) -> WriterVersion
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterVersion", "path": "WriterVersion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 17], "end": [137, 22], "filename": "src/file/properties.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/file/properties.rs:137`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a34ab899d197d3b9adf289bf"></a>
## eq

`function` · `parquet::file::properties::WriterVersion::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &WriterVersion) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterVersion", "path": "WriterVersion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 30], "end": [137, 39], "filename": "src/file/properties.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/file/properties.rs:137`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-646560b76f7c7bb81ed1fc4e"></a>
## fmt

`function` · `parquet::file::properties::WriterVersion::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterVersion", "path": "WriterVersion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 10], "end": [137, 15], "filename": "src/file/properties.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file/properties.rs:137`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8c3df2be11d9df00c8cd724a"></a>
## from_str

`function` · `parquet::file::properties::WriterVersion::from_str` · parquet 59.3.0

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterVersion", "path": "WriterVersion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [156, 1], "end": [166, 2], "filename": "src/file/properties.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/file/properties.rs:159`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
