# `buoyant_kernel::transaction::builder::create_table`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.transaction.builder.create_table.json).

<a id="op-f4e33d268c701fe0adeb4779"></a>
## create_table

`module` · `buoyant_kernel::transaction::builder::create_table` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod create_table
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/builder/create_table.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/builder/create_table.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Builder for creating new Delta tables.

This module contains [`CreateTableTransactionBuilder`](../operations/buoyant_kernel.transaction.builder.create_table.CreateTableTransactionBuilder.md#op-05656d6a5734e8cc8804084f), which validates and constructs a
[`CreateTableTransaction`](../operations/buoyant_kernel.transaction.create_table.CreateTableTransaction.md#op-32c02d6f39623e6417b55daf) from user-provided schema, properties, and data layout options.

Use [`create_table()`](super::super::create_table::create_table) as the entry point rather
than constructing the builder directly.
