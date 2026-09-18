# `parquet::file::serialized_reader::SerializedRowGroupReader`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.serialized_reader.SerializedRowGroupReader.json).

<a id="op-0c1c6a62c61f0f35b48b4eb0"></a>
## SerializedRowGroupReader

`struct` · `parquet::file::serialized_reader::SerializedRowGroupReader` · parquet 59.3.0

```rust
struct SerializedRowGroupReader<'a, R: ChunkReader>
```

Source: `src/file/serialized_reader.rs:322`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A serialized implementation for Parquet [`RowGroupReader`](../operations/parquet.file.reader.RowGroupReader.md#op-a26f29895f1d68cffb907a99).

<a id="op-929027208f869ba60276fb64"></a>
## get_column_bloom_filter

`function` · `parquet::file::serialized_reader::SerializedRowGroupReader::get_column_bloom_filter` · parquet 59.3.0

```rust
fn get_column_bloom_filter(&self, i: usize) -> Option<&Sbbf>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"type": {"generic": "R"}}], "constraints": []}}, "id": "parquet::file::serialized_reader::SerializedRowGroupReader", "path": "SerializedRowGroupReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::file::reader::ChunkReader", "path": "ChunkReader"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [357, 1], "end": [390, 2], "filename": "src/file/serialized_reader.rs"}, "trait": {"args": null, "id": "parquet::file::reader::RowGroupReader", "path": "RowGroupReader"}, "trait_path": "parquet::file::reader::RowGroupReader"}`

Source: `src/file/serialized_reader.rs:383`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

get bloom filter for the `i`th column

<a id="op-d67c95021a0b958c248c679f"></a>
## get_column_page_reader

`function` · `parquet::file::serialized_reader::SerializedRowGroupReader::get_column_page_reader` · parquet 59.3.0

```rust
fn get_column_page_reader(&self, i: usize) -> Result<Box<dyn PageReader>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"type": {"generic": "R"}}], "constraints": []}}, "id": "parquet::file::serialized_reader::SerializedRowGroupReader", "path": "SerializedRowGroupReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::file::reader::ChunkReader", "path": "ChunkReader"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [357, 1], "end": [390, 2], "filename": "src/file/serialized_reader.rs"}, "trait": {"args": null, "id": "parquet::file::reader::RowGroupReader", "path": "RowGroupReader"}, "trait_path": "parquet::file::reader::RowGroupReader"}`

Source: `src/file/serialized_reader.rs:367`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6936c8a47d899e80394e14db"></a>
## get_row_iter

`function` · `parquet::file::serialized_reader::SerializedRowGroupReader::get_row_iter` · parquet 59.3.0

```rust
fn get_row_iter(&self, projection: Option<SchemaType>) -> Result<RowIter<'_>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"type": {"generic": "R"}}], "constraints": []}}, "id": "parquet::file::serialized_reader::SerializedRowGroupReader", "path": "SerializedRowGroupReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::file::reader::ChunkReader", "path": "ChunkReader"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [357, 1], "end": [390, 2], "filename": "src/file/serialized_reader.rs"}, "trait": {"args": null, "id": "parquet::file::reader::RowGroupReader", "path": "RowGroupReader"}, "trait_path": "parquet::file::reader::RowGroupReader"}`

Source: `src/file/serialized_reader.rs:387`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3d09fc9ebd89279beb03cff1"></a>
## metadata

`function` · `parquet::file::serialized_reader::SerializedRowGroupReader::metadata` · parquet 59.3.0

```rust
fn metadata(&self) -> &RowGroupMetaData
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"type": {"generic": "R"}}], "constraints": []}}, "id": "parquet::file::serialized_reader::SerializedRowGroupReader", "path": "SerializedRowGroupReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::file::reader::ChunkReader", "path": "ChunkReader"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [357, 1], "end": [390, 2], "filename": "src/file/serialized_reader.rs"}, "trait": {"args": null, "id": "parquet::file::reader::RowGroupReader", "path": "RowGroupReader"}, "trait_path": "parquet::file::reader::RowGroupReader"}`

Source: `src/file/serialized_reader.rs:358`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4d62635092c713aca6e117f5"></a>
## new

`function` · `parquet::file::serialized_reader::SerializedRowGroupReader::new` · parquet 59.3.0

```rust
fn new(chunk_reader: Arc<R>, metadata: &'a RowGroupMetaData, offset_index: Option<&'a [OffsetIndexMetaData]>, props: ReaderPropertiesPtr) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "R"}}], "constraints": []}}, "id": "parquet::file::serialized_reader::SerializedRowGroupReader", "path": "SerializedRowGroupReader"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::file::reader::ChunkReader", "path": "ChunkReader"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [330, 1], "end": [355, 2], "filename": "src/file/serialized_reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/serialized_reader.rs:332`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates new row group reader from a file, row group metadata and custom config.

<a id="op-43987d97fa16bf57ee0a925d"></a>
## num_columns

`function` · `parquet::file::serialized_reader::SerializedRowGroupReader::num_columns` · parquet 59.3.0

```rust
fn num_columns(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"type": {"generic": "R"}}], "constraints": []}}, "id": "parquet::file::serialized_reader::SerializedRowGroupReader", "path": "SerializedRowGroupReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::file::reader::ChunkReader", "path": "ChunkReader"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [357, 1], "end": [390, 2], "filename": "src/file/serialized_reader.rs"}, "trait": {"args": null, "id": "parquet::file::reader::RowGroupReader", "path": "RowGroupReader"}, "trait_path": "parquet::file::reader::RowGroupReader"}`

Source: `src/file/serialized_reader.rs:362`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
