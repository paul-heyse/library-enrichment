# `deltalake_core::protocol::DeltaOperation::Write`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.protocol.DeltaOperation.Write.json).

<a id="op-eec13173d3544f42d40a2b95"></a>
## mode

`struct_field` · `deltalake_core::protocol::DeltaOperation::Write::mode` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
mode: SaveMode
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L259).

Source: `crates/core/src/protocol/mod.rs:259`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The save mode used during the write.

<a id="op-6f4785b4bd366cb3a0a3c953"></a>
## partition_by

`struct_field` · `deltalake_core::protocol::DeltaOperation::Write::partition_by` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
partition_by: Option<Vec<String>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L261).

Source: `crates/core/src/protocol/mod.rs:261`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The columns the write is partitioned by.

<a id="op-2f97df5202df01d8d6631c44"></a>
## predicate

`struct_field` · `deltalake_core::protocol::DeltaOperation::Write::predicate` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
predicate: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L263).

Source: `crates/core/src/protocol/mod.rs:263`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The predicate used during the write.
