# `deltalake_core::logstore::read_commit_entry`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.logstore.read_commit_entry.json).

<a id="op-bd7094861ea00b68abd5bc73"></a>
## read_commit_entry

`function` · `deltalake_core::logstore::read_commit_entry` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn read_commit_entry(storage: &dyn ObjectStore, version: kernel::Version) -> DeltaResult<Option<bytes::Bytes>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/mod.rs#L868).

Source: `crates/core/src/logstore/mod.rs:868`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Read delta log for a specific version
