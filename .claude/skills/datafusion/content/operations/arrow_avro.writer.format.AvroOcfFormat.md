# `arrow_avro::writer::format::AvroOcfFormat`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_avro.writer.format.AvroOcfFormat.json).

<a id="op-6ce0f711613e2a7fb498b630"></a>
## AvroOcfFormat

`struct` · `arrow_avro::writer::format::AvroOcfFormat` · arrow-avro 59.3.0

```rust
struct AvroOcfFormat
```

Source: `src/writer/format.rs:52`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Avro Object Container File (OCF) format writer.

<a id="op-fef5491f4ade7aa136495bce"></a>
## NEEDS_PREFIX

`assoc_const` · `arrow_avro::writer::format::AvroOcfFormat::NEEDS_PREFIX` · arrow-avro 59.3.0

```rust
NEEDS_PREFIX
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::writer::format::AvroOcfFormat", "path": "AvroOcfFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [103, 2], "filename": "src/writer/format.rs"}, "trait": {"args": null, "id": "arrow_avro::writer::format::AvroFormat", "path": "AvroFormat"}, "trait_path": "arrow_avro::writer::format::AvroFormat"}`

Source: `src/writer/format.rs:57`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c04c21d46082d38b19d1df90"></a>
## default

`function` · `arrow_avro::writer::format::AvroOcfFormat::default` · arrow-avro 59.3.0

```rust
fn default() -> AvroOcfFormat
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::writer::format::AvroOcfFormat", "path": "AvroOcfFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 17], "end": [51, 24], "filename": "src/writer/format.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/writer/format.rs:51`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a3ebcb2b7e9281553b8d3d02"></a>
## fmt

`function` · `arrow_avro::writer::format::AvroOcfFormat::fmt` · arrow-avro 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::writer::format::AvroOcfFormat", "path": "AvroOcfFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 10], "end": [51, 15], "filename": "src/writer/format.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/writer/format.rs:51`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3c945e73f7967989ccbcdf45"></a>
## start_stream

`function` · `arrow_avro::writer::format::AvroOcfFormat::start_stream` · arrow-avro 59.3.0

```rust
fn start_stream<W: Write>(&mut self, writer: &mut W, schema: &Schema, compression: Option<CompressionCodec>) -> Result<(), AvroError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::writer::format::AvroOcfFormat", "path": "AvroOcfFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [103, 2], "filename": "src/writer/format.rs"}, "trait": {"args": null, "id": "arrow_avro::writer::format::AvroFormat", "path": "AvroFormat"}, "trait_path": "arrow_avro::writer::format::AvroFormat"}`

Source: `src/writer/format.rs:58`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8fefce2a68a4d6e5995fb46a"></a>
## sync_marker

`function` · `arrow_avro::writer::format::AvroOcfFormat::sync_marker` · arrow-avro 59.3.0

```rust
fn sync_marker(&self) -> Option<&[u8; 16]>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::writer::format::AvroOcfFormat", "path": "AvroOcfFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [103, 2], "filename": "src/writer/format.rs"}, "trait": {"args": null, "id": "arrow_avro::writer::format::AvroFormat", "path": "AvroFormat"}, "trait_path": "arrow_avro::writer::format::AvroFormat"}`

Source: `src/writer/format.rs:100`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
