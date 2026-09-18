# `deltalake_lakefs::logstore::lakefs_logstore`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_lakefs.logstore.lakefs_logstore.json).

<a id="op-a6ee94abd8e92c37e69cd574"></a>
## lakefs_logstore

`function` · `deltalake_lakefs::logstore::lakefs_logstore` · deltalake-lakefs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn lakefs_logstore(store: deltalake_core::logstore::ObjectStoreRef, root_store: deltalake_core::logstore::ObjectStoreRef, location: &url::Url, options: &StorageConfig) -> deltalake_core::DeltaResult<std::sync::Arc<dyn LogStore>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/logstore.rs#L23).

Source: `crates/lakefs/src/logstore.rs:23`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Return the [LakeFSLogStore] implementation with the provided configuration options

Unresolved upstream links (retained, not inferred): `LakeFSLogStore`.
