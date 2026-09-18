# `datafusion_execution::cache::cache_manager::CacheManager`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.cache.cache_manager.CacheManager.json).

<a id="op-b6af2b372497895e325851ad"></a>
## CacheManager

`struct` · `datafusion_execution::cache::cache_manager::CacheManager` · datafusion-execution 55.1.0

```rust
struct CacheManager
```

Source: `src/cache/cache_manager.rs:307`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Manages various caches used in DataFusion.

Following DataFusion design principles, DataFusion provides default cache
implementations, while also allowing users to provide their own custom cache
implementations by implementing the relevant traits.

See [`CacheManagerConfig`](../operations/datafusion_execution.cache.cache_manager.CacheManagerConfig.md#op-f8c40b300dad7c3c1690af03) for configuration options.

<a id="op-e90109d2b44abca92b7aac13"></a>
## fmt

`function` · `datafusion_execution::cache::cache_manager::CacheManager::fmt` · datafusion-execution 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::cache_manager::CacheManager", "path": "CacheManager"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [306, 10], "end": [306, 15], "filename": "src/cache/cache_manager.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/cache/cache_manager.rs:306`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f6278ba3a4c5455caca385e4"></a>
## get_file_metadata_cache

`function` · `datafusion_execution::cache::cache_manager::CacheManager::get_file_metadata_cache` · datafusion-execution 55.1.0

```rust
fn get_file_metadata_cache(&self) -> Arc<FileMetadataCache>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::cache_manager::CacheManager", "path": "CacheManager"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [313, 1], "end": [410, 2], "filename": "src/cache/cache_manager.rs"}, "trait": null, "trait_path": null}`

Source: `src/cache/cache_manager.rs:402`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Get the file embedded metadata cache.

<a id="op-f46938a416ee76cd1ba5dd87"></a>
## get_file_statistic_cache

`function` · `datafusion_execution::cache::cache_manager::CacheManager::get_file_statistic_cache` · datafusion-execution 55.1.0

```rust
fn get_file_statistic_cache(&self) -> Option<Arc<FileStatisticsCache>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::cache_manager::CacheManager", "path": "CacheManager"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [313, 1], "end": [410, 2], "filename": "src/cache/cache_manager.rs"}, "trait": null, "trait_path": null}`

Source: `src/cache/cache_manager.rs:373`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Get the file statistics cache.

<a id="op-5da389d4aca4c97b4fd22c57"></a>
## get_file_statistic_cache_limit

`function` · `datafusion_execution::cache::cache_manager::CacheManager::get_file_statistic_cache_limit` · datafusion-execution 55.1.0

```rust
fn get_file_statistic_cache_limit(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::cache_manager::CacheManager", "path": "CacheManager"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [313, 1], "end": [410, 2], "filename": "src/cache/cache_manager.rs"}, "trait": null, "trait_path": null}`

Source: `src/cache/cache_manager.rs:378`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Get the memory limit of the file statistics cache.

<a id="op-c7bbba8140565b0556ee829d"></a>
## get_list_files_cache

`function` · `datafusion_execution::cache::cache_manager::CacheManager::get_list_files_cache` · datafusion-execution 55.1.0

```rust
fn get_list_files_cache(&self) -> Option<Arc<ListFilesCache>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::cache_manager::CacheManager", "path": "CacheManager"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [313, 1], "end": [410, 2], "filename": "src/cache/cache_manager.rs"}, "trait": null, "trait_path": null}`

Source: `src/cache/cache_manager.rs:385`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Get the cache for storing the result of listing [`ObjectMeta`](../operations/object_store.ObjectMeta.md#op-84641755fb7ee613fe92d518)s under the same path.

<a id="op-5292acb35bf2b033d6e0a228"></a>
## get_list_files_cache_limit

`function` · `datafusion_execution::cache::cache_manager::CacheManager::get_list_files_cache_limit` · datafusion-execution 55.1.0

```rust
fn get_list_files_cache_limit(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::cache_manager::CacheManager", "path": "CacheManager"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [313, 1], "end": [410, 2], "filename": "src/cache/cache_manager.rs"}, "trait": null, "trait_path": null}`

Source: `src/cache/cache_manager.rs:390`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Get the memory limit of the list files cache.

<a id="op-97cb83196ee194ecff320fc7"></a>
## get_list_files_cache_ttl

`function` · `datafusion_execution::cache::cache_manager::CacheManager::get_list_files_cache_ttl` · datafusion-execution 55.1.0

```rust
fn get_list_files_cache_ttl(&self) -> Option<Duration>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::cache_manager::CacheManager", "path": "CacheManager"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [313, 1], "end": [410, 2], "filename": "src/cache/cache_manager.rs"}, "trait": null, "trait_path": null}`

Source: `src/cache/cache_manager.rs:397`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Get the TTL (time-to-live) of the list files cache.

<a id="op-8f0badc561b8f5d3bbf704ee"></a>
## get_metadata_cache_limit

`function` · `datafusion_execution::cache::cache_manager::CacheManager::get_metadata_cache_limit` · datafusion-execution 55.1.0

```rust
fn get_metadata_cache_limit(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::cache_manager::CacheManager", "path": "CacheManager"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [313, 1], "end": [410, 2], "filename": "src/cache/cache_manager.rs"}, "trait": null, "trait_path": null}`

Source: `src/cache/cache_manager.rs:407`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Get the limit of the file embedded metadata cache.

<a id="op-c4ec709b44741de354d7df58"></a>
## try_new

`function` · `datafusion_execution::cache::cache_manager::CacheManager::try_new` · datafusion-execution 55.1.0

```rust
fn try_new(config: &CacheManagerConfig) -> Result<Arc<Self>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::cache_manager::CacheManager", "path": "CacheManager"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [313, 1], "end": [410, 2], "filename": "src/cache/cache_manager.rs"}, "trait": null, "trait_path": null}`

Source: `src/cache/cache_manager.rs:314`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
