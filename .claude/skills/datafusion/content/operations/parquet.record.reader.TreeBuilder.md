# `parquet::record::reader::TreeBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.record.reader.TreeBuilder.json).

<a id="op-bd2b6d45f069b1f30a384238"></a>
## TreeBuilder

`struct` · `parquet::record::reader::TreeBuilder` · parquet 59.3.0

```rust
struct TreeBuilder
```

Source: `src/record/reader.rs:38`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Tree builder for `Reader` enum.
Serves as a container of options for building a reader tree and a builder, and
accessing a records iterator [`RowIter`](../operations/parquet.record.reader.RowIter.md#op-b0386c6715ec7d27cee7594a).

<a id="op-037a56058d3c3837011d837f"></a>
## as_iter

`function` · `parquet::record::reader::TreeBuilder::as_iter` · parquet 59.3.0

```rust
fn as_iter(&self, descr: SchemaDescPtr, row_group_reader: &dyn RowGroupReader) -> Result<ReaderIter>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::reader::TreeBuilder", "path": "TreeBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [327, 2], "filename": "src/record/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/record/reader.rs:96`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates iterator of `Row`s directly from schema descriptor and row group.

<a id="op-e30b2b1efd5389a16fa3114a"></a>
## build

`function` · `parquet::record::reader::TreeBuilder::build` · parquet 59.3.0

```rust
fn build(&self, descr: SchemaDescPtr, row_group_reader: &dyn RowGroupReader) -> Result<Reader>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::reader::TreeBuilder", "path": "TreeBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [327, 2], "filename": "src/record/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/record/reader.rs:64`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates new root reader for provided schema and row group.

<a id="op-4f4f15c6084cb8e382e4395d"></a>
## default

`function` · `parquet::record::reader::TreeBuilder::default` · parquet 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::reader::TreeBuilder", "path": "TreeBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [47, 2], "filename": "src/record/reader.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/record/reader.rs:44`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c0780f92150ebee2646d8fc8"></a>
## new

`function` · `parquet::record::reader::TreeBuilder::new` · parquet 59.3.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::reader::TreeBuilder", "path": "TreeBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [327, 2], "filename": "src/record/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/record/reader.rs:51`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates new tree builder with default parameters.

<a id="op-1945b920cfed589297657123"></a>
## with_batch_size

`function` · `parquet::record::reader::TreeBuilder::with_batch_size` · parquet 59.3.0

```rust
fn with_batch_size(self, batch_size: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::reader::TreeBuilder", "path": "TreeBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [327, 2], "filename": "src/record/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/record/reader.rs:58`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets batch size for this tree builder.
