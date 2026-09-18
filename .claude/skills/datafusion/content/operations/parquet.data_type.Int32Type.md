# `parquet::data_type::Int32Type`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.data_type.Int32Type.json).

<a id="op-fd60a260f0c8e7fb7c70a7cf"></a>
## Int32Type

`struct` · `parquet::data_type::Int32Type` · parquet 59.3.0

```rust
struct Int32Type
```

Source: `src/data_type.rs:1294`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Parquet physical type: Int32Type

<a id="op-b6eeaedc4f6d707a144463c8"></a>
## T

`assoc_type` · `parquet::data_type::Int32Type::T` · parquet 59.3.0

```rust
T
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::Int32Type", "path": "Int32Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1294, 1], "end": [1294, 68], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::DataType", "path": "DataType"}, "trait_path": "parquet::data_type::DataType"}`

Source: `src/data_type.rs:1294`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0b4eefc84855c670f2a696a2"></a>
## clone

`function` · `parquet::data_type::Int32Type::clone` · parquet 59.3.0

```rust
fn clone(&self) -> Int32Type
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::Int32Type", "path": "Int32Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1294, 1], "end": [1294, 68], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/data_type.rs:1294`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e8c728ba64d7bd3a6aa78bbb"></a>
## get_column_reader

`function` · `parquet::data_type::Int32Type::get_column_reader` · parquet 59.3.0

```rust
fn get_column_reader(column_reader: ColumnReader) -> Option<ColumnReaderImpl<Self>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::Int32Type", "path": "Int32Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1294, 1], "end": [1294, 68], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::DataType", "path": "DataType"}, "trait_path": "parquet::data_type::DataType"}`

Source: `src/data_type.rs:1294`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e7a8f3c845336034867144d8"></a>
## get_column_writer

`function` · `parquet::data_type::Int32Type::get_column_writer` · parquet 59.3.0

```rust
fn get_column_writer(column_writer: ColumnWriter<'_>) -> Option<ColumnWriterImpl<'_, Self>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::Int32Type", "path": "Int32Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1294, 1], "end": [1294, 68], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::DataType", "path": "DataType"}, "trait_path": "parquet::data_type::DataType"}`

Source: `src/data_type.rs:1294`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-08e81aedac26d200b97c78a2"></a>
## get_column_writer_mut

`function` · `parquet::data_type::Int32Type::get_column_writer_mut` · parquet 59.3.0

```rust
fn get_column_writer_mut<'a, 'b: 'a>(column_writer: &'a mut ColumnWriter<'b>) -> Option<&'a mut ColumnWriterImpl<'b, Self>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::Int32Type", "path": "Int32Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1294, 1], "end": [1294, 68], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::DataType", "path": "DataType"}, "trait_path": "parquet::data_type::DataType"}`

Source: `src/data_type.rs:1294`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b2955c64aae1dfff795d2adf"></a>
## get_column_writer_ref

`function` · `parquet::data_type::Int32Type::get_column_writer_ref` · parquet 59.3.0

```rust
fn get_column_writer_ref<'a, 'b: 'a>(column_writer: &'a ColumnWriter<'b>) -> Option<&'a ColumnWriterImpl<'b, Self>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::Int32Type", "path": "Int32Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1294, 1], "end": [1294, 68], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::DataType", "path": "DataType"}, "trait_path": "parquet::data_type::DataType"}`

Source: `src/data_type.rs:1294`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5a840e40261996aec0430938"></a>
## get_type_size

`function` · `parquet::data_type::Int32Type::get_type_size` · parquet 59.3.0

```rust
fn get_type_size() -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::Int32Type", "path": "Int32Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1294, 1], "end": [1294, 68], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::DataType", "path": "DataType"}, "trait_path": "parquet::data_type::DataType"}`

Source: `src/data_type.rs:1294`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
