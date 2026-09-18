# `datafusion_datasource::decoder::deserialize_stream`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.decoder.deserialize_stream.json).

<a id="op-d5a18503e91756d849c51207"></a>
## deserialize_stream

`function` · `datafusion_datasource::decoder::deserialize_stream` · datafusion-datasource 55.1.0

```rust
fn deserialize_stream<'a>(input: impl Stream<Item = datafusion_common::Result<bytes::Bytes>> + Unpin + Send + 'a, deserializer: impl BatchDeserializer<bytes::Bytes> + 'a) -> futures::stream::BoxStream<'a, datafusion_common::Result<::arrow::array::RecordBatch, arrow::error::ArrowError>>
```

Source: `src/decoder.rs:174`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Deserializes a stream of bytes into a stream of [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) objects using the
provided deserializer.

Returns a boxed stream of `Result<RecordBatch, ArrowError>`. The stream yields [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)
objects as they are produced by the deserializer, or an [`ArrowError`](../operations/arrow_schema.error.ArrowError.md#op-0b9e83026812de4ef436507a) if an error
occurs while polling the input or deserializing.
