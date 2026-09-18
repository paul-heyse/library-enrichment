# `deltalake_core::datafile::writer::DeltaWriter`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.datafile.writer.DeltaWriter.json).

<a id="op-93547c21542fa50fb0da1dfb"></a>
## DeltaWriter

`struct` · `deltalake_core::datafile::writer::DeltaWriter` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct DeltaWriter
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L264).

Source: `crates/core/src/datafile/writer.rs:264`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A parquet writer implementation tailored to the needs of writing data to a delta table.

<a id="op-060df4ace4f656efc18ad2e7"></a>
## abort

`function` · `deltalake_core::datafile::writer::DeltaWriter::abort` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn abort(self) -> DeltaResult<()>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L413).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::writer::DeltaWriter", "path": "DeltaWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [279, 1], "end": [480, 2], "filename": "crates/core/src/datafile/writer.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/datafile/writer.rs:413`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Abandon the writer, aborting every partition's in-progress multipart
upload. Already-completed files are left as orphans for vacuum.

<a id="op-71d9f7e0ea284128d30febb9"></a>
## close

`function` · `deltalake_core::datafile::writer::DeltaWriter::close` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn close(self) -> DeltaResult<Vec<Add>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L444).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::writer::DeltaWriter", "path": "DeltaWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [279, 1], "end": [480, 2], "filename": "crates/core/src/datafile/writer.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/datafile/writer.rs:444`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Close the writer and get the new [Add](../operations/deltalake_core.kernel.models.actions.Add.md#op-165aeba4e3bd6a2b7f853b9a) actions.

This will flush all remaining data.

<a id="op-bbc6c414fc7a51e712983011"></a>
## new

`function` · `deltalake_core::datafile::writer::DeltaWriter::new` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new(object_store: ObjectStoreRef, config: WriterConfig) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L281).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::writer::DeltaWriter", "path": "DeltaWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [279, 1], "end": [480, 2], "filename": "crates/core/src/datafile/writer.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/datafile/writer.rs:281`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a new instance of [`DeltaWriter`](../operations/deltalake_core.datafile.writer.DeltaWriter.md#op-93547c21542fa50fb0da1dfb)

<a id="op-1eda24f9adb74b77f6149374"></a>
## with_writer_properties

`function` · `deltalake_core::datafile::writer::DeltaWriter::with_writer_properties` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_writer_properties(self, writer_properties: WriterProperties) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L292).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::writer::DeltaWriter", "path": "DeltaWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [279, 1], "end": [480, 2], "filename": "crates/core/src/datafile/writer.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/datafile/writer.rs:292`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Apply custom writer_properties to the underlying parquet writer

<a id="op-3daf65944e5035044adaa250"></a>
## write

`function` · `deltalake_core::datafile::writer::DeltaWriter::write` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn write(&mut self, batch: &RecordBatch) -> DeltaResult<()>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L391).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::writer::DeltaWriter", "path": "DeltaWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [279, 1], "end": [480, 2], "filename": "crates/core/src/datafile/writer.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/datafile/writer.rs:391`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Buffers record batches in-memory per partition up to appx. `target_file_size` for a partition.
Flushes data to storage once a full file can be written.

The `close` method has to be invoked to write all data still buffered
and get the list of all written files.

<a id="op-84e1095892ce864ee00d3e54"></a>
## write_all

`function` · `deltalake_core::datafile::writer::DeltaWriter::write_all` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn write_all(Box<self>, batches: BatchStream) -> DeltaResult<Vec<Add>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L515).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::writer::DeltaWriter", "path": "DeltaWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [514, 1], "end": [533, 2], "filename": "crates/core/src/datafile/writer.rs"}, "trait": {"args": null, "id": "deltalake_core::datafile::DeltaDataWriter", "path": "DeltaDataWriter"}, "trait_path": "deltalake_core::datafile::DeltaDataWriter"}`

Source: `crates/core/src/datafile/writer.rs:515`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-741af002b3f8dccf77f8c7af"></a>
## write_partition

`function` · `deltalake_core::datafile::writer::DeltaWriter::write_partition` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn write_partition(&mut self, record_batch: RecordBatch, partition_values: &IndexMap<String, Scalar>) -> DeltaResult<()>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L366).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::writer::DeltaWriter", "path": "DeltaWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [279, 1], "end": [480, 2], "filename": "crates/core/src/datafile/writer.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/datafile/writer.rs:366`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Write a batch to the partition induced by `partition_values`. The batch must
be pre-partitioned (all rows in one partition) but still include the
partition columns; they are stripped before encoding.

<a id="op-523f3ca098188ab34ec59598"></a>
## write_plan

`function` · `deltalake_core::datafile::writer::DeltaWriter::write_plan` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn write_plan(Box<self>, session: &dyn Session, plan: Arc<dyn ExecutionPlan>) -> DeltaResult<Vec<Add>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/datafusion_ext.rs#L75).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::writer::DeltaWriter", "path": "super::writer::DeltaWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 1], "end": [83, 2], "filename": "crates/core/src/datafile/datafusion_ext.rs"}, "trait": {"args": null, "id": "deltalake_core::datafile::datafusion_ext::DeltaDataWriterExt", "path": "DeltaDataWriterExt"}, "trait_path": "deltalake_core::datafile::datafusion_ext::DeltaDataWriterExt"}`

Source: `crates/core/src/datafile/datafusion_ext.rs:75`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1933086dd3d66a1dd0419450"></a>
## config

`struct_field` · `deltalake_core::datafile::writer::DeltaWriter::config` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
config: WriterConfig
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L268).

Source: `crates/core/src/datafile/writer.rs:268`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

configuration for the writers

<a id="op-5ef6dcf69a5a8580fb1dff32"></a>
## file_schema

`struct_field` · `deltalake_core::datafile::writer::DeltaWriter::file_schema` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
file_schema: arrow_schema::SchemaRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L274).

Source: `crates/core/src/datafile/writer.rs:274`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Physical file schema (table schema with partition columns removed), derived
once at construction. The per-batch write paths read this instead of calling
`WriterConfig::file_schema()`, which reallocates the schema on every call.
Invariant: it depends only on the config's table schema + partition columns,
so any future setter for those must refresh this field.

<a id="op-b97cf1c1ad38902b4d45f7d3"></a>
## object_store

`struct_field` · `deltalake_core::datafile::writer::DeltaWriter::object_store` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
object_store: logstore::ObjectStoreRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L266).

Source: `crates/core/src/datafile/writer.rs:266`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

An object store pointing at Delta table root

<a id="op-6110a7fcb4a41c14acdff29c"></a>
## partition_writers

`struct_field` · `deltalake_core::datafile::writer::DeltaWriter::partition_writers` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
partition_writers: std::collections::HashMap<object_store::path::Path, PartitionWriter>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L276).

Source: `crates/core/src/datafile/writer.rs:276`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

partition writers for individual partitions
