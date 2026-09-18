# `datafusion_datasource::decoder`

Crate `datafusion-datasource` · 5 public items · structured records in [`model/datafusion_datasource.decoder.json`](../model/datafusion_datasource.decoder.json)

## DeserializerOutput

`enum` · `datafusion_datasource::decoder::DeserializerOutput`

```rust
enum DeserializerOutput
```

**Variants**: `RecordBatch`, `RequiresMoreData`, `InputExhausted`

**Derives**: Debug, PartialEq, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/datafusion_datasource.decoder.DeserializerOutput.md).


Possible outputs of a [`BatchDeserializer`].

---

## deserialize_stream

`function` · `datafusion_datasource::decoder::deserialize_stream`

```rust
fn deserialize_stream<'a>(input: impl Stream<Item = datafusion_common::Result<bytes::Bytes>> + Unpin + Send + 'a, deserializer: impl BatchDeserializer<bytes::Bytes> + 'a) -> futures::stream::BoxStream<'a, datafusion_common::Result<::arrow::array::RecordBatch, arrow::error::ArrowError>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource.decoder.deserialize_stream.md).


Deserializes a stream of bytes into a stream of [`RecordBatch`] objects using the
provided deserializer.

Returns a boxed stream of `Result<RecordBatch, ArrowError>`. The stream yields [`RecordBatch`]
objects as they are produced by the deserializer, or an [`ArrowError`] if an error
occurs while polling the input or deserializing.

---

## DecoderDeserializer

`struct` · `datafusion_datasource::decoder::DecoderDeserializer`

```rust
struct DecoderDeserializer<T: Decoder>
```

**Implements**: `datafusion_datasource::decoder::BatchDeserializer`

**Derives**: Debug

**Methods** (1)

```rust
fn new(decoder: T) -> Self
```

**via `datafusion_datasource::decoder::BatchDeserializer`**

```rust
fn digest(&mut self, message: Bytes) -> usize
fn finish(&mut self)
fn next(&mut self) -> Result<DeserializerOutput, ArrowError>
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource.decoder.DecoderDeserializer.md).


A generic, decoder-based deserialization scheme for processing encoded data.

This struct is responsible for converting a stream of bytes, which represent
encoded data, into a stream of `RecordBatch` objects, following the specified
schema and formatting options. It also handles any buffering necessary to satisfy
the `Decoder` interface.

---

## BatchDeserializer

`trait` · `datafusion_datasource::decoder::BatchDeserializer`

```rust
trait BatchDeserializer<T>: Send + fmt::Debug
```

**Implementors** (1)

- `datafusion_datasource::decoder::DecoderDeserializer`

**Methods** (3)

```rust
fn digest(&mut self, message: T) -> usize
fn finish(&mut self)
fn next(&mut self) -> Result<DeserializerOutput, ArrowError>
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource.decoder.BatchDeserializer.md).


Trait defining a scheme for deserializing byte streams into structured data.
Implementors of this trait are responsible for converting raw bytes into
`RecordBatch` objects.

---

## Decoder

`trait` · `datafusion_datasource::decoder::Decoder`

```rust
trait Decoder: Send + fmt::Debug
```

**Implementors** (2)

- `datafusion_datasource_csv::file_format::CsvDecoder`
- `datafusion_datasource_json::file_format::JsonDecoder`

**Methods** (3)

```rust
fn can_flush_early(&self) -> bool
fn decode(&mut self, buf: &[u8]) -> Result<usize, ArrowError>
fn flush(&mut self) -> Result<Option<RecordBatch>, ArrowError>
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource.decoder.Decoder.md).


A general interface for decoders such as [`arrow::json::reader::Decoder`] and
[`arrow::csv::reader::Decoder`]. Defines an interface similar to
[`Decoder::decode`] and [`Decoder::flush`] methods, but also includes
a method to check if the decoder can flush early. Intended to be used in
conjunction with [`DecoderDeserializer`].

[`arrow::json::reader::Decoder`]: ::arrow::json::reader::Decoder
[`arrow::csv::reader::Decoder`]: ::arrow::csv::reader::Decoder
[`Decoder::decode`]: ::arrow::json::reader::Decoder::decode
[`Decoder::flush`]: ::arrow::json::reader::Decoder::flush

---
