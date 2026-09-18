# `deltalake_core::protocol::DeltaOperation::StreamingUpdate`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.protocol.DeltaOperation.StreamingUpdate.json).

<a id="op-8c20f7abd59a8cb7b4247207"></a>
## epoch_id

`struct_field` · `deltalake_core::protocol::DeltaOperation::StreamingUpdate::epoch_id` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
epoch_id: i64
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L322).

Source: `crates/core/src/protocol/mod.rs:322`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The epoch id of the written micro-batch.

<a id="op-8a238772ea961dee5f676b6d"></a>
## output_mode

`struct_field` · `deltalake_core::protocol::DeltaOperation::StreamingUpdate::output_mode` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
output_mode: OutputMode
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L318).

Source: `crates/core/src/protocol/mod.rs:318`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The output mode the streaming writer is using.

<a id="op-bf4a15fd2ee16ae85c5ac8bb"></a>
## query_id

`struct_field` · `deltalake_core::protocol::DeltaOperation::StreamingUpdate::query_id` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
query_id: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L320).

Source: `crates/core/src/protocol/mod.rs:320`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The query id of the streaming writer.
