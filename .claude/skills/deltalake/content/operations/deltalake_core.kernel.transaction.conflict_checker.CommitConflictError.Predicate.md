# `deltalake_core::kernel::transaction::conflict_checker::CommitConflictError::Predicate`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.transaction.conflict_checker.CommitConflictError.Predicate.json).

<a id="op-dcc4b27b6a0a588c15c7b0e1"></a>
## source

`struct_field` · `deltalake_core::kernel::transaction::conflict_checker::CommitConflictError::Predicate::source` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
source: Box<dyn std::error::Error + Send + Sync + 'static>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/conflict_checker.rs#L88).

Source: `crates/core/src/kernel/transaction/conflict_checker.rs:88`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Source error
