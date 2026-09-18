# `buoyant_kernel::snapshot::builder::SnapshotBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.snapshot.builder.SnapshotBuilder.json).

<a id="op-9d8aad6c7e18d68b3febe790"></a>
## SnapshotBuilder

`struct` · `buoyant_kernel::snapshot::builder::SnapshotBuilder` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct SnapshotBuilder
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/builder.rs#L40).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/builder.rs:40`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Builder for creating [`Snapshot`](../operations/buoyant_kernel.snapshot.Snapshot.md#op-e760c13a7e6fcf7025cb3640) instances.

# Example

```no_run
# use buoyant_kernel as delta_kernel;
# use delta_kernel::{Snapshot, Engine};
# use url::Url;
# fn example(engine: &dyn Engine) -> delta_kernel::DeltaResult<()> {
let table_root = Url::parse("file:///path/to/table")?;

// Build a snapshot
let snapshot = Snapshot::builder_for(table_root.clone())
    .at_version(5) // Optional: specify a time-travel version (default is latest version)
    .build(engine)?;

# Ok(())
# }
```

<a id="op-7c76b1d9753abc170521f930"></a>
## at_version

`function` · `buoyant_kernel::snapshot::builder::SnapshotBuilder::at_version` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn at_version(self, version: Version) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/builder.rs#L135).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::snapshot::builder::SnapshotBuilder", "path": "SnapshotBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 1], "end": [433, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/builder.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/builder.rs:135`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Set the target version of the [`Snapshot`](../operations/buoyant_kernel.snapshot.Snapshot.md#op-e760c13a7e6fcf7025cb3640). When omitted, the Snapshot is created at the
latest version of the table.

<a id="op-56826f93389b1ad048092b2d"></a>
## build

`function` · `buoyant_kernel::snapshot::builder::SnapshotBuilder::build` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn build(self, engine: &dyn Engine) -> DeltaResult<SnapshotRef>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/builder.rs#L221).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::snapshot::builder::SnapshotBuilder", "path": "SnapshotBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 1], "end": [433, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/builder.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/builder.rs:221`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a new [`Snapshot`](../operations/buoyant_kernel.snapshot.Snapshot.md#op-e760c13a7e6fcf7025cb3640). This returns a [`SnapshotRef`](../operations/buoyant_kernel.snapshot.SnapshotRef.md#op-7166b16a39037614007f3d0c) (`Arc<Snapshot>`), perhaps
returning a reference to an existing snapshot if the request to build a new snapshot
matches the version of an existing snapshot.

Reports metrics: [`MetricEvent::SnapshotBuildSuccess`] or
[`MetricEvent::SnapshotBuildFailure`].

# Parameters

- `engine`: Implementation of [`Engine`](../operations/buoyant_kernel.Engine.md#op-144f8dad57c79b7743fd1386) apis.

[`MetricEvent::SnapshotBuildSuccess`]: crate::metrics::MetricEvent::SnapshotBuildSuccess
[`MetricEvent::SnapshotBuildFailure`]: crate::metrics::MetricEvent::SnapshotBuildFailure

<a id="op-0822eafdfd8041b0b30a041b"></a>
## fmt

`function` · `buoyant_kernel::snapshot::builder::SnapshotBuilder::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/builder.rs#L39).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::snapshot::builder::SnapshotBuilder", "path": "SnapshotBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 10], "end": [39, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/builder.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/builder.rs:39`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-62e6c803785e511cdbed40ed"></a>
## with_correlation_id

`function` · `buoyant_kernel::snapshot::builder::SnapshotBuilder::with_correlation_id` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_correlation_id(self, correlation_id: impl Into<Arc<str>>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/builder.rs#L191).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::snapshot::builder::SnapshotBuilder", "path": "SnapshotBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 1], "end": [433, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/builder.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/builder.rs:191`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Attach an opaque, caller-supplied correlation id for joining this build's metric events to
the caller's own request or operation id. An empty id is treated as unset. When unset,
behavior is unchanged.

<a id="op-161a42f9764eaf091a641b38"></a>
## with_incremental_crc_replay

`function` · `buoyant_kernel::snapshot::builder::SnapshotBuilder::with_incremental_crc_replay` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_incremental_crc_replay(self, mode: IncrementalReplay) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/builder.rs#L183).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::snapshot::builder::SnapshotBuilder", "path": "SnapshotBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 1], "end": [433, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/builder.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/builder.rs:183`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Bound how many commits kernel will replay to advance a stale CRC to the target version.
See [`IncrementalReplay`](../operations/buoyant_kernel.snapshot.builder.IncrementalReplay.md#op-b10a0852126775f2b57b627c). Defaults to [`IncrementalReplay::Disabled`](../operations/buoyant_kernel.snapshot.builder.IncrementalReplay.md#op-851d89740356352b3ce74f0c).

Writers should set this to [`IncrementalReplay::Unlimited`](../operations/buoyant_kernel.snapshot.builder.IncrementalReplay.md#op-a8d535877e4623c8a01a067b) for faster writes, as should
readers that always want table-level file statistics for query optimization.

Applies to both fresh and incremental builds.

<a id="op-1e6299a48d1cc2f2ad5e4e8f"></a>
## with_log_tail

`function` · `buoyant_kernel::snapshot::builder::SnapshotBuilder::with_log_tail` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_log_tail(self, log_tail: Vec<LogPath>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/builder.rs#L150).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::snapshot::builder::SnapshotBuilder", "path": "SnapshotBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 1], "end": [433, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/builder.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/builder.rs:150`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Set the log tail to use when building the snapshot. This allows catalogs or external
systems to provide an up-to-date log tail when used to build a snapshot.

Note that the log tail must be a contiguous sequence of commits from M..=N where N is the
target version of the snapshot and 0 <= M <= N.

See [`with_max_catalog_version`] for additional constraints when loading catalog-managed
tables.

[`with_max_catalog_version`]: Self::with_max_catalog_version

<a id="op-fd13c1efbe44e357114a0f00"></a>
## with_max_catalog_version

`function` · `buoyant_kernel::snapshot::builder::SnapshotBuilder::with_max_catalog_version` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_max_catalog_version(self, max_catalog_version: Version) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/builder.rs#L171).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::snapshot::builder::SnapshotBuilder", "path": "SnapshotBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 1], "end": [433, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/builder.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/builder.rs:171`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Set the maximum catalog-ratified version. When set, the snapshot will not load versions
beyond this limit, even if later commits exist on the filesystem. This ensures the catalog
remains the source of truth for catalog-managed tables.

When no explicit time-travel version is set via [`at_version`], `max_catalog_version` is
used as the effective target version. When time-travelling to an explicit version,
`max_catalog_version` must still be set for catalog-managed tables -- the requested version
must not exceed it.

# Log tail requirements

When `max_catalog_version` is set and no time-travel version is specified, the last entry in
the log tail must match `max_catalog_version` exactly. When time-travelling, the last log
tail entry must be >= the requested version.

[`at_version`]: Self::at_version

<a id="op-e0c76e3a57d7b55298fc6d5a"></a>
## correlation_id

`struct_field` · `buoyant_kernel::snapshot::builder::SnapshotBuilder::correlation_id` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
correlation_id: Option<std::sync::Arc<str>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/builder.rs#L51).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/builder.rs:51`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Opaque, caller-supplied id recorded on this build's metric events. Not interpreted by
kernel; set via [`with_correlation_id`](Self::with_correlation_id).

Unresolved upstream links (retained, not inferred): `Self::with_correlation_id`.

<a id="op-7a45e94420bbd58bd4f8eb6a"></a>
## existing_snapshot

`struct_field` · `buoyant_kernel::snapshot::builder::SnapshotBuilder::existing_snapshot` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
existing_snapshot: Option<snapshot::SnapshotRef>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/builder.rs#L42).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/builder.rs:42`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-262fe53bb8a8f9c5c38ca6d7"></a>
## incremental_replay

`struct_field` · `buoyant_kernel::snapshot::builder::SnapshotBuilder::incremental_replay` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
incremental_replay: IncrementalReplay
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/builder.rs#L46).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/builder.rs:46`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fa07564bb652b96e650f96f9"></a>
## log_tail

`struct_field` · `buoyant_kernel::snapshot::builder::SnapshotBuilder::log_tail` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
log_tail: Vec<log_path::LogPath>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/builder.rs#L44).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/builder.rs:44`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-775c7c90c91d63125bdf6a29"></a>
## max_catalog_version

`struct_field` · `buoyant_kernel::snapshot::builder::SnapshotBuilder::max_catalog_version` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
max_catalog_version: Option<Version>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/builder.rs#L45).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/builder.rs:45`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-39e249bd402e78a87c9e8ca0"></a>
## operation_id

`struct_field` · `buoyant_kernel::snapshot::builder::SnapshotBuilder::operation_id` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
operation_id: metrics::MetricId
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/builder.rs#L48).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/builder.rs:48`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Kernel-minted id correlating this build's metric events with its child events.

<a id="op-b83e03701d4fb9c91cef3b49"></a>
## table_root

`struct_field` · `buoyant_kernel::snapshot::builder::SnapshotBuilder::table_root` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
table_root: Option<String>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/builder.rs#L41).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/builder.rs:41`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3c24a09e55e7d51e6babd415"></a>
## version

`struct_field` · `buoyant_kernel::snapshot::builder::SnapshotBuilder::version` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
version: Option<Version>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/builder.rs#L43).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/builder.rs:43`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
