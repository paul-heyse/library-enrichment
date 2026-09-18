# `buoyant_kernel::history_manager::timestamp_range_to_versions`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.history_manager.timestamp_range_to_versions.json).

<a id="op-445a752da0f5d64bcc328691"></a>
## timestamp_range_to_versions

`function` · `buoyant_kernel::history_manager::timestamp_range_to_versions` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn timestamp_range_to_versions(snapshot: &snapshot::Snapshot, engine: &dyn Engine, start_timestamp: Timestamp, end_timestamp: Option<Timestamp>) -> DeltaResult<(Version, Option<Version>)>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/history_manager/mod.rs#L632).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/mod.rs:632`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Converts a timestamp range to a corresponding version range.

This function finds the version range that corresponds to the given timestamp range.
The returned tuple contains:
- The first (earliest) version with a timestamp greater than or equal to `start_timestamp`
- If `end_timestamp` is provided, the version with a timestamp less than or equal to
  `end_timestamp`.

# Arguments
* `snapshot` - The snapshot that defines the searchable version range
* `engine` - The engine used to access version history
* `start_timestamp` - The lower bound timestamp (inclusive), in milliseconds since Unix epoch
* `end_timestamp` - The optional upper bound timestamp (inclusive), or `None` to indicate no
  upper bound

# Returns
A tuple containing the start version and optional end version (inclusive)

# Errors
Returns [`LogHistoryError::InvalidTimestampRange`](../operations/buoyant_kernel.history_manager.error.LogHistoryError.md#op-daccd834fdc4daa2e9470543) if `start_timestamp > end_timestamp`.

Returns [`LogHistoryError::TimestampOutOfRange`](../operations/buoyant_kernel.history_manager.error.LogHistoryError.md#op-d826f69b327b93b6ee4b7767) if:
- No version exists at or after `start_timestamp`
- `end_timestamp` is provided and no version exists at or before it

Returns [`LogHistoryError::EmptyTimestampRange`](../operations/buoyant_kernel.history_manager.error.LogHistoryError.md#op-02a6d6d959b277e9030bcdab) if the entire range falls between two commits.

# Examples
```ignore
use delta_kernel::snapshot::Snapshot;
use test_utils::delta_kernel_default_engine::DefaultEngine;
use delta_kernel::history_manager::timestamp_range_to_versions;

let engine = DefaultEngine::try_new(...)?;
let snapshot = Snapshot::builder_for(table_uri).build(&engine)?;

// Find versions between January 1, 2023 and March 1, 2023
let start_timestamp = 1672531200000; // Jan 1, 2023 (milliseconds since epoch)
let end_timestamp = 1677628800000;   // Mar 1, 2023 (milliseconds since epoch)

let (start_version, end_version) =
    timestamp_range_to_versions(&snapshot, &engine, start_timestamp, Some(end_timestamp))?;
```
