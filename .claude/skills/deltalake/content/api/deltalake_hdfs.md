# `deltalake_hdfs`

Crate `deltalake-hdfs` · 2 public items · structured records in [`model/deltalake_hdfs.json`](../model/deltalake_hdfs.json)

## register_handlers

`function` · `deltalake_hdfs::register_handlers`

Also reachable as `deltalake::hdfs::register_handlers`

```rust
fn register_handlers(_additional_prefixes: Option<url::Url>)
```

Register an [ObjectStoreFactory] for common HDFS [Url] schemes

---

## HdfsFactory

`struct` · `deltalake_hdfs::HdfsFactory`

Also reachable as `deltalake::hdfs::HdfsFactory`

```rust
struct HdfsFactory
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
