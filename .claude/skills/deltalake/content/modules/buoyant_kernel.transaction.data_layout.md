# `buoyant_kernel::transaction::data_layout`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.transaction.data_layout.json).

<a id="op-b58c6eac5768201d27ad1ec8"></a>
## data_layout

`module` · `buoyant_kernel::transaction::data_layout` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod data_layout
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/data_layout.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/data_layout.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Data layout configuration for Delta tables.

This module defines [`DataLayout`](../operations/buoyant_kernel.transaction.data_layout.DataLayout.md#op-561bccca31f14f965fa3442d) which specifies how data files are organized
within a Delta table. Supported layouts are:

- **None**: No special organization (default)
- **Clustered**: Data files optimized for queries on clustering columns
- **Partitioned**: Data files organized into directories by partition column values
