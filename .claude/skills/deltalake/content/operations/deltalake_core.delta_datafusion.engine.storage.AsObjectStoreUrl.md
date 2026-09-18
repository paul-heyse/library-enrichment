# `deltalake_core::delta_datafusion::engine::storage::AsObjectStoreUrl`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.delta_datafusion.engine.storage.AsObjectStoreUrl.json).

<a id="op-274c0528c01e45e917ea02b5"></a>
## AsObjectStoreUrl

`trait` · `deltalake_core::delta_datafusion::engine::storage::AsObjectStoreUrl` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait AsObjectStoreUrl
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/engine/storage.rs#L118).

Source: `crates/core/src/delta_datafusion/engine/storage.rs:118`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Conversion to a DataFusion [`ObjectStoreUrl`], the key used to register and look up an
object store in a session's runtime environment.

Unresolved upstream links (retained, not inferred): ``ObjectStoreUrl``.

<a id="op-cebb0c96fd8fa17a13a72713"></a>
## as_object_store_url

`function` · `deltalake_core::delta_datafusion::engine::storage::AsObjectStoreUrl::as_object_store_url` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn as_object_store_url(&self) -> ObjectStoreUrl
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/engine/storage.rs#L120).

Source: `crates/core/src/delta_datafusion/engine/storage.rs:120`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Return the [`ObjectStoreUrl`] that identifies the object store backing `self`.

Unresolved upstream links (retained, not inferred): ``ObjectStoreUrl``.
