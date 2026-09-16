# `deltalake_lakefs::logstore`

Crate `deltalake-lakefs` · 1 public items · structured records in [`model/deltalake_lakefs.logstore.json`](../model/deltalake_lakefs.logstore.json)

## lakefs_logstore

`function` · `deltalake_lakefs::logstore::lakefs_logstore`

```rust
fn lakefs_logstore(store: deltalake_core::logstore::ObjectStoreRef, root_store: deltalake_core::logstore::ObjectStoreRef, location: &url::Url, options: &StorageConfig) -> deltalake_core::DeltaResult<std::sync::Arc<dyn LogStore>>
```

Return the [LakeFSLogStore] implementation with the provided configuration options

---
