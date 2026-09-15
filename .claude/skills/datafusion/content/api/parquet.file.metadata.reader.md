# `parquet::file::metadata::reader`

Crate `parquet` · 2 public items · structured records in [`model/parquet.file.metadata.reader.json`](../model/parquet.file.metadata.reader.json)

## PageIndexPolicy

`enum` · `parquet::file::metadata::reader::PageIndexPolicy`

Also reachable as `parquet::file::metadata::PageIndexPolicy`

```rust
enum PageIndexPolicy
```

**Variants**: `Skip`, `Optional`, `Required`

**Implements**: `core::convert::From`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(value: bool) -> Self
```

Describes the policy for reading page indexes

---

## ParquetMetaDataReader

`struct` · `parquet::file::metadata::reader::ParquetMetaDataReader`

Also reachable as `parquet::file::metadata::ParquetMetaDataReader`

```rust
struct ParquetMetaDataReader
```

**Derives**: Debug, Default

**Methods** (25)

```rust
fn decode_metadata(buf: &[u8]) -> Result<ParquetMetaData>
fn decode_metadata_with_options(buf: &[u8], options: Option<&ParquetMetaDataOptions>) -> Result<ParquetMetaData>
fn decode_schema(buf: &[u8]) -> Result<Arc<SchemaDescriptor>>
fn finish(&mut self) -> Result<ParquetMetaData>
fn has_metadata(&self) -> bool
async fn load_and_finish<F: MetadataFetch>(self, fetch: F, file_size: u64) -> Result<ParquetMetaData>
async fn load_page_index<F: MetadataFetch>(&mut self, fetch: F) -> Result<()>
async fn load_via_suffix_and_finish<F: MetadataSuffixFetch>(self, fetch: F) -> Result<ParquetMetaData>
fn metadata_size(&self) -> Option<usize>
fn new() -> Self
fn new_with_metadata(metadata: ParquetMetaData) -> Self
fn parse_and_finish<R: ChunkReader>(self, reader: &R) -> Result<ParquetMetaData>
fn read_page_indexes<R: ChunkReader>(&mut self, reader: &R) -> Result<()>
fn read_page_indexes_sized<R: ChunkReader>(&mut self, reader: &R, file_size: u64) -> Result<()>
async fn try_load<F: MetadataFetch>(&mut self, fetch: F, file_size: u64) -> Result<()>
async fn try_load_via_suffix<F: MetadataSuffixFetch>(&mut self, fetch: F) -> Result<()>
fn try_parse<R: ChunkReader>(&mut self, reader: &R) -> Result<()>
fn try_parse_sized<R: ChunkReader>(&mut self, reader: &R, file_size: u64) -> Result<()>
fn with_arrow_reader_options(self, options: Option<&ArrowReaderOptions>) -> Self
fn with_column_index_policy(self, policy: PageIndexPolicy) -> Self
fn with_decryption_properties(self, properties: Option<std::sync::Arc<FileDecryptionProperties>>) -> Self
fn with_metadata_options(self, options: Option<ParquetMetaDataOptions>) -> Self
fn with_offset_index_policy(self, policy: PageIndexPolicy) -> Self
fn with_page_index_policy(self, policy: PageIndexPolicy) -> Self
fn with_prefetch_hint(self, prefetch: Option<usize>) -> Self
```

Reads [`ParquetMetaData`] from a byte stream, with either synchronous or
asynchronous I/O.

There are two flavors of APIs:
* Synchronous: [`Self::try_parse()`], [`Self::try_parse_sized()`], [`Self::parse_and_finish()`], etc.
* Asynchronous (requires `async` and `arrow` features): [`Self::try_load()`], etc

 See the [`ParquetMetaDataPushDecoder`] for an API that does not require I/O.

# Format Notes

Parquet metadata is not necessarily contiguous in a Parquet file: a portion is stored
in the footer (the last bytes of the file), but other portions (such as the
PageIndex) can be stored elsewhere.
See [`crate::file::metadata::ParquetMetaDataWriter#output-format`] for more details of
Parquet metadata.

This reader handles reading the footer as well as the non contiguous parts
of the metadata (`PageIndex` and `ColumnIndex`). It does not handle reading Bloom Filters.

# Example
```no_run
# use parquet::file::metadata::{PageIndexPolicy, ParquetMetaDataReader};
# fn open_parquet_file(path: &str) -> std::fs::File { unimplemented!(); }
// read parquet metadata including page indexes from a file
let file = open_parquet_file("some_path.parquet");
let mut reader = ParquetMetaDataReader::new()
    .with_page_index_policy(PageIndexPolicy::Required);
reader.try_parse(&file).unwrap();
let metadata = reader.finish().unwrap();
assert!(metadata.column_index().is_some());
assert!(metadata.offset_index().is_some());
```

---
