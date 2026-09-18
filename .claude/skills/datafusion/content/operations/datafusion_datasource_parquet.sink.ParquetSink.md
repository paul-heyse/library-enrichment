# `datafusion_datasource_parquet::sink::ParquetSink`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_parquet.sink.ParquetSink.json).

<a id="op-520af00135668daa5e2f19c1"></a>
## ParquetSink

`struct` · `datafusion_datasource_parquet::sink::ParquetSink` · datafusion-datasource-parquet 55.1.0

```rust
struct ParquetSink
```

Source: `src/sink.rs:79`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Implements [`DataSink`](../operations/datafusion_datasource.sink.DataSink.md#op-9a57c258a80fb4e5a004e0e1) for writing to a parquet file.

<a id="op-118cac3214032eca3aef35fa"></a>
## Error

`assoc_type` · `datafusion_datasource_parquet::sink::ParquetSink::Error` · datafusion-datasource-parquet 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::sink::ParquetSink", "path": "ParquetSink"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [455, 1], "end": [477, 2], "filename": "src/sink.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::ParquetSink", "path": "ParquetSink"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/sink.rs:456`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c2bb84a7c8c546d9b5bb76e6"></a>
## config

`function` · `datafusion_datasource_parquet::sink::ParquetSink::config` · datafusion-datasource-parquet 55.1.0

```rust
fn config(&self) -> &FileSinkConfig
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::sink::ParquetSink", "path": "ParquetSink"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [258, 1], "end": [398, 2], "filename": "src/sink.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_sink_config::FileSink", "path": "FileSink"}, "trait_path": "datafusion_datasource::file_sink_config::FileSink"}`

Source: `src/sink.rs:259`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a13663f02f2afc1975c8e6ed"></a>
## fmt

`function` · `datafusion_datasource_parquet::sink::ParquetSink::fmt` · datafusion-datasource-parquet 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::sink::ParquetSink", "path": "ParquetSink"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 1], "end": [97, 2], "filename": "src/sink.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sink.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-21fd858bd9edde275f1c5428"></a>
## fmt_as

`function` · `datafusion_datasource_parquet::sink::ParquetSink::fmt_as` · datafusion-datasource-parquet 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::sink::ParquetSink", "path": "ParquetSink"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [99, 1], "end": [113, 2], "filename": "src/sink.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}, "trait_path": "datafusion_physical_plan::display::DisplayAs"}`

Source: `src/sink.rs:100`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8d39ebdbd59cda9114a07faa"></a>
## metrics

`function` · `datafusion_datasource_parquet::sink::ParquetSink::metrics` · datafusion-datasource-parquet 55.1.0

```rust
fn metrics(&self) -> Option<MetricsSet>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::sink::ParquetSink", "path": "ParquetSink"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [401, 1], "end": [440, 2], "filename": "src/sink.rs"}, "trait": {"args": null, "id": "datafusion_datasource::sink::DataSink", "path": "DataSink"}, "trait_path": "datafusion_datasource::sink::DataSink"}`

Source: `src/sink.rs:402`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e7ee82cff95931c553b51003"></a>
## new

`function` · `datafusion_datasource_parquet::sink::ParquetSink::new` · datafusion-datasource-parquet 55.1.0

```rust
fn new(config: FileSinkConfig, parquet_options: TableParquetOptions) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::sink::ParquetSink", "path": "ParquetSink"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [115, 1], "end": [210, 2], "filename": "src/sink.rs"}, "trait": null, "trait_path": null}`

Source: `src/sink.rs:117`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Create from config.

<a id="op-99281c3f5069df86eac308c5"></a>
## parquet_options

`function` · `datafusion_datasource_parquet::sink::ParquetSink::parquet_options` · datafusion-datasource-parquet 55.1.0

```rust
fn parquet_options(&self) -> &TableParquetOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::sink::ParquetSink", "path": "ParquetSink"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [115, 1], "end": [210, 2], "filename": "src/sink.rs"}, "trait": null, "trait_path": null}`

Source: `src/sink.rs:207`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Parquet options

<a id="op-8f278ba3609192011785c670"></a>
## schema

`function` · `datafusion_datasource_parquet::sink::ParquetSink::schema` · datafusion-datasource-parquet 55.1.0

```rust
fn schema(&self) -> &SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::sink::ParquetSink", "path": "ParquetSink"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [401, 1], "end": [440, 2], "filename": "src/sink.rs"}, "trait": {"args": null, "id": "datafusion_datasource::sink::DataSink", "path": "DataSink"}, "trait_path": "datafusion_datasource::sink::DataSink"}`

Source: `src/sink.rs:406`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-24e8b6eb8abfdc9c540b06c6"></a>
## spawn_writer_tasks_and_join

`function` · `datafusion_datasource_parquet::sink::ParquetSink::spawn_writer_tasks_and_join` · datafusion-datasource-parquet 55.1.0

```rust
async fn spawn_writer_tasks_and_join(&self, context: &Arc<TaskContext>, demux_task: SpawnedTask<Result<()>>, file_stream_rx: DemuxedStreamReceiver, object_store: Arc<dyn ObjectStore>) -> Result<u64>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::sink::ParquetSink", "path": "ParquetSink"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [258, 1], "end": [398, 2], "filename": "src/sink.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_sink_config::FileSink", "path": "FileSink"}, "trait_path": "datafusion_datasource::file_sink_config::FileSink"}`

Source: `src/sink.rs:263`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3c445d385e82c58f40ee38ff"></a>
## try_from

`function` · `datafusion_datasource_parquet::sink::ParquetSink::try_from` · datafusion-datasource-parquet 55.1.0

```rust
fn try_from(value: &datafusion_proto_models::protobuf::ParquetSink) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::sink::ParquetSink", "path": "ParquetSink"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [455, 1], "end": [477, 2], "filename": "src/sink.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::ParquetSink", "path": "ParquetSink"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/sink.rs:458`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-15a7d0394de90374fff8eb36"></a>
## try_from_proto

`function` · `datafusion_datasource_parquet::sink::ParquetSink::try_from_proto` · datafusion-datasource-parquet 55.1.0

```rust
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &datafusion_physical_plan::proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::sink::ParquetSink", "path": "ParquetSink"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [480, 1], "end": [516, 2], "filename": "src/sink.rs"}, "trait": null, "trait_path": null}`

Source: `src/sink.rs:482`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Reconstructs a [`DataSinkExec`](../operations/datafusion_datasource.sink.DataSinkExec.md#op-79c64cf00b80b5de2d143f4e) containing a `ParquetSink` from protobuf.

<a id="op-95a6a542d10e288e3256ce99"></a>
## try_to_proto

`function` · `datafusion_datasource_parquet::sink::ParquetSink::try_to_proto` · datafusion-datasource-parquet 55.1.0

```rust
fn try_to_proto(&self, exec: &DataSinkExec, ctx: &datafusion_physical_plan::proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::sink::ParquetSink", "path": "ParquetSink"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [401, 1], "end": [440, 2], "filename": "src/sink.rs"}, "trait": {"args": null, "id": "datafusion_datasource::sink::DataSink", "path": "DataSink"}, "trait_path": "datafusion_datasource::sink::DataSink"}`

Source: `src/sink.rs:419`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-59ffdfe227ed6b0b92c183ba"></a>
## with_sorting_columns

`function` · `datafusion_datasource_parquet::sink::ParquetSink::with_sorting_columns` · datafusion-datasource-parquet 55.1.0

```rust
fn with_sorting_columns(self, sorting_columns: Option<Vec<SortingColumn>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::sink::ParquetSink", "path": "ParquetSink"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [115, 1], "end": [210, 2], "filename": "src/sink.rs"}, "trait": null, "trait_path": null}`

Source: `src/sink.rs:128`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Set sorting columns for the Parquet file metadata.

<a id="op-b8782cf53687cf25a2aabbb5"></a>
## write_all

`function` · `datafusion_datasource_parquet::sink::ParquetSink::write_all` · datafusion-datasource-parquet 55.1.0

```rust
async fn write_all(&self, data: SendableRecordBatchStream, context: &Arc<TaskContext>) -> Result<u64>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::sink::ParquetSink", "path": "ParquetSink"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [401, 1], "end": [440, 2], "filename": "src/sink.rs"}, "trait": {"args": null, "id": "datafusion_datasource::sink::DataSink", "path": "DataSink"}, "trait_path": "datafusion_datasource::sink::DataSink"}`

Source: `src/sink.rs:410`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a821bd4430810d93370be47c"></a>
## written

`function` · `datafusion_datasource_parquet::sink::ParquetSink::written` · datafusion-datasource-parquet 55.1.0

```rust
fn written(&self) -> HashMap<Path, ParquetMetaData>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::sink::ParquetSink", "path": "ParquetSink"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [115, 1], "end": [210, 2], "filename": "src/sink.rs"}, "trait": null, "trait_path": null}`

Source: `src/sink.rs:138`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Retrieve the file metadata for the written files, keyed to the path
which may be partitioned (in the case of hive style partitioning).
