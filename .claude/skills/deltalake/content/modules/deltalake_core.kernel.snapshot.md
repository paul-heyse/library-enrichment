# `deltalake_core::kernel::snapshot`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.snapshot.json).

<a id="op-4688060d44ef625f5a0489ca"></a>
## snapshot

`module` · `deltalake_core::kernel::snapshot` · deltalake-core 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod snapshot
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/mod.rs#L1).

Source: `crates/core/src/kernel/snapshot/mod.rs:1`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Delta table snapshots

A snapshot represents the state of a Delta Table at a given version.

There are two types of snapshots:

- [`Snapshot`](../operations/deltalake_core.kernel.snapshot.Snapshot.md#op-ca78a516c68e7586104c0136) is a snapshot where most data is loaded on demand and only the
  bare minimum - [`Protocol`](../operations/buoyant_kernel.actions.Protocol.md#op-0b0c1a8cbad46c3db514befe) and [`Metadata`](../operations/buoyant_kernel.actions.Metadata.md#op-1843fecb3566e0e5ad63d7aa) - is cached in memory.
- [`EagerSnapshot`](../operations/deltalake_core.kernel.snapshot.EagerSnapshot.md#op-523c35c411674d94607357a6) is a snapshot where much more log data is eagerly loaded into memory.

The submodules provide structures and methods that aid in generating
and consuming snapshots.

## Reading the log


