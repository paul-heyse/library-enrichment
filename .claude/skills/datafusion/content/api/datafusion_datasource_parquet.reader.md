# `datafusion_datasource_parquet::reader`

Crate `datafusion-datasource-parquet` · 5 public items · structured records in [`model/datafusion_datasource_parquet.reader.json`](../model/datafusion_datasource_parquet.reader.json)

## CachedParquetFileReaderFactory

`struct` · `datafusion_datasource_parquet::reader::CachedParquetFileReaderFactory`

```rust
struct CachedParquetFileReaderFactory
```

**Implements**: `datafusion_datasource_parquet::reader::ParquetFileReaderFactory`

**Derives**: Debug

**Methods** (1)

```rust
fn new(store: Arc<dyn ObjectStore>, metadata_cache: Arc<FileMetadataCache>) -> Self
```

**via `datafusion_datasource_parquet::reader::ParquetFileReaderFactory`**

```rust
fn create_reader(&self, partition_index: usize, partitioned_file: PartitionedFile, metadata_size_hint: Option<usize>, metrics: &ExecutionPlanMetricsSet) -> datafusion_common::Result<Box<dyn AsyncFileReader + Send>>
```

Implementation of [`ParquetFileReaderFactory`] supporting the caching of footer and page
metadata. Reads and updates the [`FileMetadataCache`] with the [`ParquetMetaData`] data.

[`ParquetFileReader::get_metadata`] forwards the [`parquet::file::metadata::PageIndexPolicy`] from
[`ArrowReaderOptions`] to [`DFParquetMetadata::fetch_metadata`], so callers such as the
parquet opener can skip page-index I/O during the initial metadata load.

---

## CachedParquetMetaData

`struct` · `datafusion_datasource_parquet::reader::CachedParquetMetaData`

```rust
struct CachedParquetMetaData
```

**Implements**: `datafusion_execution::cache::cache_manager::FileMetadata`

**Methods** (2)

```rust
fn new(metadata: Arc<ParquetMetaData>) -> Self
fn parquet_metadata(&self) -> &Arc<ParquetMetaData>
```

**via `datafusion_execution::cache::cache_manager::FileMetadata`**

```rust
fn as_any(&self) -> &dyn Any
fn extra_info(&self) -> HashMap<String, String>
fn memory_size(&self) -> usize
```

Wrapper to implement [`FileMetadata`] for [`ParquetMetaData`].

---

## DefaultParquetFileReaderFactory

`struct` · `datafusion_datasource_parquet::reader::DefaultParquetFileReaderFactory`

```rust
struct DefaultParquetFileReaderFactory
```

**Implements**: `datafusion_datasource_parquet::reader::ParquetFileReaderFactory`

**Derives**: Debug

**Methods** (1)

```rust
fn new(store: Arc<dyn ObjectStore>) -> Self
```

**via `datafusion_datasource_parquet::reader::ParquetFileReaderFactory`**

```rust
fn create_reader(&self, partition_index: usize, partitioned_file: PartitionedFile, metadata_size_hint: Option<usize>, metrics: &ExecutionPlanMetricsSet) -> datafusion_common::Result<Box<dyn AsyncFileReader + Send>>
```

Default implementation of [`ParquetFileReaderFactory`]

This implementation:
1. Reads parquet directly from an underlying [`ObjectStore`] instance.
2. Reads the footer and page metadata on demand.
3. Does not cache metadata or coalesce I/O operations.

---

## ParquetFileReader

`struct` · `datafusion_datasource_parquet::reader::ParquetFileReader`

```rust
struct ParquetFileReader
```

**Implements**: `core::ops::drop::Drop`, `parquet::arrow::async_reader::AsyncFileReader`

**Methods** (4)

```rust
fn file_metrics(&self) -> &ParquetFileMetrics
fn partitioned_file(&self) -> &PartitionedFile
fn with_metadata_cache(self, metadata_cache: Option<Arc<FileMetadataCache>>) -> Self
fn with_metadata_hint(self, metadata_size_hint: Option<usize>) -> Self
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

**via `parquet::arrow::async_reader::AsyncFileReader`**

```rust
fn get_byte_ranges(&mut self, ranges: Vec<Range<u64>>) -> BoxFuture<'_, parquet::errors::Result<Vec<Bytes>>> where Self: Send
fn get_bytes(&mut self, range: Range<u64>) -> BoxFuture<'_, parquet::errors::Result<Bytes>>
fn get_metadata<'a>(&'a mut self, options: Option<&'a ArrowReaderOptions>) -> BoxFuture<'a, parquet::errors::Result<Arc<ParquetMetaData>>>
```

Implements [`AsyncFileReader`] for a parquet file in object storage.

This implementation reads data directly from the underlying [`ObjectStore`]
on demand, as required, tracking the number of bytes read.

When configured via [`Self::with_metadata_cache`], [`Self::get_metadata`]
reads footer and page metadata from the cache when available and populates
the cache otherwise. Without a cache, metadata is fetched fresh on every call.

# Notes

This implementation does not coalesce I/O operations or cache bytes. Such
optimizations can be done either at the object store level or by providing
a custom implementation of [`ParquetFileReaderFactory`].

---

## ParquetFileReaderFactory

`trait` · `datafusion_datasource_parquet::reader::ParquetFileReaderFactory`

Also reachable as `datafusion::datasource::physical_plan::ParquetFileReaderFactory`

```rust
trait ParquetFileReaderFactory: Debug + Send + Sync + 'static
```

**Implementors** (2)

- `datafusion_datasource_parquet::reader::CachedParquetFileReaderFactory`
- `datafusion_datasource_parquet::reader::DefaultParquetFileReaderFactory`

**Methods** (1)

```rust
fn create_reader(&self, partition_index: usize, partitioned_file: PartitionedFile, metadata_size_hint: Option<usize>, metrics: &ExecutionPlanMetricsSet) -> datafusion_common::Result<Box<dyn AsyncFileReader + Send>>
```

Interface for reading Apache Parquet files.

The combined implementations of [`ParquetFileReaderFactory`] and
[`AsyncFileReader`] can be used to provide custom data access operations
such as pre-cached metadata, I/O coalescing, etc.

See [`DefaultParquetFileReaderFactory`] for a simple implementation.

---
