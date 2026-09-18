# `deltalake_core::table::config::TableProperty`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.table.config.TableProperty.json).

<a id="op-b2f7aafcea32c3c4e27d4277"></a>
## TableProperty

`enum` · `deltalake_core::table::config::TableProperty` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum TableProperty
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/config.rs#L18).

Source: `crates/core/src/table/config.rs:18`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Typed property keys that can be defined on a delta table

<https://docs.delta.io/latest/table-properties.html#delta-table-properties-reference>
<https://learn.microsoft.com/en-us/azure/databricks/delta/table-properties>

<a id="op-94acd153d9892ff435aaf3e8"></a>
## AppendOnly

`variant` · `deltalake_core::table::config::TableProperty::AppendOnly` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
AppendOnly
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/config.rs#L21).

Source: `crates/core/src/table/config.rs:21`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

true for this Delta table to be append-only. If append-only,
existing records cannot be deleted, and existing values cannot be updated.

<a id="op-97fc9a667f1c596ff7075b82"></a>
## AutoOptimizeAutoCompact

`variant` · `deltalake_core::table::config::TableProperty::AutoOptimizeAutoCompact` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
AutoOptimizeAutoCompact
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/config.rs#L24).

Source: `crates/core/src/table/config.rs:24`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

true for Delta Lake to automatically optimize the layout of the files for this Delta table.

<a id="op-7df9a562388d15cf144e5ee9"></a>
## AutoOptimizeOptimizeWrite

`variant` · `deltalake_core::table::config::TableProperty::AutoOptimizeOptimizeWrite` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
AutoOptimizeOptimizeWrite
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/config.rs#L27).

Source: `crates/core/src/table/config.rs:27`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

true for Delta Lake to automatically optimize the layout of the files for this Delta table during writes.

<a id="op-51cc4a704b297a97698bd3de"></a>
## CheckpointInterval

`variant` · `deltalake_core::table::config::TableProperty::CheckpointInterval` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
CheckpointInterval
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/config.rs#L30).

Source: `crates/core/src/table/config.rs:30`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Interval (number of commits) after which a new checkpoint should be created

<a id="op-b0779b609cd70b100e06eccb"></a>
## CheckpointPolicy

`variant` · `deltalake_core::table::config::TableProperty::CheckpointPolicy` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
CheckpointPolicy
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/config.rs#L121).

Source: `crates/core/src/table/config.rs:121`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

'classic' for classic Delta Lake checkpoints. 'v2' for v2 checkpoints.

<a id="op-7e3c5021d675d8ae0fbec1e9"></a>
## CheckpointUseRunLengthEncoding

`variant` · `deltalake_core::table::config::TableProperty::CheckpointUseRunLengthEncoding` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
CheckpointUseRunLengthEncoding
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/config.rs#L41).

Source: `crates/core/src/table/config.rs:41`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

true for Delta Lake to write checkpoint files using run length encoding (RLE).
Some readers don't support run length encoding (i.e. Fabric) so this can be disabled.

<a id="op-03f799dbbe34b6dda36664af"></a>
## CheckpointWriteStatsAsJson

`variant` · `deltalake_core::table::config::TableProperty::CheckpointWriteStatsAsJson` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
CheckpointWriteStatsAsJson
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/config.rs#L33).

Source: `crates/core/src/table/config.rs:33`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

true for Delta Lake to write file statistics in checkpoints in JSON format for the stats column.

<a id="op-a0cb9905342e399e4d8091aa"></a>
## CheckpointWriteStatsAsStruct

`variant` · `deltalake_core::table::config::TableProperty::CheckpointWriteStatsAsStruct` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
CheckpointWriteStatsAsStruct
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/config.rs#L37).

Source: `crates/core/src/table/config.rs:37`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

true for Delta Lake to write file statistics to checkpoints in struct format for the
stats_parsed column and to write partition values as a struct for partitionValues_parsed.

<a id="op-02592a3430bb7abb832b5f0c"></a>
## ColumnMappingMode

`variant` · `deltalake_core::table::config::TableProperty::ColumnMappingMode` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
ColumnMappingMode
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/config.rs#L45).

Source: `crates/core/src/table/config.rs:45`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Whether column mapping is enabled for Delta table columns and the corresponding
Parquet columns that use different names.

<a id="op-5a956a8de1adebb0bc2d4ddc"></a>
## DataSkippingNumIndexedCols

`variant` · `deltalake_core::table::config::TableProperty::DataSkippingNumIndexedCols` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
DataSkippingNumIndexedCols
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/config.rs#L53).

Source: `crates/core/src/table/config.rs:53`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The number of columns for Delta Lake to collect statistics about for data skipping.
A value of -1 means to collect statistics for all columns. Updating this property does
not automatically collect statistics again; instead, it redefines the statistics schema
of the Delta table. Specifically, it changes the behavior of future statistics collection
(such as during appends and optimizations) as well as data skipping (such as ignoring column
statistics beyond this number, even when such statistics exist).

<a id="op-97e8307fc6f386119a3a54ea"></a>
## DataSkippingStatsColumns

`variant` · `deltalake_core::table::config::TableProperty::DataSkippingStatsColumns` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
DataSkippingStatsColumns
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/config.rs#L58).

Source: `crates/core/src/table/config.rs:58`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A comma-separated list of column names on which Delta Lake collects statistics to enhance
data skipping functionality. This property takes precedence over
[DataSkippingNumIndexedCols](Self::DataSkippingNumIndexedCols).

<a id="op-45b5f7ec9f3c5c341dcec723"></a>
## DeletedFileRetentionDuration

`variant` · `deltalake_core::table::config::TableProperty::DeletedFileRetentionDuration` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
DeletedFileRetentionDuration
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/config.rs#L69).

Source: `crates/core/src/table/config.rs:69`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The shortest duration for Delta Lake to keep logically deleted data files before deleting
them physically. This is to prevent failures in stale readers after compactions or partition overwrites.

This value should be large enough to ensure that:

* It is larger than the longest possible duration of a job if you run VACUUM when there are
  concurrent readers or writers accessing the Delta table.
* If you run a streaming query that reads from the table, that query does not stop for longer
  than this value. Otherwise, the query may not be able to restart, as it must still read old files.

<a id="op-039c4ba2c06bc03c749e62a4"></a>
## EnableChangeDataFeed

`variant` · `deltalake_core::table::config::TableProperty::EnableChangeDataFeed` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
EnableChangeDataFeed
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/config.rs#L72).

Source: `crates/core/src/table/config.rs:72`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

true to enable change data feed.

<a id="op-3991f1173d48ace8df196f9f"></a>
## EnableDeletionVectors

`variant` · `deltalake_core::table::config::TableProperty::EnableDeletionVectors` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
EnableDeletionVectors
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/config.rs#L75).

Source: `crates/core/src/table/config.rs:75`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

true to enable deletion vectors and predictive I/O for updates.

<a id="op-4f9eda492df32fc274082605"></a>
## EnableExpiredLogCleanup

`variant` · `deltalake_core::table::config::TableProperty::EnableExpiredLogCleanup` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
EnableExpiredLogCleanup
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/config.rs#L91).

Source: `crates/core/src/table/config.rs:91`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

TODO I could not find this property in the documentation, but was defined here and makes sense..?

<a id="op-7d477ddd977a23a606db04e3"></a>
## Err

`assoc_type` · `deltalake_core::table::config::TableProperty::Err` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Err = DeltaTableError
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/config.rs#L156).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::config::TableProperty", "path": "TableProperty"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [155, 1], "end": [191, 2], "filename": "crates/core/src/table/config.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `crates/core/src/table/config.rs:156`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-78b842db459df609ac0aa4fc"></a>
## IsolationLevel

`variant` · `deltalake_core::table::config::TableProperty::IsolationLevel` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
IsolationLevel
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/config.rs#L80).

Source: `crates/core/src/table/config.rs:80`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The degree to which a transaction must be isolated from modifications made by concurrent transactions.

Valid values are `Serializable` and `WriteSerializable`.

<a id="op-f8ff58df255ad284da934826"></a>
## LogRetentionDuration

`variant` · `deltalake_core::table::config::TableProperty::LogRetentionDuration` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
LogRetentionDuration
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/config.rs#L88).

Source: `crates/core/src/table/config.rs:88`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

How long the history for a Delta table is kept.

Each time a checkpoint is written, Delta Lake automatically cleans up log entries older
than the retention interval. If you set this property to a large enough value, many log
entries are retained. This should not impact performance as operations against the log are
constant time. Operations on history are parallel but will become more expensive as the log size increases.

<a id="op-80dc2f932109ab397061fbdd"></a>
## MinReaderVersion

`variant` · `deltalake_core::table::config::TableProperty::MinReaderVersion` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
MinReaderVersion
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/config.rs#L94).

Source: `crates/core/src/table/config.rs:94`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The minimum required protocol reader version for a reader that allows to read from this Delta table.

<a id="op-245d4111fe85149cc9f2b7cd"></a>
## MinWriterVersion

`variant` · `deltalake_core::table::config::TableProperty::MinWriterVersion` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
MinWriterVersion
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/config.rs#L97).

Source: `crates/core/src/table/config.rs:97`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The minimum required protocol writer version for a writer that allows to write to this Delta table.

<a id="op-14f9895a4e0e587c5dc2b7eb"></a>
## RandomPrefixLength

`variant` · `deltalake_core::table::config::TableProperty::RandomPrefixLength` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
RandomPrefixLength
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/config.rs#L107).

Source: `crates/core/src/table/config.rs:107`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

When delta.randomizeFilePrefixes is set to true, the number of characters that Delta Lake generates for random prefixes.

<a id="op-4c80fd291aefa73988a2a10f"></a>
## RandomizeFilePrefixes

`variant` · `deltalake_core::table::config::TableProperty::RandomizeFilePrefixes` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
RandomizeFilePrefixes
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/config.rs#L104).

Source: `crates/core/src/table/config.rs:104`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

true for Delta Lake to generate a random prefix for a file path instead of partition information.

For example, this ma
y improve Amazon S3 performance when Delta Lake needs to send very high volumes
of Amazon S3 calls to better partition across S3 servers.

<a id="op-8d80b77a90bf314c0b7b6557"></a>
## SetTransactionRetentionDuration

`variant` · `deltalake_core::table::config::TableProperty::SetTransactionRetentionDuration` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
SetTransactionRetentionDuration
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/config.rs#L112).

Source: `crates/core/src/table/config.rs:112`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The shortest duration within which new snapshots will retain transaction identifiers (for example, SetTransactions).
When a new snapshot sees a transaction identifier older than or equal to the duration specified by this property,
the snapshot considers it expired and ignores it. The SetTransaction identifier is used when making the writes idempotent.

<a id="op-646ec812f77e5b5f55b505b2"></a>
## TargetFileSize

`variant` · `deltalake_core::table::config::TableProperty::TargetFileSize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
TargetFileSize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/config.rs#L115).

Source: `crates/core/src/table/config.rs:115`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The target file size in bytes or higher units for file tuning. For example, 104857600 (bytes) or 100mb.

<a id="op-fb96efc056536f99bede09b4"></a>
## TuneFileSizesForRewrites

`variant` · `deltalake_core::table::config::TableProperty::TuneFileSizesForRewrites` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
TuneFileSizesForRewrites
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/config.rs#L118).

Source: `crates/core/src/table/config.rs:118`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The target file size in bytes or higher units for file tuning. For example, 104857600 (bytes) or 100mb.

<a id="op-aabd856dfcf9ed3dca1f5fcd"></a>
## as_ref

`function` · `deltalake_core::table::config::TableProperty::as_ref` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn as_ref(&self) -> &str
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/config.rs#L125).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::config::TableProperty", "path": "TableProperty"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [124, 1], "end": [153, 2], "filename": "crates/core/src/table/config.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "str"}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}, "trait_path": "core::convert::AsRef"}`

Source: `crates/core/src/table/config.rs:125`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-34d76527a5e7e4b01ab82a44"></a>
## eq

`function` · `deltalake_core::table::config::TableProperty::eq` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &TableProperty) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/config.rs#L16).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::config::TableProperty", "path": "TableProperty"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [16, 10], "end": [16, 19], "filename": "crates/core/src/table/config.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `crates/core/src/table/config.rs:16`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e69dc6027e596a1a36c32401"></a>
## from_str

`function` · `deltalake_core::table::config::TableProperty::from_str` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/config.rs#L158).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::config::TableProperty", "path": "TableProperty"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [155, 1], "end": [191, 2], "filename": "crates/core/src/table/config.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `crates/core/src/table/config.rs:158`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9c2b3f43e9cb60582af88fc2"></a>
## hash

`function` · `deltalake_core::table::config::TableProperty::hash` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/config.rs#L16).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::config::TableProperty", "path": "TableProperty"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [16, 25], "end": [16, 29], "filename": "crates/core/src/table/config.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `crates/core/src/table/config.rs:16`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
