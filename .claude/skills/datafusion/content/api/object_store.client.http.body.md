# `object_store::client::http::body`

Crate `object_store` · 4 public items · structured records in [`model/object_store.client.http.body.json`](../model/object_store.client.http.body.json)

## HttpRequestBody

`struct` · `object_store::client::http::body::HttpRequestBody`

```rust
struct HttpRequestBody
```

**Implements**: `core::convert::From`, `http_body::Body`

**Derives**: Clone, Debug

**Methods** (4)

```rust
fn as_bytes(&self) -> Option<&Bytes>
fn content_length(&self) -> usize
fn empty() -> Self
fn is_empty(&self) -> bool
```

**via `core::convert::From`**

```rust
fn from(value: PutPayload) -> Self
fn from(value: String) -> Self
fn from(value: Vec<u8>) -> Self
fn from(value: Bytes) -> Self
```

**via `http_body::Body`**

```rust
fn is_end_stream(&self) -> bool
fn poll_frame(Pin<&mut self>, _cx: &mut Context<'_>) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>>
fn size_hint(&self) -> SizeHint
```

The [`Body`] of an [`HttpRequest`]

---

## HttpResponseBody

`struct` · `object_store::client::http::body::HttpResponseBody`

```rust
struct HttpResponseBody
```

**Implements**: `core::convert::From`, `http_body::Body`

**Derives**: Debug

**Methods** (3)

```rust
async fn bytes(self) -> Result<Bytes, HttpError>
fn bytes_stream(self) -> BoxStream<'static, Result<Bytes, HttpError>>
fn new<B>(body: B) -> Self where B: Body<Data = Bytes, Error = HttpError> + Send + Sync + 'static
```

**via `core::convert::From`**

```rust
fn from(value: Vec<u8>) -> Self
fn from(value: Bytes) -> Self
fn from(value: String) -> Self
```

**via `http_body::Body`**

```rust
fn is_end_stream(&self) -> bool
fn poll_frame(Pin<&mut self>, cx: &mut Context<'_>) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>>
fn size_hint(&self) -> SizeHint
```

The body of an [`HttpResponse`]

---

## HttpRequest

`type_alias` · `object_store::client::http::body::HttpRequest`

```rust
type HttpRequest = http::Request<HttpRequestBody>
```

An HTTP Request

---

## HttpResponse

`type_alias` · `object_store::client::http::body::HttpResponse`

```rust
type HttpResponse = http::Response<HttpResponseBody>
```

An HTTP response

---
