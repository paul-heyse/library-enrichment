# `arrow_array::ffi_stream`

Crate `arrow-array` · 2 public items · structured records in [`model/arrow_array.ffi_stream.json`](../model/arrow_array.ffi_stream.json)

## ArrowArrayStreamReader

`struct` · `arrow_array::ffi_stream::ArrowArrayStreamReader`

Also reachable as `arrow::ffi_stream::ArrowArrayStreamReader`

```rust
struct ArrowArrayStreamReader
```

**Implements**: `arrow_array::record_batch::RecordBatchReader`, `arrow_pyarrow::FromPyArrow`, `arrow_pyarrow::IntoPyArrow`, `core::iter::traits::iterator::Iterator`

**Derives**: Debug

**Methods** (2)

```rust
unsafe fn from_raw(raw_stream: *mut FFI_ArrowArrayStream) -> std::result::Result<Self, arrow_schema::ArrowError>
fn try_new(stream: FFI_ArrowArrayStream) -> std::result::Result<Self, arrow_schema::ArrowError>
```

**via `arrow_array::record_batch::RecordBatchReader`**

```rust
fn schema(&self) -> SchemaRef
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.ffi_stream.ArrowArrayStreamReader.md).


A `RecordBatchReader` which imports Arrays from `FFI_ArrowArrayStream`.

Struct used to fetch `RecordBatch` from the C Stream Interface.
Its main responsibility is to expose `RecordBatchReader` functionality
that requires [FFI_ArrowArrayStream].

---

## FFI_ArrowArrayStream

`struct` · `arrow_array::ffi_stream::FFI_ArrowArrayStream`

Also reachable as `arrow::ffi_stream::FFI_ArrowArrayStream`

```rust
struct FFI_ArrowArrayStream
```

**Fields**: `get_schema`, `get_next`, `get_last_error`, `release`, `private_data`

**Implements**: `core::ops::drop::Drop`

**Derives**: Debug, Send

**Methods** (3)

```rust
fn empty() -> Self
unsafe fn from_raw(raw_stream: *mut FFI_ArrowArrayStream) -> Self
fn new(batch_reader: Box<dyn RecordBatchReader + Send>) -> Self
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

[Full member, field, variant and typed contracts](../operations/arrow_array.ffi_stream.FFI_ArrowArrayStream.md).


ABI-compatible struct for `ArrayStream` from C Stream Interface
See <https://arrow.apache.org/docs/format/CStreamInterface.html#structure-definitions>
This was created by bindgen

---
