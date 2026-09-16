# `deltalake_core::datafile::writer`

Crate `deltalake-core` · 4 public items · structured records in [`model/deltalake_core.datafile.writer.json`](../model/deltalake_core.datafile.writer.json)

## DeltaWriter

`struct` · `deltalake_core::datafile::writer::DeltaWriter`

Also reachable as `deltalake::datafile::writer::DeltaWriter`, `deltalake::operations::write::writer::DeltaWriter`, `deltalake_core::operations::write::writer::DeltaWriter`

```rust
struct DeltaWriter
```

**Implements**: `deltalake_core::datafile::DeltaDataWriter`, `deltalake_core::datafile::datafusion_ext::DeltaDataWriterExt`

**Methods** (6)

```rust
async fn abort(self) -> DeltaResult<()>
async fn close(self) -> DeltaResult<Vec<Add>>
fn new(object_store: ObjectStoreRef, config: WriterConfig) -> Self
fn with_writer_properties(self, writer_properties: WriterProperties) -> Self
async fn write(&mut self, batch: &RecordBatch) -> DeltaResult<()>
async fn write_partition(&mut self, record_batch: RecordBatch, partition_values: &IndexMap<String, Scalar>) -> DeltaResult<()>
```

**via `deltalake_core::datafile::DeltaDataWriter`**

```rust
async fn write_all(Box<self>, batches: BatchStream) -> DeltaResult<Vec<Add>>
```

**via `deltalake_core::datafile::datafusion_ext::DeltaDataWriterExt`**

```rust
async fn write_plan(Box<self>, session: &dyn Session, plan: Arc<dyn ExecutionPlan>) -> DeltaResult<Vec<Add>>
```

A parquet writer implementation tailored to the needs of writing data to a delta table.

---

## PartitionWriter

`struct` · `deltalake_core::datafile::writer::PartitionWriter`

Also reachable as `deltalake::datafile::writer::PartitionWriter`, `deltalake::operations::write::writer::PartitionWriter`, `deltalake_core::operations::write::writer::PartitionWriter`

```rust
struct PartitionWriter
```

**Implements**: `deltalake_core::datafile::DataFileWriter`

**Methods** (4)

```rust
async fn abort(self) -> DeltaResult<()>
async fn close(self) -> DeltaResult<Vec<Add>>
fn try_with_config(object_store: ObjectStoreRef, config: PartitionWriterConfig, num_indexed_cols: DataSkippingNumIndexedCols, stats_columns: Option<Vec<String>>) -> DeltaResult<Self>
async fn write(&mut self, batch: &RecordBatch) -> DeltaResult<()>
```

**via `deltalake_core::datafile::DataFileWriter`**

```rust
async fn abort(Box<self>) -> DeltaResult<()>
async fn close(Box<self>) -> DeltaResult<Vec<Add>>
async fn write(&mut self, batch: &RecordBatch) -> DeltaResult<()>
```

Partition writer implementation
This writer takes in table data as RecordBatches and writes it out to partitioned parquet files.
It buffers data in memory until it reaches a certain size, then writes it out to optimize file sizes.
When you complete writing you get back a list of Add actions that can be used to update the Delta table commit log.

---

## PartitionWriterConfig

`struct` · `deltalake_core::datafile::writer::PartitionWriterConfig`

Also reachable as `deltalake::datafile::writer::PartitionWriterConfig`, `deltalake::operations::write::writer::PartitionWriterConfig`, `deltalake_core::operations::write::writer::PartitionWriterConfig`

```rust
struct PartitionWriterConfig
```

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn try_new(file_schema: ArrowSchemaRef, partition_values: IndexMap<String, Scalar>, writer_properties: Option<WriterProperties>, target_file_size: Option<NonZeroU64>, write_batch_size: Option<usize>, max_concurrency_tasks: Option<usize>, prefix_override: Option<Path>) -> DeltaResult<Self>
fn with_roll_on_row_group_boundary(self, roll_on_row_group_boundary: bool) -> Self
```

Write configuration for partition writers

---

## WriterConfig

`struct` · `deltalake_core::datafile::writer::WriterConfig`

Also reachable as `deltalake::datafile::writer::WriterConfig`, `deltalake::operations::write::writer::WriterConfig`, `deltalake_core::operations::write::writer::WriterConfig`

```rust
struct WriterConfig
```

**Derives**: Clone, Debug

**Methods** (3)

```rust
fn file_schema(&self) -> ArrowSchemaRef
fn new(table_schema: ArrowSchemaRef, partition_columns: Vec<String>, writer_properties: Option<WriterProperties>, target_file_size: Option<NonZeroU64>, write_batch_size: Option<usize>, num_indexed_cols: DataSkippingNumIndexedCols, stats_columns: Option<Vec<String>>) -> Self
fn with_random_prefix_length(self, length: Option<usize>) -> Self
```

Configuration to write data into Delta tables

---
