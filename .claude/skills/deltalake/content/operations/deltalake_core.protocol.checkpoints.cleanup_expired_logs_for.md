# `deltalake_core::protocol::checkpoints::cleanup_expired_logs_for`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.protocol.checkpoints.cleanup_expired_logs_for.json).

<a id="op-76add85473a0186b5641f6d5"></a>
## cleanup_expired_logs_for

`function` · `deltalake_core::protocol::checkpoints::cleanup_expired_logs_for` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn cleanup_expired_logs_for(keep_version: kernel::Version, log_store: &dyn LogStore, cutoff_timestamp: i64, operation_id: Option<uuid::Uuid>) -> DeltaResult<usize>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/checkpoints.rs#L114).

Source: `crates/core/src/protocol/checkpoints.rs:114`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

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
