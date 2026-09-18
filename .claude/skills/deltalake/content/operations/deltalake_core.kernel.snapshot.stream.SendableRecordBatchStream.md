# `deltalake_core::kernel::snapshot::stream::SendableRecordBatchStream`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.snapshot.stream.SendableRecordBatchStream.json).

<a id="op-e40db85f0e0fd4b180947a3c"></a>
## SendableRecordBatchStream

`type_alias` · `deltalake_core::kernel::snapshot::stream::SendableRecordBatchStream` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type SendableRecordBatchStream = std::pin::Pin<Box<dyn RecordBatchStream + Send>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/stream.rs#L52).

Source: `crates/core/src/kernel/snapshot/stream.rs:52`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Trait for a [`Stream`] of [`RecordBatch`]es that can be passed between threads

This trait is used to retrieve the results of DataFusion execution plan nodes.

The trait is a specialized Rust Async [`Stream`] that also knows the schema
of the data it will return (even if the stream has no data). Every
`RecordBatch` returned by the stream should have the same schema as returned
by [`schema`](`RecordBatchStream::schema`).

# See Also

* [`RecordBatchStreamAdapter`] to convert an existing [`Stream`]
  to [`SendableRecordBatchStream`](../operations/deltalake_core.kernel.snapshot.stream.SendableRecordBatchStream.md#op-e40db85f0e0fd4b180947a3c)

[`RecordBatchStreamAdapter`]: https://docs.rs/datafusion/latest/datafusion/physical_plan/stream/struct.RecordBatchStreamAdapter.html

# Error Handling

Once a stream returns an error, it should not be polled again (the caller
should stop calling `next`) and handle the error.

However, returning `Ready(None)` (end of stream) is likely the safest
behavior after an error. Like [`Stream`]s, `RecordBatchStream`s should not
be polled after end of stream or returning an error. However, also like
[`Stream`]s there is no mechanism to prevent callers polling  so returning
`Ready(None)` is recommended.

Unresolved upstream links (retained, not inferred): ``RecordBatch``, ``Stream``.
