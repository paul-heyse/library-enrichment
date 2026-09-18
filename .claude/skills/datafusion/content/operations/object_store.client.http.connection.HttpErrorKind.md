# `object_store::client::http::connection::HttpErrorKind`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.client.http.connection.HttpErrorKind.json).

<a id="op-8969bdd0584c5df0a3c11797"></a>
## HttpErrorKind

`enum` · `object_store::client::http::connection::HttpErrorKind` · object_store 0.13.2

```rust
enum HttpErrorKind
```

Source: `src/client/http/connection.rs:47`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Identifies the kind of [`HttpError`](../operations/object_store.client.http.connection.HttpError.md#op-03fc99af16ec009b19623e7b)

This is used, among other things, to determine if a request can be retried

<a id="op-1de209e735d8fcdcd1b427a4"></a>
## Connect

`variant` · `object_store::client::http::connection::HttpErrorKind::Connect` · object_store 0.13.2

```rust
Connect
```

Source: `src/client/http/connection.rs:51`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

An error occurred whilst connecting to the remote

Will be automatically retried

<a id="op-48380f8a14d1879b844ffacb"></a>
## Decode

`variant` · `object_store::client::http::connection::HttpErrorKind::Decode` · object_store 0.13.2

```rust
Decode
```

Source: `src/client/http/connection.rs:67`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

An error occurred whilst decoding the response

Will not be automatically retried

<a id="op-933f38d4e128174c3966d42c"></a>
## Interrupted

`variant` · `object_store::client::http::connection::HttpErrorKind::Interrupted` · object_store 0.13.2

```rust
Interrupted
```

Source: `src/client/http/connection.rs:63`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

The request was aborted

Will be automatically retried if the request is idempotent

<a id="op-8cb0a7852ea3b4f2c677ef8f"></a>
## Request

`variant` · `object_store::client::http::connection::HttpErrorKind::Request` · object_store 0.13.2

```rust
Request
```

Source: `src/client/http/connection.rs:55`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

An error occurred whilst making the request

Will be automatically retried

<a id="op-e34f0007da69091241c7b79c"></a>
## Timeout

`variant` · `object_store::client::http::connection::HttpErrorKind::Timeout` · object_store 0.13.2

```rust
Timeout
```

Source: `src/client/http/connection.rs:59`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Request timed out

Will be automatically retried if the request is idempotent

<a id="op-67f00e1044b7ef48a4f30f12"></a>
## Unknown

`variant` · `object_store::client::http::connection::HttpErrorKind::Unknown` · object_store 0.13.2

```rust
Unknown
```

Source: `src/client/http/connection.rs:71`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

An unknown error occurred

Will not be automatically retried

<a id="op-e182efe27bf79357d256ac1a"></a>
## clone

`function` · `object_store::client::http::connection::HttpErrorKind::clone` · object_store 0.13.2

```rust
fn clone(&self) -> HttpErrorKind
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::http::connection::HttpErrorKind", "path": "HttpErrorKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 23], "end": [45, 28], "filename": "src/client/http/connection.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/client/http/connection.rs:45`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a9c1451a766404b52633be70"></a>
## eq

`function` · `object_store::client::http::connection::HttpErrorKind::eq` · object_store 0.13.2

```rust
fn eq(&self, other: &HttpErrorKind) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::http::connection::HttpErrorKind", "path": "HttpErrorKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 30], "end": [45, 39], "filename": "src/client/http/connection.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/client/http/connection.rs:45`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0a63702714df9affdfadff6d"></a>
## fmt

`function` · `object_store::client::http::connection::HttpErrorKind::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::http::connection::HttpErrorKind", "path": "HttpErrorKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 10], "end": [45, 15], "filename": "src/client/http/connection.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/client/http/connection.rs:45`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.
