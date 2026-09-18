# `datafusion_datasource_csv::file_format::CsvSink`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_csv.file_format.CsvSink.json).

<a id="op-a7f78a8a87b0619f6f459e83"></a>
## CsvSink

`struct` · `datafusion_datasource_csv::file_format::CsvSink` · datafusion-datasource-csv 55.1.0

```rust
struct CsvSink
```

Source: `src/file_format.rs:739`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

Implements [`DataSink`](../operations/datafusion_datasource.sink.DataSink.md#op-9a57c258a80fb4e5a004e0e1) for writing to a CSV file.

<a id="op-2ea2992a7a2b81b3c9f4b103"></a>
## Error

`assoc_type` · `datafusion_datasource_csv::file_format::CsvSink::Error` · datafusion-datasource-csv 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvSink", "path": "CsvSink"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [866, 1], "end": [888, 2], "filename": "src/file_format.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::CsvSink", "path": "CsvSink"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/file_format.rs:867`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-de2f20e15480ceaca2b71743"></a>
## config

`function` · `datafusion_datasource_csv::file_format::CsvSink::config` · datafusion-datasource-csv 55.1.0

```rust
fn config(&self) -> &FileSinkConfig
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvSink", "path": "CsvSink"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [783, 1], "end": [813, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_sink_config::FileSink", "path": "FileSink"}, "trait_path": "datafusion_datasource::file_sink_config::FileSink"}`

Source: `src/file_format.rs:784`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c59c2e5ea7174231c9655e9f"></a>
## fmt

`function` · `datafusion_datasource_csv::file_format::CsvSink::fmt` · datafusion-datasource-csv 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvSink", "path": "CsvSink"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [745, 1], "end": [749, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file_format.rs:746`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0c0811ff4c5d35010895bdd0"></a>
## fmt_as

`function` · `datafusion_datasource_csv::file_format::CsvSink::fmt_as` · datafusion-datasource-csv 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvSink", "path": "CsvSink"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [751, 1], "end": [765, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}, "trait_path": "datafusion_physical_plan::display::DisplayAs"}`

Source: `src/file_format.rs:752`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d5655dfab2b4703e82991eaa"></a>
## new

`function` · `datafusion_datasource_csv::file_format::CsvSink::new` · datafusion-datasource-csv 55.1.0

```rust
fn new(config: FileSinkConfig, writer_options: CsvWriterOptions) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvSink", "path": "CsvSink"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [767, 1], "end": [780, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:769`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

Create from config.

<a id="op-d139ff9da05e1494f357be5d"></a>
## schema

`function` · `datafusion_datasource_csv::file_format::CsvSink::schema` · datafusion-datasource-csv 55.1.0

```rust
fn schema(&self) -> &SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvSink", "path": "CsvSink"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [816, 1], "end": [851, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::sink::DataSink", "path": "DataSink"}, "trait_path": "datafusion_datasource::sink::DataSink"}`

Source: `src/file_format.rs:817`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6a7550da7dfb016f421f9c33"></a>
## spawn_writer_tasks_and_join

`function` · `datafusion_datasource_csv::file_format::CsvSink::spawn_writer_tasks_and_join` · datafusion-datasource-csv 55.1.0

```rust
async fn spawn_writer_tasks_and_join(&self, context: &Arc<TaskContext>, demux_task: SpawnedTask<Result<()>>, file_stream_rx: DemuxedStreamReceiver, object_store: Arc<dyn ObjectStore>) -> Result<u64>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvSink", "path": "CsvSink"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [783, 1], "end": [813, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_sink_config::FileSink", "path": "FileSink"}, "trait_path": "datafusion_datasource::file_sink_config::FileSink"}`

Source: `src/file_format.rs:788`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-39cbfdd8ecc1df5c5a45c139"></a>
## try_from

`function` · `datafusion_datasource_csv::file_format::CsvSink::try_from` · datafusion-datasource-csv 55.1.0

```rust
fn try_from(value: &datafusion_proto_models::protobuf::CsvSink) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvSink", "path": "CsvSink"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [866, 1], "end": [888, 2], "filename": "src/file_format.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::CsvSink", "path": "CsvSink"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/file_format.rs:869`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-94af3e84f4bf141452227748"></a>
## try_from_proto

`function` · `datafusion_datasource_csv::file_format::CsvSink::try_from_proto` · datafusion-datasource-csv 55.1.0

```rust
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &datafusion_physical_plan::proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvSink", "path": "CsvSink"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [891, 1], "end": [927, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:893`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

Reconstructs a [`DataSinkExec`](../operations/datafusion_datasource.sink.DataSinkExec.md#op-79c64cf00b80b5de2d143f4e) containing a `CsvSink` from protobuf.

<a id="op-91a0c11309684e8ece65d74d"></a>
## try_to_proto

`function` · `datafusion_datasource_csv::file_format::CsvSink::try_to_proto` · datafusion-datasource-csv 55.1.0

```rust
fn try_to_proto(&self, exec: &DataSinkExec, ctx: &datafusion_physical_plan::proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvSink", "path": "CsvSink"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [816, 1], "end": [851, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::sink::DataSink", "path": "DataSink"}, "trait_path": "datafusion_datasource::sink::DataSink"}`

Source: `src/file_format.rs:830`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e71bd6d74c86860c63fa7ae4"></a>
## write_all

`function` · `datafusion_datasource_csv::file_format::CsvSink::write_all` · datafusion-datasource-csv 55.1.0

```rust
async fn write_all(&self, data: SendableRecordBatchStream, context: &Arc<TaskContext>) -> Result<u64>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvSink", "path": "CsvSink"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [816, 1], "end": [851, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::sink::DataSink", "path": "DataSink"}, "trait_path": "datafusion_datasource::sink::DataSink"}`

Source: `src/file_format.rs:821`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b7ec23bda3b6cbc7b4a72434"></a>
## writer_options

`function` · `datafusion_datasource_csv::file_format::CsvSink::writer_options` · datafusion-datasource-csv 55.1.0

```rust
fn writer_options(&self) -> &CsvWriterOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvSink", "path": "CsvSink"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [767, 1], "end": [780, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:777`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

Retrieve the writer options
