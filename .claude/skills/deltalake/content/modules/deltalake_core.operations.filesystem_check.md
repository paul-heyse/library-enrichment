# `deltalake_core::operations::filesystem_check`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.filesystem_check.json).

<a id="op-f4529fc1e7cec7f46a155392"></a>
## filesystem_check

`module` · `deltalake_core::operations::filesystem_check` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod filesystem_check
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/filesystem_check.rs#L1).

Source: `crates/core/src/operations/filesystem_check.rs:1`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Audit the Delta Table for active files that do not exist in the underlying filesystem and remove them.

Active files are ones that have an add action in the log, but no corresponding remove action.
This operation creates a new transaction containing a remove action for each of the missing files.

This can be used to repair tables where a data file has been deleted accidentally or
purposefully, if the file was corrupted.

# Example
```rust ignore
let mut table = open_table(Url::from_directory_path("/abs/path/to/table").unwrap())?;
let (table, metrics) = FileSystemCheckBuilder::new(table.object_store(), table.state).await?;
````
