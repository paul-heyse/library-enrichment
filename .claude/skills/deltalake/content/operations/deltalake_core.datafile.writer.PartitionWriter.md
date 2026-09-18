# `deltalake_core::datafile::writer::PartitionWriter`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.datafile.writer.PartitionWriter.json).

<a id="op-4307557303e40b4caa7d4a51"></a>
## PartitionWriter

`struct` · `deltalake_core::datafile::writer::PartitionWriter` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct PartitionWriter
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L734).

Source: `crates/core/src/datafile/writer.rs:734`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Partition writer implementation
This writer takes in table data as RecordBatches and writes it out to partitioned parquet files.
It buffers data in memory until it reaches a certain size, then writes it out to optimize file sizes.
When you complete writing you get back a list of Add actions that can be used to update the Delta table commit log.

<a id="op-25ffe997486a61681b5dc93c"></a>
## abort

`function` · `deltalake_core::datafile::writer::PartitionWriter::abort` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn abort(Box<self>) -> DeltaResult<()>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L984).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::writer::PartitionWriter", "path": "PartitionWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [975, 1], "end": [987, 2], "filename": "crates/core/src/datafile/writer.rs"}, "trait": {"args": null, "id": "deltalake_core::datafile::DataFileWriter", "path": "DataFileWriter"}, "trait_path": "deltalake_core::datafile::DataFileWriter"}`

Source: `crates/core/src/datafile/writer.rs:984`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e21f457d19d09d315367fbc1"></a>
## abort

`function` · `deltalake_core::datafile::writer::PartitionWriter::abort` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn abort(self) -> DeltaResult<()>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L963).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::writer::PartitionWriter", "path": "PartitionWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [750, 1], "end": [970, 2], "filename": "crates/core/src/datafile/writer.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/datafile/writer.rs:963`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Abandon the writer: let in-flight size-roll uploads finish (cancelling
mid-upload would leak parts vacuum cannot see; completed files are
orphans it can reclaim), then abort the open file's multipart upload.

<a id="op-a2fd4ead8e955a054a16f39d"></a>
## close

`function` · `deltalake_core::datafile::writer::PartitionWriter::close` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn close(self) -> DeltaResult<Vec<Add>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L910).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::writer::PartitionWriter", "path": "PartitionWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [750, 1], "end": [970, 2], "filename": "crates/core/src/datafile/writer.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/datafile/writer.rs:910`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Close the writer and get the new [Add](../operations/deltalake_core.kernel.models.actions.Add.md#op-165aeba4e3bd6a2b7f853b9a) actions.

This will flush any remaining data and collect all Add actions from background tasks.

<a id="op-d0d9d34fa68200ba57eb3054"></a>
## close

`function` · `deltalake_core::datafile::writer::PartitionWriter::close` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn close(Box<self>) -> DeltaResult<Vec<Add>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L980).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::writer::PartitionWriter", "path": "PartitionWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [975, 1], "end": [987, 2], "filename": "crates/core/src/datafile/writer.rs"}, "trait": {"args": null, "id": "deltalake_core::datafile::DataFileWriter", "path": "DataFileWriter"}, "trait_path": "deltalake_core::datafile::DataFileWriter"}`

Source: `crates/core/src/datafile/writer.rs:980`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f2e8348c2aa87c42e8a9a1e7"></a>
## try_with_config

`function` · `deltalake_core::datafile::writer::PartitionWriter::try_with_config` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_with_config(object_store: ObjectStoreRef, config: PartitionWriterConfig, num_indexed_cols: DataSkippingNumIndexedCols, stats_columns: Option<Vec<String>>) -> DeltaResult<Self>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L752).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::writer::PartitionWriter", "path": "PartitionWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [750, 1], "end": [970, 2], "filename": "crates/core/src/datafile/writer.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/datafile/writer.rs:752`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a new instance of [`PartitionWriter`](../operations/deltalake_core.datafile.writer.PartitionWriter.md#op-4307557303e40b4caa7d4a51) from [`PartitionWriterConfig`](../operations/deltalake_core.datafile.writer.PartitionWriterConfig.md#op-a2c0d2ea1c4a230409d99d11)

<a id="op-37a2ab5d34e1f5e253bb3e9a"></a>
## write

`function` · `deltalake_core::datafile::writer::PartitionWriter::write` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn write(&mut self, batch: &RecordBatch) -> DeltaResult<()>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L841).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::writer::PartitionWriter", "path": "PartitionWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [750, 1], "end": [970, 2], "filename": "crates/core/src/datafile/writer.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/datafile/writer.rs:841`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Buffers record batches in-memory up to appx. `target_file_size`.
Flushes data to storage once a full file can be written.

The `close` method has to be invoked to write all data still buffered
and get the list of all written files.

<a id="op-97a9d625c4953c7e1278b119"></a>
## write

`function` · `deltalake_core::datafile::writer::PartitionWriter::write` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn write(&mut self, batch: &RecordBatch) -> DeltaResult<()>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L976).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::writer::PartitionWriter", "path": "PartitionWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [975, 1], "end": [987, 2], "filename": "crates/core/src/datafile/writer.rs"}, "trait": {"args": null, "id": "deltalake_core::datafile::DataFileWriter", "path": "DataFileWriter"}, "trait_path": "deltalake_core::datafile::DataFileWriter"}`

Source: `crates/core/src/datafile/writer.rs:976`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a8108adb01c914def72ff5a2"></a>
## config

`struct_field` · `deltalake_core::datafile::writer::PartitionWriter::config` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
config: PartitionWriterConfig
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L737).

Source: `crates/core/src/datafile/writer.rs:737`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e6635f128c6d8b61e28218f3"></a>
## in_flight_writers

`struct_field` · `deltalake_core::datafile::writer::PartitionWriter::in_flight_writers` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
in_flight_writers: tokio::task::JoinSet<errors::DeltaResult<(object_store::path::Path, usize, parquet::file::metadata::ParquetMetaData)>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L744).

Source: `crates/core/src/datafile/writer.rs:744`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7047da3301f5475df0e996cd"></a>
## num_indexed_cols

`struct_field` · `deltalake_core::datafile::writer::PartitionWriter::num_indexed_cols` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
num_indexed_cols: delta_kernel::table_properties::DataSkippingNumIndexedCols
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L741).

Source: `crates/core/src/datafile/writer.rs:741`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Num index cols to collect stats for

<a id="op-20cfb9c2c2b1e6e6f2fc231f"></a>
## object_store

`struct_field` · `deltalake_core::datafile::writer::PartitionWriter::object_store` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
object_store: logstore::ObjectStoreRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L735).

Source: `crates/core/src/datafile/writer.rs:735`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bd9ee8373c092edf9eff252f"></a>
## part_counter

`struct_field` · `deltalake_core::datafile::writer::PartitionWriter::part_counter` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
part_counter: usize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L739).

Source: `crates/core/src/datafile/writer.rs:739`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4dbf544c889daa3d1f433414"></a>
## rolled_bytes

`struct_field` · `deltalake_core::datafile::writer::PartitionWriter::rolled_bytes` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
rolled_bytes: usize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L747).

Source: `crates/core/src/datafile/writer.rs:747`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Approximate encoded size of files already rolled to background upload;
keeps `buffered_size` monotonic across rolls.

<a id="op-ee000bbc731fa9e087b03e41"></a>
## stats_columns

`struct_field` · `deltalake_core::datafile::writer::PartitionWriter::stats_columns` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
stats_columns: Option<Vec<String>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L743).

Source: `crates/core/src/datafile/writer.rs:743`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Stats columns, specific columns to collect stats from, takes precedence over num_indexed_cols

<a id="op-495a772bca1ac6c82e5b6dc6"></a>
## writer

`struct_field` · `deltalake_core::datafile::writer::PartitionWriter::writer` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
writer: LazyArrowWriter
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L738).

Source: `crates/core/src/datafile/writer.rs:738`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4b07d929df0bd7e3f9797596"></a>
## writer_id

`struct_field` · `deltalake_core::datafile::writer::PartitionWriter::writer_id` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
writer_id: uuid::Uuid
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L736).

Source: `crates/core/src/datafile/writer.rs:736`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
