# `arrow_avro::reader::Decoder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_avro.reader.Decoder.json).

<a id="op-60e2758f8efa1bca70a870ce"></a>
## Decoder

`struct` · `arrow_avro::reader::Decoder` · arrow-avro 59.3.0

```rust
struct Decoder
```

Source: `src/reader/mod.rs:643`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

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

<a id="op-1fad3e906bfd8f79d2acfaab"></a>
## batch_is_empty

`function` · `arrow_avro::reader::Decoder::batch_is_empty` · arrow-avro 59.3.0

```rust
fn batch_is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::reader::Decoder", "path": "Decoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [654, 1], "end": [904, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:881`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Returns true if the decoder has not decoded any batches yet (i.e., the current batch is empty).

<a id="op-46a82949e6c5135ee0003a17"></a>
## batch_is_full

`function` · `arrow_avro::reader::Decoder::batch_is_full` · arrow-avro 59.3.0

```rust
fn batch_is_full(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::reader::Decoder", "path": "Decoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [654, 1], "end": [904, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:876`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Returns true if the decoder has reached its capacity for the current batch.

<a id="op-95c559c0c9214aca2d52af8f"></a>
## batch_size

`function` · `arrow_avro::reader::Decoder::batch_size` · arrow-avro 59.3.0

```rust
fn batch_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::reader::Decoder", "path": "Decoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [654, 1], "end": [904, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:683`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Returns the configured maximum number of rows per batch.

<a id="op-729177c45c704f1f9dadfe46"></a>
## capacity

`function` · `arrow_avro::reader::Decoder::capacity` · arrow-avro 59.3.0

```rust
fn capacity(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::reader::Decoder", "path": "Decoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [654, 1], "end": [904, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:871`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Returns the number of rows that can be added to this decoder before it is full.

<a id="op-ba3c6111a3583be055e585a9"></a>
## decode

`function` · `arrow_avro::reader::Decoder::decode` · arrow-avro 59.3.0

```rust
fn decode(&mut self, data: &[u8]) -> Result<usize, AvroError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::reader::Decoder", "path": "Decoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [654, 1], "end": [904, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:707`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Feed a chunk of bytes into the decoder.

This will:

* Decode at most `Self::batch_size` rows;
* Return the number of input bytes **consumed** from `data` (which may be 0 if more
  bytes are required, or less than `data.len()` if a prefix/body straddles the
  chunk boundary);
* Defer producing a `RecordBatch` until you call `Self::flush`.

# Returns
The number of bytes consumed from `data`.

# Errors
Returns an error if:

* The input indicates an unknown fingerprint (not present in the provided
  `SchemaStore`;
* The Avro body is malformed;
* A strict‑mode union rule is violated (see `ReaderBuilder::with_strict_mode`).

<a id="op-184b72a475847cda809afea4"></a>
## flush

`function` · `arrow_avro::reader::Decoder::flush` · arrow-avro 59.3.0

```rust
fn flush(&mut self) -> Result<Option<RecordBatch>, AvroError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::reader::Decoder", "path": "Decoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [654, 1], "end": [904, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:863`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Produce a `RecordBatch` if at least one row is fully decoded, returning
`Ok(None)` if no new rows are available.

If a schema change was detected while decoding rows for the current batch, the
schema switch is applied **after** flushing this batch, so the **next** batch
(if any) may have a different schema.

<a id="op-053272d553c2f2a05542e282"></a>
## fmt

`function` · `arrow_avro::reader::Decoder::fmt` · arrow-avro 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::reader::Decoder", "path": "Decoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [642, 10], "end": [642, 15], "filename": "src/reader/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/reader/mod.rs:642`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c18b3dbc140ca294dad3d6bd"></a>
## schema

`function` · `arrow_avro::reader::Decoder::schema` · arrow-avro 59.3.0

```rust
fn schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::reader::Decoder", "path": "Decoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [654, 1], "end": [904, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:678`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Returns the Arrow schema for the rows decoded by this decoder.

**Note:** With single‑object or Confluent framing, the schema may change
at a row boundary when the input indicates a new fingerprint.
