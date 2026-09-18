# `object_store::client::http::connection::HttpConnector`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.client.http.connection.HttpConnector.json).

<a id="op-04b72af01cc3e9636d12c25b"></a>
## HttpConnector

`trait` · `object_store::client::http::connection::HttpConnector` · object_store 0.13.2

```rust
trait HttpConnector: std::fmt::Debug + Send + Sync + 'static
```

Source: `src/client/http/connection.rs:283`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

A factory for [`HttpClient`](../operations/object_store.client.http.connection.HttpClient.md#op-38a7e96c8ab1bbba97a3c5aa)

<a id="op-5fe137182d0ba28e8def5545"></a>
## connect

`function` · `object_store::client::http::connection::HttpConnector::connect` · object_store 0.13.2

```rust
fn connect(&self, options: &ClientOptions) -> Result<HttpClient>
```

Source: `src/client/http/connection.rs:285`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Create a new [`HttpClient`](../operations/object_store.client.http.connection.HttpClient.md#op-38a7e96c8ab1bbba97a3c5aa) with the provided [`ClientOptions`](../operations/object_store.client.ClientOptions.md#op-9357d27ab9e13340e3ccb12f)
