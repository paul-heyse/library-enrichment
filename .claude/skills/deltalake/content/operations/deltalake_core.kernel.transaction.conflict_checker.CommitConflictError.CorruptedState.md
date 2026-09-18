# `deltalake_core::kernel::transaction::conflict_checker::CommitConflictError::CorruptedState`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.transaction.conflict_checker.CommitConflictError.CorruptedState.json).

<a id="op-0a25139c89e0348658ef4b75"></a>
## source

`struct_field` · `deltalake_core::kernel::transaction::conflict_checker::CommitConflictError::CorruptedState::source` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
source: Box<dyn std::error::Error + Send + Sync + 'static>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/conflict_checker.rs#L81).

Source: `crates/core/src/kernel/transaction/conflict_checker.rs:81`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Source error
