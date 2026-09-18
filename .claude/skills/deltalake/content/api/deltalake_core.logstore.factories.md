# `deltalake_core::logstore::factories`

Crate `deltalake-core` · 7 public items · structured records in [`model/deltalake_core.logstore.factories.json`](../model/deltalake_core.logstore.factories.json)

## logstore_factories

`function` · `deltalake_core::logstore::factories::logstore_factories`
[Full member contracts, output types and access classification](../operations/deltalake_core.logstore.factories.logstore_factories.md)

Also reachable as `deltalake::logstore::logstore_factories`, `deltalake_core::logstore::logstore_factories`

```rust
fn logstore_factories() -> LogStoreFactoryRegistry
```

Access global registry of logstore factories.

---

## object_store_factories

`function` · `deltalake_core::logstore::factories::object_store_factories`
[Full member contracts, output types and access classification](../operations/deltalake_core.logstore.factories.object_store_factories.md)

Also reachable as `deltalake::logstore::object_store_factories`, `deltalake_core::logstore::object_store_factories`

```rust
fn object_store_factories() -> ObjectStoreFactoryRegistry
```

Access global registry of object store factories

---

## store_for

`function` · `deltalake_core::logstore::factories::store_for`
[Full member contracts, output types and access classification](../operations/deltalake_core.logstore.factories.store_for.md)

Also reachable as `deltalake::logstore::store_for`, `deltalake_core::logstore::store_for`

```rust
fn store_for<K, V, I>(url: &url::Url, options: I) -> DeltaResult<super::ObjectStoreRef> where I: IntoIterator<Item = (K, V)>, K: AsRef<str> + Into<String>, V: AsRef<str> + Into<String>
```

Simpler access pattern for the [ObjectStoreFactoryRegistry] to get a single store

---

## LogStoreFactory

`trait` · `deltalake_core::logstore::factories::LogStoreFactory`
[Full member contracts, output types and access classification](../operations/deltalake_core.logstore.factories.LogStoreFactory.md)

Also reachable as `deltalake::logstore::LogStoreFactory`, `deltalake_core::logstore::LogStoreFactory`

```rust
trait LogStoreFactory: Send + Sync
```

**Implementors** (9)

- `deltalake_aws::S3LogStoreFactory`
- `deltalake_azure::AzureFactory`
- `deltalake_catalog_unity::UnityCatalogFactory`
- `deltalake_core::logstore::factories::DefaultLogStoreFactory`
- `deltalake_gcp::GcpFactory`
- `deltalake_hdfs::HdfsFactory`
- `deltalake_lakefs::LakeFSLogStoreFactory`
- `deltalake_mount::MountFactory`
- `deltalake_opendal::factory::OpendalLogStoreFactory`

**Methods** (1)

```rust
fn with_options(&self, prefixed_store: ObjectStoreRef, root_store: ObjectStoreRef, location: &Url, options: &StorageConfig) -> DeltaResult<Arc<dyn LogStore>>
```

Trait for generating [LogStore] implementations

---

## ObjectStoreFactory

`trait` · `deltalake_core::logstore::factories::ObjectStoreFactory`
[Full member contracts, output types and access classification](../operations/deltalake_core.logstore.factories.ObjectStoreFactory.md)

Also reachable as `deltalake::logstore::ObjectStoreFactory`, `deltalake_core::logstore::ObjectStoreFactory`

```rust
trait ObjectStoreFactory: Send + Sync
```

**Implementors** (9)

- `deltalake_aws::storage::S3ObjectStoreFactory`
- `deltalake_azure::AzureFactory`
- `deltalake_catalog_unity::UnityCatalogFactory`
- `deltalake_core::logstore::factories::DefaultObjectStoreFactory`
- `deltalake_gcp::GcpFactory`
- `deltalake_hdfs::HdfsFactory`
- `deltalake_lakefs::storage::LakeFSObjectStoreFactory`
- `deltalake_mount::MountFactory`
- `deltalake_opendal::factory::OpendalObjectStoreFactory`

**Methods** (1)

```rust
fn parse_url_opts(&self, url: &Url, config: &StorageConfig) -> DeltaResult<(ObjectStoreRef, Path)>
```

Factory trait for creating [`ObjectStore`](::object_store::ObjectStore) instances at runtime

---

## LogStoreFactoryRegistry

`type_alias` · `deltalake_core::logstore::factories::LogStoreFactoryRegistry`
[Full member contracts, output types and access classification](../operations/deltalake_core.logstore.factories.LogStoreFactoryRegistry.md)

Also reachable as `deltalake::logstore::LogStoreFactoryRegistry`, `deltalake_core::logstore::LogStoreFactoryRegistry`

```rust
type LogStoreFactoryRegistry = std::sync::Arc<dashmap::DashMap<url::Url, std::sync::Arc<dyn LogStoreFactory>>>
```

Registry of [`LogStoreFactory`] instances

---

## ObjectStoreFactoryRegistry

`type_alias` · `deltalake_core::logstore::factories::ObjectStoreFactoryRegistry`
[Full member contracts, output types and access classification](../operations/deltalake_core.logstore.factories.ObjectStoreFactoryRegistry.md)

Also reachable as `deltalake::logstore::ObjectStoreFactoryRegistry`, `deltalake_core::logstore::ObjectStoreFactoryRegistry`

```rust
type ObjectStoreFactoryRegistry = std::sync::Arc<dashmap::DashMap<url::Url, std::sync::Arc<dyn ObjectStoreFactory>>>
```

Factory registry to manage [`ObjectStoreFactory`] instances

---
