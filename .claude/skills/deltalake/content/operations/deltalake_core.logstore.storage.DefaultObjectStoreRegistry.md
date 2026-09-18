# `deltalake_core::logstore::storage::DefaultObjectStoreRegistry`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.logstore.storage.DefaultObjectStoreRegistry.json).

<a id="op-d733cb7adf26b0daffbf319b"></a>
## DefaultObjectStoreRegistry

`struct` · `deltalake_core::logstore::storage::DefaultObjectStoreRegistry` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct DefaultObjectStoreRegistry
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/mod.rs#L45).

Source: `crates/core/src/logstore/storage/mod.rs:45`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The default [`ObjectStoreRegistry`](../operations/deltalake_core.logstore.storage.ObjectStoreRegistry.md#op-1f62daa036850d73d69ee8b0)

<a id="op-0aed164fea0b74ad9c35a7d6"></a>
## clone

`function` · `deltalake_core::logstore::storage::DefaultObjectStoreRegistry::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> DefaultObjectStoreRegistry
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/mod.rs#L44).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::storage::DefaultObjectStoreRegistry", "path": "DefaultObjectStoreRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 10], "end": [44, 15], "filename": "crates/core/src/logstore/storage/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/logstore/storage/mod.rs:44`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-71139634cf4b6c5a70be3fbb"></a>
## default

`function` · `deltalake_core::logstore::storage::DefaultObjectStoreRegistry::default` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/mod.rs#L51).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::storage::DefaultObjectStoreRegistry", "path": "DefaultObjectStoreRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [54, 2], "filename": "crates/core/src/logstore/storage/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/core/src/logstore/storage/mod.rs:51`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2c050fbe736ba9b261e8d708"></a>
## fmt

`function` · `deltalake_core::logstore::storage::DefaultObjectStoreRegistry::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/mod.rs#L65).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::storage::DefaultObjectStoreRegistry", "path": "DefaultObjectStoreRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [77, 2], "filename": "crates/core/src/logstore/storage/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/logstore/storage/mod.rs:65`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d271403b3edb9788629ea87a"></a>
## get_store

`function` · `deltalake_core::logstore::storage::DefaultObjectStoreRegistry::get_store` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_store(&self, url: &Url) -> DeltaResult<Arc<dyn ObjectStore>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/mod.rs#L88).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::storage::DefaultObjectStoreRegistry", "path": "DefaultObjectStoreRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [96, 2], "filename": "crates/core/src/logstore/storage/mod.rs"}, "trait": {"args": null, "id": "deltalake_core::logstore::storage::ObjectStoreRegistry", "path": "ObjectStoreRegistry"}, "trait_path": "deltalake_core::logstore::storage::ObjectStoreRegistry"}`

Source: `crates/core/src/logstore/storage/mod.rs:88`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4cc4f2896a15cb843e7fcbed"></a>
## new

`function` · `deltalake_core::logstore::storage::DefaultObjectStoreRegistry::new` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new() -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/mod.rs#L58).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::storage::DefaultObjectStoreRegistry", "path": "DefaultObjectStoreRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [62, 2], "filename": "crates/core/src/logstore/storage/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/logstore/storage/mod.rs:58`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create an empty registry with no stores registered.

<a id="op-01e6ffbd8cc621aca60c8059"></a>
## register_store

`function` · `deltalake_core::logstore::storage::DefaultObjectStoreRegistry::register_store` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn register_store(&self, url: &Url, store: Arc<dyn ObjectStore>) -> Option<Arc<dyn ObjectStore>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/mod.rs#L80).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::storage::DefaultObjectStoreRegistry", "path": "DefaultObjectStoreRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [96, 2], "filename": "crates/core/src/logstore/storage/mod.rs"}, "trait": {"args": null, "id": "deltalake_core::logstore::storage::ObjectStoreRegistry", "path": "ObjectStoreRegistry"}, "trait_path": "deltalake_core::logstore::storage::ObjectStoreRegistry"}`

Source: `crates/core/src/logstore/storage/mod.rs:80`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bc05f66ddece1a8942ccf0ec"></a>
## object_stores

`struct_field` · `deltalake_core::logstore::storage::DefaultObjectStoreRegistry::object_stores` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
object_stores: dashmap::DashMap<url::Url, std::sync::Arc<dyn ObjectStore>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/mod.rs#L47).

Source: `crates/core/src/logstore/storage/mod.rs:47`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A map from scheme to object store that serve list / read operations for the store
