# `datafusion_execution::stream::SendableRecordBatchStream`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.stream.SendableRecordBatchStream.json).

<a id="op-7cf25e4c554567392af11cb6"></a>
## SendableRecordBatchStream

`type_alias` · `datafusion_execution::stream::SendableRecordBatchStream` · datafusion-execution 55.1.0

```rust
type SendableRecordBatchStream = std::pin::Pin<Box<dyn RecordBatchStream + Send>>
```

Source: `src/stream.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Trait for a [`Stream`] of [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)es that can be passed between threads

This trait is used to retrieve the results of DataFusion execution plan nodes.

The trait is a specialized Rust Async [`Stream`] that also knows the schema
of the data it will return (even if the stream has no data). Every
`RecordBatch` returned by the stream should have the same schema as returned
by [`schema`](`RecordBatchStream::schema`).

# See Also

* [`RecordBatchStreamAdapter`] to convert an existing [`Stream`]
  to [`SendableRecordBatchStream`](../operations/datafusion_execution.stream.SendableRecordBatchStream.md#op-7cf25e4c554567392af11cb6)

[`RecordBatchStreamAdapter`]: https://docs.rs/datafusion/latest/datafusion/physical_plan/stream/struct.RecordBatchStreamAdapter.html

# Error Handling

Once a stream returns an error, it should not be polled again (the caller
should stop calling `next`) and handle the error.

However, returning `Ready(None)` (end of stream) is likely the safest
behavior after an error. Like [`Stream`]s, `RecordBatchStream`s should not
be polled after end of stream or returning an error. However, also like
[`Stream`]s there is no mechanism to prevent callers polling  so returning
`Ready(None)` is recommended.

Unresolved upstream links (retained, not inferred): ``Stream``.
