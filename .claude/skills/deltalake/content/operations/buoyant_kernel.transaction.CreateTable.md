# `buoyant_kernel::transaction::CreateTable`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.transaction.CreateTable.json).

<a id="op-58c0a38333869df218438466"></a>
## CreateTable

`struct` · `buoyant_kernel::transaction::CreateTable` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct CreateTable
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L161).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:161`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Marker type for create-table transactions.

Transactions in this state have a restricted API surface — operations that are semantically
invalid for table creation (e.g. file removal, domain metadata removal) are not available.

<a id="op-d232ed8a9af870b9225ef4f8"></a>
## fmt

`function` · `buoyant_kernel::transaction::CreateTable::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L160).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transaction::CreateTable", "path": "CreateTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [160, 10], "end": [160, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:160`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
