# `buoyant_kernel::metrics::events::ScanMetadataCompleted`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.metrics.events.ScanMetadataCompleted.json).

<a id="op-0d8d1f9ee6258711093184f4"></a>
## ScanMetadataCompleted

`struct` · `buoyant_kernel::metrics::events::ScanMetadataCompleted` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct ScanMetadataCompleted
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/events.rs#L1170).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs:1170`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A `parallel_scan_metadata` scan emits **two** events (one per phase) sharing the same
`operation_id`; `scan_metadata` emits one event with [`ScanType::Full`](../operations/buoyant_kernel.metrics.events.ScanType.md#op-caef27939b1d945a5ee0303e).

<a id="op-caab632364324d4d4fb1e939"></a>
## active_add_files_bytes

`struct_field` · `buoyant_kernel::metrics::events::ScanMetadataCompleted::active_add_files_bytes` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
active_add_files_bytes: u64
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/events.rs#L1188).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs:1188`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Size in bytes of the files that survived log replay (files to read).

<a id="op-696fe494a08d5f16ae791753"></a>
## clone

`function` · `buoyant_kernel::metrics::events::ScanMetadataCompleted::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> ScanMetadataCompleted
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/events.rs#L1169).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::metrics::events::ScanMetadataCompleted", "path": "ScanMetadataCompleted"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1169, 17], "end": [1169, 22], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs:1169`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7b1d6d32885ef93769d2cbf8"></a>
## correlation_id

`struct_field` · `buoyant_kernel::metrics::events::ScanMetadataCompleted::correlation_id` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
correlation_id: Option<std::sync::Arc<str>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/events.rs#L1176).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs:1176`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Opaque, caller-supplied id for joining this operation's metric events to the caller's
own request or operation id.

<a id="op-b08642640bb8aef156d243e2"></a>
## dedup_visitor_time

`struct_field` · `buoyant_kernel::metrics::events::ScanMetadataCompleted::dedup_visitor_time` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
dedup_visitor_time: std::time::Duration
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/events.rs#L1198).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs:1198`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Time spent in the deduplication visitor.

<a id="op-d3cb0b955d4cb6a2524bb5e1"></a>
## duration

`struct_field` · `buoyant_kernel::metrics::events::ScanMetadataCompleted::duration` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
duration: std::time::Duration
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/events.rs#L1182).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs:1182`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Wall-clock time from scan start to iterator exhaustion.

<a id="op-210be9ef36df5c466353be19"></a>
## fmt

`function` · `buoyant_kernel::metrics::events::ScanMetadataCompleted::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/events.rs#L1229).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::metrics::events::ScanMetadataCompleted", "path": "ScanMetadataCompleted"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1228, 1], "end": [1257, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs:1229`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-950b8b750bf17452fe0c443c"></a>
## fmt

`function` · `buoyant_kernel::metrics::events::ScanMetadataCompleted::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/events.rs#L1169).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::metrics::events::ScanMetadataCompleted", "path": "ScanMetadataCompleted"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1169, 10], "end": [1169, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs:1169`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-66441fc5e87f4a979cc4028f"></a>
## num_active_add_files

`struct_field` · `buoyant_kernel::metrics::events::ScanMetadataCompleted::num_active_add_files` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
num_active_add_files: u64
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/events.rs#L1186).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs:1186`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Add files that survived log replay (the files the connector reads).

<a id="op-43dedd3d6f6b927603873f3a"></a>
## num_add_files_seen

`struct_field` · `buoyant_kernel::metrics::events::ScanMetadataCompleted::num_add_files_seen` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
num_add_files_seen: u64
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/events.rs#L1184).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs:1184`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Add files that entered deduplication (excludes files filtered by data skipping).

<a id="op-d38ab5d7c2eb731a72b19eba"></a>
## num_non_file_actions

`struct_field` · `buoyant_kernel::metrics::events::ScanMetadataCompleted::num_non_file_actions` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
num_non_file_actions: u64
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/events.rs#L1192).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs:1192`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Non-file actions seen (protocol, metadata, etc.).

<a id="op-c2c7a9911dfd85753d420404"></a>
## num_predicate_filtered

`struct_field` · `buoyant_kernel::metrics::events::ScanMetadataCompleted::num_predicate_filtered` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
num_predicate_filtered: u64
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/events.rs#L1194).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs:1194`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Files filtered by predicates (data skipping + partition pruning).

<a id="op-cf8885fa219f4e9349e8a9e6"></a>
## num_remove_files_seen

`struct_field` · `buoyant_kernel::metrics::events::ScanMetadataCompleted::num_remove_files_seen` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
num_remove_files_seen: u64
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/events.rs#L1190).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs:1190`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Remove files seen (from delta/commit files only).

<a id="op-ee4939a96d3f731e8a186877"></a>
## operation_id

`struct_field` · `buoyant_kernel::metrics::events::ScanMetadataCompleted::operation_id` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
operation_id: MetricId
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/events.rs#L1173).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs:1173`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Unique ID to correlate this scan with other events.

<a id="op-79d011a8d7f202357e811d97"></a>
## peak_hash_set_size

`struct_field` · `buoyant_kernel::metrics::events::ScanMetadataCompleted::peak_hash_set_size` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
peak_hash_set_size: usize
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/events.rs#L1196).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs:1196`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Peak size of the deduplication hash set.

<a id="op-61c890906da24f23175e705a"></a>
## predicate_eval_time

`struct_field` · `buoyant_kernel::metrics::events::ScanMetadataCompleted::predicate_eval_time` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
predicate_eval_time: std::time::Duration
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/events.rs#L1200).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs:1200`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Time spent evaluating predicates.

<a id="op-17cb44113f323a9738b0aa3a"></a>
## scan_type

`struct_field` · `buoyant_kernel::metrics::events::ScanMetadataCompleted::scan_type` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
scan_type: ScanType
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/events.rs#L1180).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs:1180`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Which scan execution path produced this event.

<a id="op-ebe7724cc1f538e7d2a9e761"></a>
## table_type

`struct_field` · `buoyant_kernel::metrics::events::ScanMetadataCompleted::table_type` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
table_type: TableType
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/events.rs#L1178).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs:1178`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Whether the scanned table is path-based or catalog-managed.
