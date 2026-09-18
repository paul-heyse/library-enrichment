# `deltalake_core::kernel::transaction::conflict_checker`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.transaction.conflict_checker.json).

<a id="op-56091f9164de47fa519848ed"></a>
## conflict_checker

`module` · `deltalake_core::kernel::transaction::conflict_checker` · deltalake-core 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod conflict_checker
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/conflict_checker.rs#L1).

Source: `crates/core/src/kernel/transaction/conflict_checker.rs:1`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Helper module to check if a transaction can be committed in case of conflicting commits.
