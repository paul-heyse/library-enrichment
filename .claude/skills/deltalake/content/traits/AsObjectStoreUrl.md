# AsObjectStoreUrl

`deltalake_core::delta_datafusion::engine::storage::AsObjectStoreUrl`

```rust
trait AsObjectStoreUrl
```

Also reachable as `deltalake::delta_datafusion::engine::AsObjectStoreUrl`, `deltalake_core::delta_datafusion::engine::AsObjectStoreUrl`

Prose: [`api/deltalake_core.delta_datafusion.engine.storage.md`](../api/deltalake_core.delta_datafusion.engine.storage.md#asobjectstoreurl) · records: [`model/deltalake_core.delta_datafusion.engine.storage.json`](../model/deltalake_core.delta_datafusion.engine.storage.json)

## Required

Every implementation must supply these.

```rust
fn as_object_store_url(&self) -> ObjectStoreUrl
```

## Implementors (3)

Read one before writing your own.

- `buoyant_kernel::FileMeta`
- `buoyant_kernel::FileSlice`
- `url::Url`

## Documentation

Conversion to a DataFusion [`ObjectStoreUrl`], the key used to register and look up an
object store in a session's runtime environment.
