# `object_store::client::http::connection::HttpClient`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.client.http.connection.HttpClient.json).

<a id="op-38a7e96c8ab1bbba97a3c5aa"></a>
## HttpClient

`struct` · `object_store::client::http::connection::HttpClient` · object_store 0.13.2

```rust
struct HttpClient
```

Source: `src/client/http/connection.rs:150`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

An HTTP client

<a id="op-088db9b0bd26da36014f1c30"></a>
## clone

`function` · `object_store::client::http::connection::HttpClient::clone` · object_store 0.13.2

```rust
fn clone(&self) -> HttpClient
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::http::connection::HttpClient", "path": "HttpClient"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [149, 17], "end": [149, 22], "filename": "src/client/http/connection.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/client/http/connection.rs:149`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-385d617aca4f19168308eaf3"></a>
## execute

`function` · `object_store::client::http::connection::HttpClient::execute` · object_store 0.13.2

```rust
async fn execute(&self, request: HttpRequest) -> Result<HttpResponse, HttpError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::http::connection::HttpClient", "path": "HttpClient"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [152, 1], "end": [208, 2], "filename": "src/client/http/connection.rs"}, "trait": null, "trait_path": null}`

Source: `src/client/http/connection.rs:159`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Performs [`HttpRequest`](../operations/object_store.client.http.body.HttpRequest.md#op-0f9bc5af637ad2947840b60b) using this client

<a id="op-bc931683f22777cdea8c2dd4"></a>
## fmt

`function` · `object_store::client::http::connection::HttpClient::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::http::connection::HttpClient", "path": "HttpClient"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [149, 10], "end": [149, 15], "filename": "src/client/http/connection.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/client/http/connection.rs:149`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-61cc4d5b05fce5bad70d05ef"></a>
## new

`function` · `object_store::client::http::connection::HttpClient::new` · object_store 0.13.2

```rust
fn new(service: impl HttpService + 'static) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::http::connection::HttpClient", "path": "HttpClient"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [152, 1], "end": [208, 2], "filename": "src/client/http/connection.rs"}, "trait": null, "trait_path": null}`

Source: `src/client/http/connection.rs:154`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Create a new [`HttpClient`](../operations/object_store.client.http.connection.HttpClient.md#op-38a7e96c8ab1bbba97a3c5aa) from an [`HttpService`](../operations/object_store.client.http.connection.HttpService.md#op-2e6c2f39e9313e848529838d)
