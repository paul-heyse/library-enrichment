# `object_store::throttle`

Crate `object_store` · 2 public items · structured records in [`model/object_store.throttle.json`](../model/object_store.throttle.json)

## ThrottleConfig

`struct` · `object_store::throttle::ThrottleConfig`

```rust
struct ThrottleConfig
```

**Fields**: `wait_delete_per_call`, `wait_get_per_byte`, `wait_get_per_call`, `wait_list_per_call`, `wait_list_per_entry`, `wait_list_with_delimiter_per_call`, `wait_list_with_delimiter_per_entry`, `wait_put_per_call`

**Derives**: Clone, Copy, Debug, Default

[Full member, field, variant and typed contracts](../operations/object_store.throttle.ThrottleConfig.md).


Configuration settings for throttled store

---

## ThrottledStore

`struct` · `object_store::throttle::ThrottledStore`

```rust
struct ThrottledStore<T>
```

**Implements**: `core::fmt::Display`, `object_store::ObjectStore`, `object_store::multipart::MultipartStore`

**Derives**: Debug

**Methods** (3)

```rust
fn config(&self) -> ThrottleConfig
fn config_mut<F>(&self, f: F) where F: Fn(&mut ThrottleConfig)
fn new(inner: T, config: ThrottleConfig) -> Self
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

**via `object_store::multipart::MultipartStore`**

```rust
async fn abort_multipart(&self, path: &Path, id: &MultipartId) -> Result<()>
async fn complete_multipart(&self, path: &Path, id: &MultipartId, parts: Vec<PartId>) -> Result<PutResult>
async fn create_multipart(&self, path: &Path) -> Result<MultipartId>
async fn put_part(&self, path: &Path, id: &MultipartId, part_idx: usize, data: PutPayload) -> Result<PartId>
```

[Full member, field, variant and typed contracts](../operations/object_store.throttle.ThrottledStore.md).


Store wrapper that wraps an inner store with some `sleep` calls.

This can be used for performance testing.

**Note that the behavior of the wrapper is deterministic and might not reflect real-world
conditions!**

---
