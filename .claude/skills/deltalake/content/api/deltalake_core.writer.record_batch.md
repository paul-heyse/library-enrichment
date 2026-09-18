# `deltalake_core::writer::record_batch`

Crate `deltalake-core` · 2 public items · structured records in [`model/deltalake_core.writer.record_batch.json`](../model/deltalake_core.writer.record_batch.json)

## PartitionResult

`struct` · `deltalake_core::writer::record_batch::PartitionResult`
[Full member contracts, output types and access classification](../operations/deltalake_core.writer.record_batch.PartitionResult.md)

Also reachable as `deltalake::writer::record_batch::PartitionResult`

```rust
struct PartitionResult
```

**Fields**: `partition_values`, `record_batch`

**Derives**: Clone, Debug

Helper container for partitioned record batches

---

## RecordBatchWriter

`struct` · `deltalake_core::writer::record_batch::RecordBatchWriter`
[Full member contracts, output types and access classification](../operations/deltalake_core.writer.record_batch.RecordBatchWriter.md)

Also reachable as `deltalake::writer::RecordBatchWriter`, `deltalake::writer::record_batch::RecordBatchWriter`, `deltalake_core::writer::RecordBatchWriter`

```rust
struct RecordBatchWriter
```

**Implements**: `deltalake_core::writer::DeltaWriter`

**Derives**: Debug

**Methods** (12)

```rust
fn arrow_schema(&self) -> ArrowSchemaRef
fn buffer_len(&self) -> usize
fn buffered_record_batch_count(&self) -> usize
fn for_blind_appends(table: &table::BlindDeltaTable) -> Result<Self, DeltaTableError>
fn for_table(table: &DeltaTable) -> Result<Self, DeltaTableError>
fn reset(&mut self)
fn try_new(table_uri: impl AsRef<str>, schema: ArrowSchemaRef, partition_columns: Option<Vec<String>>, storage_options: Option<HashMap<String, String>>) -> Result<Self, DeltaTableError>
async fn try_new_checked(table_uri: impl AsRef<str>, schema: ArrowSchemaRef, partition_columns: Option<Vec<String>>, storage_options: Option<HashMap<String, String>>) -> Result<Self, DeltaTableError>
fn with_commit_properties(self, properties: CommitProperties) -> Self
fn with_target_file_size(self, target_file_size: u64) -> Self
fn with_writer_properties(self, writer_properties: WriterProperties) -> Self
async fn write_partition(&mut self, record_batch: RecordBatch, partition_values: &IndexMap<String, Scalar>, mode: WriteMode) -> Result<ArrowSchemaRef, DeltaTableError>
```

**via `deltalake_core::writer::DeltaWriter`**

```rust
async fn flush(&mut self) -> Result<Vec<Add>, DeltaTableError>
async fn flush_and_commit(&mut self, table: &mut DeltaTable) -> Result<Version, DeltaTableError>
async fn write(&mut self, values: RecordBatch) -> Result<(), DeltaTableError>
async fn write_with_mode(&mut self, values: RecordBatch, mode: WriteMode) -> Result<(), DeltaTableError>
```

Writes messages to a delta lake table.

Batches are streamed to storage as they are written, and a flush window
commits all-or-nothing: if any write returns an error — including a
transient IO error from the object store — every batch buffered since the
last flush is discarded along with the failing one, and the caller must
re-write all of them. (Validation errors caught before the batch reaches
storage fail only that call and leave the window untouched.)

---
