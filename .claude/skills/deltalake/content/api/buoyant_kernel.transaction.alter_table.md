# `buoyant_kernel::transaction::alter_table`

Crate `buoyant_kernel` · 1 public items · structured records in [`model/buoyant_kernel.transaction.alter_table.json`](../model/buoyant_kernel.transaction.alter_table.json)

## AlterTableTransaction

`type_alias` · `buoyant_kernel::transaction::alter_table::AlterTableTransaction`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.transaction.alter_table.AlterTableTransaction.md)

Also reachable as `buoyant_kernel::transaction::AlterTableTransaction`, `delta_kernel::transaction::alter_table::AlterTableTransaction`

```rust
type AlterTableTransaction = transaction::Transaction<transaction::AlterTable>
```

A type alias for alter-table transactions.

This provides a restricted API surface that only exposes operations valid during ALTER
commands. Data file operations are not available at compile time because `AlterTable`
does not implement [`SupportsDataFiles`](super::SupportsDataFiles).

---
