# `datafusion_datasource_json::file_format::JsonSink`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_json.file_format.JsonSink.json).

<a id="op-3f953a32d51aa83fa3596773"></a>
## JsonSink

`struct` · `datafusion_datasource_json::file_format::JsonSink` · datafusion-datasource-json 55.1.0

```rust
struct JsonSink
```

Source: `src/file_format.rs:409`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

Implements [`DataSink`](../operations/datafusion_datasource.sink.DataSink.md#op-9a57c258a80fb4e5a004e0e1) for writing to a Json file.

<a id="op-a1b630e0caa89d81c66c131a"></a>
## Error

`assoc_type` · `datafusion_datasource_json::file_format::JsonSink::Error` · datafusion-datasource-json 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::file_format::JsonSink", "path": "JsonSink"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [531, 1], "end": [553, 2], "filename": "src/file_format.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::JsonSink", "path": "JsonSink"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/file_format.rs:532`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dd1ecb858b320664bb37e2b3"></a>
## config

`function` · `datafusion_datasource_json::file_format::JsonSink::config` · datafusion-datasource-json 55.1.0

```rust
fn config(&self) -> &FileSinkConfig
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::file_format::JsonSink", "path": "JsonSink"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [454, 1], "end": [478, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_sink_config::FileSink", "path": "FileSink"}, "trait_path": "datafusion_datasource::file_sink_config::FileSink"}`

Source: `src/file_format.rs:455`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c0265f3c7cfcb05edbc7e165"></a>
## fmt

`function` · `datafusion_datasource_json::file_format::JsonSink::fmt` · datafusion-datasource-json 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::file_format::JsonSink", "path": "JsonSink"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [416, 1], "end": [420, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file_format.rs:417`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-320c024bee4be85b9d31ce5c"></a>
## fmt_as

`function` · `datafusion_datasource_json::file_format::JsonSink::fmt_as` · datafusion-datasource-json 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::file_format::JsonSink", "path": "JsonSink"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [422, 1], "end": [436, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}, "trait_path": "datafusion_physical_plan::display::DisplayAs"}`

Source: `src/file_format.rs:423`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c809efc082da57934ecf38f0"></a>
## new

`function` · `datafusion_datasource_json::file_format::JsonSink::new` · datafusion-datasource-json 55.1.0

```rust
fn new(config: FileSinkConfig, writer_options: JsonWriterOptions) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::file_format::JsonSink", "path": "JsonSink"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [438, 1], "end": [451, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:440`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

Create from config.

<a id="op-6058155c9c31808a40754161"></a>
## schema

`function` · `datafusion_datasource_json::file_format::JsonSink::schema` · datafusion-datasource-json 55.1.0

```rust
fn schema(&self) -> &SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::file_format::JsonSink", "path": "JsonSink"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [481, 1], "end": [516, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::sink::DataSink", "path": "DataSink"}, "trait_path": "datafusion_datasource::sink::DataSink"}`

Source: `src/file_format.rs:482`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5f00e4075be49f525e174d74"></a>
## spawn_writer_tasks_and_join

`function` · `datafusion_datasource_json::file_format::JsonSink::spawn_writer_tasks_and_join` · datafusion-datasource-json 55.1.0

```rust
async fn spawn_writer_tasks_and_join(&self, context: &Arc<TaskContext>, demux_task: SpawnedTask<Result<()>>, file_stream_rx: DemuxedStreamReceiver, object_store: Arc<dyn ObjectStore>) -> Result<u64>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::file_format::JsonSink", "path": "JsonSink"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [454, 1], "end": [478, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_sink_config::FileSink", "path": "FileSink"}, "trait_path": "datafusion_datasource::file_sink_config::FileSink"}`

Source: `src/file_format.rs:459`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-656a52d391c2087cc4779b35"></a>
## try_from

`function` · `datafusion_datasource_json::file_format::JsonSink::try_from` · datafusion-datasource-json 55.1.0

```rust
fn try_from(value: &datafusion_proto_models::protobuf::JsonSink) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::file_format::JsonSink", "path": "JsonSink"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [531, 1], "end": [553, 2], "filename": "src/file_format.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::JsonSink", "path": "JsonSink"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/file_format.rs:534`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-17d4dc9ce3fea47c52b29f8e"></a>
## try_from_proto

`function` · `datafusion_datasource_json::file_format::JsonSink::try_from_proto` · datafusion-datasource-json 55.1.0

```rust
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &datafusion_physical_plan::proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::file_format::JsonSink", "path": "JsonSink"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [556, 1], "end": [592, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:558`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

Reconstructs a [`DataSinkExec`](../operations/datafusion_datasource.sink.DataSinkExec.md#op-79c64cf00b80b5de2d143f4e) containing a `JsonSink` from protobuf.

<a id="op-3aacc936c4b969672c1b4dcd"></a>
## try_to_proto

`function` · `datafusion_datasource_json::file_format::JsonSink::try_to_proto` · datafusion-datasource-json 55.1.0

```rust
fn try_to_proto(&self, exec: &DataSinkExec, ctx: &datafusion_physical_plan::proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::file_format::JsonSink", "path": "JsonSink"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [481, 1], "end": [516, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::sink::DataSink", "path": "DataSink"}, "trait_path": "datafusion_datasource::sink::DataSink"}`

Source: `src/file_format.rs:495`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-113787cf42c0522caafb7942"></a>
## write_all

`function` · `datafusion_datasource_json::file_format::JsonSink::write_all` · datafusion-datasource-json 55.1.0

```rust
async fn write_all(&self, data: SendableRecordBatchStream, context: &Arc<TaskContext>) -> Result<u64>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::file_format::JsonSink", "path": "JsonSink"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [481, 1], "end": [516, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::sink::DataSink", "path": "DataSink"}, "trait_path": "datafusion_datasource::sink::DataSink"}`

Source: `src/file_format.rs:486`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8cc3875eb2d9058faf47c356"></a>
## writer_options

`function` · `datafusion_datasource_json::file_format::JsonSink::writer_options` · datafusion-datasource-json 55.1.0

```rust
fn writer_options(&self) -> &JsonWriterOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::file_format::JsonSink", "path": "JsonSink"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [438, 1], "end": [451, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:448`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

Retrieve the writer options
