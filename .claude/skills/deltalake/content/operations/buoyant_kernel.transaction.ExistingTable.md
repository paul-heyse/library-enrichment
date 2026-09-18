# `buoyant_kernel::transaction::ExistingTable`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.transaction.ExistingTable.json).

<a id="op-6ff6c69bae5eb319bf8e678e"></a>
## ExistingTable

`struct` · `buoyant_kernel::transaction::ExistingTable` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct ExistingTable
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L154).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:154`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Marker type for transactions on existing tables.

This is the default state for [`Transaction`](../operations/buoyant_kernel.transaction.Transaction.md#op-fde417829ac3f4c320319f35) and provides the full set of operations
including file removal, deletion vector updates, and blind append semantics.

<a id="op-1bb9eaa24c481c210a83b848"></a>
## fmt

`function` · `buoyant_kernel::transaction::ExistingTable::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L153).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transaction::ExistingTable", "path": "ExistingTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [153, 10], "end": [153, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:153`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
