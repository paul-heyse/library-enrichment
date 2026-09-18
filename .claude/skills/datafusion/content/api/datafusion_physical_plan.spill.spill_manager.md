# `datafusion_physical_plan::spill::spill_manager`

Crate `datafusion-physical-plan` · 1 public items · structured records in [`model/datafusion_physical_plan.spill.spill_manager.json`](../model/datafusion_physical_plan.spill.spill_manager.json)

## SpillManager

`struct` · `datafusion_physical_plan::spill::spill_manager::SpillManager`

Also reachable as `datafusion::physical_plan::SpillManager`, `datafusion_physical_plan::SpillManager`

```rust
struct SpillManager
```

**Derives**: Clone, Debug

**Methods** (8)

```rust
fn create_in_progress_file(&self, request_msg: &str) -> Result<InProgressSpillFile>
fn new(env: Arc<RuntimeEnv>, metrics: SpillMetrics, schema: SchemaRef) -> Self
fn read_spill_as_stream(&self, spill_file_path: Arc<dyn SpillFile>, max_record_batch_memory: Option<usize>) -> Result<SendableRecordBatchStream>
fn read_spill_as_stream_unbuffered(&self, spill_file_path: Arc<dyn SpillFile>, max_record_batch_memory: Option<usize>) -> Result<SendableRecordBatchStream>
fn schema(&self) -> &SchemaRef
fn spill_record_batch_and_finish(&self, batches: &[RecordBatch], request_msg: &str) -> Result<Option<Arc<dyn SpillFile>>>
fn with_batch_read_buffer_capacity(self, batch_read_buffer_capacity: usize) -> Self
fn with_compression_type(self, spill_compression: SpillCompression) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.spill.spill_manager.SpillManager.md).


The `SpillManager` is responsible for the following tasks:
- Reading and writing `RecordBatch`es to raw files based on the provided configurations.
- Updating the associated metrics.

Note: The caller (external operators such as `SortExec`) is responsible for interpreting the spilled files.
For example, all records within the same spill file are ordered according to a specific order.

---
