# Snapshots and log replay

The Delta log is an append-only sequence of JSON commits, periodically compacted into checkpoints. A snapshot is the result of replaying that sequence to a version. `EagerSnapshot` materialises file actions up front; the kernel `Snapshot` is the lazier, kernel-owned path. Anything that answers "which files are in this table" is log replay, and it is the dominant cost of opening a large table.

## Entry points

| Type | Kind | Methods | Prose | Records |
|---|---|---:|---|---|
| `deltalake_core::kernel::snapshot::EagerSnapshot` | struct | 35 | [prose](../api/deltalake_core.kernel.snapshot.md#eagersnapshot) | [records](../model/deltalake_core.kernel.snapshot.json) |
| `deltalake_core::kernel::snapshot::Snapshot` | struct | 24 | [prose](../api/deltalake_core.kernel.snapshot.md#snapshot) | [records](../model/deltalake_core.kernel.snapshot.json) |
| `deltalake_core::table::state::DeltaTableState` | struct | 24 | [prose](../api/deltalake_core.table.state.md#deltatablestate) | [records](../model/deltalake_core.table.state.json) |
| `deltalake_core::kernel::snapshot::log_data::LogDataHandler` | struct | 13 | [prose](../api/deltalake_core.kernel.snapshot.log_data.md#logdatahandler) | [records](../model/deltalake_core.kernel.snapshot.log_data.json) |
| `deltalake_core::kernel::snapshot::iterators::LogicalFileView` | struct | 16 | [prose](../api/deltalake_core.kernel.snapshot.iterators.md#logicalfileview) | [records](../model/deltalake_core.kernel.snapshot.iterators.json) |

## Extension points

| Trait | Required | Provided | Implementors | Page |
|---|---:|---:|---:|---|
| `deltalake_core::logstore::LogStore` | 8 | 8 | 4 | [LogStore](../traits/LogStore.md) |

## Runnable examples (4)

- [`corpus/examples/basic_operations.rs`](../corpus/examples/basic_operations.rs)
- [`corpus/examples/load_table.rs`](../corpus/examples/load_table.rs)
- [`corpus/examples/read_delta_table.rs`](../corpus/examples/read_delta_table.rs)
- [`corpus/examples/recordbatch-writer.rs`](../corpus/examples/recordbatch-writer.rs)

## Decision rules

- Checkpoint regularly: replay cost grows with the number of commits since the last checkpoint, not with table size.
- File skipping happens against snapshot statistics, so a table written without statistics cannot be pruned.

## Anti-patterns

- Listing the object store to find data files. The log is authoritative; the store is not.
- Assuming a snapshot refreshes itself. It is a value, pinned to a version.

## Agent checklist

- Is the checkpoint interval set for the commit rate?
- Is file skipping actually reaching statistics, or are they absent?
