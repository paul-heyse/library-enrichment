# `deltalake_core::protocol::DeltaOperation::Restore`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.protocol.DeltaOperation.Restore.json).

<a id="op-07e873356c46def67d9445d4"></a>
## datetime

`struct_field` · `deltalake_core::protocol::DeltaOperation::Restore::datetime` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
datetime: Option<i64>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L350).

Source: `crates/core/src/protocol/mod.rs:350`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Datetime to restore

<a id="op-defe8a84df6485a0b5c026f0"></a>
## version

`struct_field` · `deltalake_core::protocol::DeltaOperation::Restore::version` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
version: Option<kernel::Version>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L348).

Source: `crates/core/src/protocol/mod.rs:348`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Version to restore
