# `buoyant_kernel::table_changes`

Crate `buoyant_kernel` · 1 public items · structured records in [`model/buoyant_kernel.table_changes.json`](../model/buoyant_kernel.table_changes.json)

## TableChanges

`struct` · `buoyant_kernel::table_changes::TableChanges`

Also reachable as `delta_kernel::table_changes::TableChanges`

```rust
struct TableChanges
```

**Derives**: Debug

**Methods** (7)

```rust
fn end_version(&self) -> Version
fn into_scan_builder(self) -> TableChangesScanBuilder
fn scan_builder(Arc<self>) -> TableChangesScanBuilder
fn schema(&self) -> &Schema
fn start_version(&self) -> Version
fn table_root(&self) -> &Url
fn try_new(table_root: Url, engine: &dyn Engine, start_version: Version, end_version: Option<Version>) -> DeltaResult<Self>
```

Represents a call to read the Change Data Feed (CDF) between two versions of a table. The schema
of `TableChanges` will be the schema of the table at the end version with three additional
columns:
- `_change_type`: String representing the type of change that for that commit. This may be one
  of `delete`, `insert`, `update_preimage`, or `update_postimage`.
- `_commit_version`: Long representing the commit the change occurred in.
- `_commit_timestamp`: Time at which the commit occurred. The timestamp is retrieved from the
  file modification time of the log file. No timezone is associated with the timestamp.

  Currently, in-commit timestamps (ICT) is not supported. In the future when ICT is enabled, the
  timestamp will be retrieved from the `inCommitTimestamp` field of the CommitInfo` action.
  See issue [#559](https://github.com/delta-io/delta-kernel-rs/issues/559)
  For details on In-Commit Timestamps, see the [Protocol](https://github.com/delta-io/delta/blob/master/PROTOCOL.md#in-commit-timestamps).


Three properties must hold for the entire CDF range:
- Reading must be supported for every commit in the range. Currently the only read feature
  allowed is deletion vectors. This will be expanded in the future to support more delta table
  features. Because only deletion vectors are supported, reader version 2 will not be allowed.
  That is
- Change Data Feed must be enabled for the entire range with the `delta.enableChangeDataFeed`
  table property set to `true`. Performing change data feed on  tables with column mapping is
  currently disallowed. We check that column mapping is disabled, or the column mapping mode is
  `None`.
- The schema for each commit must be compatible with the end schema. This means that all the
  same fields and their nullability are the same. Schema compatibility will be expanded in the
  future to allow compatible schemas that are not the exact same.
  See issue [#523](https://github.com/delta-io/delta-kernel-rs/issues/523)

 # Examples
 Get `TableChanges` for versions 0 to 1 (inclusive)
 ```rust
 # use buoyant_kernel as delta_kernel;
 # use test_utils::delta_kernel_default_engine::{storage::store_from_url, DefaultEngineBuilder};
 # use delta_kernel::{SnapshotRef, Error};
 # use delta_kernel::table_changes::TableChanges;
 # let path = "./tests/data/table-with-cdf";
 let url = delta_kernel::try_parse_uri(path)?;
 # let engine = DefaultEngineBuilder::new(store_from_url(&url)?).build();
 let table_changes = TableChanges::try_new(url, &engine, 0, Some(1))?;
 # Ok::<(), Error>(())
 ````
For more details, see the following sections of the protocol:
- [Add CDC File](https://github.com/delta-io/delta/blob/master/PROTOCOL.md#add-cdc-file)
- [Change Data Files](https://github.com/delta-io/delta/blob/master/PROTOCOL.md#change-data-files).

---
