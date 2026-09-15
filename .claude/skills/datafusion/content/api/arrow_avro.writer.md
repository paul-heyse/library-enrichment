# `arrow_avro::writer`

Crate `arrow-avro` · 6 public items · structured records in [`model/arrow_avro.writer.json`](../model/arrow_avro.writer.json)

## EncodedRows

`struct` · `arrow_avro::writer::EncodedRows`

```rust
struct EncodedRows
```

**Derives**: Clone, Debug

**Methods** (7)

```rust
fn bytes(&self) -> &Bytes
fn is_empty(&self) -> bool
fn iter(&self) -> impl ExactSizeIterator<Item = Bytes> + '_
fn len(&self) -> usize
fn new(data: Bytes, offsets: Vec<usize>) -> Self
fn offsets(&self) -> &[usize]
fn row(&self, n: usize) -> Result<Bytes, AvroError>
```

A contiguous set of Avro encoded rows.

`EncodedRows` stores:
- a single backing byte buffer (`bytes::Bytes`)
- a `Vec<usize>` of row boundary offsets (length = `rows + 1`)

This lets callers get per-row payloads as zero-copy `Bytes` slices.

For compatibility with APIs that require owned `Vec<u8>`, use:
`let vecs: Vec<Vec<u8>> = rows.iter().map(|b| b.to_vec()).collect();`

---

## Encoder

`struct` · `arrow_avro::writer::Encoder`

```rust
struct Encoder
```

**Derives**: Debug

**Methods** (5)

```rust
fn buffered_len(&self) -> usize
fn encode(&mut self, batch: &RecordBatch) -> Result<(), AvroError>
fn encode_batches(&mut self, batches: &[RecordBatch]) -> Result<(), AvroError>
fn flush(&mut self) -> EncodedRows
fn schema(&self) -> SchemaRef
```

A row-by-row encoder for Avro *stream/message* formats (SOE / registry wire formats / raw binary).

Unlike [`Writer`], which emits a single continuous byte stream to a [`std::io::Write`] sink,
`Encoder` tracks row boundaries during encoding and returns an [`EncodedRows`] containing:
- one backing buffer (`Bytes`)
- row boundary offsets

This enables zero-copy per-row payloads (for instance, one Kafka message per Arrow row) without
re-encoding or decoding the byte stream to recover record boundaries.

### Example

```
use std::sync::Arc;
use arrow_array::{ArrayRef, Int32Array, RecordBatch};
use arrow_schema::{DataType, Field, Schema};
use arrow_avro::writer::{WriterBuilder, format::AvroSoeFormat};
use arrow_avro::schema::FingerprintStrategy;

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let schema = Schema::new(vec![Field::new("value", DataType::Int32, false)]);
let batch = RecordBatch::try_new(
    Arc::new(schema.clone()),
    vec![Arc::new(Int32Array::from(vec![1, 2, 3])) as ArrayRef],
)?;

// Configure the encoder (here: Confluent Wire Format with schema ID 100)
let mut encoder = WriterBuilder::new(schema)
    .with_fingerprint_strategy(FingerprintStrategy::Id(100))
    .build_encoder::<AvroSoeFormat>()?;

// Encode the batch
encoder.encode(&batch)?;

// Get the encoded rows
let rows = encoder.flush();

// Convert to owned Vec<u8> payloads (e.g., for a Kafka producer)
let payloads: Vec<Vec<u8>> = rows.iter().map(|row| row.to_vec()).collect();

assert_eq!(payloads.len(), 3);
assert_eq!(payloads[0][0], 0x00); // Magic byte
# Ok(())
# }
```

---

## Writer

`struct` · `arrow_avro::writer::Writer`

```rust
struct Writer<W: Write, F: AvroFormat>
```

**Derives**: Debug

**Methods** (7)

```rust
fn finish(&mut self) -> Result<(), AvroError>
fn into_inner(self) -> W
fn new(writer: W, schema: Schema) -> Result<Self, AvroError>
fn new(writer: W, schema: Schema) -> Result<Self, AvroError>
fn sync_marker(&self) -> Option<&[u8; 16]>
fn write(&mut self, batch: &RecordBatch) -> Result<(), AvroError>
fn write_batches(&mut self, batches: &[&RecordBatch]) -> Result<(), AvroError>
```

Generic Avro writer.

This type is generic over the output Write sink (`W`) and the Avro format (`F`).
You’ll usually use the concrete aliases:

* **[`AvroWriter`]** for **OCF** (self‑describing container file)
* **[`AvroStreamWriter`]** for **SOE** Avro streams

---

## WriterBuilder

`struct` · `arrow_avro::writer::WriterBuilder`

```rust
struct WriterBuilder
```

**Derives**: Clone, Debug

**Methods** (7)

```rust
fn build<W, F>(self, writer: W) -> Result<Writer<W, F>, AvroError> where W: Write, F: AvroFormat
fn build_encoder<F: AvroFormat>(self) -> Result<Encoder, AvroError>
fn new(schema: Schema) -> Self
fn with_capacity(self, capacity: usize) -> Self
fn with_compression(self, codec: Option<CompressionCodec>) -> Self
fn with_fingerprint_strategy(self, strategy: FingerprintStrategy) -> Self
fn with_row_capacity(self, capacity: usize) -> Self
```

Builder to configure and create a `Writer`.

---

## AvroStreamWriter

`type_alias` · `arrow_avro::writer::AvroStreamWriter`

```rust
type AvroStreamWriter<W> = Writer<W, writer::format::AvroSoeFormat>
```

Alias for an Avro **Single Object Encoding** stream writer.

### Example

This writer automatically adds the appropriate per-record prefix (based on the
fingerprint strategy) before the Avro body of each record. The default is Single
Object Encoding (SOE) with a Rabin fingerprint.

```
use std::sync::Arc;
use arrow_array::{ArrayRef, Int64Array, RecordBatch};
use arrow_schema::{DataType, Field, Schema};
use arrow_avro::writer::AvroStreamWriter;

# fn main() -> Result<(), Box<dyn std::error::Error>> {
// One‑column Arrow batch
let schema = Schema::new(vec![Field::new("x", DataType::Int64, false)]);
let batch = RecordBatch::try_new(
    Arc::new(schema.clone()),
    vec![Arc::new(Int64Array::from(vec![10, 20])) as ArrayRef],
)?;

// Write an Avro Single Object Encoding stream to a Vec<u8>
let sink: Vec<u8> = Vec::new();
let mut w = AvroStreamWriter::new(sink, schema)?;
w.write(&batch)?;
w.finish()?;
let bytes = w.into_inner();
assert!(!bytes.is_empty());
# Ok(()) }
```

---

## AvroWriter

`type_alias` · `arrow_avro::writer::AvroWriter`

```rust
type AvroWriter<W> = Writer<W, writer::format::AvroOcfFormat>
```

Alias for an Avro **Object Container File** writer.

### Quickstart (runnable)

```
use std::io::Cursor;
use std::sync::Arc;
use arrow_array::{ArrayRef, Int64Array, StringArray, RecordBatch};
use arrow_schema::{DataType, Field, Schema};
use arrow_avro::writer::AvroWriter;
use arrow_avro::reader::ReaderBuilder;

# fn main() -> Result<(), Box<dyn std::error::Error>> {
// Writer schema: { id: long, name: string }
let writer_schema = Schema::new(vec![
    Field::new("id", DataType::Int64, false),
    Field::new("name", DataType::Utf8, false),
]);

// Build a RecordBatch with two rows
let batch = RecordBatch::try_new(
    Arc::new(writer_schema.clone()),
    vec![
        Arc::new(Int64Array::from(vec![1, 2])) as ArrayRef,
        Arc::new(StringArray::from(vec!["a", "b"])) as ArrayRef,
    ],
)?;

// Write an Avro **Object Container File** (OCF) to memory
let mut w = AvroWriter::new(Vec::<u8>::new(), writer_schema.clone())?;
w.write(&batch)?;
w.finish()?;
let bytes = w.into_inner();

// Build a Reader and decode the batch back
let mut r = ReaderBuilder::new().build(Cursor::new(bytes))?;
let out = r.next().unwrap()?;
assert_eq!(out.num_rows(), 2);
# Ok(()) }
```

---
