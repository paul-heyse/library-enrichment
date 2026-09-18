# `buoyant_kernel::snapshot::Snapshot`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.snapshot.Snapshot.json).

<a id="op-e760c13a7e6fcf7025cb3640"></a>
## Snapshot

`struct` · `buoyant_kernel::snapshot::Snapshot` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct Snapshot
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/mod.rs#L69).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs:69`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

In-memory representation of a specific snapshot of a Delta table. While a `DeltaTable` exists
throughout time, `Snapshot`s represent a view of a table at a specific point in time; they
have a defined schema (which may change over time for any given table), specific version, and
frozen log segment.

<a id="op-fbc43be0017476bed6fdd731"></a>
## alter_table

`function` · `buoyant_kernel::snapshot::Snapshot::alter_table` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn alter_table(Arc<self>) -> AlterTableTransactionBuilder
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/mod.rs#L810).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [1122, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs:810`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Creates a builder for altering this table's metadata. Currently supports schema change
operations.

The returned builder allows chaining operations before building an
[`AlterTableTransaction`] that can be committed.

[`AlterTableTransaction`]: crate::transaction::AlterTableTransaction

<a id="op-9d8bf478e7cb39020eb42a0c"></a>
## builder_for

`function` · `buoyant_kernel::snapshot::Snapshot::builder_for` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn builder_for(table_root: impl AsRef<str>) -> SnapshotBuilder
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/mod.rs#L113).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [1122, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs:113`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a new [`SnapshotBuilder`](../operations/buoyant_kernel.snapshot.builder.SnapshotBuilder.md#op-9d8aad6c7e18d68b3febe790) to build a new [`Snapshot`](../operations/buoyant_kernel.snapshot.Snapshot.md#op-e760c13a7e6fcf7025cb3640) for a given table root. If you
instead have an existing [`Snapshot`](../operations/buoyant_kernel.snapshot.Snapshot.md#op-e760c13a7e6fcf7025cb3640) you would like to do minimal work to update, consider
using [`Snapshot::builder_from`](../operations/buoyant_kernel.snapshot.Snapshot.md#op-1dc2069b376fa35067d5a0f6) instead.

<a id="op-1dc2069b376fa35067d5a0f6"></a>
## builder_from

`function` · `buoyant_kernel::snapshot::Snapshot::builder_from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn builder_from(existing_snapshot: SnapshotRef) -> SnapshotBuilder
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/mod.rs#L121).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [1122, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs:121`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a new [`SnapshotBuilder`](../operations/buoyant_kernel.snapshot.builder.SnapshotBuilder.md#op-9d8aad6c7e18d68b3febe790) to incrementally update an existing [`Snapshot`](../operations/buoyant_kernel.snapshot.Snapshot.md#op-e760c13a7e6fcf7025cb3640) to a
more recent version.

See `Snapshot::try_new_from` for the case-by-case behavior.

<a id="op-0ef8437dc52b7daa4e09715e"></a>
## checkpoint

`function` · `buoyant_kernel::snapshot::Snapshot::checkpoint` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn checkpoint(&SnapshotRef, engine: &dyn Engine, spec: Option<&CheckpointSpec>) -> DeltaResult<(CheckpointWriteResult, SnapshotRef)>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/mod.rs#L962).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [1122, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs:962`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Performs a complete checkpoint of this snapshot using the provided engine.

If a checkpoint already exists at this version, returns
[`CheckpointWriteResult::AlreadyExists`](../operations/buoyant_kernel.snapshot.CheckpointWriteResult.md#op-920bd3a4387f8652cacf7f21) with the original snapshot unchanged.
Otherwise, writes a checkpoint parquet file and the `_last_checkpoint` file and returns
[`CheckpointWriteResult::Written`](../operations/buoyant_kernel.snapshot.CheckpointWriteResult.md#op-7b6fcfc6aa95076ec9f1be7b) with an updated [`SnapshotRef`](../operations/buoyant_kernel.snapshot.SnapshotRef.md#op-7166b16a39037614007f3d0c) whose log segment
reflects the new checkpoint. Commits and compaction files subsumed by the checkpoint are
dropped from the returned snapshot.

# Parameters
- `engine`: Engine for data processing and I/O
- `spec`: Checkpoint format specification. `None` uses the default checkpoint settings
  (auto-detecting V1/V2 from table features). For V2 checkpoints, the default is to not
  write sidecar files.

# Errors
- If `CheckpointSpec::V2` is used but the table does not support the `v2Checkpoint` feature.
- If `CheckpointSpec::V1` is used but the table supports `v2Checkpoint` feature. Note: the
  Delta protocol permits writing V1 checkpoints to such tables; this is a kernel limitation.
- If `file_actions_per_sidecar_hint` is `Some(0)`.
- If the checkpoint write fails (e.g. I/O, parquet write). A `FileAlreadyExists` error is
  not propagated; it returns [`CheckpointWriteResult::AlreadyExists`](../operations/buoyant_kernel.snapshot.CheckpointWriteResult.md#op-920bd3a4387f8652cacf7f21) instead. Note: this
  also fires on the (unlikely) case of a sidecar UUID filename collision, where
  it should ideally surface as an error. Tracked in
  <https://github.com/delta-io/delta-kernel-rs/issues/2503>.

Note:
    - It is still possible that an existing checkpoint gets overwritten if that checkpoint
      was written by a concurrent writer.
    - This function uses [`crate::ParquetHandler::write_parquet_file`](../operations/buoyant_kernel.ParquetHandler.md#op-2219be84eb11bd25e10fde13) and
      [`crate::StorageHandler::head`](../operations/buoyant_kernel.StorageHandler.md#op-f3841fae6720ff56eefbd5b5), which may not be implemented by all engines. If you
      are using the default engine, make sure to build it with the multi-threaded executor
      if you want to use this method.

[`CheckpointSpec`]: crate::checkpoint::CheckpointSpec

Note: There is currently no public api for callers to determine whether a table supports V2
checkpoints directly. Tracked in <https://github.com/delta-io/delta-kernel-rs/issues/2450>.

<a id="op-7f51234d90d151cb90f847df"></a>
## crc

`function` · `buoyant_kernel::snapshot::Snapshot::crc` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn crc(&self) -> Option<&Arc<Crc>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/mod.rs#L291).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [1122, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs:291`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns the CRC for this snapshot, if one is resolved.

When `Some(crc)`, `crc.version == self.version()` and queries backed by the CRC hit
cache at zero I/O.

<a id="op-096e4c0dfe996afe80e5a73f"></a>
## create_checkpoint_writer

`function` · `buoyant_kernel::snapshot::Snapshot::create_checkpoint_writer` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn create_checkpoint_writer(Arc<self>, engine: &dyn Engine) -> DeltaResult<CheckpointWriter>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/mod.rs#L818).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [1122, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs:818`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Creates a [`CheckpointWriter`](../operations/buoyant_kernel.checkpoint.CheckpointWriter.md#op-a939c219c148fee02b6e7899) for generating a checkpoint from this snapshot.

See the [`crate::checkpoint`](../modules/buoyant_kernel.checkpoint.md#op-00431814b37f77a78d019358) module documentation for more details on checkpoint types
and the overall checkpoint process.

<a id="op-6d3caf7390378a4fd8430e86"></a>
## drop

`function` · `buoyant_kernel::snapshot::Snapshot::drop` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn drop(&mut self)
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/mod.rs#L89).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 1], "end": [92, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs"}, "trait": {"args": null, "id": "core::ops::drop::Drop", "path": "Drop"}, "trait_path": "core::ops::drop::Drop"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs:89`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6d6c46d1c5939a2d43561dc1"></a>
## eq

`function` · `buoyant_kernel::snapshot::Snapshot::eq` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &Self) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/mod.rs#L80).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [84, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs:80`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ff21215c172e8c8d2438f9ba"></a>
## estimated_owned_heap_size_bytes

`function` · `buoyant_kernel::snapshot::Snapshot::estimated_owned_heap_size_bytes` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn estimated_owned_heap_size_bytes(&self) -> usize
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/mod.rs#L332).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [1122, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs:332`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Estimated owned heap size in bytes for this snapshot. Best-effort estimate
for capacity tracking, not authoritative.

Counts only the dominant per-snapshot heap contributors, normally > 70% of the snapshot's
owned heap size:
- For every listed log path (commit, compaction, checkpoint, latest CRC, latest commit): the
  filename / extension / Url string heap.
- Vec buffer capacity (`capacity * size_of::<ParsedLogPath>()`) for the three Vec fields on
  `LogSegmentFiles`.
- The log root Url string.
- The raw `schemaString` JSON on table metadata.

The Arc-shared variables (e.g. logical/physical schemas, `crc`) are not counted,
as they can be shared between multiple snapshots and are not owned by a single snapshot.

Other variables' contributions to heap size are relatively small, so they are not counted
here.

Runs in O(n) over listed log files.

TODO(#2757): Optimize the estimation accuracy.

<a id="op-b7447b12901b8b4cb78a1011"></a>
## fmt

`function` · `buoyant_kernel::snapshot::Snapshot::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/mod.rs#L95).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 1], "end": [103, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs:95`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-948421198fbe5e6931bb69e0"></a>
## get_all_domain_metadata

`function` · `buoyant_kernel::snapshot::Snapshot::get_all_domain_metadata` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_all_domain_metadata(&self, engine: &dyn Engine) -> DeltaResult<Vec<DomainMetadata>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/mod.rs#L645).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [1122, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs:645`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Fetch all non-internal domain metadata for this snapshot as a `Vec`.

Internal (`delta.*`) domains are filtered out.

<a id="op-4dccbdcd452d0bb20878bd1f"></a>
## get_app_id_version

`function` · `buoyant_kernel::snapshot::Snapshot::get_app_id_version` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_app_id_version(&self, application_id: &str, engine: &dyn Engine) -> DeltaResult<Option<i64>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/mod.rs#L415).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [1122, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs:415`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Fetch the latest version of the provided `application_id` for this snapshot. Filters the
txn based on the delta.setTransactionRetentionDuration property and lastUpdated.

Uses the CRC fast path when available, otherwise falls back to log replay.

Reports metrics: `SetTransactionLoadSuccess` or `SetTransactionLoadFailure`.

<a id="op-643fd9f2f66c97f48bc009df"></a>
## get_domain_metadata

`function` · `buoyant_kernel::snapshot::Snapshot::get_domain_metadata` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_domain_metadata(&self, domain: &str, engine: &dyn Engine) -> DeltaResult<Option<String>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/mod.rs#L468).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [1122, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs:468`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Fetch the domainMetadata for a specific domain in this snapshot. This returns the latest
configuration for the domain, or None if the domain does not exist.

Note that this method performs log replay (fetches and processes metadata from storage).

<a id="op-adee1260c53222f6ccf89d7c"></a>
## get_domain_metadata_internal

`function` · `buoyant_kernel::snapshot::Snapshot::get_domain_metadata_internal` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_domain_metadata_internal(&self, domain: &str, engine: &dyn Engine) -> DeltaResult<Option<String>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/mod.rs#L631).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [1122, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs:631`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Fetch both user-controlled and system-controlled domain metadata for a specific domain
in this snapshot.

Returns the latest configuration for the domain, or `None` if the domain does not exist
(or was removed). Unlike [`Snapshot::get_domain_metadata`](../operations/buoyant_kernel.snapshot.Snapshot.md#op-643fd9f2f66c97f48bc009df), this does not reject `delta.*`
domains.

<a id="op-ec3b5c09e386b489eeba86ee"></a>
## get_domain_metadatas_internal

`function` · `buoyant_kernel::snapshot::Snapshot::get_domain_metadatas_internal` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_domain_metadatas_internal(&self, engine: &dyn Engine, domains: Option<&HashSet<&str>>) -> DeltaResult<std::collections::HashMap<String, actions::DomainMetadata>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/mod.rs#L565).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [1122, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs:565`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Load domain metadata: if Complete in the CRC, answer from the cache; else if every
requested domain is in a Partial cache, also answer from the cache; else full log
replay. `domains == None` means load all.

Reports metrics: `DomainMetadataLoadSuccess` or `DomainMetadataLoadFailure`.

<a id="op-c7615cee30a6d0ef38789a45"></a>
## get_file_stats_if_present

`function` · `buoyant_kernel::snapshot::Snapshot::get_file_stats_if_present` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_file_stats_if_present(&self) -> Option<FileStats>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/mod.rs#L658).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [1122, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs:658`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns file-level statistics, or `None` if this snapshot has no CRC, or its CRC does
not have `Complete` file stats. Performs no I/O (the CRC is resolved at construction).

<a id="op-b270cf72517ec37444e36dd6"></a>
## get_in_commit_timestamp

`function` · `buoyant_kernel::snapshot::Snapshot::get_in_commit_timestamp` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_in_commit_timestamp(&self, engine: &dyn Engine) -> DeltaResult<Option<i64>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/mod.rs#L673).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [1122, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs:673`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Get the In-Commit Timestamp (ICT) for this snapshot.

Returns the `inCommitTimestamp` from the CommitInfo action of the commit that created this
snapshot.

# Returns
- `Ok(Some(timestamp))` - ICT is enabled and available for this version
- `Ok(None)` - ICT is not enabled
- `Err(...)` - ICT is enabled but cannot be read, or enablement version is invalid

<a id="op-1bc1473f30478066bba21438"></a>
## get_logical_clustering_columns

`function` · `buoyant_kernel::snapshot::Snapshot::get_logical_clustering_columns` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_logical_clustering_columns(&self, engine: &dyn Engine) -> DeltaResult<Option<Vec<ColumnName>>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/mod.rs#L502).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [1122, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs:502`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Get the logical clustering columns for this snapshot, if clustering is enabled.

Returns `Ok(Some(columns))` if the ClusteredTable feature is enabled and clustering
columns are defined, `Ok(None)` if clustering is not enabled, or an error if the
clustering metadata is malformed.

The columns are returned as logical [`ColumnName`]s. When column mapping is enabled,
this converts the physical names stored in domain metadata back to logical names using
the table schema.

Note that this method performs log replay (fetches and processes metadata from storage).

# Errors

Returns an error if the clustering domain metadata is malformed, or if a physical
column name cannot be resolved to a logical name in the schema.

[`ColumnName`]: crate::expressions::ColumnName

<a id="op-82e3f44a2e723c978f9c6a73"></a>
## get_physical_clustering_columns

`function` · `buoyant_kernel::snapshot::Snapshot::get_physical_clustering_columns` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_physical_clustering_columns(&self, engine: &dyn Engine) -> DeltaResult<Option<Vec<ColumnName>>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/mod.rs#L535).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [1122, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs:535`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Get the clustering columns for this snapshot, if the table has clustering enabled.

Returns `Ok(Some(columns))` if the ClusteredTable feature is enabled and clustering
columns are defined, `Ok(None)` if clustering is not enabled, or an error if the
clustering metadata is malformed.

The columns are returned as physical column names, respecting the column mapping mode.
Note that this method performs log replay (fetches and processes metadata from storage).

<a id="op-b15b46a500caa683b68e57bd"></a>
## get_protocol_derived_properties

`function` · `buoyant_kernel::snapshot::Snapshot::get_protocol_derived_properties` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_protocol_derived_properties(&self) -> HashMap<String, String>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/mod.rs#L355).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [1122, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs:355`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns the protocol-derived table properties as a map of key-value pairs.

This includes:
- `delta.minReaderVersion` and `delta.minWriterVersion`
- `delta.feature.<name> = "supported"` for each reader and writer feature (when using table
  features protocol, i.e. reader version 3 / writer version 7)

<a id="op-b7e3f2d322743229daaec66b"></a>
## get_timestamp

`function` · `buoyant_kernel::snapshot::Snapshot::get_timestamp` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_timestamp(&self, engine: &dyn Engine) -> DeltaResult<i64>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/mod.rs#L735).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [1122, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs:735`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Get the timestamp for this snapshot's version, in milliseconds since the Unix epoch.

When In-Commit Timestamp (ICT) are enabled, returns the In-Commit Timestamp value.
Otherwise, falls back to the filesystem last-modified time of the latest commit file.

Returns an error if the commit file is missing, the ICT configuration is invalid, or the
ICT value cannot be read.

See also [`get_in_commit_timestamp`] for ICT-only semantics.

[`get_in_commit_timestamp`]: Self::get_in_commit_timestamp

<a id="op-7190e7670c8e946e3dcb5592"></a>
## incremental_scan_builder

`function` · `buoyant_kernel::snapshot::Snapshot::incremental_scan_builder` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn incremental_scan_builder(Arc<self>, base_version: Version) -> IncrementalScanBuilder
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/mod.rs#L784).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [1122, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs:784`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create an [`IncrementalScanBuilder`](../operations/buoyant_kernel.incremental_scan.IncrementalScanBuilder.md#op-9d4dcdc70da5cc4e6963af53) for the range `(base_version, self.version()]`.

Use this to advance a cached file listing from `base_version` to this snapshot's
version without doing a full scan. See [`IncrementalScanBuilder`](../operations/buoyant_kernel.incremental_scan.IncrementalScanBuilder.md#op-9d4dcdc70da5cc4e6963af53) for details.

<a id="op-2c24c0ffe8bb8be3f6eeeac6"></a>
## log_compaction_writer

`function` · `buoyant_kernel::snapshot::Snapshot::log_compaction_writer` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn log_compaction_writer(Arc<self>, start_version: Version, end_version: Version) -> DeltaResult<LogCompactionWriter>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/mod.rs#L838).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [1122, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs:838`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Creates a [`LogCompactionWriter`](../operations/buoyant_kernel.log_compaction.writer.LogCompactionWriter.md#op-17a0145eb5a991e6c3d78695) for generating a log compaction file.

Log compaction aggregates commit files in a version range into a single compacted file,
improving performance by reducing the number of files to process during log replay.

# Parameters
- `start_version`: The first version to include in the compaction (inclusive)
- `end_version`: The last version to include in the compaction (inclusive)

# Returns
A [`LogCompactionWriter`](../operations/buoyant_kernel.log_compaction.writer.LogCompactionWriter.md#op-17a0145eb5a991e6c3d78695) that can be used to generate the compaction file.

NOTE: This method is currently a no-op because log compaction is disabled (#2337)

<a id="op-bd7c5454553d2668d7070d5f"></a>
## log_segment

`function` · `buoyant_kernel::snapshot::Snapshot::log_segment` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn log_segment(&self) -> &LogSegment
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/mod.rs#L282).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [1122, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs:282`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Log segment this snapshot uses

<a id="op-fc902a87183444d4316cf0af"></a>
## metadata_configuration

`function` · `buoyant_kernel::snapshot::Snapshot::metadata_configuration` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn metadata_configuration(&self) -> &HashMap<String, String>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/mod.rs#L391).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [1122, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs:391`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Get the raw metadata configuration for this table.

This returns the `Metadata.configuration` map as stored in the Delta log, containing
user-defined properties, delta table properties (e.g., `delta.enableInCommitTimestamps`),
and application-specific properties (e.g., `io.unitycatalog.tableId`).

<a id="op-fc353aa8b255a24d0fbc6c55"></a>
## new

`function` · `buoyant_kernel::snapshot::Snapshot::new` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new(log_segment: LogSegment, table_configuration: TableConfiguration) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/mod.rs#L144).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [1122, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs:144`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a new [`Snapshot`](../operations/buoyant_kernel.snapshot.Snapshot.md#op-e760c13a7e6fcf7025cb3640) from a [`LogSegment`](../operations/buoyant_kernel.log_segment.LogSegment.md#op-e8c8c6521fbbb11defc515d2) and [`TableConfiguration`](../operations/buoyant_kernel.table_configuration.TableConfiguration.md#op-4c1bb6e90ea642d0fb312bae).

This **can panic**, to catch an error use try_new()

<a id="op-d6cd33cc68b07ae6be7efc93"></a>
## publish

`function` · `buoyant_kernel::snapshot::Snapshot::publish` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn publish(&SnapshotRef, engine: &dyn Engine, committer: &dyn Committer) -> DeltaResult<SnapshotRef>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/mod.rs#L1076).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [1122, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs:1076`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Publishes all catalog commits at this table version. Applicable only to catalog-managed
tables. This method is a no-op for filesystem-managed tables or if there are no catalog
commits to publish.

Publishing copies ratified catalog commits to the Delta log as published Delta files,
reducing catalog storage requirements and enabling some table maintenance operations,
like checkpointing.

# Parameters

- `engine`: The engine to use for publishing commits

# Errors

Returns an error if the publish operation fails, or if there are catalog commits that need
publishing but the table or committer don't support publishing.

# See Also

- [`Committer::publish`](../operations/buoyant_kernel.committer.Committer.md#op-280409d4aa4ba60251bd8819)

<a id="op-c9c6b7e23f7231ad4b5e40d8"></a>
## scan_builder

`function` · `buoyant_kernel::snapshot::Snapshot::scan_builder` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn scan_builder(Arc<self>) -> ScanBuilder
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/mod.rs#L776).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [1122, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs:776`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a [`ScanBuilder`](../operations/buoyant_kernel.scan.ScanBuilder.md#op-bca3273cb6882c664105ba29) for an `SnapshotRef`.

<a id="op-8b4c8520f7b58b13c0561636"></a>
## schema

`function` · `buoyant_kernel::snapshot::Snapshot::schema` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn schema(&self) -> SchemaRef
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/mod.rs#L307).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [1122, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs:307`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Table [`Schema`] at this `Snapshot`s version.

[`Schema`]: crate::schema::Schema

<a id="op-d704596aae63e2df719ab530"></a>
## table_configuration

`function` · `buoyant_kernel::snapshot::Snapshot::table_configuration` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn table_configuration(&self) -> &TableConfiguration
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/mod.rs#L397).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [1122, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs:397`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Get the [`TableConfiguration`](../operations/buoyant_kernel.table_configuration.TableConfiguration.md#op-4c1bb6e90ea642d0fb312bae) for this [`Snapshot`](../operations/buoyant_kernel.snapshot.Snapshot.md#op-e760c13a7e6fcf7025cb3640).

<a id="op-52a2e321688bd63594a2d191"></a>
## table_properties

`function` · `buoyant_kernel::snapshot::Snapshot::table_properties` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn table_properties(&self) -> &TableProperties
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/mod.rs#L343).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [1122, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs:343`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Get the [`TableProperties`](../operations/buoyant_kernel.table_properties.TableProperties.md#op-a6b333464916b5b04ef86dcd) for this [`Snapshot`](../operations/buoyant_kernel.snapshot.Snapshot.md#op-e760c13a7e6fcf7025cb3640).

<a id="op-bef687c0bb9809cc4ddf1272"></a>
## table_root

`function` · `buoyant_kernel::snapshot::Snapshot::table_root` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn table_root(&self) -> &Url
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/mod.rs#L295).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [1122, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs:295`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4ede2b5b88fe41bf214ef1d9"></a>
## transaction

`function` · `buoyant_kernel::snapshot::Snapshot::transaction` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn transaction(Arc<self>, committer: Box<dyn Committer>, engine: &dyn Engine) -> DeltaResult<Transaction>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/mod.rs#L795).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [1122, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs:795`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a [`Transaction`](../operations/buoyant_kernel.transaction.Transaction.md#op-fde417829ac3f4c320319f35) for this `SnapshotRef`. With the specified [`Committer`](../operations/buoyant_kernel.committer.Committer.md#op-32f11d6fd06afd07c4c2a455).

Note: For tables with clustering enabled, this performs log replay to read clustering
columns from domain metadata, which may have a performance cost.

<a id="op-89b98f2482dadabe7b80855d"></a>
## try_new

`function` · `buoyant_kernel::snapshot::Snapshot::try_new` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_new(log_segment: LogSegment, table_configuration: TableConfiguration) -> DeltaResult<Self>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/mod.rs#L132).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [1122, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs:132`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a new [`Snapshot`](../operations/buoyant_kernel.snapshot.Snapshot.md#op-e760c13a7e6fcf7025cb3640) from a [`LogSegment`](../operations/buoyant_kernel.log_segment.LogSegment.md#op-e8c8c6521fbbb11defc515d2) and [`TableConfiguration`](../operations/buoyant_kernel.table_configuration.TableConfiguration.md#op-4c1bb6e90ea642d0fb312bae).

<a id="op-01893154c1a8e52343624023"></a>
## version

`function` · `buoyant_kernel::snapshot::Snapshot::version` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn version(&self) -> Version
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/mod.rs#L300).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [1122, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs:300`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Version of this `Snapshot` in the table.

<a id="op-2bd0ee26dc0226ed458f8934"></a>
## write_checksum

`function` · `buoyant_kernel::snapshot::Snapshot::write_checksum` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn write_checksum(&SnapshotRef, engine: &dyn Engine) -> DeltaResult<(ChecksumWriteResult, SnapshotRef)>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/mod.rs#L874).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [1122, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs:874`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Writes a version checksum (CRC) file for this snapshot. Writers should call this after
every commit because checksums enable faster snapshot loading and table state validation.

Currently only supports writing from a post-commit snapshot that has pre-computed CRC
information in memory (i.e. the snapshot returned by
[`CommittedTransaction::post_commit_snapshot`]).

Returns a tuple of [`ChecksumWriteResult`](../operations/buoyant_kernel.snapshot.ChecksumWriteResult.md#op-a840d53ca7bc5e28f0c0d108) and a [`SnapshotRef`](../operations/buoyant_kernel.snapshot.SnapshotRef.md#op-7166b16a39037614007f3d0c). On
[`ChecksumWriteResult::Written`](../operations/buoyant_kernel.snapshot.ChecksumWriteResult.md#op-bd972c2d7b172900b2c431a1), the returned snapshot has the CRC file recorded in
its log segment. On [`ChecksumWriteResult::AlreadyExists`](../operations/buoyant_kernel.snapshot.ChecksumWriteResult.md#op-e9be56cc16083403d709fbc4), the original snapshot is
returned unchanged.

# Errors

- [`Error::ChecksumWriteUnsupported`](../operations/buoyant_kernel.error.Error.md#op-5c832660ea706887c1e606db) if no in-memory CRC is available at this snapshot's
  version (e.g. a snapshot loaded from disk that has no CRC file), if the CRC's
  `file_stats_state` is `Indeterminate` (a non-incremental operation like ANALYZE STATS was
  encountered, or a file action had a missing size; recoverable with a full state
  reconstruction in the future), or if `delta.enableInCommitTimestamps` is `true` but
  `inCommitTimestampOpt` is absent.
- I/O errors from the engine's storage handler if the write fails.

[`CommittedTransaction::post_commit_snapshot`]: crate::transaction::CommittedTransaction::post_commit_snapshot

<a id="op-c0038da2ba58b7a88e915d38"></a>
## crc

`struct_field` · `buoyant_kernel::snapshot::Snapshot::crc` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
crc: Option<std::sync::Arc<crc::Crc>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/mod.rs#L76).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs:76`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

CRC at this snapshot's version, eagerly resolved at construction time. `Some(crc)`
means `crc.version == self.version()` and the CRC can be queried at zero I/O. `None`
means no CRC was loadable (no CRC on disk at this version, or the read failed).

<a id="op-adcd6f17072c83ec1f58d732"></a>
## log_segment

`struct_field` · `buoyant_kernel::snapshot::Snapshot::log_segment` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
log_segment: log_segment::LogSegment
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/mod.rs#L71).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs:71`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b8d5cdb909b75eb59267fab5"></a>
## span

`struct_field` · `buoyant_kernel::snapshot::Snapshot::span` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
span: tracing::Span
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/mod.rs#L70).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs:70`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2cb3dae9ed0ef7c2dcf22331"></a>
## table_configuration

`struct_field` · `buoyant_kernel::snapshot::Snapshot::table_configuration` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
table_configuration: table_configuration::TableConfiguration
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/mod.rs#L72).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs:72`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
