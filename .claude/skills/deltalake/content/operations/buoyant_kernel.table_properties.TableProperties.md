# `buoyant_kernel::table_properties::TableProperties`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.table_properties.TableProperties.json).

<a id="op-a6b333464916b5b04ef86dcd"></a>
## TableProperties

`struct` · `buoyant_kernel::table_properties::TableProperties` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct TableProperties
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L75).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:75`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Delta table properties. These are parsed from the 'configuration' map in the most recent
'Metadata' action of a table.

Reference: <https://github.com/delta-io/delta/blob/master/spark/src/main/scala/org/apache/spark/sql/delta/DeltaConfig.scala>

<a id="op-b449bd58fda6adb7c48a6388"></a>
## append_only

`struct_field` · `buoyant_kernel::table_properties::TableProperties::append_only` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
append_only: Option<bool>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L80).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:80`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

true for this Delta table to be append-only. If append-only, existing records cannot be
deleted, and existing values cannot be updated. See [append-only tables] in the protocol.

[append-only tables]: https://github.com/delta-io/delta/blob/master/PROTOCOL.md#append-only-tables

<a id="op-42df2bb09a231e00390ce9db"></a>
## auto_compact

`struct_field` · `buoyant_kernel::table_properties::TableProperties::auto_compact` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
auto_compact: Option<bool>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L83).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:83`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

true for Delta Lake to automatically optimize the layout of the files for this Delta table.

<a id="op-04026a405bb1b176d362d127"></a>
## checkpoint_interval

`struct_field` · `buoyant_kernel::table_properties::TableProperties::checkpoint_interval` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
checkpoint_interval: Option<std::num::NonZero<u64>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L91).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:91`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Interval (expressed as number of commits) after which a new checkpoint should be created.
E.g. if checkpoint interval = 10, then a checkpoint should be written every 10 commits.

<a id="op-184f1f8fb7f34801a2637698"></a>
## checkpoint_policy

`struct_field` · `buoyant_kernel::table_properties::TableProperties::checkpoint_policy` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
checkpoint_policy: Option<CheckpointPolicy>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L202).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:202`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

'classic' for classic Delta Lake checkpoints. 'v2' for v2 checkpoints.

<a id="op-60e30a94ae471880579dfd29"></a>
## checkpoint_write_stats_as_json

`struct_field` · `buoyant_kernel::table_properties::TableProperties::checkpoint_write_stats_as_json` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
checkpoint_write_stats_as_json: Option<bool>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L95).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:95`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

true for Delta Lake to write file statistics in checkpoints in JSON format for the stats
column.

<a id="op-19be4bec0ea205ac5a6d8144"></a>
## checkpoint_write_stats_as_struct

`struct_field` · `buoyant_kernel::table_properties::TableProperties::checkpoint_write_stats_as_struct` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
checkpoint_write_stats_as_struct: Option<bool>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L99).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:99`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

true for Delta Lake to write file statistics to checkpoints in struct format for the
stats_parsed column and to write partition values as a struct for partitionValues_parsed.

<a id="op-4f5e1ac2d3567aeba6b66cae"></a>
## clone

`function` · `buoyant_kernel::table_properties::TableProperties::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> TableProperties
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L74).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_properties::TableProperties", "path": "TableProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 17], "end": [74, 22], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:74`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aeaedbe542afc35b1ed1749f"></a>
## column_mapping_max_column_id

`struct_field` · `buoyant_kernel::table_properties::TableProperties::column_mapping_max_column_id` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
column_mapping_max_column_id: Option<i64>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L108).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:108`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The largest column-mapping ID assigned in the table schema. ALTER TABLE operations
that add new column-mapped columns must allocate IDs strictly greater than this value
and bump the property accordingly.

<a id="op-f843b56eb13570b30f6c018c"></a>
## column_mapping_mode

`struct_field` · `buoyant_kernel::table_properties::TableProperties::column_mapping_mode` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
column_mapping_mode: Option<table_features::ColumnMappingMode>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L103).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:103`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Whether column mapping is enabled for Delta table columns and the corresponding
Parquet columns that use different names.

<a id="op-5335223ecb0b41b4f0876ee7"></a>
## compression_codec_or_default

`function` · `buoyant_kernel::table_properties::TableProperties::compression_codec_or_default` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn compression_codec_or_default(&self) -> ParquetCompressionCodec
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L283).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_properties::TableProperties", "path": "TableProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [248, 1], "end": [287, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:283`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns the Parquet compression codec for new data and checkpoint files, applying the
Delta protocol's recommended fallback ([`ParquetCompressionCodec::Zstd`](../operations/buoyant_kernel.table_properties.ParquetCompressionCodec.md#op-77aaec8e41daa6caa3431be5)) when the
table property is absent.

Use the returned value's `Display` impl (`codec.to_string()`) or `Into<&'static str>`
(`codec.into()`) to get the canonical Delta-protocol string for a Parquet writer.

<a id="op-c834d499f0dd4b2d7f578afa"></a>
## data_skipping_num_indexed_cols

`struct_field` · `buoyant_kernel::table_properties::TableProperties::data_skipping_num_indexed_cols` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
data_skipping_num_indexed_cols: Option<DataSkippingNumIndexedCols>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L116).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:116`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The number of columns for Delta Lake to collect statistics about for data skipping.
A value of -1 means to collect statistics for all columns. Updating this property does
not automatically collect statistics again; instead, it redefines the statistics schema
of the Delta table. Specifically, it changes the behavior of future statistics collection
(such as during appends and optimizations) as well as data skipping (such as ignoring
column statistics beyond this number, even when such statistics exist).

<a id="op-3e9599c9c6eca7dd888b53e0"></a>
## data_skipping_stats_columns

`struct_field` · `buoyant_kernel::table_properties::TableProperties::data_skipping_stats_columns` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
data_skipping_stats_columns: Option<Vec<expressions::ColumnName>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L121).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:121`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A comma-separated list of column names on which Delta Lake collects statistics to enhance
data skipping functionality. This property takes precedence over
`delta.dataSkippingNumIndexedCols`.

<a id="op-3c29579da0384be54b8881b3"></a>
## default

`function` · `buoyant_kernel::table_properties::TableProperties::default` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> TableProperties
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L74).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_properties::TableProperties", "path": "TableProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 39], "end": [74, 46], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:74`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-25d2f6884601e9c512a32095"></a>
## deleted_file_retention_duration

`struct_field` · `buoyant_kernel::table_properties::TableProperties::deleted_file_retention_duration` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
deleted_file_retention_duration: Option<std::time::Duration>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L134).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:134`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The shortest duration for Delta Lake to keep logically deleted data files before deleting
them physically. This is to prevent failures in stale readers after compactions or
partition overwrites.

This value should be large enough to ensure that:

* It is larger than the longest possible duration of a job if you run VACUUM when there are
  concurrent readers or writers accessing the Delta table.
* If you run a streaming query that reads from the table, that query does not stop for
  longer than this value. Otherwise, the query may not be able to restart, as it must still
  read old files.

<a id="op-ebdef6be75059075ef22c5cb"></a>
## enable_change_data_feed

`struct_field` · `buoyant_kernel::table_properties::TableProperties::enable_change_data_feed` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enable_change_data_feed: Option<bool>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L137).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:137`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

true to enable change data feed.

<a id="op-d465fc0a40025914ec50e225"></a>
## enable_deletion_vectors

`struct_field` · `buoyant_kernel::table_properties::TableProperties::enable_deletion_vectors` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enable_deletion_vectors: Option<bool>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L140).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:140`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

true to enable deletion vectors and predictive I/O for updates.

<a id="op-138366f1357fd493f8280188"></a>
## enable_expired_log_cleanup

`struct_field` · `buoyant_kernel::table_properties::TableProperties::enable_expired_log_cleanup` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enable_expired_log_cleanup: Option<bool>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L174).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:174`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Whether to clean up expired checkpoints/commits in the delta log.

<a id="op-7429a912602a4768c62190bd"></a>
## enable_iceberg_compat_v1

`struct_field` · `buoyant_kernel::table_properties::TableProperties::enable_iceberg_compat_v1` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enable_iceberg_compat_v1: Option<bool>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L148).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:148`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Whether Iceberg compatibility V1 is enabled for this table. When enabled, Delta Lake
ensures compatibility with Apache Iceberg V1 table format.

<a id="op-50421f4dcdfb8e4a57ecda32"></a>
## enable_iceberg_compat_v2

`struct_field` · `buoyant_kernel::table_properties::TableProperties::enable_iceberg_compat_v2` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enable_iceberg_compat_v2: Option<bool>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L152).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:152`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Whether Iceberg compatibility V2 is enabled for this table. When enabled, Delta Lake
ensures compatibility with Apache Iceberg V2 table format.

<a id="op-f803f90fa2d7f8b0df80c937"></a>
## enable_iceberg_compat_v3

`struct_field` · `buoyant_kernel::table_properties::TableProperties::enable_iceberg_compat_v3` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enable_iceberg_compat_v3: Option<bool>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L156).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:156`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Whether Iceberg compatibility V3 is enabled for this table. When enabled, Delta Lake
ensures compatibility with Apache Iceberg V3 table format.

<a id="op-27e126f5565239bc986e792a"></a>
## enable_in_commit_timestamps

`struct_field` · `buoyant_kernel::table_properties::TableProperties::enable_in_commit_timestamps` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enable_in_commit_timestamps: Option<bool>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L235).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:235`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Whether to enable [In-Commit Timestamps]. The in-commit timestamps writer feature strongly
associates a monotonically increasing timestamp with each commit by storing it in the
commit's metadata.

[In-Commit Timestamps]: https://github.com/delta-io/delta/blob/master/PROTOCOL.md#in-commit-timestamps

<a id="op-9e19122c478d22190c0164d1"></a>
## enable_row_tracking

`struct_field` · `buoyant_kernel::table_properties::TableProperties::enable_row_tracking` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enable_row_tracking: Option<bool>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L207).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:207`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Whether to enable row tracking for the table.

When row tracking is enabled, all rows are guaranteed to have a row ID and commit version.

<a id="op-e85d299e86dad2f81099efb3"></a>
## enable_type_widening

`struct_field` · `buoyant_kernel::table_properties::TableProperties::enable_type_widening` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enable_type_widening: Option<bool>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L144).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:144`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Whether widening the type of an existing column or field is allowed, either manually using
ALTER TABLE CHANGE COLUMN or automatically if automatic schema evolution is enabled.

<a id="op-75a29f0353c2e213a156bf1b"></a>
## eq

`function` · `buoyant_kernel::table_properties::TableProperties::eq` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &TableProperties) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L74).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_properties::TableProperties", "path": "TableProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 28], "end": [74, 37], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:74`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-910d0f1c7fc771de50600f27"></a>
## fmt

`function` · `buoyant_kernel::table_properties::TableProperties::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L74).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_properties::TableProperties", "path": "TableProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 10], "end": [74, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:74`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-db5332d6ac927ce1fbd89606"></a>
## from

`function` · `buoyant_kernel::table_properties::TableProperties::from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(unparsed: I) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/deserialize.rs#L26).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_properties::TableProperties", "path": "TableProperties"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "I"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"tuple": [{"generic": "K"}, {"generic": "V"}]}}}, "name": "Item"}]}}, "id": "core::iter::traits::collect::IntoIterator", "path": "IntoIterator"}}}], "generic_params": [], "type": {"generic": "I"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "str"}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "str"}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [20, 1], "end": [35, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/deserialize.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "I"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/deserialize.rs:26`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4d942212dac21200ca4b0123"></a>
## in_commit_timestamp_enablement_timestamp

`struct_field` · `buoyant_kernel::table_properties::TableProperties::in_commit_timestamp_enablement_timestamp` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
in_commit_timestamp_enablement_timestamp: Option<i64>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L242).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:242`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The timestamp of the table at which in-commit timestamps were enabled. This must be the
same as the inCommitTimestamp of the commit when this feature was enabled.

<a id="op-69dbaebe23cd75524e28182b"></a>
## in_commit_timestamp_enablement_version

`struct_field` · `buoyant_kernel::table_properties::TableProperties::in_commit_timestamp_enablement_version` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
in_commit_timestamp_enablement_version: Option<Version>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L238).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:238`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The version of the table at which in-commit timestamps were enabled.

<a id="op-a1fdda3d82d1b8198bdfe634"></a>
## isolation_level

`struct_field` · `buoyant_kernel::table_properties::TableProperties::isolation_level` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
isolation_level: Option<IsolationLevel>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L162).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:162`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The degree to which a transaction must be isolated from modifications made by concurrent
transactions.

Valid values are `Serializable` and `WriteSerializable`.

<a id="op-221f47161eef52d76e86a0e7"></a>
## log_retention_duration

`struct_field` · `buoyant_kernel::table_properties::TableProperties::log_retention_duration` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
log_retention_duration: Option<std::time::Duration>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L171).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:171`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

How long the history for a Delta table is kept.

Each time a checkpoint is written, Delta Lake automatically cleans up log entries older
than the retention interval. If you set this property to a large enough value, many log
entries are retained. This should not impact performance as operations against the log are
constant time. Operations on history are parallel but will become more expensive as the log
size increases.

<a id="op-5633f52cc4c31abe975081bc"></a>
## materialized_row_commit_version_column_name

`struct_field` · `buoyant_kernel::table_properties::TableProperties::materialized_row_commit_version_column_name` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
materialized_row_commit_version_column_name: Option<String>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L217).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:217`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The name of the internal column that contains the materialized row commit version.

<a id="op-ff04241f4bcc78a06b043f40"></a>
## materialized_row_id_column_name

`struct_field` · `buoyant_kernel::table_properties::TableProperties::materialized_row_id_column_name` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
materialized_row_id_column_name: Option<String>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L214).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:214`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The name of the internal column that contains the materialized row ID.

<a id="op-264a4e05c965ec4a9c5c1f77"></a>
## optimize_write

`struct_field` · `buoyant_kernel::table_properties::TableProperties::optimize_write` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
optimize_write: Option<bool>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L87).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:87`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

true for Delta Lake to automatically optimize the layout of the files for this Delta table
during writes.

<a id="op-eacbbf5decb7e580365d3b3a"></a>
## parquet_compression_codec

`struct_field` · `buoyant_kernel::table_properties::TableProperties::parquet_compression_codec` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
parquet_compression_codec: Option<ParquetCompressionCodec>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L228).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:228`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Compression codec to use when writing new Parquet data and checkpoint files. Connectors
SHOULD honor this property when configuring their Parquet writer. Use
[`TableProperties::compression_codec_or_default`](../operations/buoyant_kernel.table_properties.TableProperties.md#op-5335223ecb0b41b4f0876ee7) to apply the protocol-recommended
fallback ([`ParquetCompressionCodec::Zstd`](../operations/buoyant_kernel.table_properties.ParquetCompressionCodec.md#op-77aaec8e41daa6caa3431be5)) when this field is `None`.

<a id="op-ca05040f1a749cce2c76729b"></a>
## parquet_format_version

`struct_field` · `buoyant_kernel::table_properties::TableProperties::parquet_format_version` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
parquet_format_version: Option<String>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L222).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:222`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The Parquet format version used when writing data files. Valid values are `"1.0.0"`
(DataPageV1) and any `"2.x.x"` such as `"2.12.0"` (DataPageV2). Writers SHOULD default
to `"1.0.0"` when absent. Connectors read this to configure their Parquet writers.

<a id="op-0c0494d758e29a1c4f2d2347"></a>
## random_prefix_length

`struct_field` · `buoyant_kernel::table_properties::TableProperties::random_prefix_length` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
random_prefix_length: Option<std::num::NonZero<u64>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L185).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:185`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The number of characters to use for random file path prefixes. Used when
`delta.randomizeFilePrefixes` is true or when column mapping is enabled.

<a id="op-89ec992e0ffbe566087e455c"></a>
## random_prefix_length

`function` · `buoyant_kernel::table_properties::TableProperties::random_prefix_length` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn random_prefix_length(&self) -> NonZero<usize>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L269).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_properties::TableProperties", "path": "TableProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [248, 1], "end": [287, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:269`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns the number of characters to use for random file path prefixes.
Default: `2`.

<a id="op-c27f73ad2e9f2b82ad440f33"></a>
## randomize_file_prefixes

`struct_field` · `buoyant_kernel::table_properties::TableProperties::randomize_file_prefixes` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
randomize_file_prefixes: Option<bool>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L181).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:181`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

true for Delta to generate a random prefix for a file path instead of partition
information.

For example, this may improve Amazon S3 performance when Delta Lake needs to send very high
volumes of Amazon S3 calls to better partition across S3 servers.

<a id="op-4c6b7c6b4ca69e51ccdb51f7"></a>
## row_tracking_suspended

`struct_field` · `buoyant_kernel::table_properties::TableProperties::row_tracking_suspended` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
row_tracking_suspended: Option<bool>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L211).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:211`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Whether to explicitly suspend generating row tracking metadata during writes even if
row tracking is supported.

<a id="op-6c13ddff681cf8df3b9b4cf7"></a>
## set_transaction_retention_duration

`struct_field` · `buoyant_kernel::table_properties::TableProperties::set_transaction_retention_duration` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
set_transaction_retention_duration: Option<std::time::Duration>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L191).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:191`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The shortest duration within which new snapshots will retain transaction identifiers (for
example, SetTransactions). When a new snapshot sees a transaction identifier older than or
equal to the duration specified by this property, the snapshot considers it expired and
ignores it. The SetTransaction identifier is used when making the writes idempotent.

<a id="op-3edabf603ccdfa43e003e8d4"></a>
## should_randomize_file_prefixes

`function` · `buoyant_kernel::table_properties::TableProperties::should_randomize_file_prefixes` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn should_randomize_file_prefixes(&self) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L263).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_properties::TableProperties", "path": "TableProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [248, 1], "end": [287, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:263`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns whether to emit a random alphanumeric prefix in file paths regardless of column
mapping mode. Default: `false`.

<a id="op-1bd58b3db6b59c8655889f60"></a>
## should_write_stats_as_json

`function` · `buoyant_kernel::table_properties::TableProperties::should_write_stats_as_json` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn should_write_stats_as_json(&self) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L251).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_properties::TableProperties", "path": "TableProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [248, 1], "end": [287, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:251`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns whether to write file statistics as JSON in checkpoints.
Default: `true` per the Delta protocol.

<a id="op-995d7d969190cf7a2f009384"></a>
## should_write_stats_as_struct

`function` · `buoyant_kernel::table_properties::TableProperties::should_write_stats_as_struct` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn should_write_stats_as_struct(&self) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L257).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_properties::TableProperties", "path": "TableProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [248, 1], "end": [287, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:257`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns whether to write file statistics as parsed structs in checkpoints.
Default: `false` per the Delta protocol.

<a id="op-c0da060729b5bb3334c35775"></a>
## target_file_size

`struct_field` · `buoyant_kernel::table_properties::TableProperties::target_file_size` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
target_file_size: Option<std::num::NonZero<u64>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L195).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:195`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The target file size in bytes or higher units for file tuning. For example, 104857600
(bytes) or 100mb.

<a id="op-b31e52fdf8e3bcf906d0a758"></a>
## tune_file_sizes_for_rewrites

`struct_field` · `buoyant_kernel::table_properties::TableProperties::tune_file_sizes_for_rewrites` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
tune_file_sizes_for_rewrites: Option<bool>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L199).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:199`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The target file size in bytes or higher units for file tuning. For example, 104857600
(bytes) or 100mb.

<a id="op-8933b40cd15ef0eabab50a9c"></a>
## unknown_properties

`struct_field` · `buoyant_kernel::table_properties::TableProperties::unknown_properties` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
unknown_properties: std::collections::HashMap<String, String>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L245).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:245`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

any unrecognized properties are passed through and ignored by the parser
