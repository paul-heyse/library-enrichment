# `buoyant_kernel::table_changes::TableChanges`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.table_changes.TableChanges.json).

<a id="op-9b64c4444ea3b8dc26e08d7e"></a>
## TableChanges

`struct` · `buoyant_kernel::table_changes::TableChanges` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct TableChanges
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_changes/mod.rs#L115).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_changes/mod.rs:115`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

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

<a id="op-f2cc7e8cb90da72ad3a25d98"></a>
## end_version

`function` · `buoyant_kernel::table_changes::TableChanges::end_version` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn end_version(&self) -> Version
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_changes/mod.rs#L224).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_changes::TableChanges", "path": "TableChanges"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [124, 1], "end": [246, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_changes/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_changes/mod.rs:224`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The end version (inclusive) of the [`TableChanges`](../operations/buoyant_kernel.table_changes.TableChanges.md#op-9b64c4444ea3b8dc26e08d7e). If no `end_version` was specified in
[`TableChanges::try_new`](../operations/buoyant_kernel.table_changes.TableChanges.md#op-a5d450caf29d1abf2248d86a), this returns the newest version as of the call to `try_new`.

<a id="op-edf557e403fec0c346701ccb"></a>
## fmt

`function` · `buoyant_kernel::table_changes::TableChanges::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_changes/mod.rs#L114).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_changes::TableChanges", "path": "TableChanges"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 10], "end": [114, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_changes/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_changes/mod.rs:114`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0bb7103f76529fdb5712c10a"></a>
## into_scan_builder

`function` · `buoyant_kernel::table_changes::TableChanges::into_scan_builder` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn into_scan_builder(self) -> TableChangesScanBuilder
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_changes/mod.rs#L243).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_changes::TableChanges", "path": "TableChanges"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [124, 1], "end": [246, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_changes/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_changes/mod.rs:243`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Consume this `TableChanges` to create a [`TableChangesScanBuilder`](../operations/buoyant_kernel.table_changes.scan.TableChangesScanBuilder.md#op-4e86551bf6a1e8510c810267)

<a id="op-99e1d80aaaf86d070d5b762c"></a>
## scan_builder

`function` · `buoyant_kernel::table_changes::TableChanges::scan_builder` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn scan_builder(Arc<self>) -> TableChangesScanBuilder
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_changes/mod.rs#L238).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_changes::TableChanges", "path": "TableChanges"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [124, 1], "end": [246, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_changes/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_changes/mod.rs:238`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a [`TableChangesScanBuilder`](../operations/buoyant_kernel.table_changes.scan.TableChangesScanBuilder.md#op-4e86551bf6a1e8510c810267) for an `Arc<TableChanges>`.

<a id="op-b90da4e6f31ed2f12e0bdbcf"></a>
## schema

`function` · `buoyant_kernel::table_changes::TableChanges::schema` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn schema(&self) -> &Schema
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_changes/mod.rs#L229).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_changes::TableChanges", "path": "TableChanges"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [124, 1], "end": [246, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_changes/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_changes/mod.rs:229`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The logical schema of the change data feed. For details on the shape of the schema, see
[`TableChanges`](../operations/buoyant_kernel.table_changes.TableChanges.md#op-9b64c4444ea3b8dc26e08d7e).

<a id="op-b97db2b25fb41bbefe34f308"></a>
## start_version

`function` · `buoyant_kernel::table_changes::TableChanges::start_version` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn start_version(&self) -> Version
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_changes/mod.rs#L219).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_changes::TableChanges", "path": "TableChanges"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [124, 1], "end": [246, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_changes/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_changes/mod.rs:219`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The start version of the `TableChanges`.

<a id="op-c259deac2ced58df21e4babd"></a>
## table_root

`function` · `buoyant_kernel::table_changes::TableChanges::table_root` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn table_root(&self) -> &Url
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_changes/mod.rs#L233).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_changes::TableChanges", "path": "TableChanges"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [124, 1], "end": [246, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_changes/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_changes/mod.rs:233`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Path to the root of the table that is being read.

<a id="op-a5d450caf29d1abf2248d86a"></a>
## try_new

`function` · `buoyant_kernel::table_changes::TableChanges::try_new` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_new(table_root: Url, engine: &dyn Engine, start_version: Version, end_version: Option<Version>) -> DeltaResult<Self>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_changes/mod.rs#L141).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_changes::TableChanges", "path": "TableChanges"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [124, 1], "end": [246, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_changes/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_changes/mod.rs:141`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Creates a new [`TableChanges`](../operations/buoyant_kernel.table_changes.TableChanges.md#op-9b64c4444ea3b8dc26e08d7e) instance for the given version range. This function checks
these properties:
- The change data feed table feature must be enabled in both the start or end versions.
- Other than the deletion vector reader feature, no other reader features are enabled for
  the table.
- The schemas at the start and end versions are the same.

Note that this does not check that change data feed is enabled for every commit in the
range. It also does not check that the schema remains the same for the entire range.

# Parameters
- `table_root`: url pointing at the table root (where `_delta_log` folder is located)
- `engine`: Implementation of [`Engine`](../operations/buoyant_kernel.Engine.md#op-144f8dad57c79b7743fd1386) apis.
- `start_version`: The start version of the change data feed
- `end_version`: The end version (inclusive) of the change data feed. If this is none, this
  defaults to the newest table version.

<a id="op-a55dbad4b266bf8e9e6c8650"></a>
## end_snapshot

`struct_field` · `buoyant_kernel::table_changes::TableChanges::end_snapshot` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
end_snapshot: snapshot::SnapshotRef
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_changes/mod.rs#L118).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_changes/mod.rs:118`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-041f453bbe01eb0475644d57"></a>
## log_segment

`struct_field` · `buoyant_kernel::table_changes::TableChanges::log_segment` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
log_segment: log_segment::LogSegment
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_changes/mod.rs#L116).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_changes/mod.rs:116`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6535c0336b050ef4a09d3fe2"></a>
## schema

`struct_field` · `buoyant_kernel::table_changes::TableChanges::schema` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
schema: schema::Schema
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_changes/mod.rs#L120).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_changes/mod.rs:120`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0c28c25631383a1ed5fbf0c5"></a>
## start_table_config

`struct_field` · `buoyant_kernel::table_changes::TableChanges::start_table_config` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
start_table_config: table_configuration::TableConfiguration
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_changes/mod.rs#L121).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_changes/mod.rs:121`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1a46b2a002bed19b2829bdc1"></a>
## start_version

`struct_field` · `buoyant_kernel::table_changes::TableChanges::start_version` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
start_version: Version
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_changes/mod.rs#L119).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_changes/mod.rs:119`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4a193c7e950cbc1bff9c924e"></a>
## table_root

`struct_field` · `buoyant_kernel::table_changes::TableChanges::table_root` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
table_root: url::Url
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_changes/mod.rs#L117).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_changes/mod.rs:117`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
