# `datafusion_datasource::morsel::Morsel`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.morsel.Morsel.json).

<a id="op-da1332ede7380b65f9f65f35"></a>
## Morsel

`trait` · `datafusion_datasource::morsel::Morsel` · datafusion-datasource 55.1.0

```rust
trait Morsel: Send + Debug
```

Source: `src/morsel/mod.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

A Morsel of work ready to resolve to a stream of [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)es.

This represents a single morsel of work that is ready to be processed. It
has all data necessary (does not need any I/O) and is ready to be turned
into a stream of [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)es for processing by the execution engine.

<a id="op-f70959747a9da253c6d9c8cb"></a>
## into_stream

`function` · `datafusion_datasource::morsel::Morsel::into_stream` · datafusion-datasource 55.1.0

```rust
fn into_stream(Box<self>) -> BoxStream<'static, Result<RecordBatch>>
```

Source: `src/morsel/mod.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Consume this morsel and produce a stream of [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)es for processing.

Note: This may do CPU work to decode already-loaded data, but should not
do any I/O work such as reading from the file.
