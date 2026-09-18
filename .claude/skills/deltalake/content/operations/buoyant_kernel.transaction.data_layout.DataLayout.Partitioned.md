# `buoyant_kernel::transaction::data_layout::DataLayout::Partitioned`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.transaction.data_layout.DataLayout.Partitioned.json).

<a id="op-d53507e192b3147c1e71ee23"></a>
## columns

`struct_field` · `buoyant_kernel::transaction::data_layout::DataLayout::Partitioned::columns` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
columns: Vec<expressions::ColumnName>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/data_layout.rs#L45).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/data_layout.rs:45`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Columns to partition by (in order).
