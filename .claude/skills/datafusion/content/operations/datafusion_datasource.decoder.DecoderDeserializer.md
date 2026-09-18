# `datafusion_datasource::decoder::DecoderDeserializer`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.decoder.DecoderDeserializer.json).

<a id="op-e58550305595c9b3dfad6532"></a>
## DecoderDeserializer

`struct` · `datafusion_datasource::decoder::DecoderDeserializer` · datafusion-datasource 55.1.0

```rust
struct DecoderDeserializer<T: Decoder>
```

Source: `src/decoder.rs:148`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

A generic, decoder-based deserialization scheme for processing encoded data.

This struct is responsible for converting a stream of bytes, which represent
encoded data, into a stream of `RecordBatch` objects, following the specified
schema and formatting options. It also handles any buffering necessary to satisfy
the `Decoder` interface.

<a id="op-9c0a383b15cbc4dbbfc609b8"></a>
## digest

`function` · `datafusion_datasource::decoder::DecoderDeserializer::digest` · datafusion-datasource 55.1.0

```rust
fn digest(&mut self, message: Bytes) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_datasource::decoder::DecoderDeserializer", "path": "DecoderDeserializer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_datasource::decoder::Decoder", "path": "Decoder"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 1], "end": [140, 2], "filename": "src/decoder.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "Bytes"}}}], "constraints": []}}, "id": "datafusion_datasource::decoder::BatchDeserializer", "path": "BatchDeserializer"}, "trait_path": "datafusion_datasource::decoder::BatchDeserializer"}`

Source: `src/decoder.rs:99`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cdd84912e721df479b25f918"></a>
## finish

`function` · `datafusion_datasource::decoder::DecoderDeserializer::finish` · datafusion-datasource 55.1.0

```rust
fn finish(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_datasource::decoder::DecoderDeserializer", "path": "DecoderDeserializer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_datasource::decoder::Decoder", "path": "Decoder"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 1], "end": [140, 2], "filename": "src/decoder.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "Bytes"}}}], "constraints": []}}, "id": "datafusion_datasource::decoder::BatchDeserializer", "path": "BatchDeserializer"}, "trait_path": "datafusion_datasource::decoder::BatchDeserializer"}`

Source: `src/decoder.rs:135`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5872e65a6f584eec86d2ebd9"></a>
## fmt

`function` · `datafusion_datasource::decoder::DecoderDeserializer::fmt` · datafusion-datasource 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_datasource::decoder::DecoderDeserializer", "path": "DecoderDeserializer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_datasource::decoder::Decoder", "path": "Decoder"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 1], "end": [96, 2], "filename": "src/decoder.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/decoder.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-25b1979507c99a743ba062a4"></a>
## new

`function` · `datafusion_datasource::decoder::DecoderDeserializer::new` · datafusion-datasource 55.1.0

```rust
fn new(decoder: T) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_datasource::decoder::DecoderDeserializer", "path": "DecoderDeserializer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_datasource::decoder::Decoder", "path": "Decoder"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [157, 1], "end": [166, 2], "filename": "src/decoder.rs"}, "trait": null, "trait_path": null}`

Source: `src/decoder.rs:159`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Creates a new `DecoderDeserializer` with the provided decoder.

<a id="op-5928e4767a97585d2dbd7b48"></a>
## next

`function` · `datafusion_datasource::decoder::DecoderDeserializer::next` · datafusion-datasource 55.1.0

```rust
fn next(&mut self) -> Result<DeserializerOutput, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_datasource::decoder::DecoderDeserializer", "path": "DecoderDeserializer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_datasource::decoder::Decoder", "path": "Decoder"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 1], "end": [140, 2], "filename": "src/decoder.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "Bytes"}}}], "constraints": []}}, "id": "datafusion_datasource::decoder::BatchDeserializer", "path": "BatchDeserializer"}, "trait_path": "datafusion_datasource::decoder::BatchDeserializer"}`

Source: `src/decoder.rs:109`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
