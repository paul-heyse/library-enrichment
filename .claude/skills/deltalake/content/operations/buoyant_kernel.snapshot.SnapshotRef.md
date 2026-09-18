# `buoyant_kernel::snapshot::SnapshotRef`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.snapshot.SnapshotRef.json).

<a id="op-7166b16a39037614007f3d0c"></a>
## SnapshotRef

`type_alias` · `buoyant_kernel::snapshot::SnapshotRef` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type SnapshotRef = std::sync::Arc<Snapshot>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/mod.rs#L43).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs:43`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A shared, thread-safe reference to a [`Snapshot`](../operations/buoyant_kernel.snapshot.Snapshot.md#op-e760c13a7e6fcf7025cb3640).
