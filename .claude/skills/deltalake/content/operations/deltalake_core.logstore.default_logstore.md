# `deltalake_core::logstore::default_logstore`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.logstore.default_logstore.json).

<a id="op-a77c74e375f72aee2b458e73"></a>
## default_logstore

`function` · `deltalake_core::logstore::default_logstore` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default_logstore(prefixed_store: ObjectStoreRef, root_store: ObjectStoreRef, location: &url::Url, options: &StorageConfig) -> std::sync::Arc<dyn LogStore>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/mod.rs#L147).

Source: `crates/core/src/logstore/mod.rs:147`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Return the [DefaultLogStore] implementation with the provided configuration options
