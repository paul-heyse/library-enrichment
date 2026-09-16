# `deltalake_gcp`

Crate `deltalake-gcp` · 2 public items · structured records in [`model/deltalake_gcp.json`](../model/deltalake_gcp.json)

## register_handlers

`function` · `deltalake_gcp::register_handlers`

Also reachable as `deltalake::gcp::register_handlers`

```rust
fn register_handlers(_additional_prefixes: Option<url::Url>)
```

Register an [ObjectStoreFactory] for common Google Cloud [Url] schemes

---

## GcpFactory

`struct` · `deltalake_gcp::GcpFactory`

Also reachable as `deltalake::gcp::GcpFactory`

```rust
struct GcpFactory
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
