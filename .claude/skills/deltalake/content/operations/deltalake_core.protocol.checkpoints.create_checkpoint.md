# `deltalake_core::protocol::checkpoints::create_checkpoint`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.protocol.checkpoints.create_checkpoint.json).

<a id="op-e2ce7c6abdadd100ddb3dbb1"></a>
## create_checkpoint

`function` · `deltalake_core::protocol::checkpoints::create_checkpoint` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn create_checkpoint(table: &DeltaTable, operation_id: Option<uuid::Uuid>) -> DeltaResult<()>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/checkpoints.rs#L48).

Source: `crates/core/src/protocol/checkpoints.rs:48`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Creates checkpoint at current table version
