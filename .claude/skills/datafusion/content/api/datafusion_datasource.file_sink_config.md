# `datafusion_datasource::file_sink_config`

Crate `datafusion-datasource` · 3 public items · structured records in [`model/datafusion_datasource.file_sink_config.json`](../model/datafusion_datasource.file_sink_config.json)

## FileOutputMode

`enum` · `datafusion_datasource::file_sink_config::FileOutputMode`

Also reachable as `datafusion::datasource::physical_plan::FileOutputMode`

```rust
enum FileOutputMode
```

**Variants**: `Automatic`, `SingleFile`, `Directory`

**Implements**: `core::convert::From`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn single_file_output(self, base_output_path: &ListingTableUrl) -> bool
```

**via `core::convert::From`**

```rust
fn from(value: Option<bool>) -> Self
```

Determines how `FileSink` output paths are interpreted.

---

## FileSinkConfig

`struct` · `datafusion_datasource::file_sink_config::FileSinkConfig`

Also reachable as `datafusion::datasource::physical_plan::FileSinkConfig`

```rust
struct FileSinkConfig
```

**Fields**: `original_url`, `object_store_url`, `file_group`, `table_paths`, `output_schema`, `table_partition_cols`, `insert_op`, `keep_partition_by_columns`, `file_extension`, `file_output_mode`

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn output_schema(&self) -> &SchemaRef
```

**via `core::convert::TryFrom`**

```rust
fn try_from(conf: &protobuf::FileSinkConfig) -> Result<Self>
```

The base configurations to provide when creating a physical plan for
writing to any given file format.

---

## FileSink

`trait` · `datafusion_datasource::file_sink_config::FileSink`

Also reachable as `datafusion::datasource::physical_plan::FileSink`

```rust
trait FileSink: DataSink
```

**Implementors** (3)

- `datafusion_datasource_csv::file_format::CsvSink`
- `datafusion_datasource_json::file_format::JsonSink`
- `datafusion_datasource_parquet::sink::ParquetSink`

**Methods** (3)

```rust
fn config(&self) -> &FileSinkConfig
async fn spawn_writer_tasks_and_join(&self, context: &Arc<TaskContext>, demux_task: SpawnedTask<Result<()>>, file_stream_rx: DemuxedStreamReceiver, object_store: Arc<dyn ObjectStore>) -> Result<u64>
async fn write_all(&self, data: SendableRecordBatchStream, context: &Arc<TaskContext>) -> Result<u64>
```

General behaviors for files that do `DataSink` operations

---
