# `parquet::data_type::ByteArrayType`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.data_type.ByteArrayType.json).

<a id="op-7e5a177c2ae9a1b303fb5668"></a>
## ByteArrayType

`struct` · `parquet::data_type::ByteArrayType` · parquet 59.3.0

```rust
struct ByteArrayType
```

Source: `src/data_type.rs:1305`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Parquet physical type: ByteArrayType

<a id="op-c90dc3c44cbb10d693e2a95f"></a>
## T

`assoc_type` · `parquet::data_type::ByteArrayType::T` · parquet 59.3.0

```rust
T
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::ByteArrayType", "path": "ByteArrayType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1305, 1], "end": [1311, 2], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::DataType", "path": "DataType"}, "trait_path": "parquet::data_type::DataType"}`

Source: `src/data_type.rs:1305`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8bd1e3ab23fb308cdb4c7640"></a>
## clone

`function` · `parquet::data_type::ByteArrayType::clone` · parquet 59.3.0

```rust
fn clone(&self) -> ByteArrayType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::ByteArrayType", "path": "ByteArrayType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1305, 1], "end": [1311, 2], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/data_type.rs:1305`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-453cd4b8978162c6644dabd3"></a>
## get_column_reader

`function` · `parquet::data_type::ByteArrayType::get_column_reader` · parquet 59.3.0

```rust
fn get_column_reader(column_reader: ColumnReader) -> Option<ColumnReaderImpl<Self>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::ByteArrayType", "path": "ByteArrayType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1305, 1], "end": [1311, 2], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::DataType", "path": "DataType"}, "trait_path": "parquet::data_type::DataType"}`

Source: `src/data_type.rs:1305`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2d0f4c4bc3f3f019ba101174"></a>
## get_column_writer

`function` · `parquet::data_type::ByteArrayType::get_column_writer` · parquet 59.3.0

```rust
fn get_column_writer(column_writer: ColumnWriter<'_>) -> Option<ColumnWriterImpl<'_, Self>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::ByteArrayType", "path": "ByteArrayType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1305, 1], "end": [1311, 2], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::DataType", "path": "DataType"}, "trait_path": "parquet::data_type::DataType"}`

Source: `src/data_type.rs:1305`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8bdd9ea9332701dce63f6762"></a>
## get_column_writer_mut

`function` · `parquet::data_type::ByteArrayType::get_column_writer_mut` · parquet 59.3.0

```rust
fn get_column_writer_mut<'a, 'b: 'a>(column_writer: &'a mut ColumnWriter<'b>) -> Option<&'a mut ColumnWriterImpl<'b, Self>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::ByteArrayType", "path": "ByteArrayType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1305, 1], "end": [1311, 2], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::DataType", "path": "DataType"}, "trait_path": "parquet::data_type::DataType"}`

Source: `src/data_type.rs:1305`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1896426d1c796c5d74d95a27"></a>
## get_column_writer_ref

`function` · `parquet::data_type::ByteArrayType::get_column_writer_ref` · parquet 59.3.0

```rust
fn get_column_writer_ref<'a, 'b: 'a>(column_writer: &'a ColumnWriter<'b>) -> Option<&'a ColumnWriterImpl<'b, Self>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::ByteArrayType", "path": "ByteArrayType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1305, 1], "end": [1311, 2], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::DataType", "path": "DataType"}, "trait_path": "parquet::data_type::DataType"}`

Source: `src/data_type.rs:1305`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-21f37d63a6d5233eb94c6ba1"></a>
## get_type_size

`function` · `parquet::data_type::ByteArrayType::get_type_size` · parquet 59.3.0

```rust
fn get_type_size() -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::ByteArrayType", "path": "ByteArrayType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1305, 1], "end": [1311, 2], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::DataType", "path": "DataType"}, "trait_path": "parquet::data_type::DataType"}`

Source: `src/data_type.rs:1305`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
