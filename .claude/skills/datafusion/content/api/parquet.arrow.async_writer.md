# `parquet::arrow::async_writer`

Crate `parquet` · 2 public items · structured records in [`model/parquet.arrow.async_writer.json`](../model/parquet.arrow.async_writer.json)

## AsyncArrowWriter

`struct` · `parquet::arrow::async_writer::AsyncArrowWriter`

Also reachable as `parquet::arrow::AsyncArrowWriter`

```rust
struct AsyncArrowWriter<W>
```

**Methods** (13)

```rust
fn append_key_value_metadata(&mut self, kv_metadata: KeyValue)
fn bytes_written(&self) -> usize
async fn close(self) -> Result<ParquetMetaData>
async fn finish(&mut self) -> Result<ParquetMetaData>
async fn flush(&mut self) -> Result<()>
fn flushed_row_groups(&self) -> &[RowGroupMetaData]
fn in_progress_rows(&self) -> usize
fn in_progress_size(&self) -> usize
fn into_inner(self) -> W
fn memory_size(&self) -> usize
fn try_new(writer: W, arrow_schema: SchemaRef, props: Option<WriterProperties>) -> Result<Self>
fn try_new_with_options(writer: W, arrow_schema: SchemaRef, options: ArrowWriterOptions) -> Result<Self>
async fn write(&mut self, batch: &RecordBatch) -> Result<()>
```

[Full member, field, variant and typed contracts](../operations/parquet.arrow.async_writer.AsyncArrowWriter.md).


Encodes [`RecordBatch`] to parquet, outputting to an [`AsyncFileWriter`]

## Memory Usage

This writer eagerly writes data as soon as possible to the underlying [`AsyncFileWriter`],
permitting fine-grained control over buffering and I/O scheduling. However, the columnar
nature of parquet forces data for an entire row group to be buffered in memory, before
it can be flushed. Depending on the data and the configured row group size, this buffering
may be substantial.

Memory usage can be limited by calling [`Self::flush`] to flush the in progress row group,
although this will likely increase overall file size and reduce query performance.
See [ArrowWriter] for more information.

```no_run
# use tokio::fs::File;
# use arrow_array::RecordBatch;
# use parquet::arrow::AsyncArrowWriter;
# async fn test() {
let mut writer: AsyncArrowWriter<File> = todo!();
let batch: RecordBatch = todo!();
writer.write(&batch).await.unwrap();
// Trigger an early flush if buffered size exceeds 1_000_000
if writer.in_progress_size() > 1_000_000 {
    writer.flush().await.unwrap()
}
# }
```

---

## AsyncFileWriter

`trait` · `parquet::arrow::async_writer::AsyncFileWriter`

```rust
trait AsyncFileWriter: Send
```

**Implementors** (2)

- `alloc::boxed::Box`
- `parquet::arrow::async_writer::store::ParquetObjectWriter`

**Methods** (2)

```rust
fn complete(&mut self) -> BoxFuture<'_, Result<()>>
fn write(&mut self, bs: Bytes) -> BoxFuture<'_, Result<()>>
```

[Full member, field, variant and typed contracts](../operations/parquet.arrow.async_writer.AsyncFileWriter.md).


The asynchronous interface used by [`AsyncArrowWriter`] to write parquet files.

---
