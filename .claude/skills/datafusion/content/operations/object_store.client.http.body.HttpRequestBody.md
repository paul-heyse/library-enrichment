# `object_store::client::http::body::HttpRequestBody`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.client.http.body.HttpRequestBody.json).

<a id="op-7e43e4f8d43fe8dffa2ec04e"></a>
## HttpRequestBody

`struct` · `object_store::client::http::body::HttpRequestBody` · object_store 0.13.2

```rust
struct HttpRequestBody
```

Source: `src/client/http/body.rs:34`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

The [`Body`] of an [`HttpRequest`](../operations/object_store.client.http.body.HttpRequest.md#op-0f9bc5af637ad2947840b60b)

Unresolved upstream links (retained, not inferred): ``Body``.

<a id="op-6964e71a79791513bf3fd778"></a>
## Data

`assoc_type` · `object_store::client::http::body::HttpRequestBody::Data` · object_store 0.13.2

```rust
Data
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::http::body::HttpRequestBody", "path": "HttpRequestBody"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [115, 1], "end": [161, 2], "filename": "src/client/http/body.rs"}, "trait": {"args": null, "id": "http_body::Body", "path": "Body"}, "trait_path": "http_body::Body"}`

Source: `src/client/http/body.rs:116`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aaea5aeea7cfc25e33445825"></a>
## Error

`assoc_type` · `object_store::client::http::body::HttpRequestBody::Error` · object_store 0.13.2

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::http::body::HttpRequestBody", "path": "HttpRequestBody"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [115, 1], "end": [161, 2], "filename": "src/client/http/body.rs"}, "trait": {"args": null, "id": "http_body::Body", "path": "Body"}, "trait_path": "http_body::Body"}`

Source: `src/client/http/body.rs:117`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-80e4373986954d5e80c48b30"></a>
## as_bytes

`function` · `object_store::client::http::body::HttpRequestBody::as_bytes` · object_store 0.13.2

```rust
fn as_bytes(&self) -> Option<&Bytes>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::http::body::HttpRequestBody", "path": "HttpRequestBody"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 1], "end": [83, 2], "filename": "src/client/http/body.rs"}, "trait": null, "trait_path": null}`

Source: `src/client/http/body.rs:77`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

If this body consists of a single contiguous [`Bytes`], returns it

Unresolved upstream links (retained, not inferred): ``Bytes``.

<a id="op-a5b8a84aa578f9b35fc22da7"></a>
## clone

`function` · `object_store::client::http::body::HttpRequestBody::clone` · object_store 0.13.2

```rust
fn clone(&self) -> HttpRequestBody
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::http::body::HttpRequestBody", "path": "HttpRequestBody"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 17], "end": [33, 22], "filename": "src/client/http/body.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/client/http/body.rs:33`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e0e54423e6cf5a45ca9a7b26"></a>
## content_length

`function` · `object_store::client::http::body::HttpRequestBody::content_length` · object_store 0.13.2

```rust
fn content_length(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::http::body::HttpRequestBody", "path": "HttpRequestBody"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 1], "end": [83, 2], "filename": "src/client/http/body.rs"}, "trait": null, "trait_path": null}`

Source: `src/client/http/body.rs:69`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Returns the total length of the [`Bytes`] in this body

Unresolved upstream links (retained, not inferred): ``Bytes``.

<a id="op-3c0b989c72328e750ff4f332"></a>
## empty

`function` · `object_store::client::http::body::HttpRequestBody::empty` · object_store 0.13.2

```rust
fn empty() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::http::body::HttpRequestBody", "path": "HttpRequestBody"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 1], "end": [83, 2], "filename": "src/client/http/body.rs"}, "trait": null, "trait_path": null}`

Source: `src/client/http/body.rs:38`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

An empty [`HttpRequestBody`](../operations/object_store.client.http.body.HttpRequestBody.md#op-7e43e4f8d43fe8dffa2ec04e)

<a id="op-3b4e32f78a93e064dbeef07a"></a>
## fmt

`function` · `object_store::client::http::body::HttpRequestBody::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::http::body::HttpRequestBody", "path": "HttpRequestBody"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 10], "end": [33, 15], "filename": "src/client/http/body.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/client/http/body.rs:33`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-13350cb9053a34419df6bc37"></a>
## from

`function` · `object_store::client::http::body::HttpRequestBody::from` · object_store 0.13.2

```rust
fn from(value: Bytes) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::http::body::HttpRequestBody", "path": "HttpRequestBody"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [85, 1], "end": [89, 2], "filename": "src/client/http/body.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "Bytes"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/client/http/body.rs:86`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-337e49c32bb67331e01fad39"></a>
## from

`function` · `object_store::client::http::body::HttpRequestBody::from` · object_store 0.13.2

```rust
fn from(value: PutPayload) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::http::body::HttpRequestBody", "path": "HttpRequestBody"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [103, 1], "end": [107, 2], "filename": "src/client/http/body.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "object_store::payload::PutPayload", "path": "PutPayload"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/client/http/body.rs:104`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7445f99064593bb2e322617f"></a>
## from

`function` · `object_store::client::http::body::HttpRequestBody::from` · object_store 0.13.2

```rust
fn from(value: Vec<u8>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::http::body::HttpRequestBody", "path": "HttpRequestBody"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [91, 1], "end": [95, 2], "filename": "src/client/http/body.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "u8"}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/client/http/body.rs:92`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-84aefd79c0c9361b40e2480c"></a>
## from

`function` · `object_store::client::http::body::HttpRequestBody::from` · object_store 0.13.2

```rust
fn from(value: String) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::http::body::HttpRequestBody", "path": "HttpRequestBody"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 1], "end": [101, 2], "filename": "src/client/http/body.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/client/http/body.rs:98`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-07841c07357db5d28e8011dd"></a>
## is_empty

`function` · `object_store::client::http::body::HttpRequestBody::is_empty` · object_store 0.13.2

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::http::body::HttpRequestBody", "path": "HttpRequestBody"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 1], "end": [83, 2], "filename": "src/client/http/body.rs"}, "trait": null, "trait_path": null}`

Source: `src/client/http/body.rs:61`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Returns true if this body is empty

<a id="op-75ba599ee1ce4439ded060f0"></a>
## is_end_stream

`function` · `object_store::client::http::body::HttpRequestBody::is_end_stream` · object_store 0.13.2

```rust
fn is_end_stream(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::http::body::HttpRequestBody", "path": "HttpRequestBody"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [115, 1], "end": [161, 2], "filename": "src/client/http/body.rs"}, "trait": {"args": null, "id": "http_body::Body", "path": "Body"}, "trait_path": "http_body::Body"}`

Source: `src/client/http/body.rs:145`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e1236ce6e640ae58f22909c1"></a>
## poll_frame

`function` · `object_store::client::http::body::HttpRequestBody::poll_frame` · object_store 0.13.2

```rust
fn poll_frame(Pin<&mut self>, _cx: &mut Context<'_>) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::http::body::HttpRequestBody", "path": "HttpRequestBody"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [115, 1], "end": [161, 2], "filename": "src/client/http/body.rs"}, "trait": {"args": null, "id": "http_body::Body", "path": "Body"}, "trait_path": "http_body::Body"}`

Source: `src/client/http/body.rs:119`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a6a35fce8d10d6877b5d0928"></a>
## size_hint

`function` · `object_store::client::http::body::HttpRequestBody::size_hint` · object_store 0.13.2

```rust
fn size_hint(&self) -> SizeHint
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::http::body::HttpRequestBody", "path": "HttpRequestBody"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [115, 1], "end": [161, 2], "filename": "src/client/http/body.rs"}, "trait": {"args": null, "id": "http_body::Body", "path": "Body"}, "trait_path": "http_body::Body"}`

Source: `src/client/http/body.rs:152`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.
