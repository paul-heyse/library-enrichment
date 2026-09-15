# `parquet::file::writer`

Crate `parquet` · 7 public items · structured records in [`model/parquet.file.writer.json`](../model/parquet.file.writer.json)

## SerializedColumnWriter

`struct` · `parquet::file::writer::SerializedColumnWriter`

```rust
struct SerializedColumnWriter<'a>
```

**Methods** (4)

```rust
fn close(self) -> Result<()>
fn new(inner: ColumnWriter<'a>, on_close: Option<OnCloseColumnChunk<'a>>) -> Self
fn typed<T: DataType>(&mut self) -> &mut ColumnWriterImpl<'a, T>
fn untyped(&mut self) -> &mut ColumnWriter<'a>
```

A wrapper around a [`ColumnWriter`] that invokes a callback on [`Self::close`]

---

## SerializedFileWriter

`struct` · `parquet::file::writer::SerializedFileWriter`

```rust
struct SerializedFileWriter<W: Write>
```

**Derives**: Debug

**Methods** (14)

```rust
fn append_key_value_metadata(&mut self, kv_metadata: KeyValue)
fn bytes_written(&self) -> usize
fn close(self) -> Result<ParquetMetaData>
fn finish(&mut self) -> Result<ParquetMetaData>
fn flush(&mut self) -> std::io::Result<()>
fn flushed_row_groups(&self) -> &[RowGroupMetaData]
fn inner(&self) -> &W
fn inner_mut(&mut self) -> &mut W
fn into_inner(self) -> Result<W>
fn new(buf: W, schema: TypePtr, properties: WriterPropertiesPtr) -> Result<Self>
fn next_row_group(&mut self) -> Result<SerializedRowGroupWriter<'_, W>>
fn properties(&self) -> &WriterPropertiesPtr
fn schema_descr(&self) -> &SchemaDescriptor
fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()>
```

Parquet file writer API.

This is a low level API for writing Parquet files directly, and handles
tracking the location of file structures such as row groups and column
chunks, and writing the metadata and file footer.

Data is written to row groups using  [`SerializedRowGroupWriter`] and
columns using [`SerializedColumnWriter`]. The `SerializedFileWriter` tracks
where all the data is written, and assembles the final file metadata.

The main workflow should be as following:
- Create file writer, this will open a new file and potentially write some metadata.
- Request a new row group writer by calling `next_row_group`.
- Once finished writing row group, close row group writer by calling `close`
- Write subsequent row groups, if necessary.
- After all row groups have been written, close the file writer using `close` method.

---

## SerializedPageWriter

`struct` · `parquet::file::writer::SerializedPageWriter`

```rust
struct SerializedPageWriter<'a, W: Write>
```

**Implements**: `parquet::column::page::PageWriter`

**Methods** (1)

```rust
fn new(sink: &'a mut TrackedWrite<W>) -> Self
```

**via `parquet::column::page::PageWriter`**

```rust
fn close(&mut self) -> Result<()>
fn write_page(&mut self, page: CompressedPage) -> Result<PageWriteSpec>
```

A serialized implementation for Parquet [`PageWriter`].
Writes and serializes pages and metadata into output stream.

`SerializedPageWriter` should not be used after calling `close()`.

---

## SerializedRowGroupWriter

`struct` · `parquet::file::writer::SerializedRowGroupWriter`

```rust
struct SerializedRowGroupWriter<'a, W: Write>
```

**Methods** (4)

```rust
fn append_column<R: ChunkReader>(&mut self, reader: &R, close: ColumnCloseResult) -> Result<()>
fn close(self) -> Result<RowGroupMetaDataPtr>
fn new(schema_descr: SchemaDescPtr, properties: WriterPropertiesPtr, buf: &'a mut TrackedWrite<W>, row_group_index: i16, on_close: Option<OnCloseRowGroup<'a, W>>) -> Self
fn next_column(&mut self) -> Result<Option<SerializedColumnWriter<'_>>>
```

Parquet row group writer API.

Provides methods to access column writers in an iterator-like fashion, order is
guaranteed to match the order of schema leaves (column descriptors).

All columns should be written sequentially; the main workflow is:
- Request the next column using `next_column` method - this will return `None` if no
  more columns are available to write.
- Once done writing a column, close column writer with `close`
- Once all columns have been written, close row group writer with `close`
  method. The close method will return row group metadata and is no-op
  on already closed row group.

---

## TrackedWrite

`struct` · `parquet::file::writer::TrackedWrite`

```rust
struct TrackedWrite<W: Write>
```

**Implements**: `core::io::write::Write`

**Methods** (5)

```rust
fn bytes_written(&self) -> usize
fn inner(&self) -> &W
fn inner_mut(&mut self) -> &mut W
fn into_inner(self) -> Result<W>
fn new(inner: W) -> Self
```

**via `core::io::write::Write`**

```rust
fn flush(&mut self) -> std::io::Result<()>
fn write(&mut self, buf: &[u8]) -> std::io::Result<usize>
fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()>
fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> std::io::Result<usize>
```

A wrapper around a [`Write`] that keeps track of the number
of bytes that have been written. The given [`Write`] is wrapped
with a [`BufWriter`] to optimize writing performance.

---

## OnCloseColumnChunk

`type_alias` · `parquet::file::writer::OnCloseColumnChunk`

```rust
type OnCloseColumnChunk<'a> = Box<dyn FnOnce(column::writer::ColumnCloseResult) -> errors::Result<()> + 'a>
```

Callback invoked on closing a column chunk

---

## OnCloseRowGroup

`type_alias` · `parquet::file::writer::OnCloseRowGroup`

```rust
type OnCloseRowGroup<'a, W> = Box<dyn FnOnce(&'a mut TrackedWrite<W>, RowGroupMetaData, Vec<Option<bloom_filter::Sbbf>>, Vec<Option<file::page_index::column_index::ColumnIndexMetaData>>, Vec<Option<file::page_index::offset_index::OffsetIndexMetaData>>) -> errors::Result<()> + Send + 'a>
```

Callback invoked on closing a row group, arguments are:

- the row group metadata
- the column index for each column chunk
- the offset index for each column chunk

---
