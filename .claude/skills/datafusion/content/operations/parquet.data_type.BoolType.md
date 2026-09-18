# `parquet::data_type::BoolType`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.data_type.BoolType.json).

<a id="op-7df2f427c00758c2d60dcd15"></a>
## BoolType

`struct` · `parquet::data_type::BoolType` · parquet 59.3.0

```rust
struct BoolType
```

Source: `src/data_type.rs:1293`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Parquet physical type: BoolType

<a id="op-a7695ba7ffd594916d047494"></a>
## T

`assoc_type` · `parquet::data_type::BoolType::T` · parquet 59.3.0

```rust
T
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::BoolType", "path": "BoolType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1293, 1], "end": [1293, 66], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::DataType", "path": "DataType"}, "trait_path": "parquet::data_type::DataType"}`

Source: `src/data_type.rs:1293`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-da7a32540340f850e7a5c1da"></a>
## clone

`function` · `parquet::data_type::BoolType::clone` · parquet 59.3.0

```rust
fn clone(&self) -> BoolType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::BoolType", "path": "BoolType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1293, 1], "end": [1293, 66], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/data_type.rs:1293`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-715657f72a7e15a1dfe1ccb8"></a>
## get_column_reader

`function` · `parquet::data_type::BoolType::get_column_reader` · parquet 59.3.0

```rust
fn get_column_reader(column_reader: ColumnReader) -> Option<ColumnReaderImpl<Self>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::BoolType", "path": "BoolType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1293, 1], "end": [1293, 66], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::DataType", "path": "DataType"}, "trait_path": "parquet::data_type::DataType"}`

Source: `src/data_type.rs:1293`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-71d5104677962edf45a2ecea"></a>
## get_column_writer

`function` · `parquet::data_type::BoolType::get_column_writer` · parquet 59.3.0

```rust
fn get_column_writer(column_writer: ColumnWriter<'_>) -> Option<ColumnWriterImpl<'_, Self>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::BoolType", "path": "BoolType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1293, 1], "end": [1293, 66], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::DataType", "path": "DataType"}, "trait_path": "parquet::data_type::DataType"}`

Source: `src/data_type.rs:1293`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1bfa63c88bd9993f74b7a44f"></a>
## get_column_writer_mut

`function` · `parquet::data_type::BoolType::get_column_writer_mut` · parquet 59.3.0

```rust
fn get_column_writer_mut<'a, 'b: 'a>(column_writer: &'a mut ColumnWriter<'b>) -> Option<&'a mut ColumnWriterImpl<'b, Self>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::BoolType", "path": "BoolType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1293, 1], "end": [1293, 66], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::DataType", "path": "DataType"}, "trait_path": "parquet::data_type::DataType"}`

Source: `src/data_type.rs:1293`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c00be18adfd37dbc3dae507"></a>
## get_column_writer_ref

`function` · `parquet::data_type::BoolType::get_column_writer_ref` · parquet 59.3.0

```rust
fn get_column_writer_ref<'a, 'b: 'a>(column_writer: &'a ColumnWriter<'b>) -> Option<&'a ColumnWriterImpl<'b, Self>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::BoolType", "path": "BoolType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1293, 1], "end": [1293, 66], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::DataType", "path": "DataType"}, "trait_path": "parquet::data_type::DataType"}`

Source: `src/data_type.rs:1293`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f04325993b92702b44047cf9"></a>
## get_type_size

`function` · `parquet::data_type::BoolType::get_type_size` · parquet 59.3.0

```rust
fn get_type_size() -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::data_type::BoolType", "path": "BoolType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1293, 1], "end": [1293, 66], "filename": "src/data_type.rs"}, "trait": {"args": null, "id": "parquet::data_type::DataType", "path": "DataType"}, "trait_path": "parquet::data_type::DataType"}`

Source: `src/data_type.rs:1293`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
