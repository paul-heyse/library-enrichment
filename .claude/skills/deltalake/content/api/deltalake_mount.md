# `deltalake_mount`

Crate `deltalake-mount` · 2 public items · structured records in [`model/deltalake_mount.json`](../model/deltalake_mount.json)

## register_handlers

`function` · `deltalake_mount::register_handlers`

```rust
fn register_handlers(_additional_prefixes: Option<url::Url>)
```

Register an [ObjectStoreFactory] for common Mount [Url] schemes

---

## MountFactory

`struct` · `deltalake_mount::MountFactory`

```rust
struct MountFactory
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
