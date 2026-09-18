# `object_store::client::http::body::HttpResponseBody`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.client.http.body.HttpResponseBody.json).

<a id="op-58631573707950372ee2da1d"></a>
## HttpResponseBody

`struct` · `object_store::client::http::body::HttpResponseBody` · object_store 0.13.2

```rust
struct HttpResponseBody
```

Source: `src/client/http/body.rs:168`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

The body of an [`HttpResponse`](../operations/object_store.client.http.body.HttpResponse.md#op-6c88f511e46991a2e002ff3b)

<a id="op-3129000bc7d56681eb721a69"></a>
## Data

`assoc_type` · `object_store::client::http::body::HttpResponseBody::Data` · object_store 0.13.2

```rust
Data
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::http::body::HttpResponseBody", "path": "HttpResponseBody"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [206, 1], "end": [224, 2], "filename": "src/client/http/body.rs"}, "trait": {"args": null, "id": "http_body::Body", "path": "Body"}, "trait_path": "http_body::Body"}`

Source: `src/client/http/body.rs:207`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cc4e46e76063c869a6047c95"></a>
## Error

`assoc_type` · `object_store::client::http::body::HttpResponseBody::Error` · object_store 0.13.2

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::http::body::HttpResponseBody", "path": "HttpResponseBody"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [206, 1], "end": [224, 2], "filename": "src/client/http/body.rs"}, "trait": {"args": null, "id": "http_body::Body", "path": "Body"}, "trait_path": "http_body::Body"}`

Source: `src/client/http/body.rs:208`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-acedc5756c3a4f6e6b0423cf"></a>
## bytes

`function` · `object_store::client::http::body::HttpResponseBody::bytes` · object_store 0.13.2

```rust
async fn bytes(self) -> Result<Bytes, HttpError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::http::body::HttpResponseBody", "path": "HttpResponseBody"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [170, 1], "end": [204, 2], "filename": "src/client/http/body.rs"}, "trait": null, "trait_path": null}`

Source: `src/client/http/body.rs:182`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Collects this response into a [`Bytes`]

Unresolved upstream links (retained, not inferred): ``Bytes``.

<a id="op-db8bbc7586f35d4353007726"></a>
## bytes_stream

`function` · `object_store::client::http::body::HttpResponseBody::bytes_stream` · object_store 0.13.2

```rust
fn bytes_stream(self) -> BoxStream<'static, Result<Bytes, HttpError>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::http::body::HttpResponseBody", "path": "HttpResponseBody"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [170, 1], "end": [204, 2], "filename": "src/client/http/body.rs"}, "trait": null, "trait_path": null}`

Source: `src/client/http/body.rs:189`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Returns a stream of this response data

<a id="op-91a302dd87584f976d1efe39"></a>
## fmt

`function` · `object_store::client::http::body::HttpResponseBody::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::http::body::HttpResponseBody", "path": "HttpResponseBody"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [167, 10], "end": [167, 15], "filename": "src/client/http/body.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/client/http/body.rs:167`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2417eae084b51b98a42d01c6"></a>
## from

`function` · `object_store::client::http::body::HttpResponseBody::from` · object_store 0.13.2

```rust
fn from(value: String) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::http::body::HttpResponseBody", "path": "HttpResponseBody"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [238, 1], "end": [242, 2], "filename": "src/client/http/body.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/client/http/body.rs:239`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-36f0f9e27f71fe68874bff52"></a>
## from

`function` · `object_store::client::http::body::HttpResponseBody::from` · object_store 0.13.2

```rust
fn from(value: Vec<u8>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::http::body::HttpResponseBody", "path": "HttpResponseBody"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [232, 1], "end": [236, 2], "filename": "src/client/http/body.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "u8"}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/client/http/body.rs:233`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-df0bb18cc9f33891650794e0"></a>
## from

`function` · `object_store::client::http::body::HttpResponseBody::from` · object_store 0.13.2

```rust
fn from(value: Bytes) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::http::body::HttpResponseBody", "path": "HttpResponseBody"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [226, 1], "end": [230, 2], "filename": "src/client/http/body.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "Bytes"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/client/http/body.rs:227`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b8a53447568ca1adb2ae7594"></a>
## is_end_stream

`function` · `object_store::client::http::body::HttpResponseBody::is_end_stream` · object_store 0.13.2

```rust
fn is_end_stream(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::http::body::HttpResponseBody", "path": "HttpResponseBody"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [206, 1], "end": [224, 2], "filename": "src/client/http/body.rs"}, "trait": {"args": null, "id": "http_body::Body", "path": "Body"}, "trait_path": "http_body::Body"}`

Source: `src/client/http/body.rs:217`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4884b45516e925c089fa034c"></a>
## new

`function` · `object_store::client::http::body::HttpResponseBody::new` · object_store 0.13.2

```rust
fn new<B>(body: B) -> Self where B: Body<Data = Bytes, Error = HttpError> + Send + Sync + 'static
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::http::body::HttpResponseBody", "path": "HttpResponseBody"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [170, 1], "end": [204, 2], "filename": "src/client/http/body.rs"}, "trait": null, "trait_path": null}`

Source: `src/client/http/body.rs:174`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Create an [`HttpResponseBody`](../operations/object_store.client.http.body.HttpResponseBody.md#op-58631573707950372ee2da1d) from the provided [`Body`]

Note: [`BodyExt::map_err`] can be used to alter error variants

Unresolved upstream links (retained, not inferred): ``Body``, ``BodyExt::map_err``.

<a id="op-cea0523abe18e94c226a47e1"></a>
## poll_frame

`function` · `object_store::client::http::body::HttpResponseBody::poll_frame` · object_store 0.13.2

```rust
fn poll_frame(Pin<&mut self>, cx: &mut Context<'_>) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::http::body::HttpResponseBody", "path": "HttpResponseBody"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [206, 1], "end": [224, 2], "filename": "src/client/http/body.rs"}, "trait": {"args": null, "id": "http_body::Body", "path": "Body"}, "trait_path": "http_body::Body"}`

Source: `src/client/http/body.rs:210`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b23e0cab8fc0e222c5f307a7"></a>
## size_hint

`function` · `object_store::client::http::body::HttpResponseBody::size_hint` · object_store 0.13.2

```rust
fn size_hint(&self) -> SizeHint
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::http::body::HttpResponseBody", "path": "HttpResponseBody"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [206, 1], "end": [224, 2], "filename": "src/client/http/body.rs"}, "trait": {"args": null, "id": "http_body::Body", "path": "Body"}, "trait_path": "http_body::Body"}`

Source: `src/client/http/body.rs:221`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.
