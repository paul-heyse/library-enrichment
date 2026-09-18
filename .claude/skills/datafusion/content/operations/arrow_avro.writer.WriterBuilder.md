# `arrow_avro::writer::WriterBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_avro.writer.WriterBuilder.json).

<a id="op-7da2544ac462be7ca6222a9e"></a>
## WriterBuilder

`struct` · `arrow_avro::writer::WriterBuilder` · arrow-avro 59.3.0

```rust
struct WriterBuilder
```

Source: `src/writer/mod.rs:318`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Builder to configure and create a `Writer`.

<a id="op-51226c47f6c1e887614eea9d"></a>
## build

`function` · `arrow_avro::writer::WriterBuilder::build` · arrow-avro 59.3.0

```rust
fn build<W, F>(self, writer: W) -> Result<Writer<W, F>, AvroError> where W: Write, F: AvroFormat
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [326, 1], "end": [450, 2], "filename": "src/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/mod.rs:427`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Build a new [`Writer`](../operations/arrow_avro.writer.Writer.md#op-2a9e145d555e09d95bf56622) with the specified [`AvroFormat`](../operations/arrow_avro.writer.format.AvroFormat.md#op-01cd868ab7509b86a0b122ad) and builder options.

<a id="op-69f0653f3a264795661b56bf"></a>
## build_encoder

`function` · `arrow_avro::writer::WriterBuilder::build_encoder` · arrow-avro 59.3.0

```rust
fn build_encoder<F: AvroFormat>(self) -> Result<Encoder, AvroError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [326, 1], "end": [450, 2], "filename": "src/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/mod.rs:410`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Build a new [`Encoder`](../operations/arrow_avro.writer.Encoder.md#op-aabee555cff8f50e9279fede) for the given [`AvroFormat`](../operations/arrow_avro.writer.format.AvroFormat.md#op-01cd868ab7509b86a0b122ad).

`Encoder` only supports stream formats (no OCF sync markers). Attempting to build an
encoder with an OCF format (e.g. [`AvroOcfFormat`](../operations/arrow_avro.writer.format.AvroOcfFormat.md#op-6ce0f711613e2a7fb498b630)) will return an error.

<a id="op-c02bb0fca9793dd2ae667b62"></a>
## clone

`function` · `arrow_avro::writer::WriterBuilder::clone` · arrow-avro 59.3.0

```rust
fn clone(&self) -> WriterBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [317, 17], "end": [317, 22], "filename": "src/writer/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/writer/mod.rs:317`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c70aed389d2ea632cca4d1bd"></a>
## fmt

`function` · `arrow_avro::writer::WriterBuilder::fmt` · arrow-avro 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [317, 10], "end": [317, 15], "filename": "src/writer/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/writer/mod.rs:317`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0241877df51f74c30b680ade"></a>
## new

`function` · `arrow_avro::writer::WriterBuilder::new` · arrow-avro 59.3.0

```rust
fn new(schema: Schema) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [326, 1], "end": [450, 2], "filename": "src/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/mod.rs:333`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Create a new builder with default settings.

The Avro schema used for writing is determined as follows:
1) If the Arrow schema metadata contains `avro::schema` (see `SCHEMA_METADATA_KEY`),
   that JSON is used verbatim.
2) Otherwise, the Arrow schema is converted to an Avro record schema.

<a id="op-c7e8281ce89da4918f2deb87"></a>
## with_capacity

`function` · `arrow_avro::writer::WriterBuilder::with_capacity` · arrow-avro 59.3.0

```rust
fn with_capacity(self, capacity: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [326, 1], "end": [450, 2], "filename": "src/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/mod.rs:359`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Sets the expected capacity (in bytes) for internal buffers.

This is used as a hint to pre-allocate staging buffers for writing.

<a id="op-daf5c1807130571209f17f80"></a>
## with_compression

`function` · `arrow_avro::writer::WriterBuilder::with_compression` · arrow-avro 59.3.0

```rust
fn with_compression(self, codec: Option<CompressionCodec>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [326, 1], "end": [450, 2], "filename": "src/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/mod.rs:351`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Change the compression codec.

<a id="op-614ecb7dc5408a7c1eb78f44"></a>
## with_fingerprint_strategy

`function` · `arrow_avro::writer::WriterBuilder::with_fingerprint_strategy` · arrow-avro 59.3.0

```rust
fn with_fingerprint_strategy(self, strategy: FingerprintStrategy) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [326, 1], "end": [450, 2], "filename": "src/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/mod.rs:345`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Set the fingerprinting strategy for the stream writer.
This determines the per-record prefix format.

<a id="op-e3c531c161305c592d735dd7"></a>
## with_row_capacity

`function` · `arrow_avro::writer::WriterBuilder::with_row_capacity` · arrow-avro 59.3.0

```rust
fn with_row_capacity(self, capacity: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [326, 1], "end": [450, 2], "filename": "src/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/mod.rs:368`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Sets the expected byte size for each encoded row.

This setting affects [`Encoder`](../operations/arrow_avro.writer.Encoder.md#op-aabee555cff8f50e9279fede) created via [`build_encoder`](Self::build_encoder).
It is used as a hint to reduce reallocations when the typical encoded row size is known.
