# `arrow_avro::writer`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_avro.writer.json).

<a id="op-11fcfb1ac6b8ac80101af1d3"></a>
## writer

`module` · `arrow_avro::writer` · arrow-avro 59.3.0

```rust
mod writer
```

Source: `src/writer/mod.rs:18`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Core functionality for writing Arrow arrays as Avro data

Implements the primary writer interface and record encoding logic.
Avro writer implementation for the `arrow-avro` crate.

# Overview

Use this module to serialize Arrow [`arrow_array::RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) values into Avro. Three output
modes are supported:

* **[`crate::writer::AvroWriter`](../operations/arrow_avro.writer.AvroWriter.md#op-109f7d6c1f115d2ec6574736)** — writes an **Object Container File (OCF)**: a self‑describing
  file with header (schema JSON and metadata), optional compression, data blocks, and
  sync markers. See Avro 1.11.1 "Object Container Files."
  <https://avro.apache.org/docs/1.11.1/specification/#object-container-files>

* **[`crate::writer::AvroStreamWriter`](../operations/arrow_avro.writer.AvroStreamWriter.md#op-74718ef152874c77b8b4f550)** — writes a **Single Object Encoding (SOE) Stream** without
  any container framing. This is useful when the schema is known out‑of‑band (i.e.,
  via a registry) and you want minimal overhead.

* **[`crate::writer::Encoder`](../operations/arrow_avro.writer.Encoder.md#op-aabee555cff8f50e9279fede)** — a row-by-row encoder that buffers encoded records into a single
  contiguous byte buffer and returns per-row [`bytes::Bytes`] slices.
  Ideal for publishing individual messages to Kafka, Pulsar, or other message queues
  where each message must be a self-contained Avro payload.

## Which writer should you use?

| Use Case | Recommended Type |
|----------|------------------|
| Write an OCF file to disk | [`crate::writer::AvroWriter`](../operations/arrow_avro.writer.AvroWriter.md#op-109f7d6c1f115d2ec6574736) |
| Stream records continuously to a file/socket | [`crate::writer::AvroStreamWriter`](../operations/arrow_avro.writer.AvroStreamWriter.md#op-74718ef152874c77b8b4f550) |
| Publish individual records to Kafka/Pulsar | [`crate::writer::Encoder`](../operations/arrow_avro.writer.Encoder.md#op-aabee555cff8f50e9279fede) |
| Need per-row byte slices for custom framing | [`crate::writer::Encoder`](../operations/arrow_avro.writer.Encoder.md#op-aabee555cff8f50e9279fede) |

## Per-Record Prefix Formats

For [`crate::writer::AvroStreamWriter`](../operations/arrow_avro.writer.AvroStreamWriter.md#op-74718ef152874c77b8b4f550) and [`crate::writer::Encoder`](../operations/arrow_avro.writer.Encoder.md#op-aabee555cff8f50e9279fede), each record is automatically prefixed
based on the fingerprint strategy:

| Strategy | Prefix | Use Case |
|----------|--------|----------|
| `FingerprintStrategy::Rabin` (default) | `0xC3 0x01` + 8-byte LE Rabin fingerprint | Standard Avro SOE |
| `FingerprintStrategy::Id(id)` | `0x00` + 4-byte BE schema ID | [Confluent Schema Registry] |
| `FingerprintStrategy::Id64(id)` | `0x00` + 8-byte BE schema ID | [Apicurio Registry] |

[Confluent Schema Registry]: https://docs.confluent.io/platform/current/schema-registry/fundamentals/serdes-develop/index.html#wire-format
[Apicurio Registry]: https://www.apicur.io/registry/docs/apicurio-registry/1.3.3.Final/getting-started/assembly-using-kafka-client-serdes.html#registry-serdes-types-avro-registry

## Choosing the Avro Schema

By default, the writer converts your Arrow schema to Avro (including a top‑level record
name). If you already have an Avro schema JSON you want to use verbatim, put it into the
Arrow schema metadata under the [`SCHEMA_METADATA_KEY`](crate::schema::SCHEMA_METADATA_KEY)
key before constructing the writer. The builder will use that schema instead of generating
a new one.

## Compression

For OCF ([`crate::writer::AvroWriter`](../operations/arrow_avro.writer.AvroWriter.md#op-109f7d6c1f115d2ec6574736)), you may enable a compression codec via
[`crate::writer::WriterBuilder::with_compression`](../operations/arrow_avro.writer.WriterBuilder.md#op-daf5c1807130571209f17f80). The chosen codec is written into the file header
and used for subsequent blocks. SOE stream writing ([`crate::writer::AvroStreamWriter`](../operations/arrow_avro.writer.AvroStreamWriter.md#op-74718ef152874c77b8b4f550), [`crate::writer::Encoder`](../operations/arrow_avro.writer.Encoder.md#op-aabee555cff8f50e9279fede))
does not apply container‑level compression.

# Examples

## Writing an OCF File

```
use std::sync::Arc;
use arrow_array::{ArrayRef, Int64Array, StringArray, RecordBatch};
use arrow_schema::{DataType, Field, Schema};
use arrow_avro::writer::AvroWriter;

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let schema = Schema::new(vec![
    Field::new("id", DataType::Int64, false),
    Field::new("name", DataType::Utf8, false),
]);

let batch = RecordBatch::try_new(
    Arc::new(schema.clone()),
    vec![
        Arc::new(Int64Array::from(vec![1, 2])) as ArrayRef,
        Arc::new(StringArray::from(vec!["alice", "bob"])) as ArrayRef,
    ],
)?;

let mut writer = AvroWriter::new(Vec::<u8>::new(), schema)?;
writer.write(&batch)?;
writer.finish()?;
let bytes = writer.into_inner();
assert!(!bytes.is_empty());
# Ok(())
# }
```

## Using the Row-by-Row Encoder for Message Queues

```
use std::sync::Arc;
use arrow_array::{ArrayRef, Int32Array, RecordBatch};
use arrow_schema::{DataType, Field, Schema};
use arrow_avro::writer::{WriterBuilder, format::AvroSoeFormat};
use arrow_avro::schema::FingerprintStrategy;

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let schema = Schema::new(vec![Field::new("x", DataType::Int32, false)]);
let batch = RecordBatch::try_new(
    Arc::new(schema.clone()),
    vec![Arc::new(Int32Array::from(vec![1, 2, 3])) as ArrayRef],
)?;

// Build an Encoder with Confluent wire format (schema ID = 42)
let mut encoder = WriterBuilder::new(schema)
    .with_fingerprint_strategy(FingerprintStrategy::Id(42))
    .build_encoder::<AvroSoeFormat>()?;

encoder.encode(&batch)?;

// Get the buffered rows (zero-copy views into a single backing buffer)
let rows = encoder.flush();
assert_eq!(rows.len(), 3);

// Each row has Confluent wire format: magic byte + 4-byte schema ID + body
for row in rows.iter() {
    assert_eq!(row[0], 0x00); // Confluent magic byte
}
# Ok(())
# }
```

---

Unresolved upstream links (retained, not inferred): ``bytes::Bytes``.
