# `buoyant_kernel::incremental_scan::IncrementalScanSummary`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.incremental_scan.IncrementalScanSummary.json).

<a id="op-590238813d08fbbedbd5095d"></a>
## IncrementalScanSummary

`struct` · `buoyant_kernel::incremental_scan::IncrementalScanSummary` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct IncrementalScanSummary
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/incremental_scan/mod.rs#L392).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs:392`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Live file-key sets without cross-snapshot classification, returned by
[`IncrementalScanStream::into_summary`](../operations/buoyant_kernel.incremental_scan.IncrementalScanStream.md#op-b575b2704177d751b5713872).

Each set element is a [`FileActionKey`](../operations/buoyant_kernel.log_replay.FileActionKey.md#op-bf87491bbfa065a600f67087) of `(path, dv_unique_id)`. Consumers should match
on the full key rather than just the path: the same path with different DV ids (e.g. an
`add(P, dv=new) + remove(P, dv=old)` DV-replacement pair) refers to distinct logical
files, and collapsing to path alone loses that distinction.

<a id="op-ee70ad052df43298bbbab8ea"></a>
## base_version

`struct_field` · `buoyant_kernel::incremental_scan::IncrementalScanSummary::base_version` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
base_version: Version
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/incremental_scan/mod.rs#L394).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs:394`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Exclusive lower bound of the scan range, as supplied to [`IncrementalScanBuilder`](../operations/buoyant_kernel.incremental_scan.IncrementalScanBuilder.md#op-9d4dcdc70da5cc4e6963af53).

<a id="op-97bb060d3e953c4f00ddbd84"></a>
## fmt

`function` · `buoyant_kernel::incremental_scan::IncrementalScanSummary::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/incremental_scan/mod.rs#L390).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::incremental_scan::IncrementalScanSummary", "path": "IncrementalScanSummary"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [390, 10], "end": [390, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs:390`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-28698ea9a7dfe8d5e51f801f"></a>
## live_adds

`struct_field` · `buoyant_kernel::incremental_scan::IncrementalScanSummary::live_adds` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
live_adds: std::collections::HashSet<log_replay::FileActionKey>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/incremental_scan/mod.rs#L399).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs:399`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

File keys of live Add actions in `(base_version, target_version]` (i.e. Adds whose
`(path, dv_unique_id)` is still present at `target_version`).

<a id="op-65194b69f53daa04de74d47f"></a>
## removes

`struct_field` · `buoyant_kernel::incremental_scan::IncrementalScanSummary::removes` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
removes: std::collections::HashSet<log_replay::FileActionKey>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/incremental_scan/mod.rs#L402).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs:402`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

File keys of Remove actions in `(base_version, target_version]`, deduped
first-seen-wins by `(path, dv_unique_id)`.

<a id="op-59e91b3c27f56304b1a22a14"></a>
## target_version

`struct_field` · `buoyant_kernel::incremental_scan::IncrementalScanSummary::target_version` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
target_version: Version
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/incremental_scan/mod.rs#L396).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs:396`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Inclusive upper bound; equals the source snapshot's version.
