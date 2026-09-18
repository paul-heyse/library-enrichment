# `object_store::client::http::connection::HttpError`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.client.http.connection.HttpError.json).

<a id="op-03fc99af16ec009b19623e7b"></a>
## HttpError

`struct` · `object_store::client::http::connection::HttpError` · object_store 0.13.2

```rust
struct HttpError
```

Source: `src/client/http/connection.rs:36`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

An HTTP protocol error

Clients should return this when an HTTP request fails to be completed, e.g. because
of a connection issue. This does **not** include HTTP requests that are return
non 2xx Status Codes, as these should instead be returned as an [`HttpResponse`](../operations/object_store.client.http.body.HttpResponse.md#op-6c88f511e46991a2e002ff3b)
with the appropriate status code set.

<a id="op-74b84af79408a2b6c1e3e18e"></a>
## fmt

`function` · `object_store::client::http::connection::HttpError::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::http::connection::HttpError", "path": "HttpError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 10], "end": [34, 15], "filename": "src/client/http/connection.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/client/http/connection.rs:34`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8cf540fecc4a59c37010b43f"></a>
## fmt

`function` · `object_store::client::http::connection::HttpError::fmt` · object_store 0.13.2

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::http::connection::HttpError", "path": "HttpError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 17], "end": [34, 33], "filename": "src/client/http/connection.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/client/http/connection.rs:34`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a1075503bf9e8c64a8379fdf"></a>
## kind

`function` · `object_store::client::http::connection::HttpError::kind` · object_store 0.13.2

```rust
fn kind(&self) -> HttpErrorKind
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::http::connection::HttpError", "path": "HttpError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 1], "end": [139, 2], "filename": "src/client/http/connection.rs"}, "trait": null, "trait_path": null}`

Source: `src/client/http/connection.rs:136`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Returns the [`HttpErrorKind`](../operations/object_store.client.http.connection.HttpErrorKind.md#op-8969bdd0584c5df0a3c11797)

<a id="op-449c8a996fa8eeba60bb7bae"></a>
## new

`function` · `object_store::client::http::connection::HttpError::new` · object_store 0.13.2

```rust
fn new<E>(kind: HttpErrorKind, e: E) -> Self where E: Error + Send + Sync + 'static
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::http::connection::HttpError", "path": "HttpError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 1], "end": [139, 2], "filename": "src/client/http/connection.rs"}, "trait": null, "trait_path": null}`

Source: `src/client/http/connection.rs:76`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Create a new [`HttpError`](../operations/object_store.client.http.connection.HttpError.md#op-03fc99af16ec009b19623e7b) with the optional status code

<a id="op-147cf11cb16f966a4ef3d242"></a>
## source

`function` · `object_store::client::http::connection::HttpError::source` · object_store 0.13.2

```rust
fn source(&self) -> ::core::option::Option<&dyn ::thiserror::__private18::Error + 'static>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::http::connection::HttpError", "path": "HttpError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 17], "end": [34, 33], "filename": "src/client/http/connection.rs"}, "trait": {"args": null, "id": "core::error::Error", "path": "Error"}, "trait_path": "core::error::Error"}`

Source: `src/client/http/connection.rs:34`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.
