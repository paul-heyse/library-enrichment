# `deltalake_core::kernel::snapshot::stream`

Crate `deltalake-core` · 3 public items · structured records in [`model/deltalake_core.kernel.snapshot.stream.json`](../model/deltalake_core.kernel.snapshot.stream.json)

## RecordBatchStream

`trait` · `deltalake_core::kernel::snapshot::stream::RecordBatchStream`

Also reachable as `deltalake::kernel::RecordBatchStream`, `deltalake_core::kernel::RecordBatchStream`

```rust
trait RecordBatchStream: Stream<Item = errors::DeltaResult<arrow::record_batch::RecordBatch>>
```

**Methods** (1)

```rust
fn schema(&self) -> SchemaRef
```

Trait for types that stream [RecordBatch]

See [`SendableRecordBatchStream`] for more details.

---

## SendableRBStream

`type_alias` · `deltalake_core::kernel::snapshot::stream::SendableRBStream`

Also reachable as `deltalake::kernel::SendableRBStream`, `deltalake_core::kernel::SendableRBStream`

```rust
type SendableRBStream = std::pin::Pin<Box<dyn Stream<Item = errors::DeltaResult<arrow::record_batch::RecordBatch>> + Send>>
```

A boxed, `Send`able stream of [`RecordBatch`] results, without the schema guarantees of
[`SendableRecordBatchStream`].

---

## SendableRecordBatchStream

`type_alias` · `deltalake_core::kernel::snapshot::stream::SendableRecordBatchStream`

Also reachable as `deltalake::kernel::SendableRecordBatchStream`, `deltalake_core::kernel::SendableRecordBatchStream`

```rust
type SendableRecordBatchStream = std::pin::Pin<Box<dyn RecordBatchStream + Send>>
```

Trait for a [`Stream`] of [`RecordBatch`]es that can be passed between threads

This trait is used to retrieve the results of DataFusion execution plan nodes.

The trait is a specialized Rust Async [`Stream`] that also knows the schema
of the data it will return (even if the stream has no data). Every
`RecordBatch` returned by the stream should have the same schema as returned
by [`schema`](`RecordBatchStream::schema`).

# See Also

* [`RecordBatchStreamAdapter`] to convert an existing [`Stream`]
  to [`SendableRecordBatchStream`]

[`RecordBatchStreamAdapter`]: https://docs.rs/datafusion/latest/datafusion/physical_plan/stream/struct.RecordBatchStreamAdapter.html

# Error Handling

Once a stream returns an error, it should not be polled again (the caller
should stop calling `next`) and handle the error.

However, returning `Ready(None)` (end of stream) is likely the safest
behavior after an error. Like [`Stream`]s, `RecordBatchStream`s should not
be polled after end of stream or returning an error. However, also like
[`Stream`]s there is no mechanism to prevent callers polling  so returning
`Ready(None)` is recommended.

---
