# `parquet::file::metadata::KeyValue`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.metadata.KeyValue.json).

<a id="op-c5f599a409980a2c03049085"></a>
## KeyValue

`struct` · `parquet::file::metadata::KeyValue` · parquet 59.3.0

```rust
struct KeyValue
```

Source: `src/file/metadata/mod.rs:440`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A key-value pair for [`FileMetaData`](../operations/parquet.file.metadata.FileMetaData.md#op-b5fcc860595f5c38cea8c79d).

<a id="op-21e8cf89218839d0125d2526"></a>
## clone

`function` · `parquet::file::metadata::KeyValue::clone` · parquet 59.3.0

```rust
fn clone(&self) -> KeyValue
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::KeyValue", "path": "KeyValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [440, 1], "end": [446, 2], "filename": "src/file/metadata/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/file/metadata/mod.rs:440`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f1c094ead6b0c5aa5bb67b56"></a>
## eq

`function` · `parquet::file::metadata::KeyValue::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &KeyValue) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::KeyValue", "path": "KeyValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [440, 1], "end": [446, 2], "filename": "src/file/metadata/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/file/metadata/mod.rs:440`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0ad117ae3d141dfe069538bf"></a>
## fmt

`function` · `parquet::file::metadata::KeyValue::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::KeyValue", "path": "KeyValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [440, 1], "end": [446, 2], "filename": "src/file/metadata/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file/metadata/mod.rs:440`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7cc582612284b4d681d341e5"></a>
## key

`struct_field` · `parquet::file::metadata::KeyValue::key` · parquet 59.3.0

```rust
key: String
```

Source: `src/file/metadata/mod.rs:440`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-24202f96a53f7fba9d1f5067"></a>
## new

`function` · `parquet::file::metadata::KeyValue::new` · parquet 59.3.0

```rust
fn new<F2>(key: String, value: F2) -> KeyValue where F2: Into<Option<String>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::KeyValue", "path": "KeyValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [448, 1], "end": [459, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:450`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a new key value pair

<a id="op-71a3a4ff6da184d2b3bb019c"></a>
## value

`struct_field` · `parquet::file::metadata::KeyValue::value` · parquet 59.3.0

```rust
value: Option<String>
```

Source: `src/file/metadata/mod.rs:440`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
