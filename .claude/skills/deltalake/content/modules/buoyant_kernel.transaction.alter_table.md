# `buoyant_kernel::transaction::alter_table`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.transaction.alter_table.json).

<a id="op-c677b3ed0c0b65bab1964646"></a>
## alter_table

`module` · `buoyant_kernel::transaction::alter_table` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod alter_table
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/alter_table.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/alter_table.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Alter table transaction types and constructor.

This module defines the [`AlterTableTransaction`](../operations/buoyant_kernel.transaction.alter_table.AlterTableTransaction.md#op-89f87d3d6d43afebc62b7f72) type alias and the
[`try_new_alter_table`](AlterTableTransaction::try_new_alter_table) constructor.
The builder logic lives in [`builder::alter_table`](super::builder::alter_table).

Unresolved upstream links (retained, not inferred): `AlterTableTransaction::try_new_alter_table`.
