# `object_store::client::http::connection::ReqwestConnector`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.client.http.connection.ReqwestConnector.json).

<a id="op-a55af20622ccdaa29fe79bee"></a>
## ReqwestConnector

`struct` · `object_store::client::http::connection::ReqwestConnector` · object_store 0.13.2

```rust
struct ReqwestConnector
```

Source: `src/client/http/connection.rs:292`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

[`HttpConnector`](../operations/object_store.client.http.connection.HttpConnector.md#op-04b72af01cc3e9636d12c25b) using [`reqwest::Client`]

Unresolved upstream links (retained, not inferred): ``reqwest::Client``.

<a id="op-8dfe130444fad1a5cebd5073"></a>
## connect

`function` · `object_store::client::http::connection::ReqwestConnector::connect` · object_store 0.13.2

```rust
fn connect(&self, options: &ClientOptions) -> Result<HttpClient>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::http::connection::ReqwestConnector", "path": "ReqwestConnector"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [295, 1], "end": [300, 2], "filename": "src/client/http/connection.rs"}, "trait": {"args": null, "id": "object_store::client::http::connection::HttpConnector", "path": "HttpConnector"}, "trait_path": "object_store::client::http::connection::HttpConnector"}`

Source: `src/client/http/connection.rs:296`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-13b1600934d0c55a018fb63f"></a>
## default

`function` · `object_store::client::http::connection::ReqwestConnector::default` · object_store 0.13.2

```rust
fn default() -> ReqwestConnector
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::http::connection::ReqwestConnector", "path": "ReqwestConnector"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [289, 17], "end": [289, 24], "filename": "src/client/http/connection.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/client/http/connection.rs:289`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1dc7ff8ccca8cbf952f6fab7"></a>
## fmt

`function` · `object_store::client::http::connection::ReqwestConnector::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::http::connection::ReqwestConnector", "path": "ReqwestConnector"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [289, 10], "end": [289, 15], "filename": "src/client/http/connection.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/client/http/connection.rs:289`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.
