# `buoyant_kernel::action_reconciliation::log_replay`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.action_reconciliation.log_replay.json).

<a id="op-cfe3087aa02a2cbf50e49fd7"></a>
## log_replay

`module` · `buoyant_kernel::action_reconciliation::log_replay` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod log_replay
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/action_reconciliation/log_replay.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/action_reconciliation/log_replay.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The [`ActionReconciliationProcessor`] implements specialized log replay logic for performing
action reconciliation. It processes log files in reverse chronological order (newest to oldest)
and selects the set of actions to be included.

Uses cases include checkpointing and log compaction.

## Actions Included

This processor applies several filtering and deduplication steps to each batch of log actions:

1. **Protocol and Metadata**: Retains exactly one of each - keeping only the latest protocol and
   metadata actions.
2. **Txn Actions**: Keeps exactly one `txn` action for each unique app ID, always selecting the
   latest one encountered.
3. **File Actions**: Resolves file actions to produce the latest state of the table, keeping the
   most recent valid add actions and unexpired remove actions (tombstones) that are newer than
   `minimum_file_retention_timestamp`.

## Architecture

- [`ActionReconciliationVisitor`]: Implements [`RowVisitor`](../operations/buoyant_kernel.engine_data.RowVisitor.md#op-4cb6a6c8c9e1c025811a750f) to examine each action in a batch
  and determine if it should be included. It maintains state for deduplication across multiple
  actions in a batch and efficiently handles all filtering rules.

- [`ActionReconciliationProcessor`]: Implements the [`LogReplayProcessor`](../operations/buoyant_kernel.log_replay.LogReplayProcessor.md#op-e6ee4714f432a749429a820c) trait and
  orchestrates the overall process. For each batch of log actions, it:
  1. Creates a visitor with the current deduplication state
  2. Applies the visitor to filter actions in the batch
  3. Tracks state for deduplication across batches
  4. Produces a [`ActionReconciliationBatch`] result which includes both the filtered data and
     counts of actions selected

Unresolved upstream links (retained, not inferred): ``ActionReconciliationBatch``, ``ActionReconciliationVisitor``, ``ActionReconciliationProcessor``.
