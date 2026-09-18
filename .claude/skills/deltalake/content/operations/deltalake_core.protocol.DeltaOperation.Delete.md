# `deltalake_core::protocol::DeltaOperation::Delete`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.protocol.DeltaOperation.Delete.json).

<a id="op-13630a3cc0e1d8779f4e7233"></a>
## predicate

`struct_field` · `deltalake_core::protocol::DeltaOperation::Delete::predicate` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
predicate: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L269).

Source: `crates/core/src/protocol/mod.rs:269`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The condition the to be deleted data must match
