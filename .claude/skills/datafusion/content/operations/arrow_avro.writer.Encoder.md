# `arrow_avro::writer::Encoder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_avro.writer.Encoder.json).

<a id="op-aabee555cff8f50e9279fede"></a>
## Encoder

`struct` · `arrow_avro::writer::Encoder` · arrow-avro 59.3.0

```rust
struct Encoder
```

Source: `src/writer/mod.rs:498`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

A row-by-row encoder for Avro *stream/message* formats (SOE / registry wire formats / raw binary).

Unlike [`Writer`](../operations/arrow_avro.writer.Writer.md#op-2a9e145d555e09d95bf56622), which emits a single continuous byte stream to a [`std::io::Write`] sink,
`Encoder` tracks row boundaries during encoding and returns an [`EncodedRows`](../operations/arrow_avro.writer.EncodedRows.md#op-a24bf6646de2b0d806b87c41) containing:
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

Unresolved upstream links (retained, not inferred): ``std::io::Write``.

<a id="op-c2ed1fd30bb098640ee48966"></a>
## buffered_len

`function` · `arrow_avro::writer::Encoder::buffered_len` · arrow-avro 59.3.0

```rust
fn buffered_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::writer::Encoder", "path": "Encoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [506, 1], "end": [554, 2], "filename": "src/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/mod.rs:551`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Returns the number of encoded rows currently buffered.

<a id="op-41a176b09f70fafae481aae0"></a>
## encode

`function` · `arrow_avro::writer::Encoder::encode` · arrow-avro 59.3.0

```rust
fn encode(&mut self, batch: &RecordBatch) -> Result<(), AvroError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::writer::Encoder", "path": "Encoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [506, 1], "end": [554, 2], "filename": "src/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/mod.rs:508`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Serialize one [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) into the internal buffer.

<a id="op-a86bf005c0cbb0980197d33c"></a>
## encode_batches

`function` · `arrow_avro::writer::Encoder::encode_batches` · arrow-avro 59.3.0

```rust
fn encode_batches(&mut self, batches: &[RecordBatch]) -> Result<(), AvroError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::writer::Encoder", "path": "Encoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [506, 1], "end": [554, 2], "filename": "src/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/mod.rs:524`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

A convenience method to write a slice of [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) values.

<a id="op-c7a2b62f16f52a249150b213"></a>
## flush

`function` · `arrow_avro::writer::Encoder::flush` · arrow-avro 59.3.0

```rust
fn flush(&mut self) -> EncodedRows
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::writer::Encoder", "path": "Encoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [506, 1], "end": [554, 2], "filename": "src/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/mod.rs:534`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Drain and return all currently buffered encoded rows.

The returned [`EncodedRows`](../operations/arrow_avro.writer.EncodedRows.md#op-a24bf6646de2b0d806b87c41) provides per-row payloads as `Bytes` slices.

<a id="op-867da947c413cfcdfd272207"></a>
## fmt

`function` · `arrow_avro::writer::Encoder::fmt` · arrow-avro 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::writer::Encoder", "path": "Encoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [497, 10], "end": [497, 15], "filename": "src/writer/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/writer/mod.rs:497`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8323379effd1b808bdd21bd9"></a>
## schema

`function` · `arrow_avro::writer::Encoder::schema` · arrow-avro 59.3.0

```rust
fn schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::writer::Encoder", "path": "Encoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [506, 1], "end": [554, 2], "filename": "src/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/mod.rs:546`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Returns the Arrow schema used by this encoder.

The returned schema includes metadata with the Avro schema JSON under
the `avro.schema` key.
