# `object_store::limit`

Crate `object_store` · 2 public items · structured records in [`model/object_store.limit.json`](../model/object_store.limit.json)

## LimitStore

`struct` · `object_store::limit::LimitStore`

```rust
struct LimitStore<T: ObjectStore>
```

**Implements**: `core::fmt::Display`, `object_store::ObjectStore`

**Derives**: Debug

**Methods** (1)

```rust
fn new(inner: T, max_requests: usize) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
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

Store wrapper that wraps an inner store and limits the maximum number of concurrent
object store operations. Where each call to an [`ObjectStore`] member function is
considered a single operation, even if it may result in more than one network call

```
# use object_store::memory::InMemory;
# use object_store::limit::LimitStore;

// Create an in-memory `ObjectStore` limited to 20 concurrent requests
let store = LimitStore::new(InMemory::new(), 20);
```

---

## LimitUpload

`struct` · `object_store::limit::LimitUpload`

```rust
struct LimitUpload
```

**Implements**: `object_store::upload::MultipartUpload`

**Derives**: Debug

**Methods** (1)

```rust
fn new(upload: Box<dyn MultipartUpload>, max_concurrency: usize) -> Self
```

**via `object_store::upload::MultipartUpload`**

```rust
async fn abort(&mut self) -> Result<()>
async fn complete(&mut self) -> Result<PutResult>
fn put_part(&mut self, data: PutPayload) -> UploadPart
```

An [`MultipartUpload`] wrapper that limits the maximum number of concurrent requests

---
