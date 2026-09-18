# `datafusion_execution::cache::cache_manager::CacheManagerConfig`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.cache.cache_manager.CacheManagerConfig.json).

<a id="op-f8c40b300dad7c3c1690af03"></a>
## CacheManagerConfig

`struct` · `datafusion_execution::cache::cache_manager::CacheManagerConfig` · datafusion-execution 55.1.0

```rust
struct CacheManagerConfig
```

Source: `src/cache/cache_manager.rs:413`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2e081d5bd9cea3a5ab521655"></a>
## clone

`function` · `datafusion_execution::cache::cache_manager::CacheManagerConfig::clone` · datafusion-execution 55.1.0

```rust
fn clone(&self) -> CacheManagerConfig
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::cache_manager::CacheManagerConfig", "path": "CacheManagerConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [412, 10], "end": [412, 15], "filename": "src/cache/cache_manager.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/cache/cache_manager.rs:412`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-503ebc59cfeb0a64aee5bd85"></a>
## default

`function` · `datafusion_execution::cache::cache_manager::CacheManagerConfig::default` · datafusion-execution 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::cache_manager::CacheManagerConfig", "path": "CacheManagerConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [442, 1], "end": [454, 2], "filename": "src/cache/cache_manager.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/cache/cache_manager.rs:443`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-84ef2605f0d5f8f14acabd58"></a>
## file_metadata_cache

`struct_field` · `datafusion_execution::cache::cache_manager::CacheManagerConfig::file_metadata_cache` · datafusion-execution 55.1.0

```rust
file_metadata_cache: Option<std::sync::Arc<FileMetadataCache>>
```

Source: `src/cache/cache_manager.rs:437`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Cache of file-embedded metadata, used to avoid reading it multiple times when processing a
data file (e.g., Parquet footer and page metadata).
If not provided, the [`CacheManager`](../operations/datafusion_execution.cache.cache_manager.CacheManager.md#op-b6af2b372497895e325851ad) will create it.

<a id="op-3b77ae44e0d0a1057aecfdb9"></a>
## file_statistics_cache

`struct_field` · `datafusion_execution::cache::cache_manager::CacheManagerConfig::file_statistics_cache` · datafusion-execution 55.1.0

```rust
file_statistics_cache: Option<std::sync::Arc<FileStatisticsCache>>
```

Source: `src/cache/cache_manager.rs:417`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Enable caching of file statistics when listing files.
Enabling the cache avoids repeatedly reading file statistics in a DataFusion session.
Default is enabled. Currently only Parquet files are supported.

<a id="op-4081a2c6a77d999a2f0a3aa5"></a>
## file_statistics_cache_limit

`struct_field` · `datafusion_execution::cache::cache_manager::CacheManagerConfig::file_statistics_cache_limit` · datafusion-execution 55.1.0

```rust
file_statistics_cache_limit: usize
```

Source: `src/cache/cache_manager.rs:419`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Limit of the file statistics cache, in bytes. Default: 20MiB.

<a id="op-9ac6616d1d1319cd974cadae"></a>
## list_files_cache

`struct_field` · `datafusion_execution::cache::cache_manager::CacheManagerConfig::list_files_cache` · datafusion-execution 55.1.0

```rust
list_files_cache: Option<std::sync::Arc<ListFilesCache>>
```

Source: `src/cache/cache_manager.rs:427`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Enable caching of file metadata when listing files.
Enabling the cache avoids repeat list and object metadata fetch operations, which may be
expensive in certain situations (e.g. remote object storage), for objects under paths that
are cached.
Note that if this option is enabled, DataFusion will not see any updates to the underlying
storage for at least `list_files_cache_ttl` duration.
Default is enabled.

<a id="op-e5250517c0c87f9dbd54cb6d"></a>
## list_files_cache_limit

`struct_field` · `datafusion_execution::cache::cache_manager::CacheManagerConfig::list_files_cache_limit` · datafusion-execution 55.1.0

```rust
list_files_cache_limit: usize
```

Source: `src/cache/cache_manager.rs:429`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Limit of the `list_files_cache`, in bytes. Default: 1MiB.

<a id="op-2dc9af3f2940d5aa262de28b"></a>
## list_files_cache_ttl

`struct_field` · `datafusion_execution::cache::cache_manager::CacheManagerConfig::list_files_cache_ttl` · datafusion-execution 55.1.0

```rust
list_files_cache_ttl: Option<std::time::Duration>
```

Source: `src/cache/cache_manager.rs:433`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

The duration the list files cache will consider an entry valid after insertion. Note that
changes to the underlying storage system, such as adding or removing data, will not be
visible until an entry expires. Default: None (infinite).

<a id="op-589b3c462157f0f4df32e9e2"></a>
## metadata_cache_limit

`struct_field` · `datafusion_execution::cache::cache_manager::CacheManagerConfig::metadata_cache_limit` · datafusion-execution 55.1.0

```rust
metadata_cache_limit: usize
```

Source: `src/cache/cache_manager.rs:439`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Limit of the file-embedded metadata cache, in bytes.

<a id="op-d16e29e7adff3843146c6a48"></a>
## with_file_metadata_cache

`function` · `datafusion_execution::cache::cache_manager::CacheManagerConfig::with_file_metadata_cache` · datafusion-execution 55.1.0

```rust
fn with_file_metadata_cache(self, cache: Option<Arc<FileMetadataCache>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::cache_manager::CacheManagerConfig", "path": "CacheManagerConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [456, 1], "end": [510, 2], "filename": "src/cache/cache_manager.rs"}, "trait": null, "trait_path": null}`

Source: `src/cache/cache_manager.rs:497`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Sets the cache for file-embedded metadata.

<a id="op-f9c89e6515c16ed74e3e83b1"></a>
## with_file_statistics_cache

`function` · `datafusion_execution::cache::cache_manager::CacheManagerConfig::with_file_statistics_cache` · datafusion-execution 55.1.0

```rust
fn with_file_statistics_cache(self, cache: Option<Arc<FileStatisticsCache>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::cache_manager::CacheManagerConfig", "path": "CacheManagerConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [456, 1], "end": [510, 2], "filename": "src/cache/cache_manager.rs"}, "trait": null, "trait_path": null}`

Source: `src/cache/cache_manager.rs:458`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Set the cache for file statistics.

<a id="op-86c1bbe9f849fa13217392e2"></a>
## with_file_statistics_cache_limit

`function` · `datafusion_execution::cache::cache_manager::CacheManagerConfig::with_file_statistics_cache_limit` · datafusion-execution 55.1.0

```rust
fn with_file_statistics_cache_limit(self, limit: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::cache_manager::CacheManagerConfig", "path": "CacheManagerConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [456, 1], "end": [510, 2], "filename": "src/cache/cache_manager.rs"}, "trait": null, "trait_path": null}`

Source: `src/cache/cache_manager.rs:467`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Specifies the memory limit for the file statistics cache, in bytes.

<a id="op-acdc64a5664ee46342b278f8"></a>
## with_list_files_cache

`function` · `datafusion_execution::cache::cache_manager::CacheManagerConfig::with_list_files_cache` · datafusion-execution 55.1.0

```rust
fn with_list_files_cache(self, cache: Option<Arc<ListFilesCache>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::cache_manager::CacheManagerConfig", "path": "CacheManagerConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [456, 1], "end": [510, 2], "filename": "src/cache/cache_manager.rs"}, "trait": null, "trait_path": null}`

Source: `src/cache/cache_manager.rs:475`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Set the cache for listing files.

Default is `None` (disabled).

<a id="op-c42c5e008a45271cf3f2a7a7"></a>
## with_list_files_cache_limit

`function` · `datafusion_execution::cache::cache_manager::CacheManagerConfig::with_list_files_cache_limit` · datafusion-execution 55.1.0

```rust
fn with_list_files_cache_limit(self, limit: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::cache_manager::CacheManagerConfig", "path": "CacheManagerConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [456, 1], "end": [510, 2], "filename": "src/cache/cache_manager.rs"}, "trait": null, "trait_path": null}`

Source: `src/cache/cache_manager.rs:483`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Sets the limit of the list files cache, in bytes.

Default: 1MiB (1,048,576 bytes).

<a id="op-5cc606d757a5de1203317e1d"></a>
## with_list_files_cache_ttl

`function` · `datafusion_execution::cache::cache_manager::CacheManagerConfig::with_list_files_cache_ttl` · datafusion-execution 55.1.0

```rust
fn with_list_files_cache_ttl(self, ttl: Option<Duration>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::cache_manager::CacheManagerConfig", "path": "CacheManagerConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [456, 1], "end": [510, 2], "filename": "src/cache/cache_manager.rs"}, "trait": null, "trait_path": null}`

Source: `src/cache/cache_manager.rs:491`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Sets the TTL (time-to-live) for entries in the list files cache.

Default: None (infinite).

<a id="op-6d8b696a9d4f1f544750c3ca"></a>
## with_metadata_cache_limit

`function` · `datafusion_execution::cache::cache_manager::CacheManagerConfig::with_metadata_cache_limit` · datafusion-execution 55.1.0

```rust
fn with_metadata_cache_limit(self, limit: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::cache_manager::CacheManagerConfig", "path": "CacheManagerConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [456, 1], "end": [510, 2], "filename": "src/cache/cache_manager.rs"}, "trait": null, "trait_path": null}`

Source: `src/cache/cache_manager.rs:506`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Sets the limit of the file-embedded metadata cache, in bytes.
