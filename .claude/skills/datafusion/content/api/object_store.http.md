# `object_store::http`

Crate `object_store` · 2 public items · structured records in [`model/object_store.http.json`](../model/object_store.http.json)

## HttpBuilder

`struct` · `object_store::http::HttpBuilder`

```rust
struct HttpBuilder
```

**Derives**: Clone, Debug, Default

**Methods** (7)

```rust
fn build(self) -> Result<HttpStore>
fn new() -> Self
fn with_client_options(self, options: ClientOptions) -> Self
fn with_config(self, key: ClientConfigKey, value: impl Into<String>) -> Self
fn with_http_connector<C: HttpConnector>(self, connector: C) -> Self
fn with_retry(self, retry_config: RetryConfig) -> Self
fn with_url(self, url: impl Into<String>) -> Self
```

[Full member, field, variant and typed contracts](../operations/object_store.http.HttpBuilder.md).


Configure a connection to a generic HTTP server

---

## HttpStore

`struct` · `object_store::http::HttpStore`

```rust
struct HttpStore
```

**Implements**: `core::fmt::Display`, `object_store::ObjectStore`

**Derives**: Debug

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
async fn put_multipart_opts(&self, _location: &Path, _opts: PutMultipartOptions) -> Result<Box<dyn MultipartUpload>>
async fn put_opts(&self, location: &Path, payload: PutPayload, opts: PutOptions) -> Result<PutResult>
```

[Full member, field, variant and typed contracts](../operations/object_store.http.HttpStore.md).


An [`ObjectStore`] implementation for generic HTTP servers

See [`crate::http`] for more information

---
