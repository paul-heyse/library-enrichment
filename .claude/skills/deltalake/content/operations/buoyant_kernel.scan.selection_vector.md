# `buoyant_kernel::scan::selection_vector`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.scan.selection_vector.json).

<a id="op-54c4dc1a0bc880a48ce32ae0"></a>
## selection_vector

`function` · `buoyant_kernel::scan::selection_vector` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn selection_vector(engine: &dyn Engine, descriptor: &actions::deletion_vector::DeletionVectorDescriptor, table_root: &url::Url) -> DeltaResult<Vec<bool>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L1277).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:1277`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
