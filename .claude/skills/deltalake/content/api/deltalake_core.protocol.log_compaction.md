# `deltalake_core::protocol::log_compaction`

Crate `deltalake-core` · 1 public items · structured records in [`model/deltalake_core.protocol.log_compaction.json`](../model/deltalake_core.protocol.log_compaction.json)

## compact_logs

`function` · `deltalake_core::protocol::log_compaction::compact_logs`

Also reachable as `deltalake::protocol::log_compaction::compact_logs`

```rust
async fn compact_logs(table: &DeltaTable, start_version: u64, end_version: u64, operation_id: Option<uuid::Uuid>) -> DeltaResult<()>
```

Creates a log compaction file for a specified version range

---
