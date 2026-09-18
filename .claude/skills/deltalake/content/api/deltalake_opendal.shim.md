# `deltalake_opendal::shim`

Crate `deltalake-opendal` · 1 public items · structured records in [`model/deltalake_opendal.shim.json`](../model/deltalake_opendal.shim.json)

## ConditionalPutShim

`struct` · `deltalake_opendal::shim::ConditionalPutShim`
[Full member contracts, output types and access classification](../operations/deltalake_opendal.shim.ConditionalPutShim.md)

Also reachable as `deltalake::opendal::ConditionalPutShim`, `deltalake_opendal::ConditionalPutShim`

```rust
struct ConditionalPutShim
```

**Implements**: `core::fmt::Display`, `object_store::ObjectStore`

**Derives**: Debug

**Methods** (1)

```rust
fn new(inner: Arc<dyn ObjectStore>) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `object_store::ObjectStore`**

```rust
async fn copy_opts(&self, from: &Path, to: &Path, options: CopyOptions) -> ObjectStoreResult<()>
fn delete_stream(&self, locations: BoxStream<'static, ObjectStoreResult<Path>>) -> BoxStream<'static, ObjectStoreResult<Path>>
async fn get_opts(&self, location: &Path, options: GetOptions) -> ObjectStoreResult<GetResult>
async fn get_ranges(&self, location: &Path, ranges: &[Range<u64>]) -> ObjectStoreResult<Vec<Bytes>>
fn list(&self, prefix: Option<&Path>) -> BoxStream<'static, ObjectStoreResult<ObjectMeta>>
async fn list_with_delimiter(&self, prefix: Option<&Path>) -> ObjectStoreResult<ListResult>
fn list_with_offset(&self, prefix: Option<&Path>, offset: &Path) -> BoxStream<'static, ObjectStoreResult<ObjectMeta>>
async fn put_multipart_opts(&self, location: &Path, options: PutMultipartOptions) -> ObjectStoreResult<Box<dyn MultipartUpload>>
async fn put_opts(&self, location: &Path, bytes: PutPayload, options: PutOptions) -> ObjectStoreResult<PutResult>
async fn rename_opts(&self, from: &Path, to: &Path, options: RenameOptions) -> ObjectStoreResult<()>
```

Wraps an inner store and emulates [`PutMode::Create`] with a HEAD-then-PUT.

OpenDAL backends that don't implement `write_with_if_not_exists` reject
`PutMode::Create` with `Unsupported`. This shim approximates the conditional
semantics: it is racy across concurrent writers, so it is only appropriate
for single-writer stores.

---
