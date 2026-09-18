# `buoyant_kernel::parallel::parallel_scan_metadata::ParallelScanMetadata`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.parallel.parallel_scan_metadata.ParallelScanMetadata.json).

<a id="op-4809b303e2b0b643a223b957"></a>
## ParallelScanMetadata

`struct` · `buoyant_kernel::parallel::parallel_scan_metadata::ParallelScanMetadata` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct ParallelScanMetadata
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/parallel/parallel_scan_metadata.rs#L249).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_scan_metadata.rs:249`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d917cb19d56ff1a67ed3fdc"></a>
## Item

`assoc_type` · `buoyant_kernel::parallel::parallel_scan_metadata::ParallelScanMetadata::Item` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Item = Result<ScanMetadata, Error>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/parallel/parallel_scan_metadata.rs#L281).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::parallel::parallel_scan_metadata::ParallelScanMetadata", "path": "ParallelScanMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [280, 1], "end": [287, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_scan_metadata.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_scan_metadata.rs:281`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8c9ae3b5545cb5a401d1eb4d"></a>
## new_from_iter

`function` · `buoyant_kernel::parallel::parallel_scan_metadata::ParallelScanMetadata::new_from_iter` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new_from_iter(state: Arc<ParallelState>, iter: impl IntoIterator<Item = DeltaResult<Box<dyn EngineData>>> + 'static) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/parallel/parallel_scan_metadata.rs#L268).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::parallel::parallel_scan_metadata::ParallelScanMetadata", "path": "ParallelScanMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [254, 1], "end": [278, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_scan_metadata.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_scan_metadata.rs:268`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-51d7f47a231b6f40186fcabf"></a>
## next

`function` · `buoyant_kernel::parallel::parallel_scan_metadata::ParallelScanMetadata::next` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn next(&mut self) -> Option<Self::Item>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/parallel/parallel_scan_metadata.rs#L283).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::parallel::parallel_scan_metadata::ParallelScanMetadata", "path": "ParallelScanMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [280, 1], "end": [287, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_scan_metadata.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_scan_metadata.rs:283`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c24a137c0fd6846268392d6b"></a>
## try_new

`function` · `buoyant_kernel::parallel::parallel_scan_metadata::ParallelScanMetadata::try_new` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_new(engine: Arc<dyn Engine>, state: Arc<ParallelState>, leaf_files: Vec<FileMeta>) -> DeltaResult<Self>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/parallel/parallel_scan_metadata.rs#L255).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::parallel::parallel_scan_metadata::ParallelScanMetadata", "path": "ParallelScanMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [254, 1], "end": [278, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_scan_metadata.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_scan_metadata.rs:255`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a1b76519c30a3bf310329629"></a>
## processor

`struct_field` · `buoyant_kernel::parallel::parallel_scan_metadata::ParallelScanMetadata::processor` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
processor: parallel::parallel_phase::ParallelPhase<std::sync::Arc<ParallelState>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/parallel/parallel_scan_metadata.rs#L250).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_scan_metadata.rs:250`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6ff65b56c4deb21597ff0030"></a>
## span

`struct_field` · `buoyant_kernel::parallel::parallel_scan_metadata::ParallelScanMetadata::span` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
span: tracing::Span
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/parallel/parallel_scan_metadata.rs#L251).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_scan_metadata.rs:251`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
