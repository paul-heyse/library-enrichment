# `buoyant_kernel::scan::log_replay`

Crate `buoyant_kernel` · 3 public items · structured records in [`model/buoyant_kernel.scan.log_replay.json`](../model/buoyant_kernel.scan.log_replay.json)

## PARTITION_VALUES_PARSED_NAME

`static` · `buoyant_kernel::scan::log_replay::PARTITION_VALUES_PARSED_NAME`

Also reachable as `delta_kernel::scan::log_replay::PARTITION_VALUES_PARSED_NAME`

```rust
static PARTITION_VALUES_PARSED_NAME: &str
```

---

## ScanLogReplayProcessor

`struct` · `buoyant_kernel::scan::log_replay::ScanLogReplayProcessor`

Also reachable as `delta_kernel::scan::log_replay::ScanLogReplayProcessor`

```rust
struct ScanLogReplayProcessor
```

**Implements**: `buoyant_kernel::log_replay::LogReplayProcessor`, `buoyant_kernel::log_replay::ParallelLogReplayProcessor`

**Methods** (2)

```rust
fn from_serializable_state(engine: &dyn Engine, state: SerializableScanState) -> DeltaResult<Self>
fn into_serializable_state(self) -> DeltaResult<SerializableScanState>
```

**via `buoyant_kernel::log_replay::LogReplayProcessor`**

```rust
fn data_skipping_filter(&self) -> Option<&DataSkippingFilter>
fn process_actions_batch(&mut self, actions_batch: ActionsBatch) -> DeltaResult<Self::Output>
```

**via `buoyant_kernel::log_replay::ParallelLogReplayProcessor`**

```rust
fn process_actions_batch(&self, actions_batch: ActionsBatch) -> DeltaResult<Self::Output>
```

[`ScanLogReplayProcessor`] performs log replay (processes actions) specifically for doing a
table scan.

During a table scan, the processor reads batches of log actions (in reverse chronological order)
and performs the following steps:

- Data Skipping: Applies a predicate-based filter (via [`DataSkippingFilter`]) to quickly skip
  files that are irrelevant for the query. This includes both data column stats
  (min/max/nullCount) and partition value filtering in a single columnar pass. A secondary
  row-level partition filter catches remaining files the columnar pass cannot prune (e.g. null
  partition values where null-safety conservatively keeps them).
- Action Deduplication: Leverages the [`FileActionDeduplicator`] to ensure that for each unique
  file (identified by its path and deletion vector unique ID), only the latest valid Add action
  is processed.
- Transformation: Applies a built-in transformation (`log_transform` or `checkpoint_transform`)
  to convert selected Add actions into [`ScanMetadata`], the intermediate format passed to the
  engine.
- Row StructPatch passthrough: Any user-provided row-level transformation expressions (e.g.
  those derived from projection or filters) are preserved and passed through to the engine,
  which applies them as part of its scan execution logic.

As an implementation of [`LogReplayProcessor`], [`ScanLogReplayProcessor`] provides the
`process_actions_batch` method, which applies these steps to each batch of log actions and
produces a [`ScanMetadata`] result. This result includes the transformed batch, a selection
vector indicating which rows are valid, and any row-level transformation expressions that need
to be applied to the selected rows.

---

## SerializableScanState

`struct` · `buoyant_kernel::scan::log_replay::SerializableScanState`

Also reachable as `delta_kernel::scan::log_replay::SerializableScanState`

```rust
struct SerializableScanState
```

**Fields**: `predicate`, `internal_state_blob`, `seen_file_keys`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Serializable processor state for distributed processing. This can be serialized using the
default serde serialization, or through custom serialization in the engine.

This struct contains all the information needed to reconstruct a `ScanLogReplayProcessor`
on remote compute nodes, enabling distributed log replay processing.

# Serialization Limitations

- **Opaque expressions**: Predicates containing [`Predicate::Opaque`] or expressions containing
  [`Expression::Opaque`] cannot be serialized using serde. Attempting to serialize state with
  opaque expressions will result in an error. Connectors that require opaque expression support
  can work around this by serializing the predicate separately using their own serialization
  mechanism, then reconstructing the processor state on the remote node.

- **Large state**: The `seen_file_keys` field can be large for tables with many commits.
  Connectors are free to serialize this field using their own format (e.g., more compact binary
  representations) rather than using the serde-based serialization.

[`Predicate::Opaque`]: crate::expressions::Predicate::Opaque
[`Expression::Opaque`]: crate::expressions::Expression::Opaque

---
