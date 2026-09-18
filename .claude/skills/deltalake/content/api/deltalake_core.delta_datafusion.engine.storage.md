# `deltalake_core::delta_datafusion::engine::storage`

Crate `deltalake-core` · 1 public items · structured records in [`model/deltalake_core.delta_datafusion.engine.storage.json`](../model/deltalake_core.delta_datafusion.engine.storage.json)

## AsObjectStoreUrl

`trait` · `deltalake_core::delta_datafusion::engine::storage::AsObjectStoreUrl`
[Full member contracts, output types and access classification](../operations/deltalake_core.delta_datafusion.engine.storage.AsObjectStoreUrl.md)

Also reachable as `deltalake::delta_datafusion::engine::AsObjectStoreUrl`, `deltalake_core::delta_datafusion::engine::AsObjectStoreUrl`

```rust
trait AsObjectStoreUrl
```

**Implementors** (3)

- `buoyant_kernel::FileMeta`
- `buoyant_kernel::FileSlice`
- `url::Url`

**Methods** (1)

```rust
fn as_object_store_url(&self) -> ObjectStoreUrl
```

Conversion to a DataFusion [`ObjectStoreUrl`], the key used to register and look up an
object store in a session's runtime environment.

---
