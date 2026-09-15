# `datafusion_datasource_parquet::sink`

Crate `datafusion-datasource-parquet` · 1 public items · structured records in [`model/datafusion_datasource_parquet.sink.json`](../model/datafusion_datasource_parquet.sink.json)

## ParquetSink

`struct` · `datafusion_datasource_parquet::sink::ParquetSink`

Also reachable as `datafusion::datasource::file_format::parquet::ParquetSink`, `datafusion::datasource::physical_plan::parquet::ParquetSink`, `datafusion_datasource_parquet::ParquetSink`, `datafusion_datasource_parquet::file_format::ParquetSink`

```rust
struct ParquetSink
```

**Implements**: `core::convert::TryFrom`, `datafusion_datasource::file_sink_config::FileSink`, `datafusion_datasource::sink::DataSink`, `datafusion_physical_plan::display::DisplayAs`

**Derives**: Debug

**Methods** (5)

```rust
fn new(config: FileSinkConfig, parquet_options: TableParquetOptions) -> Self
fn parquet_options(&self) -> &TableParquetOptions
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &datafusion_physical_plan::proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
fn with_sorting_columns(self, sorting_columns: Option<Vec<SortingColumn>>) -> Self
fn written(&self) -> HashMap<Path, ParquetMetaData>
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: &datafusion_proto_models::protobuf::ParquetSink) -> Result<Self>
```

**via `datafusion_datasource::file_sink_config::FileSink`**

```rust
fn config(&self) -> &FileSinkConfig
async fn spawn_writer_tasks_and_join(&self, context: &Arc<TaskContext>, demux_task: SpawnedTask<Result<()>>, file_stream_rx: DemuxedStreamReceiver, object_store: Arc<dyn ObjectStore>) -> Result<u64>
```

**via `datafusion_datasource::sink::DataSink`**

```rust
fn metrics(&self) -> Option<MetricsSet>
fn schema(&self) -> &SchemaRef
fn try_to_proto(&self, exec: &DataSinkExec, ctx: &datafusion_physical_plan::proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
async fn write_all(&self, data: SendableRecordBatchStream, context: &Arc<TaskContext>) -> Result<u64>
```

**via `datafusion_physical_plan::display::DisplayAs`**

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implements [`DataSink`] for writing to a parquet file.

---
