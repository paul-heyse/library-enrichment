# `buoyant_kernel_engine::file_stream`

Crate `buoyant_kernel_engine` · 4 public items · structured records in [`model/buoyant_kernel_engine.file_stream.json`](../model/buoyant_kernel_engine.file_stream.json)

## OnError

`enum` · `buoyant_kernel_engine::file_stream::OnError`

Also reachable as `delta_kernel_default_engine::file_stream::OnError`

```rust
enum OnError
```

**Variants**: `Fail`, `Skip`

**Derives**: Default

Describes the behavior of the `FileStream` if file opening or scanning fails

---

## FileStream

`struct` · `buoyant_kernel_engine::file_stream::FileStream`

Also reachable as `delta_kernel_default_engine::file_stream::FileStream`

```rust
struct FileStream
```

**Implements**: `futures_core::stream::Stream`

**Methods** (2)

```rust
fn new(files: impl IntoIterator<Item = FileMeta>, schema: ArrowSchemaRef, file_opener: Box<dyn FileOpener>) -> DeltaResult<Self>
fn with_on_error(self, on_error: OnError) -> Self
```

**via `futures_core::stream::Stream`**

```rust
fn poll_next(Pin<&mut self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>
```

A stream that iterates record batch by record batch, file over file.

---

## FileOpener

`trait` · `buoyant_kernel_engine::file_stream::FileOpener`

Also reachable as `delta_kernel_default_engine::file_stream::FileOpener`

```rust
trait FileOpener: Send + Unpin
```

**Implementors** (1)

- `buoyant_kernel_engine::parquet::PresignedUrlOpener`

**Methods** (1)

```rust
fn open(&self, file_meta: FileMeta, range: Option<Range<i64>>) -> DeltaResult<FileOpenFuture>
```

Generic API for opening a file using an [`ObjectStore`] and resolving to a
stream of [`RecordBatch`]

[`ObjectStore`]: delta_kernel::object_store::ObjectStore

---

## FileOpenFuture

`type_alias` · `buoyant_kernel_engine::file_stream::FileOpenFuture`

Also reachable as `delta_kernel_default_engine::file_stream::FileOpenFuture`

```rust
type FileOpenFuture = futures::future::BoxFuture<'static, delta_kernel::DeltaResult<futures::stream::BoxStream<'static, delta_kernel::DeltaResult<delta_kernel::arrow::array::RecordBatch>>>>
```

A fallible future that resolves to a stream of [`RecordBatch`]
cbindgen:ignore

---
