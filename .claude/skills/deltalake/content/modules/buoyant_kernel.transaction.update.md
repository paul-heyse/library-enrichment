# `buoyant_kernel::transaction::update`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.transaction.update.json).

<a id="op-29be5648ce96625c9935cd5b"></a>
## update

`module` · `buoyant_kernel::transaction::update` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod update
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/update.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/update.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Update table transaction methods.

This module contains the constructor, public API, and deletion vector update logic for
update-table transactions. Each transaction type lives in its own file;
see [`mod.rs`](super) for shared commit logic.

Includes:
- [`try_new_existing_table`](Transaction::try_new_existing_table) constructor
- Deletion vector updates
- Blind append, operation setting, domain metadata removal, and file removal

Unresolved upstream links (retained, not inferred): `Transaction::try_new_existing_table`.
