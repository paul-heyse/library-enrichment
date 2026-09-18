# `buoyant_kernel::transaction::alter_table::AlterTableTransaction`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.transaction.alter_table.AlterTableTransaction.json).

<a id="op-89f87d3d6d43afebc62b7f72"></a>
## AlterTableTransaction

`type_alias` · `buoyant_kernel::transaction::alter_table::AlterTableTransaction` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type AlterTableTransaction = transaction::Transaction<transaction::AlterTable>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/alter_table.rs#L25).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/alter_table.rs:25`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A type alias for alter-table transactions.

This provides a restricted API surface that only exposes operations valid during ALTER
commands. Data file operations are not available at compile time because `AlterTable`
does not implement [`SupportsDataFiles`](super::SupportsDataFiles).
