# `datafusion_catalog::stream::StreamProvider`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_catalog.stream.StreamProvider.json).

<a id="op-5418487211124c7df7e41256"></a>
## StreamProvider

`trait` · `datafusion_catalog::stream::StreamProvider` · datafusion-catalog 55.1.0

```rust
trait StreamProvider: std::fmt::Debug + Send + Sync
```

Source: `src/stream.rs:116`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

The StreamProvider trait is used as a generic interface for reading and writing from streaming
data sources (such as FIFO, Websocket, Kafka, etc.).  Implementations of the provider are
responsible for providing a `RecordBatchReader` and optionally a `RecordBatchWriter`.

<a id="op-c5980781d2b75a3f53fb3a35"></a>
## reader

`function` · `datafusion_catalog::stream::StreamProvider::reader` · datafusion-catalog 55.1.0

```rust
fn reader(&self) -> Result<Box<dyn RecordBatchReader>>
```

Source: `src/stream.rs:120`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Provide `RecordBatchReader`

<a id="op-f838c0f5b141454f452d05b8"></a>
## schema

`function` · `datafusion_catalog::stream::StreamProvider::schema` · datafusion-catalog 55.1.0

```rust
fn schema(&self) -> &SchemaRef
```

Source: `src/stream.rs:118`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Get a reference to the schema for this stream

<a id="op-9f7276793a3b4e8cf39095b1"></a>
## stream_write_display

`function` · `datafusion_catalog::stream::StreamProvider::stream_write_display` · datafusion-catalog 55.1.0

```rust
fn stream_write_display(&self, t: DisplayFormatType, f: &mut Formatter<'_>) -> std::fmt::Result
```

Source: `src/stream.rs:126`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Display implementation when using as a DataSink

<a id="op-6a6a5c8aef7e0d6da7548ae9"></a>
## writer

`function` · `datafusion_catalog::stream::StreamProvider::writer` · datafusion-catalog 55.1.0

```rust
fn writer(&self) -> Result<Box<dyn RecordBatchWriter>>
```

Source: `src/stream.rs:122`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Provide `RecordBatchWriter`
