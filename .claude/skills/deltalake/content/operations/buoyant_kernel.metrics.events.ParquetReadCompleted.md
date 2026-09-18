# `buoyant_kernel::metrics::events::ParquetReadCompleted`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.metrics.events.ParquetReadCompleted.json).

<a id="op-4e3704d8bec3c673008c3bed"></a>
## ParquetReadCompleted

`struct` · `buoyant_kernel::metrics::events::ParquetReadCompleted` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct ParquetReadCompleted
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/events.rs#L998).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs:998`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Emitted once per `ParquetHandler::read_parquet_files` call when the returned iterator is
fully consumed or dropped. `bytes_read` is the sum of on-disk `FileMeta::size`, not the
deserialized payload size.

<a id="op-db4e143b5a2ffc68d2d782e6"></a>
## bytes_read

`struct_field` · `buoyant_kernel::metrics::events::ParquetReadCompleted::bytes_read` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
bytes_read: u64
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/events.rs#L1001).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs:1001`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-598940bb917d7d0cac75cbe2"></a>
## clone

`function` · `buoyant_kernel::metrics::events::ParquetReadCompleted::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> ParquetReadCompleted
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/events.rs#L997).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::metrics::events::ParquetReadCompleted", "path": "ParquetReadCompleted"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [997, 17], "end": [997, 22], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs:997`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4f2f016de75d5cdda2350900"></a>
## fmt

`function` · `buoyant_kernel::metrics::events::ParquetReadCompleted::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/events.rs#L997).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::metrics::events::ParquetReadCompleted", "path": "ParquetReadCompleted"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [997, 10], "end": [997, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs:997`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fef6c8144499be47352ad00b"></a>
## fmt

`function` · `buoyant_kernel::metrics::events::ParquetReadCompleted::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/events.rs#L1018).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::metrics::events::ParquetReadCompleted", "path": "ParquetReadCompleted"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1017, 1], "end": [1028, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs:1018`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fa7e391d44c0585b3874182c"></a>
## num_files

`struct_field` · `buoyant_kernel::metrics::events::ParquetReadCompleted::num_files` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
num_files: u64
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/events.rs#L1000).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs:1000`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
