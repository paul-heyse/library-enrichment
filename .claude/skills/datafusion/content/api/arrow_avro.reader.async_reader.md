# `arrow_avro::reader::async_reader`

Crate `arrow-avro` · 1 public items · structured records in [`model/arrow_avro.reader.async_reader.json`](../model/arrow_avro.reader.async_reader.json)

## AsyncAvroFileReader

`struct` · `arrow_avro::reader::async_reader::AsyncAvroFileReader`

Also reachable as `arrow_avro::reader::AsyncAvroFileReader`

```rust
struct AsyncAvroFileReader<R>
```

**Implements**: `futures_core::stream::Stream`

**Methods** (2)

```rust
fn builder(reader: R, file_size: u64, batch_size: usize) -> ReaderBuilder<R>
fn schema(&self) -> SchemaRef
```

**via `futures_core::stream::Stream`**

```rust
fn poll_next(Pin<&mut self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>
```

An asynchronous Avro file reader that implements `Stream<Item = Result<RecordBatch, ArrowError>>`.
This uses an [`AsyncFileReader`] to fetch data ranges as needed, starting with fetching the header,
then reading all the blocks in the provided range where:
1. Reads and decodes data until the header is fully decoded.
2. Searching from `range.start` for the first sync marker, and starting with the following block.
   (If `range.start` is less than the header length, we start at the header length minus the sync marker bytes)
3. Reading blocks sequentially, decoding them into RecordBatches.
4. If a block is incomplete (due to range ending mid-block), fetching the remaining bytes from the [`AsyncFileReader`].
5. If no range was originally provided, reads the full file.
6. If the range is 0, file_size is 0, or `range.end` is less than the header length, finish immediately.

# Example

```
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
use std::io::Cursor;
use std::sync::Arc;
use arrow_array::{ArrayRef, Int32Array, RecordBatch};
use arrow_schema::{DataType, Field, Schema};
use arrow_avro::reader::AsyncAvroFileReader;
use arrow_avro::writer::AvroWriter;
use futures::TryStreamExt;

// Build a minimal Arrow schema and batch
let schema = Schema::new(vec![Field::new("id", DataType::Int32, false)]);
let batch = RecordBatch::try_new(
    Arc::new(schema.clone()),
    vec![Arc::new(Int32Array::from(vec![1, 2, 3])) as ArrayRef],
)?;

// Write an Avro OCF to memory
let buffer: Vec<u8> = Vec::new();
let mut writer = AvroWriter::new(buffer, schema)?;
writer.write(&batch)?;
writer.finish()?;
let bytes = writer.into_inner();

// Create an async reader from the in-memory bytes
// `tokio::fs::File` also implements `AsyncFileReader` for reading from disk
let file_size = bytes.len();
let cursor = Cursor::new(bytes);
let reader = AsyncAvroFileReader::builder(cursor, file_size as u64, 1024)
    .try_build()
    .await?;

// Consume the stream of RecordBatches
let batches: Vec<RecordBatch> = reader.try_collect().await?;
assert_eq!(batches.len(), 1);
assert_eq!(batches[0].num_rows(), 3);
Ok(())
}
```

---
