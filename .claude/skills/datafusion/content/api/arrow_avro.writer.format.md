# `arrow_avro::writer::format`

Crate `arrow-avro` · 4 public items · structured records in [`model/arrow_avro.writer.format.json`](../model/arrow_avro.writer.format.json)

## AvroBinaryFormat

`struct` · `arrow_avro::writer::format::AvroBinaryFormat`

```rust
struct AvroBinaryFormat
```

**Implements**: `arrow_avro::writer::format::AvroFormat`

**Derives**: Debug, Default

**via `arrow_avro::writer::format::AvroFormat`**

```rust
fn start_stream<W: Write>(&mut self, _writer: &mut W, _schema: &Schema, compression: Option<CompressionCodec>) -> Result<(), AvroError>
fn sync_marker(&self) -> Option<&[u8; 16]>
```

[Full member, field, variant and typed contracts](../operations/arrow_avro.writer.format.AvroBinaryFormat.md).


Unframed Avro binary streaming format ("raw Avro record body bytes (no prefix, no OCF header)").

Each record written by the stream writer contains only the raw Avro
record body bytes (i.e., the Avro binary encoding of the datum) with **no**
per-record prefix and **no** Object Container File (OCF) header.

This format is useful when another transport provides framing (for example,
length-delimited buffers) or when embedding Avro record payloads inside a
larger envelope.

---

## AvroOcfFormat

`struct` · `arrow_avro::writer::format::AvroOcfFormat`

```rust
struct AvroOcfFormat
```

**Implements**: `arrow_avro::writer::format::AvroFormat`

**Derives**: Debug, Default

**via `arrow_avro::writer::format::AvroFormat`**

```rust
fn start_stream<W: Write>(&mut self, writer: &mut W, schema: &Schema, compression: Option<CompressionCodec>) -> Result<(), AvroError>
fn sync_marker(&self) -> Option<&[u8; 16]>
```

[Full member, field, variant and typed contracts](../operations/arrow_avro.writer.format.AvroOcfFormat.md).


Avro Object Container File (OCF) format writer.

---

## AvroSoeFormat

`struct` · `arrow_avro::writer::format::AvroSoeFormat`

```rust
struct AvroSoeFormat
```

**Implements**: `arrow_avro::writer::format::AvroFormat`

**Derives**: Debug, Default

**via `arrow_avro::writer::format::AvroFormat`**

```rust
fn start_stream<W: Write>(&mut self, _writer: &mut W, _schema: &Schema, compression: Option<CompressionCodec>) -> Result<(), AvroError>
fn sync_marker(&self) -> Option<&[u8; 16]>
```

[Full member, field, variant and typed contracts](../operations/arrow_avro.writer.format.AvroSoeFormat.md).


Raw Avro binary streaming format using **Single-Object Encoding** per record.

Each record written by the stream writer is framed with a prefix determined
by the schema fingerprinting algorithm.

See: <https://avro.apache.org/docs/1.11.1/specification/#single-object-encoding>
See: <https://docs.confluent.io/platform/current/schema-registry/fundamentals/serdes-develop/index.html#wire-format>

---

## AvroFormat

`trait` · `arrow_avro::writer::format::AvroFormat`

```rust
trait AvroFormat: Debug + Default
```

**Implementors** (3)

- `arrow_avro::writer::format::AvroBinaryFormat`
- `arrow_avro::writer::format::AvroOcfFormat`
- `arrow_avro::writer::format::AvroSoeFormat`

**Methods** (2)

```rust
fn start_stream<W: Write>(&mut self, writer: &mut W, schema: &Schema, compression: Option<CompressionCodec>) -> Result<(), AvroError>
fn sync_marker(&self) -> Option<&[u8; 16]>
```

[Full member, field, variant and typed contracts](../operations/arrow_avro.writer.format.AvroFormat.md).


Format abstraction implemented by each container‐level writer.

---
