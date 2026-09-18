# `deltalake_core::protocol::checkpoints`

Crate `deltalake-core` · 4 public items · structured records in [`model/deltalake_core.protocol.checkpoints.json`](../model/deltalake_core.protocol.checkpoints.json)

## cleanup_expired_logs_for

`function` · `deltalake_core::protocol::checkpoints::cleanup_expired_logs_for`
[Full member contracts, output types and access classification](../operations/deltalake_core.protocol.checkpoints.cleanup_expired_logs_for.md)

Also reachable as `deltalake::checkpoints::cleanup_expired_logs_for`, `deltalake::protocol::checkpoints::cleanup_expired_logs_for`, `deltalake_core::checkpoints::cleanup_expired_logs_for`

```rust
async fn cleanup_expired_logs_for(keep_version: kernel::Version, log_store: &dyn LogStore, cutoff_timestamp: i64, operation_id: Option<uuid::Uuid>) -> DeltaResult<usize>
```

Delete expired Delta log files up to a safe checkpoint boundary.

This routine removes JSON commit files, in-progress JSON temp files, and
checkpoint files under `_delta_log/` that are both:
- older than the provided `cutoff_timestamp` (milliseconds since epoch), and
- strictly less than the provided `until_version`.

Safety guarantee:
To avoid deleting files that might still be required to reconstruct the
table state at or before the requested cutoff, the function first identifies
the most recent checkpoint whose version is `<= until_version` and whose file
modification time is `<= cutoff_timestamp`. Only files strictly older than
this checkpoint (both by version and timestamp) are considered for deletion.
If no such checkpoint exists (including when there is no `_last_checkpoint`),
the function performs no deletions and returns `Ok(0)`.

See also: https://github.com/delta-io/delta-rs/issues/3692 for background on
why cleanup must align to an existing checkpoint.

---

## cleanup_metadata

`function` · `deltalake_core::protocol::checkpoints::cleanup_metadata`
[Full member contracts, output types and access classification](../operations/deltalake_core.protocol.checkpoints.cleanup_metadata.md)

Also reachable as `deltalake::checkpoints::cleanup_metadata`, `deltalake::protocol::checkpoints::cleanup_metadata`, `deltalake_core::checkpoints::cleanup_metadata`

```rust
async fn cleanup_metadata(table: &DeltaTable, operation_id: Option<uuid::Uuid>) -> DeltaResult<usize>
```

Delete expires log files before given version from table. The table log retention is based on
the `logRetentionDuration` property of the Delta Table, 30 days by default.

---

## create_checkpoint

`function` · `deltalake_core::protocol::checkpoints::create_checkpoint`
[Full member contracts, output types and access classification](../operations/deltalake_core.protocol.checkpoints.create_checkpoint.md)

Also reachable as `deltalake::checkpoints::create_checkpoint`, `deltalake::protocol::checkpoints::create_checkpoint`, `deltalake_core::checkpoints::create_checkpoint`

```rust
async fn create_checkpoint(table: &DeltaTable, operation_id: Option<uuid::Uuid>) -> DeltaResult<()>
```

Creates checkpoint at current table version

---

## create_checkpoint_from_table_url_and_cleanup

`function` · `deltalake_core::protocol::checkpoints::create_checkpoint_from_table_url_and_cleanup`
[Full member contracts, output types and access classification](../operations/deltalake_core.protocol.checkpoints.create_checkpoint_from_table_url_and_cleanup.md)

Also reachable as `deltalake::checkpoints::create_checkpoint_from_table_url_and_cleanup`, `deltalake::protocol::checkpoints::create_checkpoint_from_table_url_and_cleanup`, `deltalake_core::checkpoints::create_checkpoint_from_table_url_and_cleanup`

```rust
async fn create_checkpoint_from_table_url_and_cleanup(table_url: url::Url, version: kernel::Version, cleanup: Option<bool>, operation_id: Option<uuid::Uuid>) -> DeltaResult<()>
```

Loads table from given table [Url] at given `version` and creates checkpoint for it.
The `cleanup` param decides whether to run metadata cleanup of obsolete logs.
If it's empty then the table's `enableExpiredLogCleanup` is used.

---
