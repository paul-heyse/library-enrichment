# `datafusion_datasource_parquet::file_format`

Crate `datafusion-datasource-parquet` · 6 public items · structured records in [`model/datafusion_datasource_parquet.file_format.json`](../model/datafusion_datasource_parquet.file_format.json)

## fetch_parquet_metadata

`function` · `datafusion_datasource_parquet::file_format::fetch_parquet_metadata`

> **Deprecated** — since 50.0.0: Use `DFParquetMetadata::fetch_metadata` instead

Also reachable as `datafusion::datasource::file_format::parquet::fetch_parquet_metadata`, `datafusion::datasource::physical_plan::parquet::fetch_parquet_metadata`, `datafusion_datasource_parquet::fetch_parquet_metadata`

```rust
async fn fetch_parquet_metadata(store: &dyn ObjectStore, object_meta: &object_store::ObjectMeta, size_hint: Option<usize>, decryption_properties: Option<&datafusion_common::encryption::FileDecryptionProperties>, file_metadata_cache: Option<std::sync::Arc<datafusion_execution::cache::cache_manager::FileMetadataCache>>) -> datafusion_common::Result<std::sync::Arc<parquet::file::metadata::ParquetMetaData>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource_parquet.file_format.fetch_parquet_metadata.md).


Fetches parquet metadata from ObjectStore for given object

This component is a subject to **change** in near future and is exposed for low level integrations
through [`ParquetFileReaderFactory`].

[`ParquetFileReaderFactory`]: crate::ParquetFileReaderFactory

---

## fetch_statistics

`function` · `datafusion_datasource_parquet::file_format::fetch_statistics`

> **Deprecated** — since 50.0.0: Use `DFParquetMetadata::fetch_statistics` instead

Also reachable as `datafusion::datasource::file_format::parquet::fetch_statistics`, `datafusion::datasource::physical_plan::parquet::fetch_statistics`, `datafusion_datasource_parquet::fetch_statistics`

```rust
async fn fetch_statistics(store: &dyn ObjectStore, table_schema: arrow::datatypes::SchemaRef, file: &object_store::ObjectMeta, metadata_size_hint: Option<usize>, decryption_properties: Option<&datafusion_common::encryption::FileDecryptionProperties>, file_metadata_cache: Option<std::sync::Arc<datafusion_execution::cache::cache_manager::FileMetadataCache>>) -> datafusion_common::Result<datafusion_common::Statistics>
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource_parquet.file_format.fetch_statistics.md).


Read and parse the statistics of the Parquet file at location `path`

See [`statistics_from_parquet_meta_calc`] for more details

---

## statistics_from_parquet_meta_calc

`function` · `datafusion_datasource_parquet::file_format::statistics_from_parquet_meta_calc`

> **Deprecated** — since 50.0.0: Use `DFParquetMetadata::statistics_from_parquet_metadata` instead

Also reachable as `datafusion::datasource::file_format::parquet::statistics_from_parquet_meta_calc`, `datafusion::datasource::physical_plan::parquet::statistics_from_parquet_meta_calc`, `datafusion_datasource_parquet::statistics_from_parquet_meta_calc`

```rust
fn statistics_from_parquet_meta_calc(metadata: &parquet::file::metadata::ParquetMetaData, table_schema: arrow::datatypes::SchemaRef) -> datafusion_common::Result<datafusion_common::Statistics>
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource_parquet.file_format.statistics_from_parquet_meta_calc.md).


---

## ObjectStoreFetch

`struct` · `datafusion_datasource_parquet::file_format::ObjectStoreFetch`

Also reachable as `datafusion::datasource::file_format::parquet::ObjectStoreFetch`, `datafusion::datasource::physical_plan::parquet::ObjectStoreFetch`, `datafusion_datasource_parquet::ObjectStoreFetch`

```rust
struct ObjectStoreFetch<'a>
```

**Implements**: `parquet::arrow::async_reader::metadata::MetadataFetch`

**Methods** (1)

```rust
fn new(store: &'a dyn ObjectStore, meta: &'a ObjectMeta) -> Self
```

**via `parquet::arrow::async_reader::metadata::MetadataFetch`**

```rust
fn fetch(&mut self, range: Range<u64>) -> BoxFuture<'_, Result<Bytes, ParquetError>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource_parquet.file_format.ObjectStoreFetch.md).


[`MetadataFetch`] adapter for reading bytes from an [`ObjectStore`]

---

## ParquetFormat

`struct` · `datafusion_datasource_parquet::file_format::ParquetFormat`

Also reachable as `datafusion::datasource::file_format::parquet::ParquetFormat`, `datafusion::datasource::physical_plan::parquet::ParquetFormat`, `datafusion_datasource_parquet::ParquetFormat`

```rust
struct ParquetFormat
```

**Implements**: `datafusion_datasource::file_format::FileFormat`

**Derives**: Debug, Default

**Methods** (15)

```rust
fn binary_as_string(&self) -> bool
fn coerce_int96(&self) -> Option<String>
fn enable_pruning(&self) -> bool
fn force_view_types(&self) -> bool
fn metadata_size_hint(&self) -> Option<usize>
fn new() -> Self
fn options(&self) -> &TableParquetOptions
fn skip_metadata(&self) -> bool
fn with_binary_as_string(self, binary_as_string: bool) -> Self
fn with_coerce_int96(self, time_unit: Option<String>) -> Self
fn with_enable_pruning(self, enable: bool) -> Self
fn with_force_view_types(self, use_views: bool) -> Self
fn with_metadata_size_hint(self, size_hint: Option<usize>) -> Self
fn with_options(self, options: TableParquetOptions) -> Self
fn with_skip_metadata(self, skip_metadata: bool) -> Self
```

**via `datafusion_datasource::file_format::FileFormat`**

```rust
fn compression_type(&self) -> Option<FileCompressionType>
async fn create_physical_plan(&self, state: &dyn Session, conf: FileScanConfig) -> Result<Arc<dyn ExecutionPlan>>
async fn create_writer_physical_plan(&self, input: Arc<dyn ExecutionPlan>, _state: &dyn Session, conf: FileSinkConfig, order_requirements: Option<LexRequirement>) -> Result<Arc<dyn ExecutionPlan>>
fn file_source(&self, table_schema: TableSchema) -> Arc<dyn FileSource>
fn get_ext(&self) -> String
fn get_ext_with_compression(&self, file_compression_type: &FileCompressionType) -> Result<String>
async fn infer_ordering(&self, state: &dyn Session, store: &Arc<dyn ObjectStore>, table_schema: SchemaRef, object: &ObjectMeta) -> Result<Option<LexOrdering>>
async fn infer_schema(&self, state: &dyn Session, store: &Arc<dyn ObjectStore>, objects: &[ObjectMeta]) -> Result<SchemaRef>
async fn infer_stats(&self, state: &dyn Session, store: &Arc<dyn ObjectStore>, table_schema: SchemaRef, object: &ObjectMeta) -> Result<Statistics>
async fn infer_stats_and_ordering(&self, state: &dyn Session, store: &Arc<dyn ObjectStore>, table_schema: SchemaRef, object: &ObjectMeta) -> Result<datafusion_datasource::file_format::FileMeta>
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource_parquet.file_format.ParquetFormat.md).


The Apache Parquet `FileFormat` implementation

---

## ParquetFormatFactory

`struct` · `datafusion_datasource_parquet::file_format::ParquetFormatFactory`

Also reachable as `datafusion::datasource::file_format::parquet::ParquetFormatFactory`, `datafusion::datasource::physical_plan::parquet::ParquetFormatFactory`, `datafusion_datasource_parquet::ParquetFormatFactory`

```rust
struct ParquetFormatFactory
```

**Fields**: `options`

**Implements**: `datafusion_common::file_options::file_type::GetExt`, `datafusion_datasource::file_format::FileFormatFactory`

**Derives**: Debug, Default

**Methods** (2)

```rust
fn new() -> Self
fn new_with_options(options: TableParquetOptions) -> Self
```

**via `datafusion_common::file_options::file_type::GetExt`**

```rust
fn get_ext(&self) -> String
```

**via `datafusion_datasource::file_format::FileFormatFactory`**

```rust
fn create(&self, state: &dyn Session, format_options: &std::collections::HashMap<String, String>) -> Result<Arc<dyn FileFormat>>
fn default(&self) -> Arc<dyn FileFormat>
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource_parquet.file_format.ParquetFormatFactory.md).


Factory struct used to create [ParquetFormat]

---
