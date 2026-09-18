# `parquet::file::reader`

Crate `parquet` · 5 public items · structured records in [`model/parquet.file.reader.json`](../model/parquet.file.reader.json)

## FilePageIterator

`struct` · `parquet::file::reader::FilePageIterator`

```rust
struct FilePageIterator
```

**Implements**: `core::iter::traits::iterator::Iterator`, `parquet::column::page::PageIterator`

**Methods** (2)

```rust
fn new(column_index: usize, file_reader: Arc<dyn FileReader>) -> Result<Self>
fn with_row_groups(column_index: usize, row_group_indices: Box<dyn Iterator<Item = usize> + Send>, file_reader: Arc<dyn FileReader>) -> Result<Self>
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Result<Box<dyn PageReader>>>
```

[Full member, field, variant and typed contracts](../operations/parquet.file.reader.FilePageIterator.md).


Implementation of page iterator for parquet file.

---

## ChunkReader

`trait` · `parquet::file::reader::ChunkReader`

```rust
trait ChunkReader: Length + Send + Sync
```

**Implementors** (2)

- `bytes::bytes::Bytes`
- `std::fs::File`

**Methods** (2)

```rust
fn get_bytes(&self, start: u64, length: usize) -> Result<Bytes>
fn get_read(&self, start: u64) -> Result<Self::T>
```

[Full member, field, variant and typed contracts](../operations/parquet.file.reader.ChunkReader.md).


Generates [`Read`]ers to read chunks of a Parquet data source.

The Parquet reader uses [`ChunkReader`] to access Parquet data, allowing
multiple decoders to read concurrently from different locations in the same
file.

The trait functions both as a reader and a factory for readers.
* random access via [`Self::get_bytes`]
* sequential access via the reader returned via factory method [`Self::get_read`]

# Provided Implementations
* [`File`] for reading from local file system
* [`Bytes`] for reading from an in-memory buffer

User provided implementations can implement more sophisticated behaviors
such as on-demand buffering or scan sharing.

---

## FileReader

`trait` · `parquet::file::reader::FileReader`

```rust
trait FileReader: Send + Sync
```

**Implementors** (1)

- `parquet::file::serialized_reader::SerializedFileReader`

**Methods** (4)

```rust
fn get_row_group(&self, i: usize) -> Result<Box<dyn RowGroupReader + '_>>
fn get_row_iter(&self, projection: Option<SchemaType>) -> Result<RowIter<'_>>
fn metadata(&self) -> &ParquetMetaData
fn num_row_groups(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/parquet.file.reader.FileReader.md).


Parquet file reader API. With this, user can get metadata information about the
Parquet file, can get reader for each row group, and access record iterator.

---

## Length

`trait` · `parquet::file::reader::Length`

```rust
trait Length
```

**Implementors** (2)

- `bytes::bytes::Bytes`
- `std::fs::File`

**Methods** (1)

```rust
fn len(&self) -> u64
```

[Full member, field, variant and typed contracts](../operations/parquet.file.reader.Length.md).


Length should return the total number of bytes in the input source.
It's mainly used to read the metadata, which is at the end of the source.

---

## RowGroupReader

`trait` · `parquet::file::reader::RowGroupReader`

```rust
trait RowGroupReader: Send + Sync
```

**Implementors** (1)

- `parquet::file::serialized_reader::SerializedRowGroupReader`

**Methods** (6)

```rust
fn get_column_bloom_filter(&self, i: usize) -> Option<&Sbbf>
fn get_column_page_reader(&self, i: usize) -> Result<Box<dyn PageReader>>
fn get_column_reader(&self, i: usize) -> Result<ColumnReader>
fn get_row_iter(&self, projection: Option<SchemaType>) -> Result<RowIter<'_>>
fn metadata(&self) -> &RowGroupMetaData
fn num_columns(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/parquet.file.reader.RowGroupReader.md).


Parquet row group reader API. With this, user can get metadata information about the
row group, as well as readers for each individual column chunk.

---
