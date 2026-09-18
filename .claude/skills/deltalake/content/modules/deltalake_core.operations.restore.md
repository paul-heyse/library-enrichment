# `deltalake_core::operations::restore`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.restore.json).

<a id="op-5229c178ca21c2b18f739fe7"></a>
## restore

`module` · `deltalake_core::operations::restore` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod restore
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/restore.rs#L1).

Source: `crates/core/src/operations/restore.rs:1`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Perform restore of delta table to a specified version or datetime

Algorithm:
1) Read the latest state snapshot of the table.
2) Read table state for version or datetime to restore
3) Compute files available in state for restoring (files were removed by some commit)
   but missed in the latest. Add these files into commit as AddFile action.
4) Compute files available in the latest state snapshot (files were added after version to restore)
   but missed in the state to restore. Add these files into commit as RemoveFile action.
5) If ignore_missing_files option is false (default value) check availability of AddFile
   in file system.
6) Commit Protocol, all RemoveFile and AddFile actions
   into delta log using `LogStore::write_commit_entry` (commit will be failed in case of parallel transaction)
   TODO: comment is outdated
7) If table was modified in parallel then ignore restore and raise exception.

# Example
```rust ignore
let table = open_table(Url::from_directory_path("/abs/path/to/table").unwrap())?;
let (table, metrics) = RestoreBuilder::new(table.object_store(), table.state).with_version_to_restore(1).await?;
````
