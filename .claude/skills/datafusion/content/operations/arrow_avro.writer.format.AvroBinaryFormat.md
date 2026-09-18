# `arrow_avro::writer::format::AvroBinaryFormat`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_avro.writer.format.AvroBinaryFormat.json).

<a id="op-c82be6059b1fc8e623781b3d"></a>
## AvroBinaryFormat

`struct` · `arrow_avro::writer::format::AvroBinaryFormat` · arrow-avro 59.3.0

```rust
struct AvroBinaryFormat
```

Source: `src/writer/format.rs:146`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Unframed Avro binary streaming format ("raw Avro record body bytes (no prefix, no OCF header)").

Each record written by the stream writer contains only the raw Avro
record body bytes (i.e., the Avro binary encoding of the datum) with **no**
per-record prefix and **no** Object Container File (OCF) header.

This format is useful when another transport provides framing (for example,
length-delimited buffers) or when embedding Avro record payloads inside a
larger envelope.

<a id="op-8b4016f5ef27f3d1ce21d64c"></a>
## NEEDS_PREFIX

`assoc_const` · `arrow_avro::writer::format::AvroBinaryFormat::NEEDS_PREFIX` · arrow-avro 59.3.0

```rust
NEEDS_PREFIX
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::writer::format::AvroBinaryFormat", "path": "AvroBinaryFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [148, 1], "end": [168, 2], "filename": "src/writer/format.rs"}, "trait": {"args": null, "id": "arrow_avro::writer::format::AvroFormat", "path": "AvroFormat"}, "trait_path": "arrow_avro::writer::format::AvroFormat"}`

Source: `src/writer/format.rs:149`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8460d0ec4d8b88d739cbcb44"></a>
## default

`function` · `arrow_avro::writer::format::AvroBinaryFormat::default` · arrow-avro 59.3.0

```rust
fn default() -> AvroBinaryFormat
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::writer::format::AvroBinaryFormat", "path": "AvroBinaryFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [145, 17], "end": [145, 24], "filename": "src/writer/format.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/writer/format.rs:145`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-34493cf97388552d84693c4b"></a>
## fmt

`function` · `arrow_avro::writer::format::AvroBinaryFormat::fmt` · arrow-avro 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::writer::format::AvroBinaryFormat", "path": "AvroBinaryFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [145, 10], "end": [145, 15], "filename": "src/writer/format.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/writer/format.rs:145`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d53671cb1d494676093ed024"></a>
## start_stream

`function` · `arrow_avro::writer::format::AvroBinaryFormat::start_stream` · arrow-avro 59.3.0

```rust
fn start_stream<W: Write>(&mut self, _writer: &mut W, _schema: &Schema, compression: Option<CompressionCodec>) -> Result<(), AvroError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::writer::format::AvroBinaryFormat", "path": "AvroBinaryFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [148, 1], "end": [168, 2], "filename": "src/writer/format.rs"}, "trait": {"args": null, "id": "arrow_avro::writer::format::AvroFormat", "path": "AvroFormat"}, "trait_path": "arrow_avro::writer::format::AvroFormat"}`

Source: `src/writer/format.rs:151`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06927a7355be68187d6b522f"></a>
## sync_marker

`function` · `arrow_avro::writer::format::AvroBinaryFormat::sync_marker` · arrow-avro 59.3.0

```rust
fn sync_marker(&self) -> Option<&[u8; 16]>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::writer::format::AvroBinaryFormat", "path": "AvroBinaryFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [148, 1], "end": [168, 2], "filename": "src/writer/format.rs"}, "trait": {"args": null, "id": "arrow_avro::writer::format::AvroFormat", "path": "AvroFormat"}, "trait_path": "arrow_avro::writer::format::AvroFormat"}`

Source: `src/writer/format.rs:165`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
