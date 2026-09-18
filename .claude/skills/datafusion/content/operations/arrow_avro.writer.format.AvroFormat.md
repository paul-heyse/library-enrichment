# `arrow_avro::writer::format::AvroFormat`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_avro.writer.format.AvroFormat.json).

<a id="op-01cd868ab7509b86a0b122ad"></a>
## AvroFormat

`trait` · `arrow_avro::writer::format::AvroFormat` · arrow-avro 59.3.0

```rust
trait AvroFormat: Debug + Default
```

Source: `src/writer/format.rs:30`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Format abstraction implemented by each container‐level writer.

<a id="op-294dd7f2a527b92a3593ed0f"></a>
## NEEDS_PREFIX

`assoc_const` · `arrow_avro::writer::format::AvroFormat::NEEDS_PREFIX` · arrow-avro 59.3.0

```rust
NEEDS_PREFIX
```

Source: `src/writer/format.rs:34`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

If `true`, the writer for this format will query `single_object_prefix()`
and write the prefix before each record. If `false`, the writer can
skip this step. This is a performance hint for the writer.

<a id="op-559cdd73b60a087343923368"></a>
## start_stream

`function` · `arrow_avro::writer::format::AvroFormat::start_stream` · arrow-avro 59.3.0

```rust
fn start_stream<W: Write>(&mut self, writer: &mut W, schema: &Schema, compression: Option<CompressionCodec>) -> Result<(), AvroError>
```

Source: `src/writer/format.rs:39`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Write any bytes required at the very beginning of the output stream
(file header, etc.).
Implementations **must not** write any record data.

<a id="op-49c5db5bbfdcbeafcdd16ca9"></a>
## sync_marker

`function` · `arrow_avro::writer::format::AvroFormat::sync_marker` · arrow-avro 59.3.0

```rust
fn sync_marker(&self) -> Option<&[u8; 16]>
```

Source: `src/writer/format.rs:47`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Return the 16‑byte sync marker (OCF) or `None` (binary stream).
