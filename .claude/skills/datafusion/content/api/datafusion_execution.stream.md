# `datafusion_execution::stream`

Crate `datafusion-execution` · 2 public items · structured records in [`model/datafusion_execution.stream.json`](../model/datafusion_execution.stream.json)

## RecordBatchStream

`trait` · `datafusion_execution::stream::RecordBatchStream`

Also reachable as `datafusion::execution::RecordBatchStream`, `datafusion::physical_plan::RecordBatchStream`, `datafusion_execution::RecordBatchStream`, `datafusion_physical_plan::RecordBatchStream`, `datafusion_physical_plan::execution_plan::RecordBatchStream`

```rust
trait RecordBatchStream: Stream<Item = datafusion_common::Result<arrow::record_batch::RecordBatch>>
```

**Implementors** (12)

- `datafusion_datasource::file_stream::FileStream`
- `datafusion_ffi::record_batch_stream::FFI_RecordBatchStream`
- `datafusion_physical_plan::coop::CooperativeStream`
- `datafusion_physical_plan::limit::LimitStream`
- `datafusion_physical_plan::memory::LazyMemoryStream`
- `datafusion_physical_plan::memory::MemoryStream`
- `datafusion_physical_plan::spill::spill_pool::SpillPoolReader`
- `datafusion_physical_plan::stream::BatchSplitStream`
- `datafusion_physical_plan::stream::EmptyRecordBatchStream`
- `datafusion_physical_plan::stream::RecordBatchStreamAdapter`
- `datafusion_physical_plan::test::exec::BlockingStream`
- `datafusion_physical_plan::test::exec::TestStream`

**Methods** (1)

```rust
fn schema(&self) -> SchemaRef
```

[Full member, field, variant and typed contracts](../operations/datafusion_execution.stream.RecordBatchStream.md).


Trait for types that stream [RecordBatch]

See [`SendableRecordBatchStream`] for more details.

---

## SendableRecordBatchStream

`type_alias` · `datafusion_execution::stream::SendableRecordBatchStream`

Also reachable as `datafusion::execution::SendableRecordBatchStream`, `datafusion::physical_plan::SendableRecordBatchStream`, `datafusion_execution::SendableRecordBatchStream`, `datafusion_physical_plan::SendableRecordBatchStream`, `datafusion_physical_plan::execution_plan::SendableRecordBatchStream`

```rust
type SendableRecordBatchStream = std::pin::Pin<Box<dyn RecordBatchStream + Send>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_execution.stream.SendableRecordBatchStream.md).


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
