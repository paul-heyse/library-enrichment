# `buoyant_kernel::parallel::parallel_scan_metadata::SequentialScanMetadata`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.parallel.parallel_scan_metadata.SequentialScanMetadata.json).

<a id="op-05d3f38fd8bc5fd11a0e993a"></a>
## SequentialScanMetadata

`struct` · `buoyant_kernel::parallel::parallel_scan_metadata::SequentialScanMetadata` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct SequentialScanMetadata
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/parallel/parallel_scan_metadata.rs#L35).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_scan_metadata.rs:35`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Sequential scan metadata processing.

This phase processes commits and single-part checkpoint manifests sequentially.
After exhaustion, call `finish()` to get the result which indicates whether
a distributed phase is needed.

<a id="op-8373dab19ee3da806c3b40b4"></a>
## Item

`assoc_type` · `buoyant_kernel::parallel::parallel_scan_metadata::SequentialScanMetadata::Item` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Item = Result<ScanMetadata, Error>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/parallel/parallel_scan_metadata.rs#L107).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::parallel::parallel_scan_metadata::SequentialScanMetadata", "path": "SequentialScanMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [106, 1], "end": [113, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_scan_metadata.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_scan_metadata.rs:107`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-93631f275a8bf6b22fffb7d9"></a>
## finish

`function` · `buoyant_kernel::parallel::parallel_scan_metadata::SequentialScanMetadata::finish` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn finish(self) -> DeltaResult<AfterSequentialScanMetadata>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/parallel/parallel_scan_metadata.rs#L61).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::parallel::parallel_scan_metadata::SequentialScanMetadata", "path": "SequentialScanMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 1], "end": [104, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_scan_metadata.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_scan_metadata.rs:61`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bd3449211580ea1aecbd839f"></a>
## next

`function` · `buoyant_kernel::parallel::parallel_scan_metadata::SequentialScanMetadata::next` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn next(&mut self) -> Option<Self::Item>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/parallel/parallel_scan_metadata.rs#L109).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::parallel::parallel_scan_metadata::SequentialScanMetadata", "path": "SequentialScanMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [106, 1], "end": [113, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_scan_metadata.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_scan_metadata.rs:109`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-222c3835d4449a44f24371a1"></a>
## correlation_id

`struct_field` · `buoyant_kernel::parallel::parallel_scan_metadata::SequentialScanMetadata::correlation_id` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
correlation_id: Option<std::sync::Arc<str>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/parallel/parallel_scan_metadata.rs#L39).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_scan_metadata.rs:39`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Opaque, caller-supplied correlation id propagated to both phases' metric events.

<a id="op-94c473c4f0232246278dfb5f"></a>
## operation_id

`struct_field` · `buoyant_kernel::parallel::parallel_scan_metadata::SequentialScanMetadata::operation_id` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
operation_id: metrics::MetricId
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/parallel/parallel_scan_metadata.rs#L37).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_scan_metadata.rs:37`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-26cd9426983590544bd2713a"></a>
## sequential

`struct_field` · `buoyant_kernel::parallel::parallel_scan_metadata::SequentialScanMetadata::sequential` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
sequential: parallel::sequential_phase::SequentialPhase<scan::log_replay::ScanLogReplayProcessor>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/parallel/parallel_scan_metadata.rs#L36).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_scan_metadata.rs:36`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-50674fa9435ef36ac1152d71"></a>
## span

`struct_field` · `buoyant_kernel::parallel::parallel_scan_metadata::SequentialScanMetadata::span` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
span: tracing::Span
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/parallel/parallel_scan_metadata.rs#L41).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_scan_metadata.rs:41`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-26baf0a52a2b91a116b171e9"></a>
## start

`struct_field` · `buoyant_kernel::parallel::parallel_scan_metadata::SequentialScanMetadata::start` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
start: std::time::Instant
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/parallel/parallel_scan_metadata.rs#L40).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_scan_metadata.rs:40`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
