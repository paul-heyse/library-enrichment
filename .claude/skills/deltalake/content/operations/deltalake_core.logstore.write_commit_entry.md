# `deltalake_core::logstore::write_commit_entry`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.logstore.write_commit_entry.json).

<a id="op-fcaab2ef40ec9b940d25518c"></a>
## write_commit_entry

`function` · `deltalake_core::logstore::write_commit_entry` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn write_commit_entry(storage: &dyn ObjectStore, version: kernel::Version, tmp_commit: &object_store::path::Path) -> Result<(), kernel::transaction::TransactionError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/mod.rs#L892).

Source: `crates/core/src/logstore/mod.rs:892`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Default implementation for writing a commit entry
