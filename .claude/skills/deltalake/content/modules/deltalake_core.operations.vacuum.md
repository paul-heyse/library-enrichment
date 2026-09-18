# `deltalake_core::operations::vacuum`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.vacuum.json).

<a id="op-eb901b710fff44a6e8e619a8"></a>
## vacuum

`module` · `deltalake_core::operations::vacuum` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod vacuum
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L1).

Source: `crates/core/src/operations/vacuum.rs:1`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Vacuum a Delta table

Run the Vacuum command on the Delta Table: delete files no longer referenced by a Delta table and are older than the retention threshold.
We do not recommend that you set a retention interval shorter than 7 days, because old snapshots
and uncommitted files can still be in use by concurrent readers or writers to the table.

If vacuum cleans up active files, concurrent readers can fail or, worse, tables can be
corrupted when vacuum deletes files that have not yet been committed.
If `retention_period` is not set then the `configuration.deletedFileRetentionDuration` of
delta table is used or if that's missing too, then the default value of 7 days otherwise.

When you run vacuum then you cannot use time travel to a version older than
the specified retention period.

Warning: Vacuum does not support partitioned tables on Windows. This is due
to Windows not using unix style paths. See #682

# Example
```rust ignore
let mut table = open_table(Url::from_directory_path("/abs/path/to/table").unwrap())?;
let (table, metrics) = VacuumBuilder::new(table.object_store(). table.state).await?;
````
