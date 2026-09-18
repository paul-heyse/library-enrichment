# `datafusion_execution::cache::cache_manager`

Crate `datafusion-execution` · 14 public items · structured records in [`model/datafusion_execution.cache.cache_manager.json`](../model/datafusion_execution.cache.cache_manager.json)

## DEFAULT_FILE_STATISTICS_MEMORY_LIMIT

`constant` · `datafusion_execution::cache::cache_manager::DEFAULT_FILE_STATISTICS_MEMORY_LIMIT`

```rust
const DEFAULT_FILE_STATISTICS_MEMORY_LIMIT: usize = _
```

[Full member, field, variant and typed contracts](../operations/datafusion_execution.cache.cache_manager.DEFAULT_FILE_STATISTICS_MEMORY_LIMIT.md).


---

## DEFAULT_LIST_FILES_CACHE_MEMORY_LIMIT

`constant` · `datafusion_execution::cache::cache_manager::DEFAULT_LIST_FILES_CACHE_MEMORY_LIMIT`

```rust
const DEFAULT_LIST_FILES_CACHE_MEMORY_LIMIT: usize = _
```

[Full member, field, variant and typed contracts](../operations/datafusion_execution.cache.cache_manager.DEFAULT_LIST_FILES_CACHE_MEMORY_LIMIT.md).


---

## DEFAULT_LIST_FILES_CACHE_TTL

`constant` · `datafusion_execution::cache::cache_manager::DEFAULT_LIST_FILES_CACHE_TTL`

```rust
const DEFAULT_LIST_FILES_CACHE_TTL: Option<std::time::Duration> = None
```

[Full member, field, variant and typed contracts](../operations/datafusion_execution.cache.cache_manager.DEFAULT_LIST_FILES_CACHE_TTL.md).


---

## DEFAULT_METADATA_CACHE_LIMIT

`constant` · `datafusion_execution::cache::cache_manager::DEFAULT_METADATA_CACHE_LIMIT`

```rust
const DEFAULT_METADATA_CACHE_LIMIT: usize = _
```

[Full member, field, variant and typed contracts](../operations/datafusion_execution.cache.cache_manager.DEFAULT_METADATA_CACHE_LIMIT.md).


---

## meta_heap_bytes

`function` · `datafusion_execution::cache::cache_manager::meta_heap_bytes`

```rust
fn meta_heap_bytes(object_meta: &object_store::ObjectMeta) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_execution.cache.cache_manager.meta_heap_bytes.md).


Calculates the number of bytes an [`ObjectMeta`] occupies in the heap.

---

## CacheManager

`struct` · `datafusion_execution::cache::cache_manager::CacheManager`

```rust
struct CacheManager
```

**Derives**: Debug

**Methods** (8)

```rust
fn get_file_metadata_cache(&self) -> Arc<FileMetadataCache>
fn get_file_statistic_cache(&self) -> Option<Arc<FileStatisticsCache>>
fn get_file_statistic_cache_limit(&self) -> usize
fn get_list_files_cache(&self) -> Option<Arc<ListFilesCache>>
fn get_list_files_cache_limit(&self) -> usize
fn get_list_files_cache_ttl(&self) -> Option<Duration>
fn get_metadata_cache_limit(&self) -> usize
fn try_new(config: &CacheManagerConfig) -> Result<Arc<Self>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_execution.cache.cache_manager.CacheManager.md).


Manages various caches used in DataFusion.

Following DataFusion design principles, DataFusion provides default cache
implementations, while also allowing users to provide their own custom cache
implementations by implementing the relevant traits.

See [`CacheManagerConfig`] for configuration options.

---

## CacheManagerConfig

`struct` · `datafusion_execution::cache::cache_manager::CacheManagerConfig`

```rust
struct CacheManagerConfig
```

**Fields**: `file_statistics_cache`, `file_statistics_cache_limit`, `list_files_cache`, `list_files_cache_limit`, `list_files_cache_ttl`, `file_metadata_cache`, `metadata_cache_limit`

**Derives**: Clone, Default

**Methods** (7)

```rust
fn with_file_metadata_cache(self, cache: Option<Arc<FileMetadataCache>>) -> Self
fn with_file_statistics_cache(self, cache: Option<Arc<FileStatisticsCache>>) -> Self
fn with_file_statistics_cache_limit(self, limit: usize) -> Self
fn with_list_files_cache(self, cache: Option<Arc<ListFilesCache>>) -> Self
fn with_list_files_cache_limit(self, limit: usize) -> Self
fn with_list_files_cache_ttl(self, ttl: Option<Duration>) -> Self
fn with_metadata_cache_limit(self, limit: usize) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_execution.cache.cache_manager.CacheManagerConfig.md).


---

## CachedFileList

`struct` · `datafusion_execution::cache::cache_manager::CachedFileList`

```rust
struct CachedFileList
```

**Fields**: `files`

**Implements**: `core::convert::From`, `core::ops::deref::Deref`, `datafusion_execution::cache::CacheValue`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn files_matching_prefix(&self, prefix: &Option<Path>) -> Arc<Vec<ObjectMeta>>
fn new(files: Vec<ObjectMeta>) -> Self
```

**via `core::convert::From`**

```rust
fn from(files: Vec<ObjectMeta>) -> Self
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

**via `datafusion_execution::cache::CacheValue`**

```rust
fn size(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_execution.cache.cache_manager.CachedFileList.md).


Cached file listing.

TTL expiration is handled internally by the cache implementation.

---

## CachedFileMetadata

`struct` · `datafusion_execution::cache::cache_manager::CachedFileMetadata`

```rust
struct CachedFileMetadata
```

**Fields**: `meta`, `schema_fingerprint`, `statistics`, `ordering`

**Implements**: `datafusion_common::heap_size::DFHeapSize`, `datafusion_execution::cache::CacheValue`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn is_valid_for(&self, current_meta: &ObjectMeta, current_schema_fingerprint: &Arc<SchemaFingerprint>) -> bool
fn new(meta: ObjectMeta, schema_fingerprint: Arc<SchemaFingerprint>, statistics: Arc<Statistics>, ordering: Option<LexOrdering>) -> Self
```

**via `datafusion_common::heap_size::DFHeapSize`**

```rust
fn heap_size(&self, ctx: &mut DFHeapSizeCtx) -> usize
```

**via `datafusion_execution::cache::CacheValue`**

```rust
fn size(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_execution.cache.cache_manager.CachedFileMetadata.md).


Cached metadata for a file, including statistics and ordering.

This struct embeds the [`ObjectMeta`] used for cache validation,
the `file_schema` fingerprint, cached statistics, and ordering information.

---

## CachedFileMetadataEntry

`struct` · `datafusion_execution::cache::cache_manager::CachedFileMetadataEntry`

```rust
struct CachedFileMetadataEntry
```

**Fields**: `meta`, `file_metadata`

**Implements**: `datafusion_execution::cache::CacheValue`

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn is_valid_for(&self, current_meta: &ObjectMeta) -> bool
fn new(meta: ObjectMeta, file_metadata: Arc<dyn FileMetadata>) -> Self
```

**via `datafusion_execution::cache::CacheValue`**

```rust
fn size(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_execution.cache.cache_manager.CachedFileMetadataEntry.md).


Cached file metadata entry with validation information.

---

## FileMetadata

`trait` · `datafusion_execution::cache::cache_manager::FileMetadata`

```rust
trait FileMetadata: Any + Send + Sync
```

**Implementors** (2)

- `datafusion_datasource_parquet::metadata::CachedParquetMetaData`
- `datafusion_datasource_parquet::reader::CachedParquetMetaData`

**Methods** (3)

```rust
fn as_any(&self) -> &dyn Any
fn extra_info(&self) -> HashMap<String, String>
fn memory_size(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_execution.cache.cache_manager.FileMetadata.md).


Generic file-embedded metadata used with [`FileMetadataCache`].

For example, Parquet footers and page metadata can be represented
using this trait.

See [`crate::runtime_env::RuntimeEnv`] for more details

---

## FileMetadataCache

`type_alias` · `datafusion_execution::cache::cache_manager::FileMetadataCache`

```rust
type FileMetadataCache = dyn Cache<object_store::path::Path, CachedFileMetadataEntry>
```

[Full member, field, variant and typed contracts](../operations/datafusion_execution.cache.cache_manager.FileMetadataCache.md).


A cache for storing file-embedded metadata.

This cache stores per-file metadata in the form of [`CachedFileMetadataEntry`],
which includes the [`ObjectMeta`] for validation.

For example, the built in [`ListingTable`] uses this cache to avoid parsing
Parquet footers multiple times for the same file.

The typical usage pattern is:
1. Call `get(path)` to check for cached value
2. If `Some(cached)`, validate with `cached.is_valid_for(&current_meta)`
3. If invalid or missing, compute new value and call `put(path, new_value)`

See [`crate::runtime_env::RuntimeEnv`] for more details.

[`ListingTable`]: https://docs.rs/datafusion/latest/datafusion/datasource/listing/struct.ListingTable.html

---

## FileStatisticsCache

`type_alias` · `datafusion_execution::cache::cache_manager::FileStatisticsCache`

```rust
type FileStatisticsCache = dyn Cache<TableScopedPath, CachedFileMetadata>
```

[Full member, field, variant and typed contracts](../operations/datafusion_execution.cache.cache_manager.FileStatisticsCache.md).


A cache for file statistics and orderings.

This cache stores [`CachedFileMetadata`] which includes:
- File metadata for validation (size, last_modified)
- Statistics for the file
- Ordering information for the file

If enabled via [`CacheManagerConfig::with_file_statistics_cache`] this
cache avoids inferring the same file statistics repeatedly during the
session lifetime.

The typical usage pattern is:
1. Call `get(path)` to check for cached value
2. If `Some(cached)`, validate with
   `cached.is_valid_for(&current_meta, &current_schema_fingerprint)`
3. If invalid or missing, compute new value and call `put(path, new_value)`

See [`crate::runtime_env::RuntimeEnv`] for more details

---

## ListFilesCache

`type_alias` · `datafusion_execution::cache::cache_manager::ListFilesCache`

```rust
type ListFilesCache = dyn Cache<TableScopedPath, CachedFileList>
```

[Full member, field, variant and typed contracts](../operations/datafusion_execution.cache.cache_manager.ListFilesCache.md).


A cache for storing the [`ObjectMeta`]s that result from listing a path.

Listing a path means doing an object store "list" operation or `ls`
command on the local filesystem. This operation can be expensive,
especially when done over remote object stores.

The cache key is always the table's base path, ensuring a stable cache key.
The cached value is a [`CachedFileList`] containing the files and a timestamp.

Partition filtering is done after retrieval using [`CachedFileList::files_matching_prefix`].

See [`crate::runtime_env::RuntimeEnv`] for more details.

---
