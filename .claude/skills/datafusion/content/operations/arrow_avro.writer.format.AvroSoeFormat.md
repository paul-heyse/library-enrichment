# `arrow_avro::writer::format::AvroSoeFormat`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_avro.writer.format.AvroSoeFormat.json).

<a id="op-4049ec7a448ab2be3411d3fd"></a>
## AvroSoeFormat

`struct` · `arrow_avro::writer::format::AvroSoeFormat` · arrow-avro 59.3.0

```rust
struct AvroSoeFormat
```

Source: `src/writer/format.rs:113`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Raw Avro binary streaming format using **Single-Object Encoding** per record.

Each record written by the stream writer is framed with a prefix determined
by the schema fingerprinting algorithm.

See: <https://avro.apache.org/docs/1.11.1/specification/#single-object-encoding>
See: <https://docs.confluent.io/platform/current/schema-registry/fundamentals/serdes-develop/index.html#wire-format>

<a id="op-b33a184faf4d1f81e49d395f"></a>
## NEEDS_PREFIX

`assoc_const` · `arrow_avro::writer::format::AvroSoeFormat::NEEDS_PREFIX` · arrow-avro 59.3.0

```rust
NEEDS_PREFIX
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::writer::format::AvroSoeFormat", "path": "AvroSoeFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [115, 1], "end": [134, 2], "filename": "src/writer/format.rs"}, "trait": {"args": null, "id": "arrow_avro::writer::format::AvroFormat", "path": "AvroFormat"}, "trait_path": "arrow_avro::writer::format::AvroFormat"}`

Source: `src/writer/format.rs:116`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fededc937ec626e1f80abd7f"></a>
## default

`function` · `arrow_avro::writer::format::AvroSoeFormat::default` · arrow-avro 59.3.0

```rust
fn default() -> AvroSoeFormat
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::writer::format::AvroSoeFormat", "path": "AvroSoeFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [112, 17], "end": [112, 24], "filename": "src/writer/format.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/writer/format.rs:112`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-07a0b5fc66a71d5a474c4f6a"></a>
## fmt

`function` · `arrow_avro::writer::format::AvroSoeFormat::fmt` · arrow-avro 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::writer::format::AvroSoeFormat", "path": "AvroSoeFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [112, 10], "end": [112, 15], "filename": "src/writer/format.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/writer/format.rs:112`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0c8f1cbb454b01ff8b50290f"></a>
## start_stream

`function` · `arrow_avro::writer::format::AvroSoeFormat::start_stream` · arrow-avro 59.3.0

```rust
fn start_stream<W: Write>(&mut self, _writer: &mut W, _schema: &Schema, compression: Option<CompressionCodec>) -> Result<(), AvroError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::writer::format::AvroSoeFormat", "path": "AvroSoeFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [115, 1], "end": [134, 2], "filename": "src/writer/format.rs"}, "trait": {"args": null, "id": "arrow_avro::writer::format::AvroFormat", "path": "AvroFormat"}, "trait_path": "arrow_avro::writer::format::AvroFormat"}`

Source: `src/writer/format.rs:117`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d3fdad371b318e771674557"></a>
## sync_marker

`function` · `arrow_avro::writer::format::AvroSoeFormat::sync_marker` · arrow-avro 59.3.0

```rust
fn sync_marker(&self) -> Option<&[u8; 16]>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::writer::format::AvroSoeFormat", "path": "AvroSoeFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [115, 1], "end": [134, 2], "filename": "src/writer/format.rs"}, "trait": {"args": null, "id": "arrow_avro::writer::format::AvroFormat", "path": "AvroFormat"}, "trait_path": "arrow_avro::writer::format::AvroFormat"}`

Source: `src/writer/format.rs:131`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
