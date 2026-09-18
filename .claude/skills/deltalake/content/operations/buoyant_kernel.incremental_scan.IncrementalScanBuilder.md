# `buoyant_kernel::incremental_scan::IncrementalScanBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.incremental_scan.IncrementalScanBuilder.json).

<a id="op-9d4dcdc70da5cc4e6963af53"></a>
## IncrementalScanBuilder

`struct` · `buoyant_kernel::incremental_scan::IncrementalScanBuilder` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct IncrementalScanBuilder
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/incremental_scan/mod.rs#L24).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs:24`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Builder for an incremental scan over `(base_version, target_version]`. Construct via
[`crate::Snapshot::incremental_scan_builder`](../operations/buoyant_kernel.snapshot.Snapshot.md#op-7190e7670c8e946e3dcb5592) and drive with
[`IncrementalScanBuilder::build`](../operations/buoyant_kernel.incremental_scan.IncrementalScanBuilder.md#op-3a54fd61804755c3505b7c49).

<a id="op-3a54fd61804755c3505b7c49"></a>
## build

`function` · `buoyant_kernel::incremental_scan::IncrementalScanBuilder::build` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn build(self, engine: &dyn Engine) -> DeltaResult<Option<IncrementalScanStream>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/incremental_scan/mod.rs#L60).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::incremental_scan::IncrementalScanBuilder", "path": "IncrementalScanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 1], "end": [133, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs:60`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Build the incremental scan stream, or `None` if the target snapshot's commit list
cannot serve `(base_version, target_version]` (consumers should fall back to a full
scan via [`crate::Snapshot::scan_builder`](../operations/buoyant_kernel.snapshot.Snapshot.md#op-c9c6b7e23f7231ad4b5e40d8)).

Does not re-list `_delta_log/`: walks the target snapshot's already-validated commit
list and clips it to `(base_version, target_version]`. For catalog-managed tables
that's important — the snapshot's `log_tail` carries staged commits that a fresh
storage listing would miss.

Validates only the target's reader features via [`Operation::Scan`](../operations/buoyant_kernel.table_features.Operation.md#op-23cb84b501e6c89c971fc7f2). By the protocol's
feature-immutability rule, the target's `readerFeatures` is a superset of every
reader feature used in any commit in `(base_version, target_version]`. Of the fields
kernel itself decodes from each row (path + deletionVector.*), only
`deletionVector.*` is feature-gated, and pre-`deletionVectors`-enable commits cannot
populate it. Consumers that decode pass-through fields (stats, partitionValues,
baseRowId) should interpret each row against the protocol at that row's commit
version, not naively against the target snapshot.

# Errors
- `Err` if `base_version >= target_snapshot.version()` (caller error).
- `Err` if the target snapshot's protocol contains an unsupported reader feature.
- `Err` if the engine fails to open the commit stream.

<a id="op-1f87e4449ae397e3578181ba"></a>
## fmt

`function` · `buoyant_kernel::incremental_scan::IncrementalScanBuilder::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/incremental_scan/mod.rs#L23).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::incremental_scan::IncrementalScanBuilder", "path": "IncrementalScanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [23, 10], "end": [23, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs:23`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a349990b44520f8dfcd366d7"></a>
## base_version

`struct_field` · `buoyant_kernel::incremental_scan::IncrementalScanBuilder::base_version` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
base_version: Version
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/incremental_scan/mod.rs#L26).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs:26`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ac060cb1606157d07bcb98de"></a>
## target_snapshot

`struct_field` · `buoyant_kernel::incremental_scan::IncrementalScanBuilder::target_snapshot` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
target_snapshot: snapshot::SnapshotRef
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/incremental_scan/mod.rs#L25).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs:25`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
