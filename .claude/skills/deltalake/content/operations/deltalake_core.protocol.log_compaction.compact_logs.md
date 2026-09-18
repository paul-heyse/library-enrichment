# `deltalake_core::protocol::log_compaction::compact_logs`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.protocol.log_compaction.compact_logs.json).

<a id="op-61d3677df5d7bdb69b950905"></a>
## compact_logs

`function` · `deltalake_core::protocol::log_compaction::compact_logs` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn compact_logs(table: &DeltaTable, start_version: u64, end_version: u64, operation_id: Option<uuid::Uuid>) -> DeltaResult<()>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/log_compaction.rs#L92).

Source: `crates/core/src/protocol/log_compaction.rs:92`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Creates a log compaction file for a specified version range
