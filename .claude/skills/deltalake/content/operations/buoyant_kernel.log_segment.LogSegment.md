# `buoyant_kernel::log_segment::LogSegment`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.log_segment.LogSegment.json).

<a id="op-e8c8c6521fbbb11defc515d2"></a>
## LogSegment

`struct` · `buoyant_kernel::log_segment::LogSegment` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct LogSegment
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_segment/mod.rs#L108).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment/mod.rs:108`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A [`LogSegment`](../operations/buoyant_kernel.log_segment.LogSegment.md#op-e8c8c6521fbbb11defc515d2) represents a contiguous section of the log and is made of checkpoint files
and commit files and guarantees the following:
    1. Commit file versions will not have any gaps between them.
    2. If checkpoint(s) is/are present in the range, only commits with versions greater than the
       most recent checkpoint version are retained. There will not be a gap between the
       checkpoint version and the first commit version.
    3. All checkpoint_parts must belong to the same checkpoint version, and must form a complete
       version. Multi-part checkpoints must have all their parts.

[`LogSegment`](../operations/buoyant_kernel.log_segment.LogSegment.md#op-e8c8c6521fbbb11defc515d2) is used in [`Snapshot`] when built with [`LogSegment::for_snapshot`](../operations/buoyant_kernel.log_segment.LogSegment.md#op-71dbb4a07bdc838ea31637d3), and
in `TableChanges` when built with [`LogSegment::for_table_changes`](../operations/buoyant_kernel.log_segment.LogSegment.md#op-e67c5ede3272c6c5c4f42218).

[`Snapshot`]: crate::snapshot::Snapshot

<a id="op-3cada69f813acb8aa1cc28c3"></a>
## checkpoint_version

`struct_field` · `buoyant_kernel::log_segment::LogSegment::checkpoint_version` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
checkpoint_version: Option<Version>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_segment/mod.rs#L110).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment/mod.rs:110`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d95a2e8296f4ce58692468bc"></a>
## clone

`function` · `buoyant_kernel::log_segment::LogSegment::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> LogSegment
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_segment/mod.rs#L106).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::log_segment::LogSegment", "path": "LogSegment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [106, 17], "end": [106, 22], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment/mod.rs:106`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7a349d2a9eaefe8c4716a4b1"></a>
## end_version

`struct_field` · `buoyant_kernel::log_segment::LogSegment::end_version` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
end_version: Version
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_segment/mod.rs#L109).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment/mod.rs:109`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6054e7d7f6a5fa4353681598"></a>
## eq

`function` · `buoyant_kernel::log_segment::LogSegment::eq` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &LogSegment) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_segment/mod.rs#L106).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::log_segment::LogSegment", "path": "LogSegment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [106, 24], "end": [106, 33], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment/mod.rs:106`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4f67f5caa2401781cec12a2c"></a>
## fmt

`function` · `buoyant_kernel::log_segment::LogSegment::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_segment/mod.rs#L106).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::log_segment::LogSegment", "path": "LogSegment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [106, 10], "end": [106, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment/mod.rs:106`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-71dbb4a07bdc838ea31637d3"></a>
## for_snapshot

`function` · `buoyant_kernel::log_segment::LogSegment::for_snapshot` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn for_snapshot(storage: &dyn StorageHandler, log_root: Url, log_tail: Vec<ParsedLogPath>, time_travel_version: impl Into<Option<Version>>, metric_context: SnapshotLoadMetricContext) -> DeltaResult<Self>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_segment/mod.rs#L330).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::log_segment::LogSegment", "path": "LogSegment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [155, 1], "end": [1383, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment/mod.rs:330`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Constructs a [`LogSegment`](../operations/buoyant_kernel.log_segment.LogSegment.md#op-e8c8c6521fbbb11defc515d2) to be used for [`Snapshot`]. For a `Snapshot` at version `n`:
Its LogSegment is made of zero or one checkpoint, and all commits between the checkpoint up
to and including the end version `n`. Note that a checkpoint may be made of multiple
parts. All these parts will have the same checkpoint version.

The options for constructing a LogSegment for Snapshot are as follows:
- `checkpoint_hint`: a `LastCheckpointHint` to start the log segment from (e.g. from reading
  the `last_checkpoint` file).
- `time_travel_version`: The version of the log that the Snapshot will be at.

[`Snapshot`]: crate::snapshot::Snapshot

Reports metrics: `LogSegmentLoadSuccess` or `LogSegmentLoadFailure`.

<a id="op-e67c5ede3272c6c5c4f42218"></a>
## for_table_changes

`function` · `buoyant_kernel::log_segment::LogSegment::for_table_changes` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn for_table_changes(storage: &dyn StorageHandler, log_root: Url, start_version: Version, end_version: impl Into<Option<Version>>) -> DeltaResult<Self>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_segment/mod.rs#L427).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::log_segment::LogSegment", "path": "LogSegment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [155, 1], "end": [1383, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment/mod.rs:427`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Constructs a [`LogSegment`](../operations/buoyant_kernel.log_segment.LogSegment.md#op-e8c8c6521fbbb11defc515d2) to be used for `TableChanges`. For a TableChanges between
versions `start_version` and `end_version`: Its LogSegment is made of zero checkpoints
and all commits between versions `start_version` (inclusive) and `end_version`
(inclusive). If no `end_version` is specified it will be the most recent version by
default.

<a id="op-0564175dae64fb1cb83b18f2"></a>
## listed

`struct_field` · `buoyant_kernel::log_segment::LogSegment::listed` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
listed: LogSegmentFiles
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_segment/mod.rs#L113).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment/mod.rs:113`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The set of log files found during listing.

<a id="op-1833b313d82690788cc1b58c"></a>
## log_root

`struct_field` · `buoyant_kernel::log_segment::LogSegment::log_root` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
log_root: url::Url
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_segment/mod.rs#L111).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment/mod.rs:111`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e143e0748a42bf233b452f48"></a>
## read_actions

`function` · `buoyant_kernel::log_segment::LogSegment::read_actions` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn read_actions(&self, engine: &dyn Engine, action_schema: SchemaRef) -> DeltaResult<impl Iterator<Item = DeltaResult<ActionsBatch>> + Send>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_segment/mod.rs#L742).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::log_segment::LogSegment", "path": "LogSegment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [155, 1], "end": [1383, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment/mod.rs:742`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Same as [`Self::read_actions_with_projected_checkpoint_actions`](../operations/buoyant_kernel.log_segment.LogSegment.md#op-b0e84d7fd1d5fe348871f834), but uses the same schema
for reading checkpoints and commits. IS NOT NULL predicates are automatically derived from
the schema, so callers do not need to supply them.

<a id="op-b0e84d7fd1d5fe348871f834"></a>
## read_actions_with_projected_checkpoint_actions

`function` · `buoyant_kernel::log_segment::LogSegment::read_actions_with_projected_checkpoint_actions` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn read_actions_with_projected_checkpoint_actions(&self, engine: &dyn Engine, commit_read_schema: SchemaRef, checkpoint_read_schema: SchemaRef, meta_predicate: Option<PredicateRef>, stats_schema: Option<&StructType>, partition_schema: Option<&StructType>) -> DeltaResult<ActionsWithCheckpointInfo<impl Iterator<Item = DeltaResult<ActionsBatch>> + Send>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_segment/mod.rs#L700).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::log_segment::LogSegment", "path": "LogSegment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [155, 1], "end": [1383, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment/mod.rs:700`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Read a stream of actions from this log segment. This returns an iterator of
[`ActionsBatch`](../operations/buoyant_kernel.log_replay.ActionsBatch.md#op-72ca0249e147b15cf59dc008)s which includes EngineData of actions + a boolean flag indicating whether
the data was read from a commit file (true) or a checkpoint file (false).

The log files will be read from most recent to oldest.

`commit_read_schema` is the (physical) schema to read the commit files with, and
`checkpoint_read_schema` is the (physical) schema to read checkpoint files with. This can be
used to project the log files to a subset of the columns. Having two different
schemas can be useful as a cheap way of doing additional filtering on the checkpoint files
(e.g. filtering out remove actions).

 The engine data returned might have extra non-log actions (e.g. sidecar
 actions) that are not part of the schema but this is an implementation
 detail that should not be relied on and will likely change.

Read a stream of actions from this log segment. This returns an iterator of
[`ActionsBatch`](../operations/buoyant_kernel.log_replay.ActionsBatch.md#op-72ca0249e147b15cf59dc008)s which includes EngineData of actions + a boolean flag indicating whether
the data was read from a commit file (true) or a checkpoint file (false).

Also returns `CheckpointReadInfo` with stats_parsed compatibility and the checkpoint schema.

`meta_predicate` is an optional expression for row group skipping in checkpoint parquet
files. It is _NOT_ the query's data predicate, but a hint for skipping irrelevant data.
IS NOT NULL predicates are automatically derived from `checkpoint_read_schema` and combined
(AND) with `meta_predicate`, so callers only need to supply query-based skipping predicates.

<a id="op-e60d11e3f9aa884a2c2354d1"></a>
## try_new

`function` · `buoyant_kernel::log_segment::LogSegment::try_new` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_new(listed_files: LogSegmentFiles, log_root: Url, end_version: Option<Version>, last_checkpoint_metadata: Option<LastCheckpointHint>) -> DeltaResult<Self>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_segment/mod.rs#L198).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::log_segment::LogSegment", "path": "LogSegment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [155, 1], "end": [1383, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment/mod.rs:198`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b387c782bd910055e45e6b91"></a>
## last_checkpoint_metadata

`struct_field` · `buoyant_kernel::log_segment::LogSegment::last_checkpoint_metadata` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
last_checkpoint_metadata: Option<last_checkpoint_hint::LastCheckpointHint>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_segment/mod.rs#L120).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment/mod.rs:120`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The retained `_last_checkpoint` hint, if one was read when this segment was built. The hint
may describe a different checkpoint than the one this segment selected, so for validated
access use [`Self::checkpoint_hint`] (and the [`Self::checkpoint_schema`] /
[`Self::checkpoint_sidecars`] accessors built on it). Read this field directly only when
the raw hint is wanted as-is -- e.g. re-threading it into a derived segment.

Unresolved upstream links (retained, not inferred): ``Self::checkpoint_schema``, ``Self::checkpoint_sidecars``, ``Self::checkpoint_hint``.
