# `buoyant_kernel::metrics::events::SnapshotBuildSuccess`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.metrics.events.SnapshotBuildSuccess.json).

<a id="op-96f0671cbdddb81ab68666ad"></a>
## SnapshotBuildSuccess

`struct` · `buoyant_kernel::metrics::events::SnapshotBuildSuccess` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct SnapshotBuildSuccess
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/events.rs#L507).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs:507`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A snapshot was built successfully.

<a id="op-f162309e3b2cb8ecb1474586"></a>
## clone

`function` · `buoyant_kernel::metrics::events::SnapshotBuildSuccess::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> SnapshotBuildSuccess
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/events.rs#L506).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::metrics::events::SnapshotBuildSuccess", "path": "SnapshotBuildSuccess"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [506, 17], "end": [506, 22], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs:506`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4aeea8ff4b493c587a7c3857"></a>
## correlation_id

`struct_field` · `buoyant_kernel::metrics::events::SnapshotBuildSuccess::correlation_id` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
correlation_id: Option<std::sync::Arc<str>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/events.rs#L512).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs:512`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Opaque, caller-supplied id for joining this operation's metric events to the caller's
own request or operation id.

<a id="op-62c7f435417fa20db6603fd1"></a>
## duration

`struct_field` · `buoyant_kernel::metrics::events::SnapshotBuildSuccess::duration` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
duration: std::time::Duration
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/events.rs#L519).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs:519`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-af37d519bb5a19f149881d62"></a>
## fmt

`function` · `buoyant_kernel::metrics::events::SnapshotBuildSuccess::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/events.rs#L506).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::metrics::events::SnapshotBuildSuccess", "path": "SnapshotBuildSuccess"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [506, 10], "end": [506, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs:506`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fffa0335c9ed09178c7e891b"></a>
## fmt

`function` · `buoyant_kernel::metrics::events::SnapshotBuildSuccess::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/events.rs#L549).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::metrics::events::SnapshotBuildSuccess", "path": "SnapshotBuildSuccess"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [548, 1], "end": [563, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs:549`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-07c15208370f55f4be1307e0"></a>
## operation_id

`struct_field` · `buoyant_kernel::metrics::events::SnapshotBuildSuccess::operation_id` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
operation_id: MetricId
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/events.rs#L509).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs:509`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5fcb550065beaefe39373e7e"></a>
## table_type

`struct_field` · `buoyant_kernel::metrics::events::SnapshotBuildSuccess::table_type` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
table_type: TableType
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/events.rs#L513).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs:513`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-140a55854d9a0ee904f1deae"></a>
## version

`struct_field` · `buoyant_kernel::metrics::events::SnapshotBuildSuccess::version` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
version: u64
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/events.rs#L516).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs:516`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
