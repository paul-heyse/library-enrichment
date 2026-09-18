# `datafusion_datasource::decoder::BatchDeserializer`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.decoder.BatchDeserializer.json).

<a id="op-d0c1e759d47bbcbfde2d9707"></a>
## BatchDeserializer

`trait` · `datafusion_datasource::decoder::BatchDeserializer` · datafusion-datasource 55.1.0

```rust
trait BatchDeserializer<T>: Send + fmt::Debug
```

Source: `src/decoder.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Trait defining a scheme for deserializing byte streams into structured data.
Implementors of this trait are responsible for converting raw bytes into
`RecordBatch` objects.

<a id="op-1f34e184f9cf551da099434a"></a>
## digest

`function` · `datafusion_datasource::decoder::BatchDeserializer::digest` · datafusion-datasource 55.1.0

```rust
fn digest(&mut self, message: T) -> usize
```

Source: `src/decoder.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Feeds a message for deserialization, updating the internal state of
this `BatchDeserializer`. Note that one can call this function multiple
times before calling `next`, which will queue multiple messages for
deserialization. Returns the number of bytes consumed.

<a id="op-2bb339867dc68748aa92c704"></a>
## finish

`function` · `datafusion_datasource::decoder::BatchDeserializer::finish` · datafusion-datasource 55.1.0

```rust
fn finish(&mut self)
```

Source: `src/decoder.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Informs the deserializer that no more messages will be provided for
deserialization.

<a id="op-6252be900c76b43c4c057e90"></a>
## next

`function` · `datafusion_datasource::decoder::BatchDeserializer::next` · datafusion-datasource 55.1.0

```rust
fn next(&mut self) -> Result<DeserializerOutput, ArrowError>
```

Source: `src/decoder.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Attempts to deserialize any pending messages and returns a
`DeserializerOutput` to indicate progress.
