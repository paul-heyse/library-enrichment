# `deltalake_core::logstore::get_latest_version`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.logstore.get_latest_version.json).

<a id="op-a666b463115f2c7886e3ef2c"></a>
## get_latest_version

`function` · `deltalake_core::logstore::get_latest_version` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn get_latest_version(log_store: &dyn LogStore, current_version: kernel::Version) -> DeltaResult<kernel::Version>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/mod.rs#L785).

Source: `crates/core/src/logstore/mod.rs:785`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Default implementation for retrieving the latest version
