# `parquet::data_type::Int96Type`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.data_type.Int96Type.json).

<a id="op-d978904a3e18d9d5fae0cad1"></a>
## Int96Type

`struct` · `parquet::data_type::Int96Type` · parquet 59.3.0

```rust
struct Int96Type
```

Source: `src/data_type.rs:1296`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Parquet physical type: Int96Type

<a id="op-979d94ad52539ae048f10051"></a>
## T

`assoc_type` · `parquet::data_type::Int96Type::T` · parquet 59.3.0

```rust
T
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::Int96Type", "path": "Int96Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1296, 1], "end": [1302, 2], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::DataType", "path": "DataType"}, "trait_path": "parquet::data_type::DataType"}`

Source: `src/data_type.rs:1296`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8fdc8b0592b7b8c9ef425213"></a>
## clone

`function` · `parquet::data_type::Int96Type::clone` · parquet 59.3.0

```rust
fn clone(&self) -> Int96Type
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::Int96Type", "path": "Int96Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1296, 1], "end": [1302, 2], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/data_type.rs:1296`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-843ea73004589e571ac9dd6b"></a>
## get_column_reader

`function` · `parquet::data_type::Int96Type::get_column_reader` · parquet 59.3.0

```rust
fn get_column_reader(column_reader: ColumnReader) -> Option<ColumnReaderImpl<Self>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::Int96Type", "path": "Int96Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1296, 1], "end": [1302, 2], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::DataType", "path": "DataType"}, "trait_path": "parquet::data_type::DataType"}`

Source: `src/data_type.rs:1296`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b9d8ebf005ee160651ad6c86"></a>
## get_column_writer

`function` · `parquet::data_type::Int96Type::get_column_writer` · parquet 59.3.0

```rust
fn get_column_writer(column_writer: ColumnWriter<'_>) -> Option<ColumnWriterImpl<'_, Self>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::Int96Type", "path": "Int96Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1296, 1], "end": [1302, 2], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::DataType", "path": "DataType"}, "trait_path": "parquet::data_type::DataType"}`

Source: `src/data_type.rs:1296`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c66f4227b65eddc3a9e11526"></a>
## get_column_writer_mut

`function` · `parquet::data_type::Int96Type::get_column_writer_mut` · parquet 59.3.0

```rust
fn get_column_writer_mut<'a, 'b: 'a>(column_writer: &'a mut ColumnWriter<'b>) -> Option<&'a mut ColumnWriterImpl<'b, Self>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::Int96Type", "path": "Int96Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1296, 1], "end": [1302, 2], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::DataType", "path": "DataType"}, "trait_path": "parquet::data_type::DataType"}`

Source: `src/data_type.rs:1296`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a15dfc45fd63501bdf535ae6"></a>
## get_column_writer_ref

`function` · `parquet::data_type::Int96Type::get_column_writer_ref` · parquet 59.3.0

```rust
fn get_column_writer_ref<'a, 'b: 'a>(column_writer: &'a ColumnWriter<'b>) -> Option<&'a ColumnWriterImpl<'b, Self>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::Int96Type", "path": "Int96Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1296, 1], "end": [1302, 2], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::DataType", "path": "DataType"}, "trait_path": "parquet::data_type::DataType"}`

Source: `src/data_type.rs:1296`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-800ce6e7950e3fbe3a2afcaf"></a>
## get_type_size

`function` · `parquet::data_type::Int96Type::get_type_size` · parquet 59.3.0

```rust
fn get_type_size() -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::Int96Type", "path": "Int96Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1296, 1], "end": [1302, 2], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::DataType", "path": "DataType"}, "trait_path": "parquet::data_type::DataType"}`

Source: `src/data_type.rs:1296`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
