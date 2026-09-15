# `object_store::memory`

Crate `object_store` · 1 public items · structured records in [`model/object_store.memory.json`](../model/object_store.memory.json)

## InMemory

`struct` · `object_store::memory::InMemory`

```rust
struct InMemory
```

**Implements**: `core::fmt::Display`, `object_store::ObjectStore`, `object_store::multipart::MultipartStore`

**Derives**: Clone, Debug, Default

**Methods** (2)

```rust
fn fork(&self) -> Self
fn new() -> Self
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
async fn put_multipart_opts(&self, location: &Path, opts: PutMultipartOptions) -> Result<Box<dyn MultipartUpload>>
async fn put_opts(&self, location: &Path, payload: PutPayload, opts: PutOptions) -> Result<PutResult>
```

**via `object_store::multipart::MultipartStore`**

```rust
async fn abort_multipart(&self, _path: &Path, id: &MultipartId) -> Result<()>
async fn complete_multipart(&self, path: &Path, id: &MultipartId, _parts: Vec<PartId>) -> Result<PutResult>
async fn create_multipart(&self, _path: &Path) -> Result<MultipartId>
async fn put_part(&self, _path: &Path, id: &MultipartId, part_idx: usize, payload: PutPayload) -> Result<PartId>
```

In-memory storage suitable for testing or for opting out of using a cloud
storage provider.

---
