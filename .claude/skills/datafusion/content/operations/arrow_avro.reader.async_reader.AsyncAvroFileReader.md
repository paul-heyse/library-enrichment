# `arrow_avro::reader::async_reader::AsyncAvroFileReader`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_avro.reader.async_reader.AsyncAvroFileReader.json).

<a id="op-1556b0e38a5fc44a0cc6248c"></a>
## AsyncAvroFileReader

`struct` · `arrow_avro::reader::async_reader::AsyncAvroFileReader` · arrow-avro 59.3.0

```rust
struct AsyncAvroFileReader<R>
```

Source: `src/reader/async_reader/mod.rs:140`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

An asynchronous Avro file reader that implements `Stream<Item = Result<RecordBatch, ArrowError>>`.
This uses an [`AsyncFileReader`](../operations/arrow_avro.reader.async_reader.async_file_reader.AsyncFileReader.md#op-b37e46224b914587014a7456) to fetch data ranges as needed, starting with fetching the header,
then reading all the blocks in the provided range where:
1. Reads and decodes data until the header is fully decoded.
2. Searching from `range.start` for the first sync marker, and starting with the following block.
   (If `range.start` is less than the header length, we start at the header length minus the sync marker bytes)
3. Reading blocks sequentially, decoding them into RecordBatches.
4. If a block is incomplete (due to range ending mid-block), fetching the remaining bytes from the [`AsyncFileReader`](../operations/arrow_avro.reader.async_reader.async_file_reader.AsyncFileReader.md#op-b37e46224b914587014a7456).
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

<a id="op-7bd6008150691a3ab1dc6f97"></a>
## Item

`assoc_type` · `arrow_avro::reader::async_reader::AsyncAvroFileReader::Item` · arrow-avro 59.3.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_avro::reader::async_reader::AsyncAvroFileReader", "path": "AsyncAvroFileReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_avro::reader::async_reader::async_file_reader::AsyncFileReader", "path": "AsyncFileReader"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Unpin", "path": "Unpin"}}}, {"outlives": "'static"}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [541, 1], "end": [547, 2], "filename": "src/reader/async_reader/mod.rs"}, "trait": {"args": null, "id": "futures_core::stream::Stream", "path": "Stream"}, "trait_path": "futures_core::stream::Stream"}`

Source: `src/reader/async_reader/mod.rs:542`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4daf468b7ade50d5bcf93eba"></a>
## builder

`function` · `arrow_avro::reader::async_reader::AsyncAvroFileReader::builder` · arrow-avro 59.3.0

```rust
fn builder(reader: R, file_size: u64, batch_size: usize) -> ReaderBuilder<R>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_avro::reader::async_reader::AsyncAvroFileReader", "path": "AsyncAvroFileReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [156, 1], "end": [246, 2], "filename": "src/reader/async_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/async_reader/mod.rs:158`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Returns a builder for a new [`Self`](../operations/arrow_avro.reader.async_reader.AsyncAvroFileReader.md#op-1556b0e38a5fc44a0cc6248c), allowing some optional parameters.

<a id="op-6b77510f3c39f30727f9e12c"></a>
## poll_next

`function` · `arrow_avro::reader::async_reader::AsyncAvroFileReader::poll_next` · arrow-avro 59.3.0

```rust
fn poll_next(Pin<&mut self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_avro::reader::async_reader::AsyncAvroFileReader", "path": "AsyncAvroFileReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_avro::reader::async_reader::async_file_reader::AsyncFileReader", "path": "AsyncFileReader"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Unpin", "path": "Unpin"}}}, {"outlives": "'static"}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [541, 1], "end": [547, 2], "filename": "src/reader/async_reader/mod.rs"}, "trait": {"args": null, "id": "futures_core::stream::Stream", "path": "Stream"}, "trait_path": "futures_core::stream::Stream"}`

Source: `src/reader/async_reader/mod.rs:544`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3d074994c8732f84d6e4e44f"></a>
## schema

`function` · `arrow_avro::reader::async_reader::AsyncAvroFileReader::schema` · arrow-avro 59.3.0

```rust
fn schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_avro::reader::async_reader::AsyncAvroFileReader", "path": "AsyncAvroFileReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [156, 1], "end": [246, 2], "filename": "src/reader/async_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/async_reader/mod.rs:187`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Returns the Arrow schema for batches produced by this reader.

The schema is determined by the writer schema in the file and the reader schema provided to the builder.
