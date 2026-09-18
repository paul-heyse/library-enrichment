# `arrow_avro::reader`

Crate `arrow-avro` · 3 public items · structured records in [`model/arrow_avro.reader.json`](../model/arrow_avro.reader.json)

## Decoder

`struct` · `arrow_avro::reader::Decoder`

```rust
struct Decoder
```

**Derives**: Debug

**Methods** (7)

```rust
fn batch_is_empty(&self) -> bool
fn batch_is_full(&self) -> bool
fn batch_size(&self) -> usize
fn capacity(&self) -> usize
fn decode(&mut self, data: &[u8]) -> Result<usize, AvroError>
fn flush(&mut self) -> Result<Option<RecordBatch>, AvroError>
fn schema(&self) -> SchemaRef
```

[Full member, field, variant and typed contracts](../operations/arrow_avro.reader.Decoder.md).


A low‑level, push‑based decoder from Avro bytes to Arrow `RecordBatch`.

`Decoder` is designed for **streaming** scenarios:

* You *feed* freshly received bytes using `Self::decode`, potentially multiple times,
  until at least one row is complete.
* You then *drain* completed rows with `Self::flush`, which yields a `RecordBatch`
  if any rows were finished since the last flush.

Unlike `Reader`, which is specialized for Avro **Object Container Files**, `Decoder`
understands **framed single‑object** inputs and **Confluent Schema Registry** messages,
switching schemas mid‑stream when the framing indicates a new fingerprint.

### Supported prefixes

On each new row boundary, `Decoder` tries to match one of the following "prefixes":

* **Single‑Object encoding**: magic `0xC3 0x01` + schema fingerprint (length depends on
  the configured `FingerprintAlgorithm`); see `SINGLE_OBJECT_MAGIC`.
* **Confluent wire format**: magic `0x00` + 4‑byte big‑endian schema id; see
  `CONFLUENT_MAGIC`.

The active fingerprint determines which cached row decoder is used to decode the following
record body bytes.

### Schema switching semantics

When a new fingerprint is observed:

* If the current batch is empty, the decoder switches immediately;
* Otherwise, the current batch is finalized on the next `flush` and only then
  does the decoder switch to the new schema. This guarantees that a single `RecordBatch`
  never mixes rows with different schemas.

### Examples

Build and use a `Decoder` for single‑object encoding:

```
use arrow_avro::schema::{AvroSchema, SchemaStore};
use arrow_avro::reader::ReaderBuilder;

# fn main() -> Result<(), Box<dyn std::error::Error>> {
// Use a record schema at the top level so we can build an Arrow RecordBatch
let mut store = SchemaStore::new(); // Rabin fingerprinting by default
let avro = AvroSchema::new(
    r#"{"type":"record","name":"E","fields":[{"name":"x","type":"long"}]}"#.to_string()
);
let fp = store.register(avro)?;

// --- Hidden: write a single-object framed row {x:7} ---
# use std::sync::Arc;
# use std::collections::HashMap;
# use arrow_array::{ArrayRef, Int64Array, RecordBatch};
# use arrow_schema::{DataType, Field, Schema};
# use arrow_avro::schema::{SCHEMA_METADATA_KEY, FingerprintStrategy};
# use arrow_avro::writer::{WriterBuilder, format::AvroSoeFormat};
# let mut md = HashMap::new();
# md.insert(SCHEMA_METADATA_KEY.to_string(),
#     r#"{"type":"record","name":"E","fields":[{"name":"x","type":"long"}]}"#.to_string());
# let arrow = Schema::new_with_metadata(vec![Field::new("x", DataType::Int64, false)], md);
# let batch = RecordBatch::try_new(Arc::new(arrow.clone()), vec![Arc::new(Int64Array::from(vec![7])) as ArrayRef])?;
# let mut w = WriterBuilder::new(arrow)
#     .with_fingerprint_strategy(fp.into())
#     .build::<_, AvroSoeFormat>(Vec::new())?;
# w.write(&batch)?; w.finish()?; let frame = w.into_inner();

let mut decoder = ReaderBuilder::new()
    .with_writer_schema_store(store)
    .with_batch_size(16)
    .build_decoder()?;

# decoder.decode(&frame)?;
let batch = decoder.flush()?.expect("one row");
assert_eq!(batch.num_rows(), 1);
# Ok(()) }
```

*Background:* Avro's single‑object encoding is defined as `0xC3 0x01` + 8‑byte
little‑endian CRC‑64‑AVRO fingerprint of the **writer schema** + Avro binary body.
See the Avro 1.11.1 spec for details. <https://avro.apache.org/docs/1.11.1/specification/#single-object-encoding>

Build and use a `Decoder` for Confluent Registry messages:

```
use arrow_avro::schema::{AvroSchema, SchemaStore, Fingerprint, FingerprintAlgorithm};
use arrow_avro::reader::ReaderBuilder;

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let mut store = SchemaStore::new_with_type(FingerprintAlgorithm::Id);
store.set(Fingerprint::Id(1234), AvroSchema::new(r#"{"type":"record","name":"E","fields":[{"name":"x","type":"long"}]}"#.to_string()))?;

// --- Hidden: encode two Confluent-framed messages {x:1} and {x:2} ---
# use std::sync::Arc;
# use std::collections::HashMap;
# use arrow_array::{ArrayRef, Int64Array, RecordBatch};
# use arrow_schema::{DataType, Field, Schema};
# use arrow_avro::schema::{SCHEMA_METADATA_KEY, FingerprintStrategy};
# use arrow_avro::writer::{WriterBuilder, format::AvroSoeFormat};
# fn msg(x: i64) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
#   let mut md = HashMap::new();
#   md.insert(SCHEMA_METADATA_KEY.to_string(),
#     r#"{"type":"record","name":"E","fields":[{"name":"x","type":"long"}]}"#.to_string());
#   let arrow = Schema::new_with_metadata(vec![Field::new("x", DataType::Int64, false)], md);
#   let batch = RecordBatch::try_new(Arc::new(arrow.clone()), vec![Arc::new(Int64Array::from(vec![x])) as ArrayRef])?;
#   let mut w = WriterBuilder::new(arrow)
#       .with_fingerprint_strategy(FingerprintStrategy::Id(1234))
#       .build::<_, AvroSoeFormat>(Vec::new())?;
#   w.write(&batch)?; w.finish()?; Ok(w.into_inner())
# }
# let m1 = msg(1)?;
# let m2 = msg(2)?;

let mut decoder = ReaderBuilder::new()
    .with_writer_schema_store(store)
    .build_decoder()?;
# decoder.decode(&m1)?;
# decoder.decode(&m2)?;
let batch = decoder.flush()?.expect("two rows");
assert_eq!(batch.num_rows(), 2);
# Ok(()) }
```

---

## Reader

`struct` · `arrow_avro::reader::Reader`

```rust
struct Reader<R: BufRead>
```

**Implements**: `arrow_array::record_batch::RecordBatchReader`, `core::iter::traits::iterator::Iterator`

**Derives**: Debug

**Methods** (2)

```rust
fn avro_header(&self) -> &Header
fn schema(&self) -> SchemaRef
```

**via `arrow_array::record_batch::RecordBatchReader`**

```rust
fn schema(&self) -> SchemaRef
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
```

[Full member, field, variant and typed contracts](../operations/arrow_avro.reader.Reader.md).


A high‑level Avro **Object Container File** reader.

`Reader` pulls blocks from a `BufRead` source, handles optional block compression,
and decodes them row‑by‑row into Arrow `RecordBatch` values using an internal
`Decoder`. It implements both:

* [`Iterator<Item = Result<RecordBatch, ArrowError>>`], and
* `RecordBatchReader`, guaranteeing a consistent schema across all produced batches.

---

## ReaderBuilder

`struct` · `arrow_avro::reader::ReaderBuilder`

```rust
struct ReaderBuilder
```

**Derives**: Debug, Default

**Methods** (12)

```rust
fn build<R: BufRead>(self, reader: R) -> Result<Reader<R>, ArrowError>
fn build_decoder(self) -> Result<Decoder, ArrowError>
fn new() -> Self
fn use_utf8view(&self) -> bool
fn with_active_fingerprint(self, fp: Fingerprint) -> Self
fn with_batch_size(self, batch_size: usize) -> Self
fn with_projection(self, projection: Vec<usize>) -> Self
fn with_reader_schema(self, schema: AvroSchema) -> Self
fn with_strict_mode(self, strict_mode: bool) -> Self
fn with_tz(self, tz: Tz) -> Self
fn with_utf8_view(self, utf8_view: bool) -> Self
fn with_writer_schema_store(self, store: SchemaStore) -> Self
```

[Full member, field, variant and typed contracts](../operations/arrow_avro.reader.ReaderBuilder.md).


A builder that configures and constructs Avro readers and decoders.

`ReaderBuilder` is the primary entry point for this module. It supports:

* OCF reading via `Self::build`, returning a `Reader` over any `BufRead`;
* streaming decoding via `Self::build_decoder`, returning a `Decoder`.

### Options

* **`batch_size`**: Max rows per `RecordBatch` (default: `1024`). See `Self::with_batch_size`.
* **`utf8_view`**: Use Arrow `StringViewArray` for string columns (default: `false`).
  See `Self::with_utf8_view`.
* **`strict_mode`**: Opt‑in to stricter union handling (default: `false`).
  See `Self::with_strict_mode`.
* **`reader_schema`**: Optional reader schema (projection / evolution) used when decoding
  values (default: `None`). See `Self::with_reader_schema`.
* **`projection`**: Optional projection of **top‑level record fields** by index (default: `None`).

  If set, the effective reader schema is **pruned** to include only the projected fields, in the
  specified order:

  * If a reader schema is provided, that schema is pruned.
  * Otherwise, a reader schema is derived from the writer schema and then pruned.
  * For streaming `Decoder` with multiple writer schemas and no reader schema, a projected reader
    schema is derived **per writer schema** in the `SchemaStore`.

  See `Self::with_projection`.
* **`writer_schema_store`**: Required for building a `Decoder` for single‑object or
  Confluent framing. Maps fingerprints to Avro schemas. See `Self::with_writer_schema_store`.
* **`active_fingerprint`**: Optional starting fingerprint for streaming decode when the
  first frame omits one (rare). See `Self::with_active_fingerprint`.

### Examples

Read an OCF file in batches of 4096 rows:

```no_run
use std::fs::File;
use std::io::BufReader;
use arrow_avro::reader::ReaderBuilder;

let file = File::open("data.avro")?;
let mut reader = ReaderBuilder::new()
    .with_batch_size(4096)
    .build(BufReader::new(file))?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

Build a `Decoder` for Confluent messages:

```
use arrow_avro::schema::{AvroSchema, SchemaStore, Fingerprint, FingerprintAlgorithm};
use arrow_avro::reader::ReaderBuilder;

let mut store = SchemaStore::new_with_type(FingerprintAlgorithm::Id);
store.set(Fingerprint::Id(1234), AvroSchema::new(r#"{"type":"record","name":"E","fields":[]}"#.to_string()))?;

let decoder = ReaderBuilder::new()
    .with_writer_schema_store(store)
    .build_decoder()?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

---
