# `datafusion_datasource_parquet::metadata`

Crate `datafusion-datasource-parquet` · 3 public items · structured records in [`model/datafusion_datasource_parquet.metadata.json`](../model/datafusion_datasource_parquet.metadata.json)

## ordering_from_parquet_metadata

`function` · `datafusion_datasource_parquet::metadata::ordering_from_parquet_metadata`

```rust
fn ordering_from_parquet_metadata(metadata: &parquet::file::metadata::ParquetMetaData, schema: &arrow::datatypes::SchemaRef) -> datafusion_common::Result<Option<datafusion_physical_expr_common::sort_expr::LexOrdering>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource_parquet.metadata.ordering_from_parquet_metadata.md).


Extracts ordering information from Parquet metadata.

This function reads the sorting_columns from the first row group's metadata
and converts them into a [`LexOrdering`] that can be used by the query engine.

# Arguments
* `metadata` - The Parquet metadata containing sorting_columns information
* `schema` - The Arrow schema to use for column lookup

# Returns
* `Ok(Some(ordering))` if valid ordering information was found
* `Ok(None)` if no sorting columns were specified or they couldn't be resolved

---

## CachedParquetMetaData

`struct` · `datafusion_datasource_parquet::metadata::CachedParquetMetaData`

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

[Full member, field, variant and typed contracts](../operations/datafusion_datasource_parquet.metadata.CachedParquetMetaData.md).


Wrapper to implement [`FileMetadata`] for [`ParquetMetaData`].

---

## DFParquetMetadata

`struct` · `datafusion_datasource_parquet::metadata::DFParquetMetadata`

```rust
struct DFParquetMetadata<'a>
```

**Fields**: `coerce_int96`, `coerce_int96_tz`

**Derives**: Debug

**Methods** (11)

```rust
async fn fetch_metadata(&self) -> Result<Arc<ParquetMetaData>>
async fn fetch_schema(&self) -> Result<Schema>
async fn fetch_statistics(&self, table_schema: &SchemaRef) -> Result<Statistics>
fn new(store: &'a dyn ObjectStore, object_meta: &'a ObjectMeta) -> Self
fn statistics_from_parquet_metadata(metadata: &ParquetMetaData, logical_file_schema: &SchemaRef) -> Result<Statistics>
fn with_coerce_int96(self, time_unit: Option<TimeUnit>) -> Self
fn with_coerce_int96_tz(self, timezone: Option<Arc<str>>) -> Self
fn with_decryption_properties(self, decryption_properties: Option<Arc<FileDecryptionProperties>>) -> Self
fn with_file_metadata_cache(self, file_metadata_cache: Option<Arc<FileMetadataCache>>) -> Self
fn with_metadata_size_hint(self, metadata_size_hint: Option<usize>) -> Self
fn with_page_index_policy(self, page_index_policy: Option<PageIndexPolicy>) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource_parquet.metadata.DFParquetMetadata.md).


Handles fetching Parquet file schema, metadata and statistics
from object store.

This component is exposed for low level integrations through
[`ParquetFileReaderFactory`].

[`ParquetFileReaderFactory`]: crate::ParquetFileReaderFactory

---
