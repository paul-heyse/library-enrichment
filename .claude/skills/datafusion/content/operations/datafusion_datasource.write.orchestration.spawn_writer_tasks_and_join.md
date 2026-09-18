# `datafusion_datasource::write::orchestration::spawn_writer_tasks_and_join`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.write.orchestration.spawn_writer_tasks_and_join.json).

<a id="op-ce1c994a51cea07dcdbdfe9b"></a>
## spawn_writer_tasks_and_join

`function` · `datafusion_datasource::write::orchestration::spawn_writer_tasks_and_join` · datafusion-datasource 55.1.0

```rust
async fn spawn_writer_tasks_and_join(context: &std::sync::Arc<datafusion_execution::TaskContext>, serializer: std::sync::Arc<dyn BatchSerializer>, compression: file_compression_type::FileCompressionType, compression_level: Option<u32>, object_store: std::sync::Arc<dyn ObjectStore>, demux_task: datafusion_common_runtime::SpawnedTask<datafusion_common::error::Result<()>>, file_stream_rx: super::demux::DemuxedStreamReceiver) -> datafusion_common::error::Result<u64>
```

Source: `src/write/orchestration.rs:247`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Orchestrates multipart put of a dynamic number of output files from a single input stream
for any statelessly serialized file type. That is, any file type for which each [RecordBatch](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)
can be serialized independently of all other [RecordBatch](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)s.
