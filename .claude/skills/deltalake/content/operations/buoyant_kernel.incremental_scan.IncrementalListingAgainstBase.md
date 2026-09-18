# `buoyant_kernel::incremental_scan::IncrementalListingAgainstBase`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.incremental_scan.IncrementalListingAgainstBase.json).

<a id="op-7b6d50d0ae6015aa96ee0b19"></a>
## IncrementalListingAgainstBase

`struct` · `buoyant_kernel::incremental_scan::IncrementalListingAgainstBase` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct IncrementalListingAgainstBase
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/incremental_scan/mod.rs#L454).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs:454`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Eager output of [`IncrementalScanStream::into_listing_against_base_iter`](../operations/buoyant_kernel.incremental_scan.IncrementalScanStream.md#op-7afed1cd4e74bd33ef2ded5b) (or its
closure variant): the buffered
Add batches plus the classified summary.

<a id="op-5687640be7cd80b1efeec389"></a>
## add_files

`struct_field` · `buoyant_kernel::incremental_scan::IncrementalListingAgainstBase::add_files` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
add_files: Vec<engine_data::FilteredEngineData>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/incremental_scan/mod.rs#L458).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs:458`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

All live Add batches in descending commit-version order.

<a id="op-44120b1730544d2655c8cd62"></a>
## fmt

`function` · `buoyant_kernel::incremental_scan::IncrementalListingAgainstBase::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/incremental_scan/mod.rs#L462).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::incremental_scan::IncrementalListingAgainstBase", "path": "IncrementalListingAgainstBase"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [461, 1], "end": [468, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs:462`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c9725be2d689ed15379b115d"></a>
## summary

`struct_field` · `buoyant_kernel::incremental_scan::IncrementalListingAgainstBase::summary` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
summary: IncrementalScanSummaryAgainstBase
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/incremental_scan/mod.rs#L456).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs:456`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Classified file-key sets for the range; see [`IncrementalScanSummaryAgainstBase`](../operations/buoyant_kernel.incremental_scan.IncrementalScanSummaryAgainstBase.md#op-8e13849c7e77de1fd41f78c4).
