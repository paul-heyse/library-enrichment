# `deltalake_core::logstore::factories::ObjectStoreFactory`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.logstore.factories.ObjectStoreFactory.json).

<a id="op-ecc00d1a52536f5b5d865cff"></a>
## ObjectStoreFactory

`trait` · `deltalake_core::logstore::factories::ObjectStoreFactory` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait ObjectStoreFactory: Send + Sync
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/factories.rs#L17).

Source: `crates/core/src/logstore/factories.rs:17`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Factory trait for creating [`ObjectStore`](::object_store::ObjectStore) instances at runtime

Unresolved upstream links (retained, not inferred): `::object_store::ObjectStore`.

<a id="op-a04dc2f004a3fc9ca15af0c9"></a>
## parse_url_opts

`function` · `deltalake_core::logstore::factories::ObjectStoreFactory::parse_url_opts` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn parse_url_opts(&self, url: &Url, config: &StorageConfig) -> DeltaResult<(ObjectStoreRef, Path)>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/factories.rs#L26).

Source: `crates/core/src/logstore/factories.rs:26`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Parse URL options and create an object store instance.

The object store instance returned by this method must point at the root of the storage location.
Root in this case means scheme, authority/host and maybe port.
The path segment is returned as second element of the tuple. It must point at the path
corresponding to the path segment of the URL.

The store should __NOT__ apply the decorations via the passed `options`
