# `buoyant_kernel::scan::log_replay::ScanLogReplayProcessor`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.scan.log_replay.ScanLogReplayProcessor.json).

<a id="op-00b81acb4a4714f3ad69b815"></a>
## ScanLogReplayProcessor

`struct` · `buoyant_kernel::scan::log_replay::ScanLogReplayProcessor` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct ScanLogReplayProcessor
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/log_replay.rs#L154).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/log_replay.rs:154`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

[`ScanLogReplayProcessor`](../operations/buoyant_kernel.scan.log_replay.ScanLogReplayProcessor.md#op-00b81acb4a4714f3ad69b815) performs log replay (processes actions) specifically for doing a
table scan.

During a table scan, the processor reads batches of log actions (in reverse chronological order)
and performs the following steps:

- Data Skipping: Applies a predicate-based filter (via [`DataSkippingFilter`](../operations/buoyant_kernel.scan.data_skipping.DataSkippingFilter.md#op-5dd036bf42f274f2ce23a0b0)) to quickly skip
  files that are irrelevant for the query. This includes both data column stats
  (min/max/nullCount) and partition value filtering in a single columnar pass. A secondary
  row-level partition filter catches remaining files the columnar pass cannot prune (e.g. null
  partition values where null-safety conservatively keeps them).
- Action Deduplication: Leverages the [`FileActionDeduplicator`] to ensure that for each unique
  file (identified by its path and deletion vector unique ID), only the latest valid Add action
  is processed.
- Transformation: Applies a built-in transformation (`log_transform` or `checkpoint_transform`)
  to convert selected Add actions into [`ScanMetadata`](../operations/buoyant_kernel.scan.ScanMetadata.md#op-291499a74f7b78c8790cd5b2), the intermediate format passed to the
  engine.
- Row StructPatch passthrough: Any user-provided row-level transformation expressions (e.g.
  those derived from projection or filters) are preserved and passed through to the engine,
  which applies them as part of its scan execution logic.

As an implementation of [`LogReplayProcessor`](../operations/buoyant_kernel.log_replay.LogReplayProcessor.md#op-e6ee4714f432a749429a820c), [`ScanLogReplayProcessor`](../operations/buoyant_kernel.scan.log_replay.ScanLogReplayProcessor.md#op-00b81acb4a4714f3ad69b815) provides the
`process_actions_batch` method, which applies these steps to each batch of log actions and
produces a [`ScanMetadata`](../operations/buoyant_kernel.scan.ScanMetadata.md#op-291499a74f7b78c8790cd5b2) result. This result includes the transformed batch, a selection
vector indicating which rows are valid, and any row-level transformation expressions that need
to be applied to the selected rows.

Unresolved upstream links (retained, not inferred): ``FileActionDeduplicator``.

<a id="op-17c68c89685e37279239ae53"></a>
## Output

`assoc_type` · `buoyant_kernel::scan::log_replay::ScanLogReplayProcessor::Output` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Output = <ScanLogReplayProcessor as LogReplayProcessor>::Output
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/log_replay.rs#L814).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::scan::log_replay::ScanLogReplayProcessor", "path": "ScanLogReplayProcessor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [813, 1], "end": [889, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/log_replay.rs"}, "trait": {"args": null, "id": "buoyant_kernel::log_replay::ParallelLogReplayProcessor", "path": "ParallelLogReplayProcessor"}, "trait_path": "buoyant_kernel::log_replay::ParallelLogReplayProcessor"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/log_replay.rs:814`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f3233fe9e7c53352015fe2ae"></a>
## Output

`assoc_type` · `buoyant_kernel::scan::log_replay::ScanLogReplayProcessor::Output` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Output = ScanMetadata
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/log_replay.rs#L892).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::scan::log_replay::ScanLogReplayProcessor", "path": "ScanLogReplayProcessor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [891, 1], "end": [974, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/log_replay.rs"}, "trait": {"args": null, "id": "buoyant_kernel::log_replay::LogReplayProcessor", "path": "LogReplayProcessor"}, "trait_path": "buoyant_kernel::log_replay::LogReplayProcessor"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/log_replay.rs:892`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b9d5db7b9b0141d5fdd682d6"></a>
## data_skipping_filter

`function` · `buoyant_kernel::scan::log_replay::ScanLogReplayProcessor::data_skipping_filter` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn data_skipping_filter(&self) -> Option<&DataSkippingFilter>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/log_replay.rs#L971).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::scan::log_replay::ScanLogReplayProcessor", "path": "ScanLogReplayProcessor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [891, 1], "end": [974, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/log_replay.rs"}, "trait": {"args": null, "id": "buoyant_kernel::log_replay::LogReplayProcessor", "path": "LogReplayProcessor"}, "trait_path": "buoyant_kernel::log_replay::LogReplayProcessor"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/log_replay.rs:971`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-720d4efae778aeb88537988f"></a>
## from_serializable_state

`function` · `buoyant_kernel::scan::log_replay::ScanLogReplayProcessor::from_serializable_state` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from_serializable_state(engine: &dyn Engine, state: SerializableScanState) -> DeltaResult<Self>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/log_replay.rs#L424).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::scan::log_replay::ScanLogReplayProcessor", "path": "ScanLogReplayProcessor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [177, 1], "end": [467, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/log_replay.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/log_replay.rs:424`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Reconstruct a processor from serialized state.

Creates a new processor with the provided state. All fields (partition_filter,
data_skipping_filter, log_transform, checkpoint_transform, and seen_file_keys) are
reconstructed from the serialized state and engine.

# Parameters
- `engine`: Engine for creating evaluators and filters
- `state`: The serialized state containing predicate, internal state blob, and seen file
  keys

# Returns
A new `ScanLogReplayProcessor` wrapped in an Arc.

<a id="op-4d42bcefdb0edff3b8bafda6"></a>
## into_serializable_state

`function` · `buoyant_kernel::scan::log_replay::ScanLogReplayProcessor::into_serializable_state` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn into_serializable_state(self) -> DeltaResult<SerializableScanState>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/log_replay.rs#L363).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::scan::log_replay::ScanLogReplayProcessor", "path": "ScanLogReplayProcessor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [177, 1], "end": [467, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/log_replay.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/log_replay.rs:363`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Serialize the processor state for distributed processing.

Consumes the processor and returns a `SerializableScanState` containing:
- The predicate (if any) for data skipping
- An opaque internal state blob (schemas, transform spec, column mapping mode)
- The set of seen file keys including their deletion vector information

The returned state can be used with `from_serializable_state` to reconstruct the
processor on remote compute nodes.

WARNING: The SerializableScanState may only be deserialized using an equal binary version
of delta-kernel-rs. Using different versions for serialization and deserialization leads to
undefined behaviour!

<a id="op-58aa03977b800553822b5da6"></a>
## process_actions_batch

`function` · `buoyant_kernel::scan::log_replay::ScanLogReplayProcessor::process_actions_batch` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn process_actions_batch(&mut self, actions_batch: ActionsBatch) -> DeltaResult<Self::Output>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/log_replay.rs#L899).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::scan::log_replay::ScanLogReplayProcessor", "path": "ScanLogReplayProcessor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [891, 1], "end": [974, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/log_replay.rs"}, "trait": {"args": null, "id": "buoyant_kernel::log_replay::LogReplayProcessor", "path": "LogReplayProcessor"}, "trait_path": "buoyant_kernel::log_replay::LogReplayProcessor"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/log_replay.rs:899`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6330a53d2313416faf697db5"></a>
## process_actions_batch

`function` · `buoyant_kernel::scan::log_replay::ScanLogReplayProcessor::process_actions_batch` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn process_actions_batch(&self, actions_batch: ActionsBatch) -> DeltaResult<Self::Output>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/log_replay.rs#L822).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::scan::log_replay::ScanLogReplayProcessor", "path": "ScanLogReplayProcessor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [813, 1], "end": [889, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/log_replay.rs"}, "trait": {"args": null, "id": "buoyant_kernel::log_replay::ParallelLogReplayProcessor", "path": "ParallelLogReplayProcessor"}, "trait_path": "buoyant_kernel::log_replay::ParallelLogReplayProcessor"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/log_replay.rs:822`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea8edc8f3f7751e5606af471"></a>
## checkpoint_info

`struct_field` · `buoyant_kernel::scan::log_replay::ScanLogReplayProcessor::checkpoint_info` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
checkpoint_info: log_segment::CheckpointReadInfo
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/log_replay.rs#L172).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/log_replay.rs:172`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Information about checkpoint reading for stats optimization

<a id="op-4089c6ceeab4a1e61987dbda"></a>
## checkpoint_transform

`struct_field` · `buoyant_kernel::scan::log_replay::ScanLogReplayProcessor::checkpoint_transform` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
checkpoint_transform: std::sync::Arc<dyn ExpressionEvaluator>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/log_replay.rs#L161).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/log_replay.rs:161`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

StructPatch for checkpoint batches - reads pre-parsed stats_parsed and
partitionValues_parsed directly when available, otherwise parses from raw columns

<a id="op-d65a50f9ca24892878022d35"></a>
## data_skipping_filter

`struct_field` · `buoyant_kernel::scan::log_replay::ScanLogReplayProcessor::data_skipping_filter` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
data_skipping_filter: Option<super::data_skipping::DataSkippingFilter>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/log_replay.rs#L155).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/log_replay.rs:155`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b564fa1cee26286820add027"></a>
## log_transform

`struct_field` · `buoyant_kernel::scan::log_replay::ScanLogReplayProcessor::log_transform` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
log_transform: std::sync::Arc<dyn ExpressionEvaluator>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/log_replay.rs#L158).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/log_replay.rs:158`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

StructPatch for log batches (commit files) - uses ParseJson for stats and MapToStruct
for partition values

<a id="op-2d3ef550832d1cec7fbbe6d9"></a>
## metrics

`struct_field` · `buoyant_kernel::scan::log_replay::ScanLogReplayProcessor::metrics` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
metrics: std::sync::Arc<super::metrics::ScanMetrics>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/log_replay.rs#L174).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/log_replay.rs:174`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Metrics related to the scan

<a id="op-15af5f482f2a25e265532bda"></a>
## partition_values_options

`struct_field` · `buoyant_kernel::scan::log_replay::ScanLogReplayProcessor::partition_values_options` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
partition_values_options: ScanPartitionValuesOptions
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/log_replay.rs#L170).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/log_replay.rs:170`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Read-time partition value options.

<a id="op-1fbd45736a835a5c7aeafa9b"></a>
## seen_file_keys

`struct_field` · `buoyant_kernel::scan::log_replay::ScanLogReplayProcessor::seen_file_keys` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
seen_file_keys: std::collections::HashSet<log_replay::FileActionKey>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/log_replay.rs#L166).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/log_replay.rs:166`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A set of (data file path, dv_unique_id) pairs that have been seen thus
far in the log. This is used to filter out files with Remove actions as
well as duplicate entries in the log.

<a id="op-f515bab772842e921f214f2b"></a>
## state_info

`struct_field` · `buoyant_kernel::scan::log_replay::ScanLogReplayProcessor::state_info` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
state_info: std::sync::Arc<super::state_info::StateInfo>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/log_replay.rs#L162).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/log_replay.rs:162`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-66dd86e43762bed0819536d7"></a>
## stats_options

`struct_field` · `buoyant_kernel::scan::log_replay::ScanLogReplayProcessor::stats_options` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
stats_options: ScanStatsOptions
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/log_replay.rs#L168).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/log_replay.rs:168`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Read-time stats options.
