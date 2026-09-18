# `datafusion_datasource::file_sink_config::FileSink`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.file_sink_config.FileSink.json).

<a id="op-4b127d53a23ced55ba491938"></a>
## FileSink

`trait` · `datafusion_datasource::file_sink_config::FileSink` · datafusion-datasource 55.1.0

```rust
trait FileSink: DataSink
```

Source: `src/file_sink_config.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

General behaviors for files that do `DataSink` operations

<a id="op-17dbf620376a1d9fb8cbdad5"></a>
## config

`function` · `datafusion_datasource::file_sink_config::FileSink::config` · datafusion-datasource 55.1.0

```rust
fn config(&self) -> &FileSinkConfig
```

Source: `src/file_sink_config.rs:88`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Retrieves the file sink configuration.

<a id="op-8ed990510dd9595c6eb0350a"></a>
## spawn_writer_tasks_and_join

`function` · `datafusion_datasource::file_sink_config::FileSink::spawn_writer_tasks_and_join` · datafusion-datasource 55.1.0

```rust
async fn spawn_writer_tasks_and_join(&self, context: &Arc<TaskContext>, demux_task: SpawnedTask<Result<()>>, file_stream_rx: DemuxedStreamReceiver, object_store: Arc<dyn ObjectStore>) -> Result<u64>
```

Source: `src/file_sink_config.rs:110`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Spawns writer tasks and joins them to perform file writing operations.
Is a critical part of `FileSink` trait, since it's the very last step for `write_all`.

This function handles the process of writing data to files by:
1. Spawning tasks for writing data to individual files.
2. Coordinating the tasks using a demuxer to distribute data among files.
3. Collecting results using `tokio::join`, ensuring that all tasks complete successfully.

# Parameters
- `context`: The execution context (`TaskContext`) that provides resources
  like memory management and runtime environment.
- `demux_task`: A spawned task that handles demuxing, responsible for splitting
  an input [`SendableRecordBatchStream`](../operations/datafusion_execution.stream.SendableRecordBatchStream.md#op-7cf25e4c554567392af11cb6) into dynamically determined partitions.
  See `start_demuxer_task()`
- `file_stream_rx`: A receiver that yields streams of record batches and their
  corresponding file paths for writing. See `start_demuxer_task()`
- `object_store`: A handle to the object store where the files are written.

# Returns
- `Result<u64>`: Returns the total number of rows written across all files.

<a id="op-66076d9ad788aea44ec2c9b2"></a>
## write_all

`function` · `datafusion_datasource::file_sink_config::FileSink::write_all` · datafusion-datasource 55.1.0

```rust
async fn write_all(&self, data: SendableRecordBatchStream, context: &Arc<TaskContext>) -> Result<u64>
```

Source: `src/file_sink_config.rs:119`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

File sink implementation of the [`DataSink::write_all`](../operations/datafusion_datasource.sink.DataSink.md#op-03153e06dfae84f047577032) method.
