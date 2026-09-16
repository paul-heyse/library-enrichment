# `buoyant_kernel::history_manager`

Crate `buoyant_kernel` · 7 public items · structured records in [`model/buoyant_kernel.history_manager.json`](../model/buoyant_kernel.history_manager.json)

## HistoryCommitType

`enum` · `buoyant_kernel::history_manager::HistoryCommitType`

Also reachable as `delta_kernel::history_manager::HistoryCommitType`

```rust
enum HistoryCommitType
```

**Variants**: `Published`, `Recreatable`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

Selects which commit the [`get_earliest_commit`] query returns.

---

## first_version_after

`function` · `buoyant_kernel::history_manager::first_version_after`

Also reachable as `delta_kernel::history_manager::first_version_after`

```rust
fn first_version_after(snapshot: &snapshot::Snapshot, engine: &dyn Engine, timestamp: Timestamp, resolved_commit_type: HistoryCommitType) -> DeltaResult<CommitAt>
```

Gets the first [`CommitAt`] (version and timestamp) with a timestamp at or after `timestamp`.

`resolved_commit_type` constrains the returned version:
- [`HistoryCommitType::Published`]: may return any version present in the log, even one whose
  table cannot be reconstructed.
- [`HistoryCommitType::Recreatable`]: only returns a version whose table can be fully
  reconstructed at the query time.

Returns [`LogHistoryError::TimestampOutOfRange`] if no version exists at or after
the given timestamp.

# Examples
```ignore
use delta_kernel::snapshot::Snapshot;
use test_utils::delta_kernel_default_engine::DefaultEngine;
use delta_kernel::history_manager::{first_version_after, HistoryCommitType};

let engine = DefaultEngine::try_new(...)?;
let snapshot = Snapshot::builder_for(table_uri).build(&engine)?;

// Find the first commit that occurred at or after January 1, 2023
let timestamp = 1672531200000; // Milliseconds since epoch for 2023-01-01
let commit = first_version_after(&snapshot, &engine, timestamp, HistoryCommitType::Recreatable)?;
```

---

## get_earliest_commit

`function` · `buoyant_kernel::history_manager::get_earliest_commit`

Also reachable as `delta_kernel::history_manager::get_earliest_commit`

```rust
fn get_earliest_commit(engine: &dyn Engine, log_root: &url::Url, earliest_ratified_commit_version: Option<Version>, commit_type: HistoryCommitType) -> DeltaResult<Version>
```

Returns the earliest table version available on the file system at `log_root`. The returned
version is not guaranteed to exist by the time the caller acts on it: a concurrent log-cleanup
operation may delete the underlying file.

# Parameters
- `engine`: kernel engine used to list `log_root`.
- `log_root`: URL of the table's `_delta_log/` directory (must end with `/`).
- `earliest_ratified_commit_version`: For catalog-managed tables, the earliest version the
  catalog has ratified a commit at. Pass `None` for filesystem-only tables.
- `commit_type`: selects the query. [`HistoryCommitType::Published`] returns the lowest-numbered
  published commit; [`HistoryCommitType::Recreatable`] returns the earliest version whose state
  can be fully reconstructed (commit 0 or the earliest complete checkpoint).

# Errors
- Propagates any error from listing the log directory.
- [`LogHistoryError::NoCommitsFound`] when the log directory contains no commits and
  `earliest_ratified_commit_version` is not `Some(0)`.
- [`LogHistoryError::NoRecreatableCommit`] when `commit_type` is
  [`HistoryCommitType::Recreatable`], commits exist, but neither `00...00.json` nor a checkpoint
  anchoring the smallest commit is present.
- [`DeltaError::Generic`] when the listing yields no commits and
  `earliest_ratified_commit_version` is `Some(0)`, flagging a broken catalog-managed invariant
  (ratified commit 0 with no published filesystem commit).

---

## latest_version_as_of

`function` · `buoyant_kernel::history_manager::latest_version_as_of`

Also reachable as `delta_kernel::history_manager::latest_version_as_of`

```rust
fn latest_version_as_of(snapshot: &snapshot::Snapshot, engine: &dyn Engine, timestamp: Timestamp, resolved_commit_type: HistoryCommitType) -> DeltaResult<CommitAt>
```

Gets the latest [`CommitAt`] (version and timestamp) with a timestamp at or before `timestamp`.

`resolved_commit_type` constrains the returned version:
- [`HistoryCommitType::Published`]: may return any version present in the log, even one whose
  table cannot be reconstructed.
- [`HistoryCommitType::Recreatable`]: only returns a version whose table can be fully
  reconstructed at the query time.

# Errors
Returns [`LogHistoryError::TimestampOutOfRange`] if no version exists at or before
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

---

## timestamp_range_to_versions

`function` · `buoyant_kernel::history_manager::timestamp_range_to_versions`

Also reachable as `delta_kernel::history_manager::timestamp_range_to_versions`

```rust
fn timestamp_range_to_versions(snapshot: &snapshot::Snapshot, engine: &dyn Engine, start_timestamp: Timestamp, end_timestamp: Option<Timestamp>) -> DeltaResult<(Version, Option<Version>)>
```

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
Returns [`LogHistoryError::InvalidTimestampRange`] if `start_timestamp > end_timestamp`.

Returns [`LogHistoryError::TimestampOutOfRange`] if:
- No version exists at or after `start_timestamp`
- `end_timestamp` is provided and no version exists at or before it

Returns [`LogHistoryError::EmptyTimestampRange`] if the entire range falls between two commits.

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

---

## CommitAt

`struct` · `buoyant_kernel::history_manager::CommitAt`

Also reachable as `delta_kernel::history_manager::CommitAt`

```rust
struct CommitAt
```

**Fields**: `version`, `timestamp`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(version: Version, timestamp: Timestamp) -> Self
```

A commit located by a timestamp query: the commit [`Version`] paired with its timestamp.

---

## Timestamp

`type_alias` · `buoyant_kernel::history_manager::Timestamp`

Also reachable as `delta_kernel::history_manager::Timestamp`

```rust
type Timestamp = i64
```

A timestamp representing milliseconds since the Unix epoch (1970-01-01 00:00:00 UTC).

This type is used throughout the history_manager module for timestamp-to-version conversion.
All timestamp values should be specified in milliseconds, not seconds or nanoseconds.

---
