# `datafusion_execution::cache::cache_manager::FileMetadataCache`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.cache.cache_manager.FileMetadataCache.json).

<a id="op-eae1fe1b1bd732ee9949f01d"></a>
## FileMetadataCache

`type_alias` · `datafusion_execution::cache::cache_manager::FileMetadataCache` · datafusion-execution 55.1.0

```rust
type FileMetadataCache = dyn Cache<object_store::path::Path, CachedFileMetadataEntry>
```

Source: `src/cache/cache_manager.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

A cache for storing file-embedded metadata.

This cache stores per-file metadata in the form of [`CachedFileMetadataEntry`](../operations/datafusion_execution.cache.cache_manager.CachedFileMetadataEntry.md#op-5d7fdb873e28f330be5a251a),
which includes the [`ObjectMeta`](../operations/object_store.ObjectMeta.md#op-84641755fb7ee613fe92d518) for validation.

For example, the built in [`ListingTable`] uses this cache to avoid parsing
Parquet footers multiple times for the same file.

The typical usage pattern is:
1. Call `get(path)` to check for cached value
2. If `Some(cached)`, validate with `cached.is_valid_for(&current_meta)`
3. If invalid or missing, compute new value and call `put(path, new_value)`

See [`crate::runtime_env::RuntimeEnv`](../operations/datafusion_execution.runtime_env.RuntimeEnv.md#op-c598f4df4cf51824ae4b9a67) for more details.

[`ListingTable`]: https://docs.rs/datafusion/latest/datafusion/datasource/listing/struct.ListingTable.html
