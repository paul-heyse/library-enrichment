# `parquet::data_type::FixedLenByteArrayType`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.data_type.FixedLenByteArrayType.json).

<a id="op-d1b1dce03aced28c4dbc6f10"></a>
## FixedLenByteArrayType

`struct` · `parquet::data_type::FixedLenByteArrayType` · parquet 59.3.0

```rust
struct FixedLenByteArrayType
```

Source: `src/data_type.rs:1312`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Parquet physical type: FixedLenByteArrayType

<a id="op-6d6d606c2ab3654921bbc45c"></a>
## T

`assoc_type` · `parquet::data_type::FixedLenByteArrayType::T` · parquet 59.3.0

```rust
T
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::FixedLenByteArrayType", "path": "FixedLenByteArrayType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1312, 1], "end": [1318, 2], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::DataType", "path": "DataType"}, "trait_path": "parquet::data_type::DataType"}`

Source: `src/data_type.rs:1312`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-22865eadafcd67639c1f2dc8"></a>
## clone

`function` · `parquet::data_type::FixedLenByteArrayType::clone` · parquet 59.3.0

```rust
fn clone(&self) -> FixedLenByteArrayType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::FixedLenByteArrayType", "path": "FixedLenByteArrayType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1312, 1], "end": [1318, 2], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/data_type.rs:1312`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dfcf41dc90eb49a4be93fad7"></a>
## get_column_reader

`function` · `parquet::data_type::FixedLenByteArrayType::get_column_reader` · parquet 59.3.0

```rust
fn get_column_reader(column_reader: ColumnReader) -> Option<ColumnReaderImpl<Self>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::FixedLenByteArrayType", "path": "FixedLenByteArrayType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1312, 1], "end": [1318, 2], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::DataType", "path": "DataType"}, "trait_path": "parquet::data_type::DataType"}`

Source: `src/data_type.rs:1312`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c89ecc16a9754ec3d981b62d"></a>
## get_column_writer

`function` · `parquet::data_type::FixedLenByteArrayType::get_column_writer` · parquet 59.3.0

```rust
fn get_column_writer(column_writer: ColumnWriter<'_>) -> Option<ColumnWriterImpl<'_, Self>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::FixedLenByteArrayType", "path": "FixedLenByteArrayType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1312, 1], "end": [1318, 2], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::DataType", "path": "DataType"}, "trait_path": "parquet::data_type::DataType"}`

Source: `src/data_type.rs:1312`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8ac797aecaef77d89f3fbd50"></a>
## get_column_writer_mut

`function` · `parquet::data_type::FixedLenByteArrayType::get_column_writer_mut` · parquet 59.3.0

```rust
fn get_column_writer_mut<'a, 'b: 'a>(column_writer: &'a mut ColumnWriter<'b>) -> Option<&'a mut ColumnWriterImpl<'b, Self>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::FixedLenByteArrayType", "path": "FixedLenByteArrayType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1312, 1], "end": [1318, 2], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::DataType", "path": "DataType"}, "trait_path": "parquet::data_type::DataType"}`

Source: `src/data_type.rs:1312`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b106d7a4a92dbd8f603f65f"></a>
## get_column_writer_ref

`function` · `parquet::data_type::FixedLenByteArrayType::get_column_writer_ref` · parquet 59.3.0

```rust
fn get_column_writer_ref<'a, 'b: 'a>(column_writer: &'a ColumnWriter<'b>) -> Option<&'a ColumnWriterImpl<'b, Self>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::FixedLenByteArrayType", "path": "FixedLenByteArrayType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1312, 1], "end": [1318, 2], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::DataType", "path": "DataType"}, "trait_path": "parquet::data_type::DataType"}`

Source: `src/data_type.rs:1312`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-441e0a2a47909078183f63dc"></a>
## get_type_size

`function` · `parquet::data_type::FixedLenByteArrayType::get_type_size` · parquet 59.3.0

```rust
fn get_type_size() -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::FixedLenByteArrayType", "path": "FixedLenByteArrayType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1312, 1], "end": [1318, 2], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::DataType", "path": "DataType"}, "trait_path": "parquet::data_type::DataType"}`

Source: `src/data_type.rs:1312`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
