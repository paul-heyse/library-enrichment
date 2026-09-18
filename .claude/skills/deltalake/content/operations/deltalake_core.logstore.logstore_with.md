# `deltalake_core::logstore::logstore_with`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.logstore.logstore_with.json).

<a id="op-02dc337d46777aca7b33b21b"></a>
## logstore_with

`function` · `deltalake_core::logstore::logstore_with` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn logstore_with(root_store: ObjectStoreRef, location: &url::Url, storage_config: StorageConfig) -> DeltaResult<LogStoreRef>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/mod.rs#L291).

Source: `crates/core/src/logstore/mod.rs:291`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Return the [LogStoreRef](../operations/deltalake_core.logstore.LogStoreRef.md#op-1b56118828480c7f5e758964) using the given [ObjectStoreRef](../operations/deltalake_core.logstore.storage.ObjectStoreRef.md#op-962a74119c8af47bb70e3501)
