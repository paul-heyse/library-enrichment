# `parquet::file::serialized_reader::SerializedPageReader`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.serialized_reader.SerializedPageReader.json).

<a id="op-bfbe499ed29e9d6b0ef4538b"></a>
## SerializedPageReader

`struct` · `parquet::file::serialized_reader::SerializedPageReader` · parquet 59.3.0

```rust
struct SerializedPageReader<R: ChunkReader>
```

Source: `src/file/serialized_reader.rs:565`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A serialized implementation for Parquet [`PageReader`](../operations/parquet.column.page.PageReader.md#op-7f8c0bacd485ada3bba28e40).

<a id="op-293af79f13720eb04d1a4f66"></a>
## Item

`assoc_type` · `parquet::file::serialized_reader::SerializedPageReader::Item` · parquet 59.3.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "parquet::file::serialized_reader::SerializedPageReader", "path": "SerializedPageReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::file::reader::ChunkReader", "path": "ChunkReader"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [885, 1], "end": [891, 2], "filename": "src/file/serialized_reader.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/file/serialized_reader.rs:886`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d019ddb47ceb08a8cbed09ba"></a>
## at_record_boundary

`function` · `parquet::file::serialized_reader::SerializedPageReader::at_record_boundary` · parquet 59.3.0

```rust
fn at_record_boundary(&mut self) -> Result<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "parquet::file::serialized_reader::SerializedPageReader", "path": "SerializedPageReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::file::reader::ChunkReader", "path": "ChunkReader"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [914, 1], "end": [1169, 2], "filename": "src/file/serialized_reader.rs"}, "trait": {"args": null, "id": "parquet::column::page::PageReader", "path": "PageReader"}, "trait_path": "parquet::column::page::PageReader"}`

Source: `src/file/serialized_reader.rs:1158`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-748b09ddc4016b851e653707"></a>
## get_next_page

`function` · `parquet::file::serialized_reader::SerializedPageReader::get_next_page` · parquet 59.3.0

```rust
fn get_next_page(&mut self) -> Result<Option<Page>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "parquet::file::serialized_reader::SerializedPageReader", "path": "SerializedPageReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::file::reader::ChunkReader", "path": "ChunkReader"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [914, 1], "end": [1169, 2], "filename": "src/file/serialized_reader.rs"}, "trait": {"args": null, "id": "parquet::column::page::PageReader", "path": "PageReader"}, "trait_path": "parquet::column::page::PageReader"}`

Source: `src/file/serialized_reader.rs:915`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ddcc859750b3d261d8b62c5b"></a>
## new

`function` · `parquet::file::serialized_reader::SerializedPageReader::new` · parquet 59.3.0

```rust
fn new(reader: Arc<R>, column_chunk_metadata: &ColumnChunkMetaData, total_rows: usize, page_locations: Option<Vec<PageLocation>>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "parquet::file::serialized_reader::SerializedPageReader", "path": "SerializedPageReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::file::reader::ChunkReader", "path": "ChunkReader"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [580, 1], "end": [786, 2], "filename": "src/file/serialized_reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/serialized_reader.rs:582`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates a new serialized page reader from a chunk reader and metadata

<a id="op-d19e8509f63fd1818e8451d3"></a>
## new_with_properties

`function` · `parquet::file::serialized_reader::SerializedPageReader::new_with_properties` · parquet 59.3.0

```rust
fn new_with_properties(reader: Arc<R>, meta: &ColumnChunkMetaData, total_rows: usize, page_locations: Option<Vec<PageLocation>>, props: ReaderPropertiesPtr) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "parquet::file::serialized_reader::SerializedPageReader", "path": "SerializedPageReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::file::reader::ChunkReader", "path": "ChunkReader"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [580, 1], "end": [786, 2], "filename": "src/file/serialized_reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/serialized_reader.rs:632`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates a new serialized page with custom options.

<a id="op-417c5196981fa1e1ec5e57f8"></a>
## next

`function` · `parquet::file::serialized_reader::SerializedPageReader::next` · parquet 59.3.0

```rust
fn next(&mut self) -> Option<Self::Item>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "parquet::file::serialized_reader::SerializedPageReader", "path": "SerializedPageReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::file::reader::ChunkReader", "path": "ChunkReader"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [885, 1], "end": [891, 2], "filename": "src/file/serialized_reader.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/file/serialized_reader.rs:888`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1cb71639f0de5051171cae35"></a>
## peek_next_page

`function` · `parquet::file::serialized_reader::SerializedPageReader::peek_next_page` · parquet 59.3.0

```rust
fn peek_next_page(&mut self) -> Result<Option<PageMetadata>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "parquet::file::serialized_reader::SerializedPageReader", "path": "SerializedPageReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::file::reader::ChunkReader", "path": "ChunkReader"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [914, 1], "end": [1169, 2], "filename": "src/file/serialized_reader.rs"}, "trait": {"args": null, "id": "parquet::column::page::PageReader", "path": "PageReader"}, "trait_path": "parquet::column::page::PageReader"}`

Source: `src/file/serialized_reader.rs:1021`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e7fb15e81f08ae572baa70f2"></a>
## skip_next_page

`function` · `parquet::file::serialized_reader::SerializedPageReader::skip_next_page` · parquet 59.3.0

```rust
fn skip_next_page(&mut self) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "parquet::file::serialized_reader::SerializedPageReader", "path": "SerializedPageReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::file::reader::ChunkReader", "path": "ChunkReader"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [914, 1], "end": [1169, 2], "filename": "src/file/serialized_reader.rs"}, "trait": {"args": null, "id": "parquet::column::page::PageReader", "path": "PageReader"}, "trait_path": "parquet::column::page::PageReader"}`

Source: `src/file/serialized_reader.rs:1094`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
