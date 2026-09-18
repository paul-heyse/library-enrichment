# `datafusion_execution::cache::cache_manager::FileStatisticsCache`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.cache.cache_manager.FileStatisticsCache.json).

<a id="op-5a9ca2374f0f0f017e5506c9"></a>
## FileStatisticsCache

`type_alias` · `datafusion_execution::cache::cache_manager::FileStatisticsCache` · datafusion-execution 55.1.0

```rust
type FileStatisticsCache = dyn Cache<TableScopedPath, CachedFileMetadata>
```

Source: `src/cache/cache_manager.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

A cache for file statistics and orderings.

This cache stores [`CachedFileMetadata`](../operations/datafusion_execution.cache.cache_manager.CachedFileMetadata.md#op-735ab08141f86091bef3968c) which includes:
- File metadata for validation (size, last_modified)
- Statistics for the file
- Ordering information for the file

If enabled via [`CacheManagerConfig::with_file_statistics_cache`](../operations/datafusion_execution.cache.cache_manager.CacheManagerConfig.md#op-f9c89e6515c16ed74e3e83b1) this
cache avoids inferring the same file statistics repeatedly during the
session lifetime.

The typical usage pattern is:
1. Call `get(path)` to check for cached value
2. If `Some(cached)`, validate with
   `cached.is_valid_for(&current_meta, &current_schema_fingerprint)`
3. If invalid or missing, compute new value and call `put(path, new_value)`

See [`crate::runtime_env::RuntimeEnv`](../operations/datafusion_execution.runtime_env.RuntimeEnv.md#op-c598f4df4cf51824ae4b9a67) for more details
