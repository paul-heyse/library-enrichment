# `object_store::chunked`

Crate `object_store` · 1 public items · structured records in [`model/object_store.chunked.json`](../model/object_store.chunked.json)

## ChunkedStore

`struct` · `object_store::chunked::ChunkedStore`

```rust
struct ChunkedStore
```

**Implements**: `core::fmt::Display`, `object_store::ObjectStore`

**Derives**: Debug

**Methods** (1)

```rust
fn new(inner: Arc<dyn ObjectStore>, chunk_size: usize) -> Self
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

[Full member, field, variant and typed contracts](../operations/object_store.chunked.ChunkedStore.md).


Wraps a [`ObjectStore`] and makes its get response return chunks
in a controllable manner.

A `ChunkedStore` makes the memory consumption and performance of
the wrapped [`ObjectStore`] worse. It is intended for use within
tests, to control the chunks in the produced output streams. For
example, it is used to verify the delimiting logic in
newline_delimited_stream.

---
