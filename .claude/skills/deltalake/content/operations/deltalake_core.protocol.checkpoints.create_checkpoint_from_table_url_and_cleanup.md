# `deltalake_core::protocol::checkpoints::create_checkpoint_from_table_url_and_cleanup`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.protocol.checkpoints.create_checkpoint_from_table_url_and_cleanup.json).

<a id="op-4ece223fc0e1bafa0881f508"></a>
## create_checkpoint_from_table_url_and_cleanup

`function` · `deltalake_core::protocol::checkpoints::create_checkpoint_from_table_url_and_cleanup` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn create_checkpoint_from_table_url_and_cleanup(table_url: url::Url, version: kernel::Version, cleanup: Option<bool>, operation_id: Option<uuid::Uuid>) -> DeltaResult<()>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/checkpoints.rs#L75).

Source: `crates/core/src/protocol/checkpoints.rs:75`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Loads table from given table [Url] at given `version` and creates checkpoint for it.
The `cleanup` param decides whether to run metadata cleanup of obsolete logs.
If it's empty then the table's `enableExpiredLogCleanup` is used.

Unresolved upstream links (retained, not inferred): `Url`.
