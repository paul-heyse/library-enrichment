# `deltalake_azure`

Crate `deltalake-azure` · 2 public items · structured records in [`model/deltalake_azure.json`](../model/deltalake_azure.json)

## register_handlers

`function` · `deltalake_azure::register_handlers`

Also reachable as `deltalake::azure::register_handlers`

```rust
fn register_handlers(_additional_prefixes: Option<url::Url>)
```

Register an [ObjectStoreFactory] for common Azure [Url] schemes

---

## AzureFactory

`struct` · `deltalake_azure::AzureFactory`

Also reachable as `deltalake::azure::AzureFactory`

```rust
struct AzureFactory
```

**Implements**: `deltalake_core::logstore::factories::LogStoreFactory`, `deltalake_core::logstore::factories::ObjectStoreFactory`

**Derives**: Clone, Debug, Default

**via `deltalake_core::logstore::factories::LogStoreFactory`**

```rust
fn with_options(&self, prefixed_store: ObjectStoreRef, root_store: ObjectStoreRef, location: &Url, options: &StorageConfig) -> DeltaResult<Arc<dyn LogStore>>
```

**via `deltalake_core::logstore::factories::ObjectStoreFactory`**

```rust
fn parse_url_opts(&self, url: &Url, config: &StorageConfig) -> DeltaResult<(ObjectStoreRef, Path)>
```

---
