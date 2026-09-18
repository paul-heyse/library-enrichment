# `parquet::data_type::Int64Type`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.data_type.Int64Type.json).

<a id="op-4f8c4a9d396c9c1fe43f12d9"></a>
## Int64Type

`struct` · `parquet::data_type::Int64Type` · parquet 59.3.0

```rust
struct Int64Type
```

Source: `src/data_type.rs:1295`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Parquet physical type: Int64Type

<a id="op-643e3a627d29eba2f5642c5a"></a>
## T

`assoc_type` · `parquet::data_type::Int64Type::T` · parquet 59.3.0

```rust
T
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::Int64Type", "path": "Int64Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1295, 1], "end": [1295, 68], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::DataType", "path": "DataType"}, "trait_path": "parquet::data_type::DataType"}`

Source: `src/data_type.rs:1295`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2476488ceb929b4779243882"></a>
## clone

`function` · `parquet::data_type::Int64Type::clone` · parquet 59.3.0

```rust
fn clone(&self) -> Int64Type
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::Int64Type", "path": "Int64Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1295, 1], "end": [1295, 68], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/data_type.rs:1295`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-280e6388ea1220d9ad425741"></a>
## get_column_reader

`function` · `parquet::data_type::Int64Type::get_column_reader` · parquet 59.3.0

```rust
fn get_column_reader(column_reader: ColumnReader) -> Option<ColumnReaderImpl<Self>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::Int64Type", "path": "Int64Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1295, 1], "end": [1295, 68], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::DataType", "path": "DataType"}, "trait_path": "parquet::data_type::DataType"}`

Source: `src/data_type.rs:1295`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1d0987ad0b052771a0c61208"></a>
## get_column_writer

`function` · `parquet::data_type::Int64Type::get_column_writer` · parquet 59.3.0

```rust
fn get_column_writer(column_writer: ColumnWriter<'_>) -> Option<ColumnWriterImpl<'_, Self>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::Int64Type", "path": "Int64Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1295, 1], "end": [1295, 68], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::DataType", "path": "DataType"}, "trait_path": "parquet::data_type::DataType"}`

Source: `src/data_type.rs:1295`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-87df1148d94780df2832c029"></a>
## get_column_writer_mut

`function` · `parquet::data_type::Int64Type::get_column_writer_mut` · parquet 59.3.0

```rust
fn get_column_writer_mut<'a, 'b: 'a>(column_writer: &'a mut ColumnWriter<'b>) -> Option<&'a mut ColumnWriterImpl<'b, Self>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::Int64Type", "path": "Int64Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1295, 1], "end": [1295, 68], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::DataType", "path": "DataType"}, "trait_path": "parquet::data_type::DataType"}`

Source: `src/data_type.rs:1295`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ec7aedac68af5756377b2ea3"></a>
## get_column_writer_ref

`function` · `parquet::data_type::Int64Type::get_column_writer_ref` · parquet 59.3.0

```rust
fn get_column_writer_ref<'a, 'b: 'a>(column_writer: &'a ColumnWriter<'b>) -> Option<&'a ColumnWriterImpl<'b, Self>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::Int64Type", "path": "Int64Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1295, 1], "end": [1295, 68], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::DataType", "path": "DataType"}, "trait_path": "parquet::data_type::DataType"}`

Source: `src/data_type.rs:1295`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7aee29b9349e5aa9c4e407ba"></a>
## get_type_size

`function` · `parquet::data_type::Int64Type::get_type_size` · parquet 59.3.0

```rust
fn get_type_size() -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::Int64Type", "path": "Int64Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1295, 1], "end": [1295, 68], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::DataType", "path": "DataType"}, "trait_path": "parquet::data_type::DataType"}`

Source: `src/data_type.rs:1295`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
