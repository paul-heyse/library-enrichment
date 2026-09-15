# `object_store::azure`

Crate `object_store` · 2 public items · structured records in [`model/object_store.azure.json`](../model/object_store.azure.json)

## MicrosoftAzure

`struct` · `object_store::azure::MicrosoftAzure`

```rust
struct MicrosoftAzure
```

**Implements**: `core::fmt::Display`, `object_store::ObjectStore`, `object_store::list::PaginatedListStore`, `object_store::multipart::MultipartStore`, `object_store::signer::Signer`

**Derives**: Debug

**Methods** (1)

```rust
fn credentials(&self) -> &AzureCredentialProvider
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
fn list(&self, prefix: Option<&Path>) -> BoxStream<'static, Result<ObjectMeta>>
async fn list_with_delimiter(&self, prefix: Option<&Path>) -> Result<ListResult>
fn list_with_offset(&self, prefix: Option<&Path>, offset: &Path) -> BoxStream<'static, Result<ObjectMeta>>
async fn put_multipart_opts(&self, location: &Path, opts: PutMultipartOptions) -> Result<Box<dyn MultipartUpload>>
async fn put_opts(&self, location: &Path, payload: PutPayload, opts: PutOptions) -> Result<PutResult>
```

**via `object_store::list::PaginatedListStore`**

```rust
async fn list_paginated(&self, prefix: Option<&str>, opts: PaginatedListOptions) -> Result<PaginatedListResult>
```

**via `object_store::multipart::MultipartStore`**

```rust
async fn abort_multipart(&self, _: &Path, _: &MultipartId) -> Result<()>
async fn complete_multipart(&self, path: &Path, _: &MultipartId, parts: Vec<PartId>) -> Result<PutResult>
async fn create_multipart(&self, _: &Path) -> Result<MultipartId>
async fn put_part(&self, path: &Path, _: &MultipartId, part_idx: usize, data: PutPayload) -> Result<PartId>
```

**via `object_store::signer::Signer`**

```rust
async fn signed_url(&self, method: Method, path: &Path, expires_in: Duration) -> Result<Url>
async fn signed_urls(&self, method: Method, paths: &[Path], expires_in: Duration) -> Result<Vec<Url>>
```

Interface for [Microsoft Azure Blob Storage](https://azure.microsoft.com/en-us/services/storage/blobs/).

---

## AzureCredentialProvider

`type_alias` · `object_store::azure::AzureCredentialProvider`

```rust
type AzureCredentialProvider = std::sync::Arc<dyn CredentialProvider<Credential = AzureCredential>>
```

[`CredentialProvider`] for [`MicrosoftAzure`]

---
