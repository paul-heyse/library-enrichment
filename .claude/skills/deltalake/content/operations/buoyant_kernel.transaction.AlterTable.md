# `buoyant_kernel::transaction::AlterTable`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.transaction.AlterTable.json).

<a id="op-4e05b38f70320061382d881f"></a>
## AlterTable

`struct` · `buoyant_kernel::transaction::AlterTable` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct AlterTable
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L168).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:168`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Marker type for alter-table (schema evolution) transactions.

Transactions in this state perform metadata-only commits. Data file operations are not
available at compile time because `AlterTable` does not implement [`SupportsDataFiles`](../operations/buoyant_kernel.transaction.SupportsDataFiles.md#op-aaa24a96fb4d2961e35193b9).

<a id="op-a71589bcc02c0b94a34e8af2"></a>
## fmt

`function` · `buoyant_kernel::transaction::AlterTable::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L167).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transaction::AlterTable", "path": "AlterTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [167, 10], "end": [167, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:167`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
