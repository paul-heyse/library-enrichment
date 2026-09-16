# ObjectStoreRegistry

`deltalake_core::logstore::storage::ObjectStoreRegistry`

```rust
trait ObjectStoreRegistry: Send + Sync + std::fmt::Debug + 'static
```

Also reachable as `deltalake::logstore::ObjectStoreRegistry`, `deltalake_core::logstore::ObjectStoreRegistry`

Prose: [`api/deltalake_core.logstore.storage.md`](../api/deltalake_core.logstore.storage.md#objectstoreregistry) · records: [`model/deltalake_core.logstore.storage.json`](../model/deltalake_core.logstore.storage.json)

## Required

Every implementation must supply these.

```rust
fn get_store(&self, url: &Url) -> DeltaResult<Arc<dyn ObjectStore>>
fn register_store(&self, url: &Url, store: Arc<dyn ObjectStore>) -> Option<Arc<dyn ObjectStore>>
```

## Implementors (1)

Read one before writing your own.

- `deltalake_core::logstore::storage::DefaultObjectStoreRegistry`

## Documentation

A registry mapping URLs to [`ObjectStore`] instances, supporting registration and lookup
(with optional lazy, ad-hoc discovery on miss).
