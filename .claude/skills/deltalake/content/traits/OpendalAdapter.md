# OpendalAdapter

`deltalake_opendal::adapter::OpendalAdapter`

```rust
trait OpendalAdapter: Send + Sync + std::fmt::Debug
```

Also reachable as `deltalake::opendal::OpendalAdapter`, `deltalake_opendal::OpendalAdapter`

Prose: [`api/deltalake_opendal.adapter.md`](../api/deltalake_opendal.adapter.md#opendaladapter) · records: [`model/deltalake_opendal.adapter.json`](../model/deltalake_opendal.adapter.json)

## Required

Every implementation must supply these.

```rust
fn resolve(&self, url: &Url, config: &StorageConfig) -> DeltaResult<OperatorSpec>
```

## Provided

Defaulted, and this is where the capability hides. The default is the conservative answer -- no pushdown, no statistics, no specialization -- so an implementation that overrides none of these works correctly and performs badly.

```rust
fn logstore_prefix(&self, spec: &OperatorSpec) -> Path
fn wrap_store(&self, store: ObjectStoreRef, _spec: &OperatorSpec) -> ObjectStoreRef
```

## Implementors (1)

Read one before writing your own.

- `deltalake_opendal::adapter::GenericAdapter`

## Documentation

Per-service specialization for the generic OpenDAL factories.

Only three things vary between OpenDAL services: how a delta URL plus storage
options map onto an [`OperatorSpec`], whether the resulting store needs
wrapping, and what prefix the log store should use. Everything else is shared
by [`crate::factory`].
