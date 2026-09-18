# `object_store::client::http::connection::HttpService`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.client.http.connection.HttpService.json).

<a id="op-2e6c2f39e9313e848529838d"></a>
## HttpService

`trait` · `object_store::client::http::connection::HttpService` · object_store 0.13.2

```rust
trait HttpService: std::fmt::Debug + Send + Sync + 'static
```

Source: `src/client/http/connection.rs:143`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

An asynchronous function from a [`HttpRequest`](../operations/object_store.client.http.body.HttpRequest.md#op-0f9bc5af637ad2947840b60b) to a [`HttpResponse`](../operations/object_store.client.http.body.HttpResponse.md#op-6c88f511e46991a2e002ff3b).

<a id="op-8ee38906955a036017f28537"></a>
## call

`function` · `object_store::client::http::connection::HttpService::call` · object_store 0.13.2

```rust
async fn call(&self, req: HttpRequest) -> Result<HttpResponse, HttpError>
```

Source: `src/client/http/connection.rs:145`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Perform [`HttpRequest`](../operations/object_store.client.http.body.HttpRequest.md#op-0f9bc5af637ad2947840b60b) returning [`HttpResponse`](../operations/object_store.client.http.body.HttpResponse.md#op-6c88f511e46991a2e002ff3b)
