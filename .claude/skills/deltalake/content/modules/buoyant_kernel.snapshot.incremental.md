# `buoyant_kernel::snapshot::incremental`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.snapshot.incremental.json).

<a id="op-fa4ae47bc66c472401c08400"></a>
## incremental

`module` · `buoyant_kernel::snapshot::incremental` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod incremental
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/incremental.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/incremental.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Incremental [`Snapshot`](../operations/buoyant_kernel.snapshot.Snapshot.md#op-e760c13a7e6fcf7025cb3640) update.

Public API: `Snapshot::builder_from(existing).build(engine)`. The case-by-case
behavior lives in [`Snapshot::try_new_from`].

Unresolved upstream links (retained, not inferred): ``Snapshot::try_new_from``.
