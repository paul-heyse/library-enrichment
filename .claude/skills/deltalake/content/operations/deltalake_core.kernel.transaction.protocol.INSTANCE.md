# `deltalake_core::kernel::transaction::protocol::INSTANCE`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.transaction.protocol.INSTANCE.json).

<a id="op-236b8e8fb2324430ad955f2c"></a>
## INSTANCE

`static` · `deltalake_core::kernel::transaction::protocol::INSTANCE` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
static INSTANCE: std::sync::LazyLock<ProtocolChecker>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/protocol.rs#L308).

Source: `crates/core/src/kernel/transaction/protocol.rs:308`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The global protocol checker instance to validate table versions and features.

This instance is used by default in all transaction operations, since feature
support is not configurable but rather decided at compile time.

As we implement new features, we need to update this instance accordingly.
resulting version support is determined by the supported table feature set.
