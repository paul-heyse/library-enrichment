# `object_store::client::http::connection::SpawnedReqwestConnector`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.client.http.connection.SpawnedReqwestConnector.json).

<a id="op-720e899e044572240f59b96f"></a>
## SpawnedReqwestConnector

`struct` · `object_store::client::http::connection::SpawnedReqwestConnector` · object_store 0.13.2

```rust
struct SpawnedReqwestConnector
```

Source: `src/client/http/connection.rs:340`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

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

Unresolved upstream links (retained, not inferred): `tokio::runtime::Runtime`, ``reqwest::Client``.

<a id="op-7339b90d5295c7c9ac219d78"></a>
## connect

`function` · `object_store::client::http::connection::SpawnedReqwestConnector::connect` · object_store 0.13.2

```rust
fn connect(&self, options: &ClientOptions) -> Result<HttpClient>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::http::connection::SpawnedReqwestConnector", "path": "SpawnedReqwestConnector"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [356, 1], "end": [361, 2], "filename": "src/client/http/connection.rs"}, "trait": {"args": null, "id": "object_store::client::http::connection::HttpConnector", "path": "HttpConnector"}, "trait_path": "object_store::client::http::connection::HttpConnector"}`

Source: `src/client/http/connection.rs:357`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d089a4a7df8b62a0983d06ab"></a>
## fmt

`function` · `object_store::client::http::connection::SpawnedReqwestConnector::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::http::connection::SpawnedReqwestConnector", "path": "SpawnedReqwestConnector"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [337, 10], "end": [337, 15], "filename": "src/client/http/connection.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/client/http/connection.rs:337`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-769ef1c4193b2b22ba148fdc"></a>
## new

`function` · `object_store::client::http::connection::SpawnedReqwestConnector::new` · object_store 0.13.2

```rust
fn new(runtime: Handle) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::http::connection::SpawnedReqwestConnector", "path": "SpawnedReqwestConnector"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [345, 1], "end": [353, 2], "filename": "src/client/http/connection.rs"}, "trait": null, "trait_path": null}`

Source: `src/client/http/connection.rs:350`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Create a new [`SpawnedReqwestConnector`](../operations/object_store.client.http.connection.SpawnedReqwestConnector.md#op-720e899e044572240f59b96f) with the provided [`Handle`] to
a tokio [`Runtime`]

[`Runtime`]: tokio::runtime::Runtime

Unresolved upstream links (retained, not inferred): `tokio::runtime::Runtime`, ``Handle``.
