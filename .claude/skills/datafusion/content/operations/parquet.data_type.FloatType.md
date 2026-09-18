# `parquet::data_type::FloatType`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.data_type.FloatType.json).

<a id="op-2051b133db70eb97b7041da4"></a>
## FloatType

`struct` · `parquet::data_type::FloatType` · parquet 59.3.0

```rust
struct FloatType
```

Source: `src/data_type.rs:1303`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Parquet physical type: FloatType

<a id="op-d238ccf3754fe5469e56ce32"></a>
## T

`assoc_type` · `parquet::data_type::FloatType::T` · parquet 59.3.0

```rust
T
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::FloatType", "path": "FloatType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1303, 1], "end": [1303, 68], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::DataType", "path": "DataType"}, "trait_path": "parquet::data_type::DataType"}`

Source: `src/data_type.rs:1303`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8da8bb9d6abe45c0b36d435b"></a>
## clone

`function` · `parquet::data_type::FloatType::clone` · parquet 59.3.0

```rust
fn clone(&self) -> FloatType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::FloatType", "path": "FloatType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1303, 1], "end": [1303, 68], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/data_type.rs:1303`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f65efa1b7ce3adc3cc613d61"></a>
## get_column_reader

`function` · `parquet::data_type::FloatType::get_column_reader` · parquet 59.3.0

```rust
fn get_column_reader(column_reader: ColumnReader) -> Option<ColumnReaderImpl<Self>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::FloatType", "path": "FloatType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1303, 1], "end": [1303, 68], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::DataType", "path": "DataType"}, "trait_path": "parquet::data_type::DataType"}`

Source: `src/data_type.rs:1303`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-92c220db1e8e8a0cbdfee62b"></a>
## get_column_writer

`function` · `parquet::data_type::FloatType::get_column_writer` · parquet 59.3.0

```rust
fn get_column_writer(column_writer: ColumnWriter<'_>) -> Option<ColumnWriterImpl<'_, Self>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::FloatType", "path": "FloatType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1303, 1], "end": [1303, 68], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::DataType", "path": "DataType"}, "trait_path": "parquet::data_type::DataType"}`

Source: `src/data_type.rs:1303`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3df620e3f1d38ab5fa954bdc"></a>
## get_column_writer_mut

`function` · `parquet::data_type::FloatType::get_column_writer_mut` · parquet 59.3.0

```rust
fn get_column_writer_mut<'a, 'b: 'a>(column_writer: &'a mut ColumnWriter<'b>) -> Option<&'a mut ColumnWriterImpl<'b, Self>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::FloatType", "path": "FloatType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1303, 1], "end": [1303, 68], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::DataType", "path": "DataType"}, "trait_path": "parquet::data_type::DataType"}`

Source: `src/data_type.rs:1303`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f7df8264dbe344d72f1dda55"></a>
## get_column_writer_ref

`function` · `parquet::data_type::FloatType::get_column_writer_ref` · parquet 59.3.0

```rust
fn get_column_writer_ref<'a, 'b: 'a>(column_writer: &'a ColumnWriter<'b>) -> Option<&'a ColumnWriterImpl<'b, Self>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::FloatType", "path": "FloatType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1303, 1], "end": [1303, 68], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::DataType", "path": "DataType"}, "trait_path": "parquet::data_type::DataType"}`

Source: `src/data_type.rs:1303`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7deeff1bf85a9417451925f2"></a>
## get_type_size

`function` · `parquet::data_type::FloatType::get_type_size` · parquet 59.3.0

```rust
fn get_type_size() -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::FloatType", "path": "FloatType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1303, 1], "end": [1303, 68], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::DataType", "path": "DataType"}, "trait_path": "parquet::data_type::DataType"}`

Source: `src/data_type.rs:1303`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
