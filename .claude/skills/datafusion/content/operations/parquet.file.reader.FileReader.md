# `parquet::file::reader::FileReader`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.reader.FileReader.json).

<a id="op-9b66de5d3c7d389f75983bc1"></a>
## FileReader

`trait` · `parquet::file::reader::FileReader` · parquet 59.3.0

```rust
trait FileReader: Send + Sync
```

Source: `src/file/reader.rs:155`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Parquet file reader API. With this, user can get metadata information about the
Parquet file, can get reader for each row group, and access record iterator.

<a id="op-9b63658a98127eb0a44c4704"></a>
## get_row_group

`function` · `parquet::file::reader::FileReader::get_row_group` · parquet 59.3.0

```rust
fn get_row_group(&self, i: usize) -> Result<Box<dyn RowGroupReader + '_>>
```

Source: `src/file/reader.rs:163`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Get the `i`th row group reader. Note this doesn't do bound check.

<a id="op-e6af8f15abadcba3b309c4cd"></a>
## get_row_iter

`function` · `parquet::file::reader::FileReader::get_row_iter` · parquet 59.3.0

```rust
fn get_row_iter(&self, projection: Option<SchemaType>) -> Result<RowIter<'_>>
```

Source: `src/file/reader.rs:171`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Get an iterator over the row in this file, see [`RowIter`](../operations/parquet.record.reader.RowIter.md#op-b0386c6715ec7d27cee7594a) for caveats.

Iterator will automatically load the next row group to advance.

Projected schema can be a subset of or equal to the file schema, when it is None,
full file schema is assumed.

<a id="op-8f912669a469e5d7ad4e6ccb"></a>
## metadata

`function` · `parquet::file::reader::FileReader::metadata` · parquet 59.3.0

```rust
fn metadata(&self) -> &ParquetMetaData
```

Source: `src/file/reader.rs:157`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Get metadata information about this file.

<a id="op-922b55487a015a897ed41457"></a>
## num_row_groups

`function` · `parquet::file::reader::FileReader::num_row_groups` · parquet 59.3.0

```rust
fn num_row_groups(&self) -> usize
```

Source: `src/file/reader.rs:160`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Get the total number of row groups for this file.
