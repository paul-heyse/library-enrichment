# `parquet::file::reader::RowGroupReader`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.reader.RowGroupReader.json).

<a id="op-a26f29895f1d68cffb907a99"></a>
## RowGroupReader

`trait` · `parquet::file::reader::RowGroupReader` · parquet 59.3.0

```rust
trait RowGroupReader: Send + Sync
```

Source: `src/file/reader.rs:176`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Parquet row group reader API. With this, user can get metadata information about the
row group, as well as readers for each individual column chunk.

<a id="op-58d68f08b4bbcf32c5fb933f"></a>
## get_column_bloom_filter

`function` · `parquet::file::reader::RowGroupReader::get_column_bloom_filter` · parquet 59.3.0

```rust
fn get_column_bloom_filter(&self, i: usize) -> Option<&Sbbf>
```

Source: `src/file/reader.rs:223`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Get bloom filter for the `i`th column chunk, if present and the reader was configured
to read bloom filters.

<a id="op-41ccdb7a716647c37c4aaa57"></a>
## get_column_page_reader

`function` · `parquet::file::reader::RowGroupReader::get_column_page_reader` · parquet 59.3.0

```rust
fn get_column_page_reader(&self, i: usize) -> Result<Box<dyn PageReader>>
```

Source: `src/file/reader.rs:184`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Get page reader for the `i`th column chunk.

<a id="op-a93b268ea554d3d14aaac5a9"></a>
## get_column_reader

`function` · `parquet::file::reader::RowGroupReader::get_column_reader` · parquet 59.3.0

```rust
fn get_column_reader(&self, i: usize) -> Result<ColumnReader>
```

Source: `src/file/reader.rs:187`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Get value reader for the `i`th column chunk.

<a id="op-08117afbbe68c812439a61ae"></a>
## get_row_iter

`function` · `parquet::file::reader::RowGroupReader::get_row_iter` · parquet 59.3.0

```rust
fn get_row_iter(&self, projection: Option<SchemaType>) -> Result<RowIter<'_>>
```

Source: `src/file/reader.rs:229`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Get an iterator over the row in this file, see [`RowIter`](../operations/parquet.record.reader.RowIter.md#op-b0386c6715ec7d27cee7594a) for caveats.

Projected schema can be a subset of or equal to the file schema, when it is None,
full file schema is assumed.

<a id="op-cbc9ec8bb1a3da832dce0761"></a>
## metadata

`function` · `parquet::file::reader::RowGroupReader::metadata` · parquet 59.3.0

```rust
fn metadata(&self) -> &RowGroupMetaData
```

Source: `src/file/reader.rs:178`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Get metadata information about this row group.

<a id="op-7e047a131c6681be22da1f71"></a>
## num_columns

`function` · `parquet::file::reader::RowGroupReader::num_columns` · parquet 59.3.0

```rust
fn num_columns(&self) -> usize
```

Source: `src/file/reader.rs:181`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Get the total number of column chunks in this row group.
