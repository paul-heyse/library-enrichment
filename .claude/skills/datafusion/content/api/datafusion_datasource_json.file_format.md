# `datafusion_datasource_json::file_format`

Crate `datafusion-datasource-json` · 5 public items · structured records in [`model/datafusion_datasource_json.file_format.json`](../model/datafusion_datasource_json.file_format.json)

## JsonDecoder

`struct` · `datafusion_datasource_json::file_format::JsonDecoder`

Also reachable as `datafusion::datasource::file_format::json::JsonDecoder`, `datafusion_datasource_json::JsonDecoder`

```rust
struct JsonDecoder
```

**Implements**: `datafusion_datasource::decoder::Decoder`

**Derives**: Debug

**Methods** (1)

```rust
fn new(decoder: json::reader::Decoder) -> Self
```

**via `datafusion_datasource::decoder::Decoder`**

```rust
fn can_flush_early(&self) -> bool
fn decode(&mut self, buf: &[u8]) -> Result<usize, ArrowError>
fn flush(&mut self) -> Result<Option<RecordBatch>, ArrowError>
```

---

## JsonFormat

`struct` · `datafusion_datasource_json::file_format::JsonFormat`

Also reachable as `datafusion::datasource::file_format::json::JsonFormat`, `datafusion_datasource_json::JsonFormat`

```rust
struct JsonFormat
```

**Implements**: `datafusion_datasource::file_format::FileFormat`

**Derives**: Debug, Default

**Methods** (6)

```rust
fn is_newline_delimited(&self) -> bool
fn options(&self) -> &JsonOptions
fn with_file_compression_type(self, file_compression_type: FileCompressionType) -> Self
fn with_newline_delimited(self, newline_delimited: bool) -> Self
fn with_options(self, options: JsonOptions) -> Self
fn with_schema_infer_max_rec(self, max_rec: usize) -> Self
```

**via `datafusion_datasource::file_format::FileFormat`**

```rust
fn compression_type(&self) -> Option<FileCompressionType>
async fn create_physical_plan(&self, _state: &dyn Session, conf: FileScanConfig) -> Result<Arc<dyn ExecutionPlan>>
async fn create_writer_physical_plan(&self, input: Arc<dyn ExecutionPlan>, _state: &dyn Session, conf: FileSinkConfig, order_requirements: Option<LexRequirement>) -> Result<Arc<dyn ExecutionPlan>>
fn file_source(&self, table_schema: TableSchema) -> Arc<dyn FileSource>
fn get_ext(&self) -> String
fn get_ext_with_compression(&self, file_compression_type: &FileCompressionType) -> Result<String>
async fn infer_schema(&self, _state: &dyn Session, store: &Arc<dyn ObjectStore>, objects: &[ObjectMeta]) -> Result<SchemaRef>
async fn infer_stats(&self, _state: &dyn Session, _store: &Arc<dyn ObjectStore>, table_schema: SchemaRef, _object: &ObjectMeta) -> Result<Statistics>
```

JSON `FileFormat` implementation supporting both line-delimited and array formats.

# Supported Formats

## Line-Delimited JSON (default, `newline_delimited = true`)
```text
{"key1": 1, "key2": "val"}
{"key1": 2, "key2": "vals"}
```

## JSON Array Format (`newline_delimited = false`)
```text
[
    {"key1": 1, "key2": "val"},
    {"key1": 2, "key2": "vals"}
]
```

Note: JSON array format is processed using streaming conversion,
which is memory-efficient even for large files.

---

## JsonFormatFactory

`struct` · `datafusion_datasource_json::file_format::JsonFormatFactory`

Also reachable as `datafusion::datasource::file_format::json::JsonFormatFactory`, `datafusion_datasource_json::JsonFormatFactory`

```rust
struct JsonFormatFactory
```

**Fields**: `options`

**Implements**: `datafusion_common::file_options::file_type::GetExt`, `datafusion_datasource::file_format::FileFormatFactory`

**Derives**: Debug, Default

**Methods** (2)

```rust
fn new() -> Self
fn new_with_options(options: JsonOptions) -> Self
```

**via `datafusion_common::file_options::file_type::GetExt`**

```rust
fn get_ext(&self) -> String
```

**via `datafusion_datasource::file_format::FileFormatFactory`**

```rust
fn create(&self, state: &dyn Session, format_options: &HashMap<String, String>) -> Result<Arc<dyn FileFormat>>
fn default(&self) -> Arc<dyn FileFormat>
```

Factory struct used to create [JsonFormat]

---

## JsonSerializer

`struct` · `datafusion_datasource_json::file_format::JsonSerializer`

Also reachable as `datafusion::datasource::file_format::json::JsonSerializer`, `datafusion_datasource_json::JsonSerializer`

```rust
struct JsonSerializer
```

**Implements**: `datafusion_datasource::write::BatchSerializer`

**Derives**: Default

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_datasource::write::BatchSerializer`**

```rust
fn serialize(&self, batch: RecordBatch, _initial: bool) -> Result<Bytes>
```

Define a struct for serializing Json records to a stream

---

## JsonSink

`struct` · `datafusion_datasource_json::file_format::JsonSink`

Also reachable as `datafusion::datasource::file_format::json::JsonSink`, `datafusion_datasource_json::JsonSink`

```rust
struct JsonSink
```

**Implements**: `core::convert::TryFrom`, `datafusion_datasource::file_sink_config::FileSink`, `datafusion_datasource::sink::DataSink`, `datafusion_physical_plan::display::DisplayAs`

**Derives**: Debug

**Methods** (3)

```rust
fn new(config: FileSinkConfig, writer_options: JsonWriterOptions) -> Self
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &datafusion_physical_plan::proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
fn writer_options(&self) -> &JsonWriterOptions
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: &datafusion_proto_models::protobuf::JsonSink) -> Result<Self>
```

**via `datafusion_datasource::file_sink_config::FileSink`**

```rust
fn config(&self) -> &FileSinkConfig
async fn spawn_writer_tasks_and_join(&self, context: &Arc<TaskContext>, demux_task: SpawnedTask<Result<()>>, file_stream_rx: DemuxedStreamReceiver, object_store: Arc<dyn ObjectStore>) -> Result<u64>
```

**via `datafusion_datasource::sink::DataSink`**

```rust
fn schema(&self) -> &SchemaRef
fn try_to_proto(&self, exec: &DataSinkExec, ctx: &datafusion_physical_plan::proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
async fn write_all(&self, data: SendableRecordBatchStream, context: &Arc<TaskContext>) -> Result<u64>
```

**via `datafusion_physical_plan::display::DisplayAs`**

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implements [`DataSink`] for writing to a Json file.

---
