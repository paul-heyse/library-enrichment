# `datafusion_execution::cache::cache_manager::ListFilesCache`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.cache.cache_manager.ListFilesCache.json).

<a id="op-f2e8eee145806d67bc877dba"></a>
## ListFilesCache

`type_alias` · `datafusion_execution::cache::cache_manager::ListFilesCache` · datafusion-execution 55.1.0

```rust
type ListFilesCache = dyn Cache<TableScopedPath, CachedFileList>
```

Source: `src/cache/cache_manager.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

A cache for storing the [`ObjectMeta`](../operations/object_store.ObjectMeta.md#op-84641755fb7ee613fe92d518)s that result from listing a path.

Listing a path means doing an object store "list" operation or `ls`
command on the local filesystem. This operation can be expensive,
especially when done over remote object stores.

The cache key is always the table's base path, ensuring a stable cache key.
The cached value is a [`CachedFileList`](../operations/datafusion_execution.cache.cache_manager.CachedFileList.md#op-b8a81d2ea198f9c7a9cb8287) containing the files and a timestamp.

Partition filtering is done after retrieval using [`CachedFileList::files_matching_prefix`](../operations/datafusion_execution.cache.cache_manager.CachedFileList.md#op-343007d57f6ec89fd9bbacf9).

See [`crate::runtime_env::RuntimeEnv`](../operations/datafusion_execution.runtime_env.RuntimeEnv.md#op-c598f4df4cf51824ae4b9a67) for more details.
