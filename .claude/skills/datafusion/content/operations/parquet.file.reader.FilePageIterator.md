# `parquet::file::reader::FilePageIterator`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.reader.FilePageIterator.json).

<a id="op-ceb50a97bcff18f7433d5c4a"></a>
## FilePageIterator

`struct` · `parquet::file::reader::FilePageIterator` · parquet 59.3.0

```rust
struct FilePageIterator
```

Source: `src/file/reader.rs:236`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Implementation of page iterator for parquet file.

<a id="op-604671c923e5ebce9f63d119"></a>
## Item

`assoc_type` · `parquet::file::reader::FilePageIterator::Item` · parquet 59.3.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::reader::FilePageIterator", "path": "FilePageIterator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [278, 1], "end": [288, 2], "filename": "src/file/reader.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/file/reader.rs:279`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3f44b3933eebacd81044a445"></a>
## new

`function` · `parquet::file::reader::FilePageIterator::new` · parquet 59.3.0

```rust
fn new(column_index: usize, file_reader: Arc<dyn FileReader>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::reader::FilePageIterator", "path": "FilePageIterator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [242, 1], "end": [276, 2], "filename": "src/file/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/reader.rs:244`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates a page iterator for all row groups in file.

<a id="op-52f85c4134531fd1da229123"></a>
## next

`function` · `parquet::file::reader::FilePageIterator::next` · parquet 59.3.0

```rust
fn next(&mut self) -> Option<Result<Box<dyn PageReader>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::reader::FilePageIterator", "path": "FilePageIterator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [278, 1], "end": [288, 2], "filename": "src/file/reader.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/file/reader.rs:281`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-35b29760099fee2a2346b5a3"></a>
## with_row_groups

`function` · `parquet::file::reader::FilePageIterator::with_row_groups` · parquet 59.3.0

```rust
fn with_row_groups(column_index: usize, row_group_indices: Box<dyn Iterator<Item = usize> + Send>, file_reader: Arc<dyn FileReader>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::reader::FilePageIterator", "path": "FilePageIterator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [242, 1], "end": [276, 2], "filename": "src/file/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/reader.rs:253`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create page iterator from parquet file reader with only some row groups.
