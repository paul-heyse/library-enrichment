# `datafusion_execution::stream::RecordBatchStream`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.stream.RecordBatchStream.json).

<a id="op-6a21cda33374b9fc18902881"></a>
## RecordBatchStream

`trait` · `datafusion_execution::stream::RecordBatchStream` · datafusion-execution 55.1.0

```rust
trait RecordBatchStream: Stream<Item = datafusion_common::Result<arrow::record_batch::RecordBatch>>
```

Source: `src/stream.rs:26`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Trait for types that stream [RecordBatch](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)

See [`SendableRecordBatchStream`](../operations/datafusion_execution.stream.SendableRecordBatchStream.md#op-7cf25e4c554567392af11cb6) for more details.

<a id="op-316aefa7c7f29920bee6927e"></a>
## schema

`function` · `datafusion_execution::stream::RecordBatchStream::schema` · datafusion-execution 55.1.0

```rust
fn schema(&self) -> SchemaRef
```

Source: `src/stream.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Returns the schema of this `RecordBatchStream`.

Implementation of this trait should guarantee that all `RecordBatch`'s returned by this
stream should have the same schema as returned from this method.
