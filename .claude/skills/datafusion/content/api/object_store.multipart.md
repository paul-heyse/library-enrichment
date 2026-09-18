# `object_store::multipart`

Crate `object_store` · 2 public items · structured records in [`model/object_store.multipart.json`](../model/object_store.multipart.json)

## PartId

`struct` · `object_store::multipart::PartId`

```rust
struct PartId
```

**Fields**: `content_id`

**Derives**: Clone, Debug

[Full member, field, variant and typed contracts](../operations/object_store.multipart.PartId.md).


Represents a part of a file that has been successfully uploaded in a multipart upload process.

---

## MultipartStore

`trait` · `object_store::multipart::MultipartStore`

```rust
trait MultipartStore: Send + Sync + 'static
```

**Implementors** (6)

- `object_store::aws::AmazonS3`
- `object_store::azure::MicrosoftAzure`
- `object_store::gcp::GoogleCloudStorage`
- `object_store::memory::InMemory`
- `object_store::prefix::PrefixStore`
- `object_store::throttle::ThrottledStore`

**Methods** (4)

```rust
async fn abort_multipart(&self, path: &Path, id: &MultipartId) -> Result<()>
async fn complete_multipart(&self, path: &Path, id: &MultipartId, parts: Vec<PartId>) -> Result<PutResult>
async fn create_multipart(&self, path: &Path) -> Result<MultipartId>
async fn put_part(&self, path: &Path, id: &MultipartId, part_idx: usize, data: PutPayload) -> Result<PartId>
```

[Full member, field, variant and typed contracts](../operations/object_store.multipart.MultipartStore.md).


A low-level interface for interacting with multipart upload APIs

Most use-cases should prefer [`ObjectStore::put_multipart_opts`] as this is supported by more
backends, including [`LocalFileSystem`], and automatically handles uploading fixed
size parts of sufficient size in parallel

[`ObjectStore::put_multipart_opts`]: crate::ObjectStore::put_multipart_opts
[`LocalFileSystem`]: crate::local::LocalFileSystem

---
