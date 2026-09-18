# `buoyant_kernel::log_replay`

Crate `buoyant_kernel` · 5 public items · structured records in [`model/buoyant_kernel.log_replay.json`](../model/buoyant_kernel.log_replay.json)

## ActionsBatch

`struct` · `buoyant_kernel::log_replay::ActionsBatch`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.log_replay.ActionsBatch.md)

Also reachable as `delta_kernel::log_replay::ActionsBatch`

```rust
struct ActionsBatch
```

**Fields**: `actions`, `is_log_batch`

**Methods** (1)

```rust
fn actions(&self) -> &dyn EngineData
```

---

## FileActionKey

`struct` · `buoyant_kernel::log_replay::FileActionKey`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.log_replay.FileActionKey.md)

Also reachable as `delta_kernel::log_replay::FileActionKey`

```rust
struct FileActionKey
```

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn dv_unique_id(&self) -> Option<&str>
fn new(path: impl Into<String>, dv_unique_id: Option<String>) -> Self
fn path(&self) -> &str
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

The subset of file action fields that uniquely identifies it in the log, used for deduplication
of adds and removes during log replay.

---

## HasSelectionVector

`trait` · `buoyant_kernel::log_replay::HasSelectionVector`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.log_replay.HasSelectionVector.md)

Also reachable as `delta_kernel::log_replay::HasSelectionVector`

```rust
trait HasSelectionVector
```

**Implementors** (3)

- `buoyant_kernel::action_reconciliation::log_replay::ActionReconciliationBatch`
- `buoyant_kernel::engine_data::FilteredEngineData`
- `buoyant_kernel::scan::ScanMetadata`

**Methods** (1)

```rust
fn has_selected_rows(&self) -> bool
```

This trait is used to determine if a processor's output contains any selected rows.
This is used to filter out batches with no selected rows from the log replay results.

---

## LogReplayProcessor

`trait` · `buoyant_kernel::log_replay::LogReplayProcessor`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.log_replay.LogReplayProcessor.md)

Also reachable as `delta_kernel::log_replay::LogReplayProcessor`

```rust
trait LogReplayProcessor: Sized
```

**Implementors** (2)

- `buoyant_kernel::action_reconciliation::log_replay::ActionReconciliationProcessor`
- `buoyant_kernel::scan::log_replay::ScanLogReplayProcessor`

**Methods** (4)

```rust
fn build_selection_vector(&self, batch: &dyn EngineData) -> DeltaResult<Vec<bool>>
fn data_skipping_filter(&self) -> Option<&DataSkippingFilter>
fn process_actions_batch(&mut self, actions_batch: ActionsBatch) -> DeltaResult<Self::Output>
fn process_actions_iter(self, action_iter: impl Iterator<Item = DeltaResult<ActionsBatch>>) -> impl Iterator<Item = DeltaResult<Self::Output>>
```

A trait for processing batches of actions from Delta transaction logs during log replay.

Log replay processors scan transaction logs in **reverse chronological order** (newest to
oldest), filtering and transforming action batches into specialized output types. These
processors:

- **Track and deduplicate file actions** to apply appropriate `Remove` actions to corresponding
  `Add` actions (and omit the file from the log replay output)
- **Maintain selection vectors** to indicate which actions in each batch should be included.
- **Apply custom filtering logic** based on the processor’s purpose (e.g., checkpointing,
  scanning).
- **Data skipping** filters are applied to the initial selection vector to reduce the number of
  rows processed by the processor, (if a filter is provided).

# Implementations

- [`ScanLogReplayProcessor`]: Used for table scans, this processor filters and selects
  deduplicated `Add` actions from log batches to reconstruct the view of the table at a specific
  point in time. Note that scans do not expose `Remove` actions. Data skipping may be applied
  when a predicate is provided.

- [`ActionReconciliationProcessor`]: Used for action reconciliation (including checkpoint
  writing), this processor filters and selects actions from log batches for inclusion in V1 spec
  checkpoint files. Unlike scans, action reconciliation processing includes additional actions,
  such as `Remove`, `Metadata`, and `Protocol`, required to fully reconstruct table state. Data
  skipping is not applied during action reconciliation processing.

[`ActionReconciliationProcessor`]: crate::action_reconciliation::log_replay::ActionReconciliationProcessor

# Action Iterator Input

The [`LogReplayProcessor::process_actions_iter`] method is the entry point for log replay
processing. It takes as input an iterator of (actions batch, is_commit_batch flag) tuples and
returns an iterator of processor-specific output types with selection vectors. The
is_commit_batch bool flag in each tuple indicates whether the batch came from a commit log
(`true`) or checkpoint (`false`). Action batches **must** be sorted by the order of the actions
in the log from most recent to oldest.

Each row that is selected in the returned output **must** be included in the processor's result
(e.g., in scan results or checkpoint files), while non-selected rows **must** be ignored.

# Output Types

The [`LogReplayProcessor::Output`] type represents the material result of log replay, and it
must implement the [`HasSelectionVector`] trait to allow filtering of irrelevant rows:

- For **scans**, the output type is [`ScanMetadata`], which contains the file actions (`Add`
  actions) that need to be applied to build the table's view, accompanied by a **selection
  vector** that identifies which rows should be included. A transform vector may also be
  included to handle schema changes, such as renaming columns or modifying data types.

- For **checkpoints**, the output type is [`FilteredEngineData`], which includes the actions
  necessary to write to the checkpoint file (`Add`, `Remove`, `Metadata`, `Protocol` actions),
  filtered by the **selection vector** to determine which rows are included in the final
  checkpoint.

TODO: Refactor the Change Data Feed (CDF) processor to use this trait.

---

## ParallelLogReplayProcessor

`trait` · `buoyant_kernel::log_replay::ParallelLogReplayProcessor`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.log_replay.ParallelLogReplayProcessor.md)

Also reachable as `delta_kernel::log_replay::ParallelLogReplayProcessor`

```rust
trait ParallelLogReplayProcessor
```

**Implementors** (2)

- `alloc::sync::Arc`
- `buoyant_kernel::scan::log_replay::ScanLogReplayProcessor`

**Methods** (1)

```rust
fn process_actions_batch(&self, actions_batch: ActionsBatch) -> DeltaResult<Self::Output>
```

---
