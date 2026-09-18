# `buoyant_kernel::transaction::data_layout::DataLayout::Clustered`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.transaction.data_layout.DataLayout.Clustered.json).

<a id="op-40369facc4f930a5ca72c998"></a>
## columns

`struct_field` · `buoyant_kernel::transaction::data_layout::DataLayout::Clustered::columns` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
columns: Vec<expressions::ColumnName>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/data_layout.rs#L37).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/data_layout.rs:37`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Columns to cluster by (in order).
