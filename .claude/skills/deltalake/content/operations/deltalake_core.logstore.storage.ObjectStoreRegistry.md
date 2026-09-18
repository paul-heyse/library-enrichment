# `deltalake_core::logstore::storage::ObjectStoreRegistry`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.logstore.storage.ObjectStoreRegistry.json).

<a id="op-1f62daa036850d73d69ee8b0"></a>
## ObjectStoreRegistry

`trait` · `deltalake_core::logstore::storage::ObjectStoreRegistry` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait ObjectStoreRegistry: Send + Sync + std::fmt::Debug + 'static
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/mod.rs#L28).

Source: `crates/core/src/logstore/storage/mod.rs:28`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A registry mapping URLs to [`ObjectStore`] instances, supporting registration and lookup
(with optional lazy, ad-hoc discovery on miss).

Unresolved upstream links (retained, not inferred): ``ObjectStore``.

<a id="op-7c680c2519206ab6fbc161aa"></a>
## get_store

`function` · `deltalake_core::logstore::storage::ObjectStoreRegistry::get_store` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_store(&self, url: &Url) -> DeltaResult<Arc<dyn ObjectStore>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/mod.rs#L40).

Source: `crates/core/src/logstore/storage/mod.rs:40`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Get a suitable store for the provided URL. For example:
If no [`ObjectStore`] found for the `url`, ad-hoc discovery may be executed depending on
the `url` and [`ObjectStoreRegistry`](../operations/deltalake_core.logstore.storage.ObjectStoreRegistry.md#op-1f62daa036850d73d69ee8b0) implementation. An [`ObjectStore`] may be lazily
created and registered.

Unresolved upstream links (retained, not inferred): ``ObjectStore``.

<a id="op-0babc695868b0ef36fe18d64"></a>
## register_store

`function` · `deltalake_core::logstore::storage::ObjectStoreRegistry::register_store` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn register_store(&self, url: &Url, store: Arc<dyn ObjectStore>) -> Option<Arc<dyn ObjectStore>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/mod.rs#L30).

Source: `crates/core/src/logstore/storage/mod.rs:30`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

If a store with the same key existed before, it is replaced and returned
