# `deltalake_core::protocol::DeltaOperation::VacuumStart`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.protocol.DeltaOperation.VacuumStart.json).

<a id="op-3d00ef8c6b04475086e90fce"></a>
## default_retention_millis

`struct_field` · `deltalake_core::protocol::DeltaOperation::VacuumStart::default_retention_millis` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
default_retention_millis: i64
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L361).

Source: `crates/core/src/protocol/mod.rs:361`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The default delta table retention milliseconds policy

<a id="op-47eb9beecb9f38eebd93b76c"></a>
## retention_check_enabled

`struct_field` · `deltalake_core::protocol::DeltaOperation::VacuumStart::retention_check_enabled` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
retention_check_enabled: bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L357).

Source: `crates/core/src/protocol/mod.rs:357`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Whether the retention check is enforced

<a id="op-ef2b4f8fe914a8848fc0e964"></a>
## specified_retention_millis

`struct_field` · `deltalake_core::protocol::DeltaOperation::VacuumStart::specified_retention_millis` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
specified_retention_millis: Option<i64>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L359).

Source: `crates/core/src/protocol/mod.rs:359`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The specified retetion period in milliseconds
