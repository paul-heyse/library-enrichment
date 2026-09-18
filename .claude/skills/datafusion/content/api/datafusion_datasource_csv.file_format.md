# `datafusion_datasource_csv::file_format`

Crate `datafusion-datasource-csv` · 5 public items · structured records in [`model/datafusion_datasource_csv.file_format.json`](../model/datafusion_datasource_csv.file_format.json)

## CsvDecoder

`struct` · `datafusion_datasource_csv::file_format::CsvDecoder`

Also reachable as `datafusion::datasource::file_format::csv::CsvDecoder`, `datafusion_datasource_csv::CsvDecoder`

```rust
struct CsvDecoder
```

**Implements**: `datafusion_datasource::decoder::Decoder`

**Derives**: Debug

**Methods** (1)

```rust
fn new(decoder: arrow::csv::reader::Decoder) -> Self
```

**via `datafusion_datasource::decoder::Decoder`**

```rust
fn can_flush_early(&self) -> bool
fn decode(&mut self, buf: &[u8]) -> Result<usize, ArrowError>
fn flush(&mut self) -> Result<Option<RecordBatch>, ArrowError>
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource_csv.file_format.CsvDecoder.md).


---

## CsvFormat

`struct` · `datafusion_datasource_csv::file_format::CsvFormat`

Also reachable as `datafusion::datasource::file_format::csv::CsvFormat`, `datafusion_datasource_csv::CsvFormat`

```rust
struct CsvFormat
```

**Implements**: `datafusion_datasource::file_format::FileFormat`

**Derives**: Debug, Default

**Methods** (20)

```rust
fn delimiter(&self) -> u8
fn escape(&self) -> Option<u8>
fn has_header(&self) -> Option<bool>
async fn infer_schema_from_stream(&self, state: &dyn Session, records_to_read: usize, stream: impl Stream<Item = Result<Bytes>>) -> Result<(Schema, usize)>
fn options(&self) -> &CsvOptions
fn quote(&self) -> u8
fn read_to_delimited_chunks_from_stream<'a>(&self, stream: BoxStream<'a, Result<Bytes>>) -> BoxStream<'a, Result<Bytes>>
fn with_comment(self, comment: Option<u8>) -> Self
fn with_delimiter(self, delimiter: u8) -> Self
fn with_escape(self, escape: Option<u8>) -> Self
fn with_file_compression_type(self, file_compression_type: FileCompressionType) -> Self
fn with_has_header(self, has_header: bool) -> Self
fn with_newlines_in_values(self, newlines_in_values: bool) -> Self
fn with_null_regex(self, null_regex: Option<String>) -> Self
fn with_options(self, options: CsvOptions) -> Self
fn with_quote(self, quote: u8) -> Self
fn with_schema_infer_max_rec(self, max_rec: usize) -> Self
fn with_terminator(self, terminator: Option<u8>) -> Self
fn with_truncate_rows(self, truncate_rows: bool) -> Self
fn with_truncated_rows(self, truncated_rows: bool) -> Self
```

**via `datafusion_datasource::file_format::FileFormat`**

```rust
fn compression_type(&self) -> Option<FileCompressionType>
async fn create_physical_plan(&self, state: &dyn Session, conf: FileScanConfig) -> Result<Arc<dyn ExecutionPlan>>
async fn create_writer_physical_plan(&self, input: Arc<dyn ExecutionPlan>, state: &dyn Session, conf: FileSinkConfig, order_requirements: Option<LexRequirement>) -> Result<Arc<dyn ExecutionPlan>>
fn file_source(&self, table_schema: TableSchema) -> Arc<dyn FileSource>
fn get_ext(&self) -> String
fn get_ext_with_compression(&self, file_compression_type: &FileCompressionType) -> Result<String>
async fn infer_schema(&self, state: &dyn Session, store: &Arc<dyn ObjectStore>, objects: &[ObjectMeta]) -> Result<SchemaRef>
async fn infer_stats(&self, _state: &dyn Session, _store: &Arc<dyn ObjectStore>, table_schema: SchemaRef, _object: &ObjectMeta) -> Result<Statistics>
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource_csv.file_format.CsvFormat.md).


Character Separated Value [`FileFormat`] implementation.

---

## CsvFormatFactory

`struct` · `datafusion_datasource_csv::file_format::CsvFormatFactory`

Also reachable as `datafusion::datasource::file_format::csv::CsvFormatFactory`, `datafusion_datasource_csv::CsvFormatFactory`

```rust
struct CsvFormatFactory
```

**Fields**: `options`

**Implements**: `datafusion_common::file_options::file_type::GetExt`, `datafusion_datasource::file_format::FileFormatFactory`

**Derives**: Debug, Default

**Methods** (2)

```rust
fn new() -> Self
fn new_with_options(options: CsvOptions) -> Self
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

[Full member, field, variant and typed contracts](../operations/datafusion_datasource_csv.file_format.CsvFormatFactory.md).


Factory used to create [`CsvFormat`]

---

## CsvSerializer

`struct` · `datafusion_datasource_csv::file_format::CsvSerializer`

Also reachable as `datafusion::datasource::file_format::csv::CsvSerializer`, `datafusion_datasource_csv::CsvSerializer`

```rust
struct CsvSerializer
```

**Implements**: `datafusion_datasource::write::BatchSerializer`

**Derives**: Debug, Default

**Methods** (3)

```rust
fn new() -> Self
fn with_builder(self, builder: WriterBuilder) -> Self
fn with_header(self, header: bool) -> Self
```

**via `datafusion_datasource::write::BatchSerializer`**

```rust
fn serialize(&self, batch: RecordBatch, initial: bool) -> Result<Bytes>
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource_csv.file_format.CsvSerializer.md).


Define a struct for serializing CSV records to a stream

---

## CsvSink

`struct` · `datafusion_datasource_csv::file_format::CsvSink`

Also reachable as `datafusion::datasource::file_format::csv::CsvSink`, `datafusion_datasource_csv::CsvSink`

```rust
struct CsvSink
```

**Implements**: `core::convert::TryFrom`, `datafusion_datasource::file_sink_config::FileSink`, `datafusion_datasource::sink::DataSink`, `datafusion_physical_plan::display::DisplayAs`

**Derives**: Debug

**Methods** (3)

```rust
fn new(config: FileSinkConfig, writer_options: CsvWriterOptions) -> Self
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &datafusion_physical_plan::proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
fn writer_options(&self) -> &CsvWriterOptions
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: &datafusion_proto_models::protobuf::CsvSink) -> Result<Self>
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

[Full member, field, variant and typed contracts](../operations/datafusion_datasource_csv.file_format.CsvSink.md).


Implements [`DataSink`] for writing to a CSV file.

---
