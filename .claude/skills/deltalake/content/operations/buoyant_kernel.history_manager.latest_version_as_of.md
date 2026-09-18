# `buoyant_kernel::history_manager::latest_version_as_of`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.history_manager.latest_version_as_of.json).

<a id="op-d510741eeda9606439503cdd"></a>
## latest_version_as_of

`function` · `buoyant_kernel::history_manager::latest_version_as_of` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn latest_version_as_of(snapshot: &snapshot::Snapshot, engine: &dyn Engine, timestamp: Timestamp, resolved_commit_type: HistoryCommitType) -> DeltaResult<CommitAt>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/history_manager/mod.rs#L531).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/mod.rs:531`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Gets the latest [`CommitAt`](../operations/buoyant_kernel.history_manager.CommitAt.md#op-86a3435c27e3b39467f81178) (version and timestamp) with a timestamp at or before `timestamp`.

`resolved_commit_type` constrains the returned version:
- [`HistoryCommitType::Published`](../operations/buoyant_kernel.history_manager.HistoryCommitType.md#op-a3c94aac99585cdad3e98df1): may return any version present in the log, even one whose
  table cannot be reconstructed.
- [`HistoryCommitType::Recreatable`](../operations/buoyant_kernel.history_manager.HistoryCommitType.md#op-ed9a706cf7eb4e3fc81425ca): only returns a version whose table can be fully
  reconstructed at the query time.

# Errors
Returns [`LogHistoryError::TimestampOutOfRange`](../operations/buoyant_kernel.history_manager.error.LogHistoryError.md#op-d826f69b327b93b6ee4b7767) if no version exists at or before
the given timestamp.

# Examples
```ignore
use delta_kernel::snapshot::Snapshot;
use test_utils::delta_kernel_default_engine::DefaultEngine;
use delta_kernel::history_manager::{latest_version_as_of, HistoryCommitType};

let engine = DefaultEngine::try_new(...)?;
let snapshot = Snapshot::builder_for(table_uri).build(&engine)?;

// Get the latest commit as of January 1, 2023
let timestamp = 1672531200000; // Milliseconds since epoch for 2023-01-01
let commit = latest_version_as_of(&snapshot, &engine, timestamp, HistoryCommitType::Recreatable)?;
```
