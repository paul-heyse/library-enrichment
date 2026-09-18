# `datafusion_datasource::write::orchestration`

Crate `datafusion-datasource` · 1 public items · structured records in [`model/datafusion_datasource.write.orchestration.json`](../model/datafusion_datasource.write.orchestration.json)

## spawn_writer_tasks_and_join

`function` · `datafusion_datasource::write::orchestration::spawn_writer_tasks_and_join`

```rust
async fn spawn_writer_tasks_and_join(context: &std::sync::Arc<datafusion_execution::TaskContext>, serializer: std::sync::Arc<dyn BatchSerializer>, compression: file_compression_type::FileCompressionType, compression_level: Option<u32>, object_store: std::sync::Arc<dyn ObjectStore>, demux_task: datafusion_common_runtime::SpawnedTask<datafusion_common::error::Result<()>>, file_stream_rx: super::demux::DemuxedStreamReceiver) -> datafusion_common::error::Result<u64>
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource.write.orchestration.spawn_writer_tasks_and_join.md).


Orchestrates multipart put of a dynamic number of output files from a single input stream
for any statelessly serialized file type. That is, any file type for which each [RecordBatch]
can be serialized independently of all other [RecordBatch]s.

---
