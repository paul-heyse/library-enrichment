# `deltalake_core::protocol::DeltaOperation::DropColumnNotNull`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.protocol.DeltaOperation.DropColumnNotNull.json).

<a id="op-28c13c4ebf58ec8e40d196fa"></a>
## column

`struct_field` · `deltalake_core::protocol::DeltaOperation::DropColumnNotNull::column` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
column: kernel::StructField
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L385).

Source: `crates/core/src/protocol/mod.rs:385`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The name of the column whose `NOT NULL` constraint was dropped
