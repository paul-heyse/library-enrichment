# `object_store::client::http::spawn`

Crate `object_store` · 1 public items · structured records in [`model/object_store.client.http.spawn.json`](../model/object_store.client.http.spawn.json)

## SpawnService

`struct` · `object_store::client::http::spawn::SpawnService`

```rust
struct SpawnService<T: HttpService + Clone>
```

**Implements**: `object_store::client::http::connection::HttpService`

**Derives**: Debug

**Methods** (1)

```rust
fn new(inner: T, runtime: Handle) -> Self
```

**via `object_store::client::http::connection::HttpService`**

```rust
async fn call(&self, req: HttpRequest) -> Result<HttpResponse, HttpError>
```

[Full member, field, variant and typed contracts](../operations/object_store.client.http.spawn.SpawnService.md).


Wraps a provided [`HttpService`] and runs it on a separate tokio runtime

See example on [`SpawnedReqwestConnector`]

[`SpawnedReqwestConnector`]: crate::client::http::SpawnedReqwestConnector

---
