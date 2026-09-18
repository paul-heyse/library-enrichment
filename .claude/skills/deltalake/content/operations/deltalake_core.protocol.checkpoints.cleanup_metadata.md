# `deltalake_core::protocol::checkpoints::cleanup_metadata`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.protocol.checkpoints.cleanup_metadata.json).

<a id="op-ae4846259bd3b5dfbb8aa11d"></a>
## cleanup_metadata

`function` · `deltalake_core::protocol::checkpoints::cleanup_metadata` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn cleanup_metadata(table: &DeltaTable, operation_id: Option<uuid::Uuid>) -> DeltaResult<usize>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/checkpoints.rs#L56).

Source: `crates/core/src/protocol/checkpoints.rs:56`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Delete expires log files before given version from table. The table log retention is based on
the `logRetentionDuration` property of the Delta Table, 30 days by default.
