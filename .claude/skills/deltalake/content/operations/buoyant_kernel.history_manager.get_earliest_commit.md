# `buoyant_kernel::history_manager::get_earliest_commit`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.history_manager.get_earliest_commit.json).

<a id="op-c7b46a5888a0e61479251c50"></a>
## get_earliest_commit

`function` · `buoyant_kernel::history_manager::get_earliest_commit` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_earliest_commit(engine: &dyn Engine, log_root: &url::Url, earliest_ratified_commit_version: Option<Version>, commit_type: HistoryCommitType) -> DeltaResult<Version>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/history_manager/mod.rs#L875).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/mod.rs:875`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns the earliest table version available on the file system at `log_root`. The returned
version is not guaranteed to exist by the time the caller acts on it: a concurrent log-cleanup
operation may delete the underlying file.

# Parameters
- `engine`: kernel engine used to list `log_root`.
- `log_root`: URL of the table's `_delta_log/` directory (must end with `/`).
- `earliest_ratified_commit_version`: For catalog-managed tables, the earliest version the
  catalog has ratified a commit at. Pass `None` for filesystem-only tables.
- `commit_type`: selects the query. [`HistoryCommitType::Published`](../operations/buoyant_kernel.history_manager.HistoryCommitType.md#op-a3c94aac99585cdad3e98df1) returns the lowest-numbered
  published commit; [`HistoryCommitType::Recreatable`](../operations/buoyant_kernel.history_manager.HistoryCommitType.md#op-ed9a706cf7eb4e3fc81425ca) returns the earliest version whose state
  can be fully reconstructed (commit 0 or the earliest complete checkpoint).

# Errors
- Propagates any error from listing the log directory.
- [`LogHistoryError::NoCommitsFound`](../operations/buoyant_kernel.history_manager.error.LogHistoryError.md#op-2a7f0eaf1a66b3e6bdd40163) when the log directory contains no commits and
  `earliest_ratified_commit_version` is not `Some(0)`.
- [`LogHistoryError::NoRecreatableCommit`](../operations/buoyant_kernel.history_manager.error.LogHistoryError.md#op-2a2a6bd69549c86d59a54763) when `commit_type` is
  [`HistoryCommitType::Recreatable`](../operations/buoyant_kernel.history_manager.HistoryCommitType.md#op-ed9a706cf7eb4e3fc81425ca), commits exist, but neither `00...00.json` nor a checkpoint
  anchoring the smallest commit is present.
- [`DeltaError::Generic`](../operations/buoyant_kernel.error.Error.md#op-5448ac938c976df2479cb9fb) when the listing yields no commits and
  `earliest_ratified_commit_version` is `Some(0)`, flagging a broken catalog-managed invariant
  (ratified commit 0 with no published filesystem commit).
