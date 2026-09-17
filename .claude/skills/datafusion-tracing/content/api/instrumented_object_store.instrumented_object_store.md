# `instrumented_object_store::instrumented_object_store`

Crate `instrumented-object-store` · 5 public items · structured records in [`model/instrumented_object_store.instrumented_object_store.json`](../model/instrumented_object_store.instrumented_object_store.json)

## instrument_object_store

`function` · `instrumented_object_store::instrumented_object_store::instrument_object_store`

Also reachable as `instrumented_object_store::instrument_object_store`

```rust
fn instrument_object_store(store: std::sync::Arc<dyn ObjectStore>, name: &str) -> std::sync::Arc<dyn ObjectStore>
```

Instruments the provided `ObjectStore` with tracing.

---

## instrument_result

`function` · `instrumented_object_store::instrumented_object_store::instrument_result`

```rust
fn instrument_result<T, E>(result: object_store::Result<T, E>) -> object_store::Result<T, E> where T: Instrumentable, E: std::error::Error
```

---

## InstrumentedMultiPartUpload

`struct` · `instrumented_object_store::instrumented_object_store::InstrumentedMultiPartUpload`

```rust
struct InstrumentedMultiPartUpload
```

**Implements**: `object_store::upload::MultipartUpload`

**Derives**: Debug

**Methods** (1)

```rust
fn new(upload: Box<dyn MultipartUpload>, name: &str) -> Self
```

**via `object_store::upload::MultipartUpload`**

```rust
async fn abort(&mut self) -> Result<()>
async fn complete(&mut self) -> Result<PutResult>
fn put_part(&mut self, data: PutPayload) -> UploadPart
```

A wrapper around an `ObjectStore` that instruments all public methods with tracing.

---

## InstrumentedObjectStore

`struct` · `instrumented_object_store::instrumented_object_store::InstrumentedObjectStore`

```rust
struct InstrumentedObjectStore
```

**Implements**: `core::fmt::Display`, `object_store::ObjectStore`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn new(store: Arc<dyn ObjectStore>, name: &str) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `object_store::ObjectStore`**

```rust
async fn copy_opts(&self, from: &Path, to: &Path, options: CopyOptions) -> Result<()>
fn delete_stream(&self, locations: BoxStream<'static, Result<Path>>) -> BoxStream<'static, Result<Path>>
async fn get_opts(&self, location: &Path, options: GetOptions) -> Result<GetResult>
async fn get_ranges(&self, location: &Path, ranges: &[Range<u64>]) -> Result<Vec<Bytes>>
fn list(&self, prefix: Option<&Path>) -> BoxStream<'static, Result<ObjectMeta>>
async fn list_with_delimiter(&self, prefix: Option<&Path>) -> Result<ListResult>
fn list_with_offset(&self, prefix: Option<&Path>, offset: &Path) -> BoxStream<'static, Result<ObjectMeta>>
async fn put_multipart_opts(&self, location: &Path, opts: PutMultipartOptions) -> Result<Box<dyn MultipartUpload>>
async fn put_opts(&self, location: &Path, payload: PutPayload, opts: PutOptions) -> Result<PutResult>
async fn rename_opts(&self, from: &Path, to: &Path, options: RenameOptions) -> Result<()>
```

A wrapper around an `ObjectStore` that instruments all public methods with tracing.

---

## Instrumentable

`trait` · `instrumented_object_store::instrumented_object_store::Instrumentable`

```rust
trait Instrumentable
```

**Implementors** (6)

- `alloc::vec::Vec`
- `bytes::bytes::Bytes`
- `object_store::GetResult`
- `object_store::ListResult`
- `object_store::ObjectMeta`
- `object_store::PutResult`

**Methods** (1)

```rust
fn record_fields(&self, _: &Span)
```

---
