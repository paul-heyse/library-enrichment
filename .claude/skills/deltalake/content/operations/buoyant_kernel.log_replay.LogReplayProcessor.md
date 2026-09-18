# `buoyant_kernel::log_replay::LogReplayProcessor`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.log_replay.LogReplayProcessor.json).

<a id="op-e6ee4714f432a749429a820c"></a>
## LogReplayProcessor

`trait` · `buoyant_kernel::log_replay::LogReplayProcessor` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait LogReplayProcessor: Sized
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_replay/mod.rs#L319).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_replay/mod.rs:319`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

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

The [`LogReplayProcessor::process_actions_iter`](../operations/buoyant_kernel.log_replay.LogReplayProcessor.md#op-1b0ee76968676b614f584243) method is the entry point for log replay
processing. It takes as input an iterator of (actions batch, is_commit_batch flag) tuples and
returns an iterator of processor-specific output types with selection vectors. The
is_commit_batch bool flag in each tuple indicates whether the batch came from a commit log
(`true`) or checkpoint (`false`). Action batches **must** be sorted by the order of the actions
in the log from most recent to oldest.

Each row that is selected in the returned output **must** be included in the processor's result
(e.g., in scan results or checkpoint files), while non-selected rows **must** be ignored.

# Output Types

The [`LogReplayProcessor::Output`](../operations/buoyant_kernel.log_replay.LogReplayProcessor.md#op-47ad7bf7a9f16c13fdbcbd40) type represents the material result of log replay, and it
must implement the [`HasSelectionVector`](../operations/buoyant_kernel.log_replay.HasSelectionVector.md#op-36cc3f5a3879263a644b7e2a) trait to allow filtering of irrelevant rows:

- For **scans**, the output type is [`ScanMetadata`], which contains the file actions (`Add`
  actions) that need to be applied to build the table's view, accompanied by a **selection
  vector** that identifies which rows should be included. A transform vector may also be
  included to handle schema changes, such as renaming columns or modifying data types.

- For **checkpoints**, the output type is [`FilteredEngineData`], which includes the actions
  necessary to write to the checkpoint file (`Add`, `Remove`, `Metadata`, `Protocol` actions),
  filtered by the **selection vector** to determine which rows are included in the final
  checkpoint.

TODO: Refactor the Change Data Feed (CDF) processor to use this trait.

Unresolved upstream links (retained, not inferred): `crate::action_reconciliation::log_replay::ActionReconciliationProcessor`.

<a id="op-47ad7bf7a9f16c13fdbcbd40"></a>
## Output

`assoc_type` · `buoyant_kernel::log_replay::LogReplayProcessor::Output` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Output
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_replay/mod.rs#L322).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_replay/mod.rs:322`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The type of results produced by this processor must implement the
[`HasSelectionVector`](../operations/buoyant_kernel.log_replay.HasSelectionVector.md#op-36cc3f5a3879263a644b7e2a) trait to allow filtering out batches with no selected rows.

<a id="op-e030288d72ddf326ed6f6daa"></a>
## build_selection_vector

`function` · `buoyant_kernel::log_replay::LogReplayProcessor::build_selection_vector` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn build_selection_vector(&self, batch: &dyn EngineData) -> DeltaResult<Vec<bool>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_replay/mod.rs#L379).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_replay/mod.rs:379`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Builds the initial selection vector for the action batch, used to filter out rows that
are not relevant to the current processor's purpose (e.g., checkpointing, scanning).
This method performs a first pass of filtering using an optional [`DataSkippingFilter`](../operations/buoyant_kernel.scan.data_skipping.DataSkippingFilter.md#op-5dd036bf42f274f2ce23a0b0).
If no filter is provided, it assumes that all rows should be selected.

The selection vector is further updated based on the processor's logic in the
`process_actions_batch` method.

# Parameters
- `batch`: A reference to the batch of actions to be processed.

# Returns
A `DeltaResult<Vec<bool>>`, where each boolean indicates if the corresponding row should be
included. If no filter is provided, all rows are selected.

<a id="op-d237704be3a8953c3f832bab"></a>
## data_skipping_filter

`function` · `buoyant_kernel::log_replay::LogReplayProcessor::data_skipping_filter` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn data_skipping_filter(&self) -> Option<&DataSkippingFilter>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_replay/mod.rs#L389).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_replay/mod.rs:389`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns an optional reference to the [`DataSkippingFilter`](../operations/buoyant_kernel.scan.data_skipping.DataSkippingFilter.md#op-5dd036bf42f274f2ce23a0b0) used to filter rows
when building the initial selection vector in `build_selection_vector`.
If `None` is returned, no filter is applied, and all rows are selected.

<a id="op-dfd6d40374bcf37d9984e95e"></a>
## process_actions_batch

`function` · `buoyant_kernel::log_replay::LogReplayProcessor::process_actions_batch` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn process_actions_batch(&mut self, actions_batch: ActionsBatch) -> DeltaResult<Self::Output>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_replay/mod.rs#L335).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_replay/mod.rs:335`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Processes a batch of actions and returns the filtered results.
# Parameters
- `actions_batch` - An [`ActionsBatch`](../operations/buoyant_kernel.log_replay.ActionsBatch.md#op-72ca0249e147b15cf59dc008) which includes a boxed [`EngineData`](../operations/buoyant_kernel.engine_data.EngineData.md#op-f9036ab52623c01f96e1f809) instance
  representing a batch of actions and a boolean flag indicating whether the batch originates
  from a commit log, `false` if from a checkpoint.

Returns a [`DeltaResult`](../operations/buoyant_kernel.error.DeltaResult.md#op-3db788f17aa90cfeefaa890f) containing the processor’s output, which includes only selected
actions.

Note: Since log replay is stateful, processing may update internal processor state (e.g.,
deduplication sets).

<a id="op-1b0ee76968676b614f584243"></a>
## process_actions_iter

`function` · `buoyant_kernel::log_replay::LogReplayProcessor::process_actions_iter` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn process_actions_iter(self, action_iter: impl Iterator<Item = DeltaResult<ActionsBatch>>) -> impl Iterator<Item = DeltaResult<Self::Output>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_replay/mod.rs#L352).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_replay/mod.rs:352`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Applies the processor to an actions iterator and filters out empty results.

This method:
1. Applies `process_actions_batch` to each action batch
2. Maintains processor state across all batches
3. Automatically filters out batches with no selected rows

# Parameters
- `action_iter`: Iterator of [`ActionsBatch`](../operations/buoyant_kernel.log_replay.ActionsBatch.md#op-72ca0249e147b15cf59dc008), where each batch contains actions and the
  boolean flag indicates whether the batch came from a commit log (`true`) or checkpoint
  (`false`). Actions _must_ be provided in reverse chronological order.

# Returns
An iterator that yields the output type of the processor, containing only non-empty results
(batches where at least one row was selected).
