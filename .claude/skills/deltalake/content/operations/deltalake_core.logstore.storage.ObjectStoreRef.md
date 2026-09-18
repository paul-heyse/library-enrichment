# `deltalake_core::logstore::storage::ObjectStoreRef`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.logstore.storage.ObjectStoreRef.json).

<a id="op-962a74119c8af47bb70e3501"></a>
## ObjectStoreRef

`type_alias` · `deltalake_core::logstore::storage::ObjectStoreRef` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type ObjectStoreRef = std::sync::Arc<object_store::DynObjectStore>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/mod.rs#L24).

Source: `crates/core/src/logstore/storage/mod.rs:24`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Sharable reference to [`ObjectStore`]

Unresolved upstream links (retained, not inferred): ``ObjectStore``.
