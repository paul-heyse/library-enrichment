# `deltalake_lakefs`

Crate `deltalake-lakefs` · 2 public items · structured records in [`model/deltalake_lakefs.json`](../model/deltalake_lakefs.json)

## register_handlers

`function` · `deltalake_lakefs::register_handlers`
[Full member contracts, output types and access classification](../operations/deltalake_lakefs.register_handlers.md)

Also reachable as `deltalake::lakefs::register_handlers`

```rust
fn register_handlers(_additional_prefixes: Option<url::Url>)
```

Register an [ObjectStoreFactory] for common LakeFS [Url] schemes

---

## LakeFSLogStoreFactory

`struct` · `deltalake_lakefs::LakeFSLogStoreFactory`
[Full member contracts, output types and access classification](../operations/deltalake_lakefs.LakeFSLogStoreFactory.md)

Also reachable as `deltalake::lakefs::LakeFSLogStoreFactory`

```rust
struct LakeFSLogStoreFactory
```

**Implements**: `deltalake_core::logstore::factories::LogStoreFactory`, `deltalake_lakefs::storage::S3StorageOptionsConversion`

**Derives**: Clone, Debug, Default

**via `deltalake_core::logstore::factories::LogStoreFactory`**

```rust
fn with_options(&self, prefixed_store: ObjectStoreRef, root_store: ObjectStoreRef, location: &Url, config: &StorageConfig) -> DeltaResult<Arc<dyn LogStore>>
```

---
