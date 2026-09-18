# `parquet::file::metadata::SortingColumn`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.metadata.SortingColumn.json).

<a id="op-17666c85adecd1729f202ddb"></a>
## SortingColumn

`struct` · `parquet::file::metadata::SortingColumn` · parquet 59.3.0

```rust
struct SortingColumn
```

Source: `src/file/metadata/mod.rs:607`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sort order within a RowGroup of a leaf column

<a id="op-bde6762b7965e27f7970a021"></a>
## clone

`function` · `parquet::file::metadata::SortingColumn::clone` · parquet 59.3.0

```rust
fn clone(&self) -> SortingColumn
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::SortingColumn", "path": "SortingColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [607, 1], "end": [620, 2], "filename": "src/file/metadata/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/file/metadata/mod.rs:607`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ab3d8d61be0c24241563f725"></a>
## column_idx

`struct_field` · `parquet::file::metadata::SortingColumn::column_idx` · parquet 59.3.0

```rust
column_idx: i32
```

Source: `src/file/metadata/mod.rs:607`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

The ordinal position of the column (in this row group)

<a id="op-eaea2632d18fc2d56681cda6"></a>
## descending

`struct_field` · `parquet::file::metadata::SortingColumn::descending` · parquet 59.3.0

```rust
descending: bool
```

Source: `src/file/metadata/mod.rs:607`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

If true, indicates this column is sorted in descending order.

<a id="op-88564b17017f785652b3c394"></a>
## eq

`function` · `parquet::file::metadata::SortingColumn::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &SortingColumn) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::SortingColumn", "path": "SortingColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [607, 1], "end": [620, 2], "filename": "src/file/metadata/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/file/metadata/mod.rs:607`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-422319f884b29491af4b8629"></a>
## fmt

`function` · `parquet::file::metadata::SortingColumn::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::SortingColumn", "path": "SortingColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [607, 1], "end": [620, 2], "filename": "src/file/metadata/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file/metadata/mod.rs:607`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1365e3d19d4ebb405803915a"></a>
## nulls_first

`struct_field` · `parquet::file::metadata::SortingColumn::nulls_first` · parquet 59.3.0

```rust
nulls_first: bool
```

Source: `src/file/metadata/mod.rs:607`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

If true, nulls will come before non-null values, otherwise,
nulls go at the end. */
