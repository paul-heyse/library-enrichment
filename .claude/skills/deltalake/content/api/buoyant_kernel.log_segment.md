# `buoyant_kernel::log_segment`

Crate `buoyant_kernel` · 3 public items · structured records in [`model/buoyant_kernel.log_segment.json`](../model/buoyant_kernel.log_segment.json)

## ActionsWithCheckpointInfo

`struct` · `buoyant_kernel::log_segment::ActionsWithCheckpointInfo`

Also reachable as `delta_kernel::log_segment::ActionsWithCheckpointInfo`

```rust
struct ActionsWithCheckpointInfo<A: Iterator<Item = DeltaResult<log_replay::ActionsBatch>>>
```

**Fields**: `actions`, `checkpoint_info`

Result of reading actions from a log segment, containing both the actions iterator
and checkpoint metadata.

This struct provides named access to the return values instead of tuple indexing.

---

## CheckpointReadInfo

`struct` · `buoyant_kernel::log_segment::CheckpointReadInfo`

Also reachable as `delta_kernel::log_segment::CheckpointReadInfo`

```rust
struct CheckpointReadInfo
```

**Fields**: `has_stats_parsed`, `has_partition_values_parsed`, `checkpoint_read_schema`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Information about checkpoint reading for data skipping optimization.

Returned alongside the actions iterator from checkpoint reading functions.

---

## LogSegment

`struct` · `buoyant_kernel::log_segment::LogSegment`

Also reachable as `delta_kernel::log_segment::LogSegment`

```rust
struct LogSegment
```

**Fields**: `end_version`, `checkpoint_version`, `log_root`, `listed`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (5)

```rust
fn for_snapshot(storage: &dyn StorageHandler, log_root: Url, log_tail: Vec<ParsedLogPath>, time_travel_version: impl Into<Option<Version>>, metric_context: SnapshotLoadMetricContext) -> DeltaResult<Self>
fn for_table_changes(storage: &dyn StorageHandler, log_root: Url, start_version: Version, end_version: impl Into<Option<Version>>) -> DeltaResult<Self>
fn read_actions(&self, engine: &dyn Engine, action_schema: SchemaRef) -> DeltaResult<impl Iterator<Item = DeltaResult<ActionsBatch>> + Send>
fn read_actions_with_projected_checkpoint_actions(&self, engine: &dyn Engine, commit_read_schema: SchemaRef, checkpoint_read_schema: SchemaRef, meta_predicate: Option<PredicateRef>, stats_schema: Option<&StructType>, partition_schema: Option<&StructType>) -> DeltaResult<ActionsWithCheckpointInfo<impl Iterator<Item = DeltaResult<ActionsBatch>> + Send>>
fn try_new(listed_files: LogSegmentFiles, log_root: Url, end_version: Option<Version>, last_checkpoint_metadata: Option<LastCheckpointHint>) -> DeltaResult<Self>
```

A [`LogSegment`] represents a contiguous section of the log and is made of checkpoint files
and commit files and guarantees the following:
    1. Commit file versions will not have any gaps between them.
    2. If checkpoint(s) is/are present in the range, only commits with versions greater than the
       most recent checkpoint version are retained. There will not be a gap between the
       checkpoint version and the first commit version.
    3. All checkpoint_parts must belong to the same checkpoint version, and must form a complete
       version. Multi-part checkpoints must have all their parts.

[`LogSegment`] is used in [`Snapshot`] when built with [`LogSegment::for_snapshot`], and
in `TableChanges` when built with [`LogSegment::for_table_changes`].

[`Snapshot`]: crate::snapshot::Snapshot

---
