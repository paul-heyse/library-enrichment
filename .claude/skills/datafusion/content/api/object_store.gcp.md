# `object_store::gcp`

Crate `object_store` · 3 public items · structured records in [`model/object_store.gcp.json`](../model/object_store.gcp.json)

## GoogleCloudStorage

`struct` · `object_store::gcp::GoogleCloudStorage`

```rust
struct GoogleCloudStorage
```

**Implements**: `core::fmt::Display`, `object_store::ObjectStore`, `object_store::list::PaginatedListStore`, `object_store::multipart::MultipartStore`, `object_store::signer::Signer`

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn credentials(&self) -> &GcpCredentialProvider
fn signing_credentials(&self) -> &GcpSigningCredentialProvider
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
async fn abort_multipart(&self, path: &Path, id: &MultipartId) -> Result<()>
async fn complete_multipart(&self, path: &Path, id: &MultipartId, parts: Vec<PartId>) -> Result<PutResult>
async fn create_multipart(&self, path: &Path) -> Result<MultipartId>
async fn put_part(&self, path: &Path, id: &MultipartId, part_idx: usize, payload: PutPayload) -> Result<PartId>
```

**via `object_store::signer::Signer`**

```rust
async fn signed_url(&self, method: Method, path: &Path, expires_in: Duration) -> Result<Url>
```

Interface for [Google Cloud Storage](https://cloud.google.com/storage/).

---

## GcpCredentialProvider

`type_alias` · `object_store::gcp::GcpCredentialProvider`

```rust
type GcpCredentialProvider = std::sync::Arc<dyn CredentialProvider<Credential = GcpCredential>>
```

[`CredentialProvider`] for [`GoogleCloudStorage`]

---

## GcpSigningCredentialProvider

`type_alias` · `object_store::gcp::GcpSigningCredentialProvider`

```rust
type GcpSigningCredentialProvider = std::sync::Arc<dyn CredentialProvider<Credential = GcpSigningCredential>>
```

[`GcpSigningCredential`] for [`GoogleCloudStorage`]

---
