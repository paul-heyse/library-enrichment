# `object_store::client::http::spawn::SpawnService`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.client.http.spawn.SpawnService.json).

<a id="op-f7ec7c1b7826ea69804f425f"></a>
## SpawnService

`struct` · `object_store::client::http::spawn::SpawnService` · object_store 0.13.2

```rust
struct SpawnService<T: HttpService + Clone>
```

Source: `src/client/http/spawn.rs:49`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Wraps a provided [`HttpService`](../operations/object_store.client.http.connection.HttpService.md#op-2e6c2f39e9313e848529838d) and runs it on a separate tokio runtime

See example on [`SpawnedReqwestConnector`]

[`SpawnedReqwestConnector`]: crate::client::http::SpawnedReqwestConnector

<a id="op-5e6ec87f0e09dbe5c7e9c1e4"></a>
## call

`function` · `object_store::client::http::spawn::SpawnService::call` · object_store 0.13.2

```rust
async fn call(&self, req: HttpRequest) -> Result<HttpResponse, HttpError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "object_store::client::http::spawn::SpawnService", "path": "SpawnService"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "object_store::client::http::connection::HttpService", "path": "HttpService"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 1], "end": [102, 2], "filename": "src/client/http/spawn.rs"}, "trait": {"args": null, "id": "object_store::client::http::connection::HttpService", "path": "HttpService"}, "trait_path": "object_store::client::http::connection::HttpService"}`

Source: `src/client/http/spawn.rs:63`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-15eb6302a72a2e85f4b6b0d9"></a>
## fmt

`function` · `object_store::client::http::spawn::SpawnService::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "object_store::client::http::spawn::SpawnService", "path": "SpawnService"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "object_store::client::http::connection::HttpService", "path": "HttpService"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 10], "end": [48, 15], "filename": "src/client/http/spawn.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/client/http/spawn.rs:48`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6cb3f473f9b455e7ba236f35"></a>
## new

`function` · `object_store::client::http::spawn::SpawnService::new` · object_store 0.13.2

```rust
fn new(inner: T, runtime: Handle) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "object_store::client::http::spawn::SpawnService", "path": "SpawnService"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "object_store::client::http::connection::HttpService", "path": "HttpService"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 1], "end": [59, 2], "filename": "src/client/http/spawn.rs"}, "trait": null, "trait_path": null}`

Source: `src/client/http/spawn.rs:56`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Creates a new [`SpawnService`](../operations/object_store.client.http.spawn.SpawnService.md#op-f7ec7c1b7826ea69804f425f) from the provided
