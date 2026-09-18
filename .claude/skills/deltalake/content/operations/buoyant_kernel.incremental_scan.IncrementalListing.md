# `buoyant_kernel::incremental_scan::IncrementalListing`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.incremental_scan.IncrementalListing.json).

<a id="op-60643c52b43823c0265b9afa"></a>
## IncrementalListing

`struct` · `buoyant_kernel::incremental_scan::IncrementalListing` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct IncrementalListing
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/incremental_scan/mod.rs#L408).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs:408`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Eager output of [`IncrementalScanStream::into_listing`](../operations/buoyant_kernel.incremental_scan.IncrementalScanStream.md#op-b3a7df95276505f9561d2219): the buffered Add
batches plus the summary (no cross-snapshot classification).

<a id="op-5afdf60bb465ef096855524b"></a>
## add_files

`struct_field` · `buoyant_kernel::incremental_scan::IncrementalListing::add_files` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
add_files: Vec<engine_data::FilteredEngineData>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/incremental_scan/mod.rs#L413).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs:413`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

All live Add batches in descending commit-version order. One entry per source
commit batch that produced any live Adds.

<a id="op-21fa1ac4851d222705080d70"></a>
## fmt

`function` · `buoyant_kernel::incremental_scan::IncrementalListing::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/incremental_scan/mod.rs#L417).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::incremental_scan::IncrementalListing", "path": "IncrementalListing"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [416, 1], "end": [423, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs:417`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9ece8d88360142d8e5794f88"></a>
## summary

`struct_field` · `buoyant_kernel::incremental_scan::IncrementalListing::summary` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
summary: IncrementalScanSummary
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/incremental_scan/mod.rs#L410).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs:410`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Live Add and Remove file-key sets for the range.
