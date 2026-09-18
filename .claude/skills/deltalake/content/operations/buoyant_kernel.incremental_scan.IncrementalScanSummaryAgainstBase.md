# `buoyant_kernel::incremental_scan::IncrementalScanSummaryAgainstBase`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.incremental_scan.IncrementalScanSummaryAgainstBase.json).

<a id="op-8e13849c7e77de1fd41f78c4"></a>
## IncrementalScanSummaryAgainstBase

`struct` · `buoyant_kernel::incremental_scan::IncrementalScanSummaryAgainstBase` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct IncrementalScanSummaryAgainstBase
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/incremental_scan/mod.rs#L436).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs:436`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Cross-snapshot-classified file-key sets, returned by
[`IncrementalScanStream::into_summary_against_base_iter`](../operations/buoyant_kernel.incremental_scan.IncrementalScanStream.md#op-f583ebb1d9cacd91a9711db1) (or its closure variant).

To advance a delta-on-base file listing cache, append the streamed Add batches to
the delta layer and use the union `removes U duplicate_adds` as the remove-mask
against the base. Both sets are required: `removes` masks files that left the table;
`duplicate_adds` masks the stale base entry of each file the range re-added with new
metadata. Matching against the full `(path, dv_unique_id)` key (not path alone) keeps
distinct DV-revision entries separate.

<a id="op-fdbb3bc433194b559085d151"></a>
## base_version

`struct_field` · `buoyant_kernel::incremental_scan::IncrementalScanSummaryAgainstBase::base_version` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
base_version: Version
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/incremental_scan/mod.rs#L438).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs:438`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Exclusive lower bound of the scan range.

<a id="op-090c053b787678996f262f68"></a>
## duplicate_adds

`struct_field` · `buoyant_kernel::incremental_scan::IncrementalScanSummaryAgainstBase::duplicate_adds` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
duplicate_adds: std::collections::HashSet<log_replay::FileActionKey>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/incremental_scan/mod.rs#L444).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs:444`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

File keys from the live Add stream that also appear in the consumer's base
listing (metadata-only re-adds, e.g. OPTIMIZE / liquid clustering re-tag).
The corresponding rows are still in the streamed Adds.

<a id="op-41263fe586381ea06d9fb423"></a>
## fmt

`function` · `buoyant_kernel::incremental_scan::IncrementalScanSummaryAgainstBase::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/incremental_scan/mod.rs#L434).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::incremental_scan::IncrementalScanSummaryAgainstBase", "path": "IncrementalScanSummaryAgainstBase"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [434, 10], "end": [434, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs:434`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-23dfdf3e5fb7fb891d105d2b"></a>
## removes

`struct_field` · `buoyant_kernel::incremental_scan::IncrementalScanSummaryAgainstBase::removes` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
removes: std::collections::HashSet<log_replay::FileActionKey>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/incremental_scan/mod.rs#L447).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs:447`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

File keys of Remove actions in the range. Consumers must union this with
`duplicate_adds` when masking the base.

<a id="op-6a697002bb226e5696ce254a"></a>
## target_version

`struct_field` · `buoyant_kernel::incremental_scan::IncrementalScanSummaryAgainstBase::target_version` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
target_version: Version
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/incremental_scan/mod.rs#L440).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs:440`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Inclusive upper bound; equals the source snapshot's version.
