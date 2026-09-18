# `deltalake_core::protocol::DeltaOperation::Optimize`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.protocol.DeltaOperation.Optimize.json).

<a id="op-4e9741fa74dc9b057ffbb2fc"></a>
## predicate

`struct_field` · `deltalake_core::protocol::DeltaOperation::Optimize::predicate` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
predicate: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L337).

Source: `crates/core/src/protocol/mod.rs:337`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The filter used to determine which partitions to filter

<a id="op-30f8fe922162b3ce1f76eb4f"></a>
## target_size

`struct_field` · `deltalake_core::protocol::DeltaOperation::Optimize::target_size` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
target_size: i64
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L339).

Source: `crates/core/src/protocol/mod.rs:339`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Target optimize size
