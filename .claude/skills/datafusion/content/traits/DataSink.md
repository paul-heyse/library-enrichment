# DataSink

`datafusion_datasource::sink::DataSink`

```rust
trait DataSink: Any + DisplayAs + Debug + Send + Sync
```

Also reachable as `datafusion::datasource::sink::DataSink`

Prose: [`api/datafusion_datasource.sink.md`](../api/datafusion_datasource.sink.md#datasink) · records: [`model/datafusion_datasource.sink.json`](../model/datafusion_datasource.sink.json)

## Required

Every implementation must supply these.

```rust
fn schema(&self) -> &SchemaRef
async fn write_all(&self, data: SendableRecordBatchStream, context: &Arc<TaskContext>) -> Result<u64>
```

## Provided

These methods have defaults. Read each full contract before overriding: some defaults reject unsupported operations, while others provide suitable general behavior. Required methods alone do not prove correctness or performance.

```rust
fn metrics(&self) -> Option<MetricsSet>
fn try_to_proto(&self, _exec: &DataSinkExec, _ctx: &datafusion_physical_plan::proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
```

## Implementors (4)

Read one before writing your own.

- `datafusion_datasource::memory::MemSink`
- `datafusion_datasource_csv::file_format::CsvSink`
- `datafusion_datasource_json::file_format::JsonSink`
- `datafusion_datasource_parquet::sink::ParquetSink`

## Documentation

`DataSink` implements writing streams of [`RecordBatch`]es to
user defined destinations.

The `Display` impl is used to format the sink for explain plan
output.
