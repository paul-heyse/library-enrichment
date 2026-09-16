# `deltalake_opendal::factory`

Crate `deltalake-opendal` · 2 public items · structured records in [`model/deltalake_opendal.factory.json`](../model/deltalake_opendal.factory.json)

## OpendalLogStoreFactory

`struct` · `deltalake_opendal::factory::OpendalLogStoreFactory`

Also reachable as `deltalake::opendal::OpendalLogStoreFactory`, `deltalake_opendal::OpendalLogStoreFactory`

```rust
struct OpendalLogStoreFactory<A: OpendalAdapter>
```

**Implements**: `deltalake_core::logstore::factories::LogStoreFactory`

**Derives**: Debug

**via `deltalake_core::logstore::factories::LogStoreFactory`**

```rust
fn with_options(&self, _prefixed_store: ObjectStoreRef, root_store: ObjectStoreRef, location: &Url, options: &StorageConfig) -> DeltaResult<Arc<dyn LogStore>>
```

[`LogStoreFactory`] that pairs with [`OpendalObjectStoreFactory`].

The PrefixStore is always recomputed from the adapter's
[`OpendalAdapter::logstore_prefix`] rather than trusting the `prefixed_store`
passed in by delta's `decorate_store` (which derives its prefix from
`url.path()`). For bucket-root services the two agree; for services whose
operator is scoped deeper than the bucket root, the adapter is authoritative.

---

## OpendalObjectStoreFactory

`struct` · `deltalake_opendal::factory::OpendalObjectStoreFactory`

Also reachable as `deltalake::opendal::OpendalObjectStoreFactory`, `deltalake_opendal::OpendalObjectStoreFactory`

```rust
struct OpendalObjectStoreFactory<A: OpendalAdapter>
```

**Implements**: `deltalake_core::logstore::factories::ObjectStoreFactory`

**Derives**: Debug

**via `deltalake_core::logstore::factories::ObjectStoreFactory`**

```rust
fn parse_url_opts(&self, url: &Url, config: &StorageConfig) -> DeltaResult<(ObjectStoreRef, Path)>
```

[`ObjectStoreFactory`] that builds an OpenDAL operator via an [`OpendalAdapter`].

---
