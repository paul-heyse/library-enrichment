# `parquet::data_type::DoubleType`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.data_type.DoubleType.json).

<a id="op-6c5da218cdb92885ec4876a2"></a>
## DoubleType

`struct` · `parquet::data_type::DoubleType` · parquet 59.3.0

```rust
struct DoubleType
```

Source: `src/data_type.rs:1304`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Parquet physical type: DoubleType

<a id="op-3b58d78bb296fd918b751d88"></a>
## T

`assoc_type` · `parquet::data_type::DoubleType::T` · parquet 59.3.0

```rust
T
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::DoubleType", "path": "DoubleType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1304, 1], "end": [1304, 71], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::DataType", "path": "DataType"}, "trait_path": "parquet::data_type::DataType"}`

Source: `src/data_type.rs:1304`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dd530219ccd7d71add0b7bf5"></a>
## clone

`function` · `parquet::data_type::DoubleType::clone` · parquet 59.3.0

```rust
fn clone(&self) -> DoubleType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::DoubleType", "path": "DoubleType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1304, 1], "end": [1304, 71], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/data_type.rs:1304`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-838bc403cd0820e45b619eed"></a>
## get_column_reader

`function` · `parquet::data_type::DoubleType::get_column_reader` · parquet 59.3.0

```rust
fn get_column_reader(column_reader: ColumnReader) -> Option<ColumnReaderImpl<Self>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::DoubleType", "path": "DoubleType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1304, 1], "end": [1304, 71], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::DataType", "path": "DataType"}, "trait_path": "parquet::data_type::DataType"}`

Source: `src/data_type.rs:1304`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-63e6046272ff926c3b1cfa3a"></a>
## get_column_writer

`function` · `parquet::data_type::DoubleType::get_column_writer` · parquet 59.3.0

```rust
fn get_column_writer(column_writer: ColumnWriter<'_>) -> Option<ColumnWriterImpl<'_, Self>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::DoubleType", "path": "DoubleType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1304, 1], "end": [1304, 71], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::DataType", "path": "DataType"}, "trait_path": "parquet::data_type::DataType"}`

Source: `src/data_type.rs:1304`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c2704cca82c63a9dd44c7954"></a>
## get_column_writer_mut

`function` · `parquet::data_type::DoubleType::get_column_writer_mut` · parquet 59.3.0

```rust
fn get_column_writer_mut<'a, 'b: 'a>(column_writer: &'a mut ColumnWriter<'b>) -> Option<&'a mut ColumnWriterImpl<'b, Self>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::DoubleType", "path": "DoubleType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1304, 1], "end": [1304, 71], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::DataType", "path": "DataType"}, "trait_path": "parquet::data_type::DataType"}`

Source: `src/data_type.rs:1304`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c502ee3a77ac9c50f6f1f6ad"></a>
## get_column_writer_ref

`function` · `parquet::data_type::DoubleType::get_column_writer_ref` · parquet 59.3.0

```rust
fn get_column_writer_ref<'a, 'b: 'a>(column_writer: &'a ColumnWriter<'b>) -> Option<&'a ColumnWriterImpl<'b, Self>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::DoubleType", "path": "DoubleType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1304, 1], "end": [1304, 71], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::DataType", "path": "DataType"}, "trait_path": "parquet::data_type::DataType"}`

Source: `src/data_type.rs:1304`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-01a5c7a23014c331a8e1acbf"></a>
## get_type_size

`function` · `parquet::data_type::DoubleType::get_type_size` · parquet 59.3.0

```rust
fn get_type_size() -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::DoubleType", "path": "DoubleType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1304, 1], "end": [1304, 71], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::DataType", "path": "DataType"}, "trait_path": "parquet::data_type::DataType"}`

Source: `src/data_type.rs:1304`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
