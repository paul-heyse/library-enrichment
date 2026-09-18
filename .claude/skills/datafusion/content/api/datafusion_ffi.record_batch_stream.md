# `datafusion_ffi::record_batch_stream`

Crate `datafusion-ffi` · 2 public items · structured records in [`model/datafusion_ffi.record_batch_stream.json`](../model/datafusion_ffi.record_batch_stream.json)

## FFI_RecordBatchStream

`struct` · `datafusion_ffi::record_batch_stream::FFI_RecordBatchStream`

```rust
struct FFI_RecordBatchStream
```

**Fields**: `poll_next`, `schema`, `release`, `private_data`

**Implements**: `core::convert::From`, `core::ops::drop::Drop`, `datafusion_execution::stream::RecordBatchStream`, `futures_core::stream::Stream`

**Derives**: Debug, Send

**Methods** (1)

```rust
fn new(stream: SendableRecordBatchStream, runtime: Option<Handle>) -> Self
```

**via `core::convert::From`**

```rust
fn from(stream: SendableRecordBatchStream) -> Self
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

**via `datafusion_execution::stream::RecordBatchStream`**

```rust
fn schema(&self) -> arrow::datatypes::SchemaRef
```

**via `futures_core::stream::Stream`**

```rust
fn poll_next(std::pin::Pin<&mut self>, cx: &mut std::task::Context<'_>) -> Poll<Option<Self::Item>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.record_batch_stream.FFI_RecordBatchStream.md).


A stable struct for sharing [`RecordBatchStream`] across FFI boundaries.
We use the async-ffi crate for handling async calls across libraries.

---

## RecordBatchStreamPrivateData

`struct` · `datafusion_ffi::record_batch_stream::RecordBatchStreamPrivateData`

```rust
struct RecordBatchStreamPrivateData
```

**Fields**: `rbs`, `runtime`

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.record_batch_stream.RecordBatchStreamPrivateData.md).


---
