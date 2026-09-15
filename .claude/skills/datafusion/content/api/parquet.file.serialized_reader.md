# `parquet::file::serialized_reader`

Crate `parquet` · 6 public items · structured records in [`model/parquet.file.serialized_reader.json`](../model/parquet.file.serialized_reader.json)

## ReadOptions

`struct` · `parquet::file::serialized_reader::ReadOptions`

```rust
struct ReadOptions
```

A collection of options for reading a Parquet file.

Predicates are currently only supported on row group metadata.
All predicates will be chained using 'AND' to filter the row groups.

---

## ReadOptionsBuilder

`struct` · `parquet::file::serialized_reader::ReadOptionsBuilder`

```rust
struct ReadOptionsBuilder
```

**Derives**: Default

**Methods** (11)

```rust
fn build(self) -> ReadOptions
fn new() -> Self
fn with_column_stats_policy(self, policy: ParquetStatisticsPolicy) -> Self
fn with_encoding_stats_as_mask(self, val: bool) -> Self
fn with_encoding_stats_policy(self, policy: ParquetStatisticsPolicy) -> Self
fn with_page_index(self) -> Self
fn with_parquet_schema(self, schema: SchemaDescPtr) -> Self
fn with_predicate(self, predicate: ReadGroupPredicate) -> Self
fn with_range(self, start: i64, end: i64) -> Self
fn with_reader_properties(self, properties: ReaderProperties) -> Self
fn with_size_stats_policy(self, policy: ParquetStatisticsPolicy) -> Self
```

A builder for [`ReadOptions`].
For the predicates that are added to the builder,
they will be chained using 'AND' to filter the row groups.

---

## SerializedFileReader

`struct` · `parquet::file::serialized_reader::SerializedFileReader`

Also reachable as `parquet::file::reader::SerializedFileReader`

```rust
struct SerializedFileReader<R: ChunkReader>
```

**Implements**: `core::convert::TryFrom`, `core::iter::traits::collect::IntoIterator`, `parquet::file::reader::FileReader`

**Methods** (2)

```rust
fn new(chunk_reader: R) -> Result<Self>
fn new_with_options(chunk_reader: R, options: ReadOptions) -> Result<Self>
```

**via `core::convert::TryFrom`**

```rust
fn try_from(path: &Path) -> Result<Self>
fn try_from(path: String) -> Result<Self>
fn try_from(path: &str) -> Result<Self>
fn try_from(file: File) -> Result<Self>
```

**via `core::iter::traits::collect::IntoIterator`**

```rust
fn into_iter(self) -> Self::IntoIter
```

**via `parquet::file::reader::FileReader`**

```rust
fn get_row_group(&self, i: usize) -> Result<Box<dyn RowGroupReader + '_>>
fn get_row_iter(&self, projection: Option<SchemaType>) -> Result<RowIter<'_>>
fn metadata(&self) -> &ParquetMetaData
fn num_row_groups(&self) -> usize
```

A serialized implementation for Parquet [`FileReader`].

---

## SerializedPageReader

`struct` · `parquet::file::serialized_reader::SerializedPageReader`

Also reachable as `parquet::file::reader::SerializedPageReader`

```rust
struct SerializedPageReader<R: ChunkReader>
```

**Implements**: `core::iter::traits::iterator::Iterator`, `parquet::column::page::PageReader`

**Methods** (2)

```rust
fn new(reader: Arc<R>, column_chunk_metadata: &ColumnChunkMetaData, total_rows: usize, page_locations: Option<Vec<PageLocation>>) -> Result<Self>
fn new_with_properties(reader: Arc<R>, meta: &ColumnChunkMetaData, total_rows: usize, page_locations: Option<Vec<PageLocation>>, props: ReaderPropertiesPtr) -> Result<Self>
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
```

**via `parquet::column::page::PageReader`**

```rust
fn at_record_boundary(&mut self) -> Result<bool>
fn get_next_page(&mut self) -> Result<Option<Page>>
fn peek_next_page(&mut self) -> Result<Option<PageMetadata>>
fn skip_next_page(&mut self) -> Result<()>
```

A serialized implementation for Parquet [`PageReader`].

---

## SerializedRowGroupReader

`struct` · `parquet::file::serialized_reader::SerializedRowGroupReader`

```rust
struct SerializedRowGroupReader<'a, R: ChunkReader>
```

**Implements**: `parquet::file::reader::RowGroupReader`

**Methods** (1)

```rust
fn new(chunk_reader: Arc<R>, metadata: &'a RowGroupMetaData, offset_index: Option<&'a [OffsetIndexMetaData]>, props: ReaderPropertiesPtr) -> Result<Self>
```

**via `parquet::file::reader::RowGroupReader`**

```rust
fn get_column_bloom_filter(&self, i: usize) -> Option<&Sbbf>
fn get_column_page_reader(&self, i: usize) -> Result<Box<dyn PageReader>>
fn get_row_iter(&self, projection: Option<SchemaType>) -> Result<RowIter<'_>>
fn metadata(&self) -> &RowGroupMetaData
fn num_columns(&self) -> usize
```

A serialized implementation for Parquet [`RowGroupReader`].

---

## ReadGroupPredicate

`type_alias` · `parquet::file::serialized_reader::ReadGroupPredicate`

```rust
type ReadGroupPredicate = Box<dyn FnMut(&RowGroupMetaData, usize) -> bool>
```

A predicate for filtering row groups, invoked with the metadata and index
of each row group in the file. Only row groups for which the predicate
evaluates to `true` will be scanned

---
