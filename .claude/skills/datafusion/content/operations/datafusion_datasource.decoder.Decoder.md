# `datafusion_datasource::decoder::Decoder`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.decoder.Decoder.json).

<a id="op-54d4c8f9611be6ffc85a10c3"></a>
## Decoder

`trait` · `datafusion_datasource::decoder::Decoder` · datafusion-datasource 55.1.0

```rust
trait Decoder: Send + fmt::Debug
```

Source: `src/decoder.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

A general interface for decoders such as [`arrow::json::reader::Decoder`] and
[`arrow::csv::reader::Decoder`]. Defines an interface similar to
[`Decoder::decode`] and [`Decoder::flush`] methods, but also includes
a method to check if the decoder can flush early. Intended to be used in
conjunction with [`DecoderDeserializer`](../operations/datafusion_datasource.decoder.DecoderDeserializer.md#op-e58550305595c9b3dfad6532).

[`arrow::json::reader::Decoder`]: ::arrow::json::reader::Decoder
[`arrow::csv::reader::Decoder`]: ::arrow::csv::reader::Decoder
[`Decoder::decode`]: ::arrow::json::reader::Decoder::decode
[`Decoder::flush`]: ::arrow::json::reader::Decoder::flush

Unresolved upstream links (retained, not inferred): `::arrow::json::reader::Decoder::decode`, `::arrow::json::reader::Decoder::flush`.

<a id="op-fabe614c3bebd66a390a0556"></a>
## can_flush_early

`function` · `datafusion_datasource::decoder::Decoder::can_flush_early` · datafusion-datasource 55.1.0

```rust
fn can_flush_early(&self) -> bool
```

Source: `src/decoder.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Whether the decoder can flush early in its current state.

<a id="op-2d2fee2898673dcfc99ce82c"></a>
## decode

`function` · `datafusion_datasource::decoder::Decoder::decode` · datafusion-datasource 55.1.0

```rust
fn decode(&mut self, buf: &[u8]) -> Result<usize, ArrowError>
```

Source: `src/decoder.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

See [`arrow::json::reader::Decoder::decode`].

[`arrow::json::reader::Decoder::decode`]: ::arrow::json::reader::Decoder::decode

Unresolved upstream links (retained, not inferred): `::arrow::json::reader::Decoder::decode`.

<a id="op-02e4bbf51cae63a987b6ea55"></a>
## flush

`function` · `datafusion_datasource::decoder::Decoder::flush` · datafusion-datasource 55.1.0

```rust
fn flush(&mut self) -> Result<Option<RecordBatch>, ArrowError>
```

Source: `src/decoder.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

See [`arrow::json::reader::Decoder::flush`].

[`arrow::json::reader::Decoder::flush`]: ::arrow::json::reader::Decoder::flush

Unresolved upstream links (retained, not inferred): `::arrow::json::reader::Decoder::flush`.
