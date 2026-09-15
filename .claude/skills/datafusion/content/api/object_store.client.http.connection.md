# `object_store::client::http::connection`

Crate `object_store` · 7 public items · structured records in [`model/object_store.client.http.connection.json`](../model/object_store.client.http.connection.json)

## HttpErrorKind

`enum` · `object_store::client::http::connection::HttpErrorKind`

```rust
enum HttpErrorKind
```

**Variants**: `Connect`, `Request`, `Timeout`, `Interrupted`, `Decode`, `Unknown`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

Identifies the kind of [`HttpError`]

This is used, among other things, to determine if a request can be retried

---

## HttpClient

`struct` · `object_store::client::http::connection::HttpClient`

```rust
struct HttpClient
```

**Derives**: Clone, Debug

**Methods** (2)

```rust
async fn execute(&self, request: HttpRequest) -> Result<HttpResponse, HttpError>
fn new(service: impl HttpService + 'static) -> Self
```

An HTTP client

---

## HttpError

`struct` · `object_store::client::http::connection::HttpError`

```rust
struct HttpError
```

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**Methods** (2)

```rust
fn kind(&self) -> HttpErrorKind
fn new<E>(kind: HttpErrorKind, e: E) -> Self where E: Error + Send + Sync + 'static
```

**via `core::error::Error`**

```rust
fn source(&self) -> ::core::option::Option<&dyn ::thiserror::__private18::Error + 'static>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

An HTTP protocol error

Clients should return this when an HTTP request fails to be completed, e.g. because
of a connection issue. This does **not** include HTTP requests that are return
non 2xx Status Codes, as these should instead be returned as an [`HttpResponse`]
with the appropriate status code set.

---

## ReqwestConnector

`struct` · `object_store::client::http::connection::ReqwestConnector`

```rust
struct ReqwestConnector
```

**Implements**: `object_store::client::http::connection::HttpConnector`

**Derives**: Debug, Default

**via `object_store::client::http::connection::HttpConnector`**

```rust
fn connect(&self, options: &ClientOptions) -> Result<HttpClient>
```

[`HttpConnector`] using [`reqwest::Client`]

---

## SpawnedReqwestConnector

`struct` · `object_store::client::http::connection::SpawnedReqwestConnector`

```rust
struct SpawnedReqwestConnector
```

**Implements**: `object_store::client::http::connection::HttpConnector`

**Derives**: Debug

**Methods** (1)

```rust
fn new(runtime: Handle) -> Self
```

**via `object_store::client::http::connection::HttpConnector`**

```rust
fn connect(&self, options: &ClientOptions) -> Result<HttpClient>
```

[`reqwest::Client`] connector that performs all I/O on the provided tokio
[`Runtime`] (thread pool).

This adapter is most useful when you wish to segregate I/O from CPU bound
work that may be happening on the [`Runtime`].

[`Runtime`]: tokio::runtime::Runtime

# Example: Spawning requests on separate runtime

```
# use std::sync::Arc;
# use tokio::runtime::Runtime;
# use object_store::azure::MicrosoftAzureBuilder;
# use object_store::client::SpawnedReqwestConnector;
# use object_store::ObjectStore;
# fn get_io_runtime() -> Runtime {
#   tokio::runtime::Builder::new_current_thread().build().unwrap()
# }
# fn main() -> Result<(), object_store::Error> {
// create a tokio runtime for I/O.
let io_runtime: Runtime = get_io_runtime();
// configure a store using the runtime.
let handle = io_runtime.handle().clone(); // get a handle to the same runtime
let store: Arc<dyn ObjectStore> = Arc::new(
  MicrosoftAzureBuilder::new()
    .with_http_connector(SpawnedReqwestConnector::new(handle))
    .with_container_name("my_container")
    .with_account("my_account")
    .build()?
 );
// any requests made using store will be spawned on the io_runtime
# Ok(())
# }
```

---

## HttpConnector

`trait` · `object_store::client::http::connection::HttpConnector`

```rust
trait HttpConnector: std::fmt::Debug + Send + Sync + 'static
```

**Implementors** (2)

- `object_store::client::http::connection::ReqwestConnector`
- `object_store::client::http::connection::SpawnedReqwestConnector`

**Methods** (1)

```rust
fn connect(&self, options: &ClientOptions) -> Result<HttpClient>
```

A factory for [`HttpClient`]

---

## HttpService

`trait` · `object_store::client::http::connection::HttpService`

```rust
trait HttpService: std::fmt::Debug + Send + Sync + 'static
```

**Implementors** (2)

- `object_store::client::http::spawn::SpawnService`
- `reqwest::async_impl::client::Client`

**Methods** (1)

```rust
async fn call(&self, req: HttpRequest) -> Result<HttpResponse, HttpError>
```

An asynchronous function from a [`HttpRequest`] to a [`HttpResponse`].

---
