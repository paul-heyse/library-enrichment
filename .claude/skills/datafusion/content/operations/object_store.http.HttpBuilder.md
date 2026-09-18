# `object_store::http::HttpBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.http.HttpBuilder.json).

<a id="op-7a8de6e5acfc6874f14518f6"></a>
## HttpBuilder

`struct` · `object_store::http::HttpBuilder` · object_store 0.13.2

```rust
struct HttpBuilder
```

Source: `src/http/mod.rs:230`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Configure a connection to a generic HTTP server

<a id="op-5d180b9f9aea85f81e34cc3c"></a>
## build

`function` · `object_store::http::HttpBuilder::build` · object_store 0.13.2

```rust
fn build(self) -> Result<HttpStore>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::http::HttpBuilder", "path": "HttpBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [237, 1], "end": [291, 2], "filename": "src/http/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/http/mod.rs:276`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Build an [`HttpStore`](../operations/object_store.http.HttpStore.md#op-547611b00a8e9d96e730371c) with the configured options

<a id="op-b54f835b9a7979a0aaafbf93"></a>
## clone

`function` · `object_store::http::HttpBuilder::clone` · object_store 0.13.2

```rust
fn clone(&self) -> HttpBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::http::HttpBuilder", "path": "HttpBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [229, 26], "end": [229, 31], "filename": "src/http/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/http/mod.rs:229`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c98ae55dcaf6fa260f9ebd3b"></a>
## default

`function` · `object_store::http::HttpBuilder::default` · object_store 0.13.2

```rust
fn default() -> HttpBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::http::HttpBuilder", "path": "HttpBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [229, 17], "end": [229, 24], "filename": "src/http/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/http/mod.rs:229`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1a865481d085947deb0d4647"></a>
## fmt

`function` · `object_store::http::HttpBuilder::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::http::HttpBuilder", "path": "HttpBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [229, 10], "end": [229, 15], "filename": "src/http/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/http/mod.rs:229`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9940c6c86cfef312a1efa021"></a>
## new

`function` · `object_store::http::HttpBuilder::new` · object_store 0.13.2

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::http::HttpBuilder", "path": "HttpBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [237, 1], "end": [291, 2], "filename": "src/http/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/http/mod.rs:239`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Create a new [`HttpBuilder`](../operations/object_store.http.HttpBuilder.md#op-7a8de6e5acfc6874f14518f6) with default values.

<a id="op-f9a69df71b853469916c9d39"></a>
## with_client_options

`function` · `object_store::http::HttpBuilder::with_client_options` · object_store 0.13.2

```rust
fn with_client_options(self, options: ClientOptions) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::http::HttpBuilder", "path": "HttpBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [237, 1], "end": [291, 2], "filename": "src/http/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/http/mod.rs:262`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Sets the client options, overriding any already set

<a id="op-520b9cc7bceadee58aa0a27c"></a>
## with_config

`function` · `object_store::http::HttpBuilder::with_config` · object_store 0.13.2

```rust
fn with_config(self, key: ClientConfigKey, value: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::http::HttpBuilder", "path": "HttpBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [237, 1], "end": [291, 2], "filename": "src/http/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/http/mod.rs:256`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set individual client configuration without overriding the entire config

<a id="op-daa70f045cdce0ef87900ff1"></a>
## with_http_connector

`function` · `object_store::http::HttpBuilder::with_http_connector` · object_store 0.13.2

```rust
fn with_http_connector<C: HttpConnector>(self, connector: C) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::http::HttpBuilder", "path": "HttpBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [237, 1], "end": [291, 2], "filename": "src/http/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/http/mod.rs:270`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

The [`HttpConnector`](../operations/object_store.client.http.connection.HttpConnector.md#op-04b72af01cc3e9636d12c25b) to use

On non-WASM32 platforms uses [`reqwest`] by default, on WASM32 platforms must be provided

Unresolved upstream links (retained, not inferred): ``reqwest``.

<a id="op-47d3cc3e12946b170ea76a4d"></a>
## with_retry

`function` · `object_store::http::HttpBuilder::with_retry` · object_store 0.13.2

```rust
fn with_retry(self, retry_config: RetryConfig) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::http::HttpBuilder", "path": "HttpBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [237, 1], "end": [291, 2], "filename": "src/http/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/http/mod.rs:250`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set the retry configuration

<a id="op-e1cf3706208be9bce7c86fda"></a>
## with_url

`function` · `object_store::http::HttpBuilder::with_url` · object_store 0.13.2

```rust
fn with_url(self, url: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::http::HttpBuilder", "path": "HttpBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [237, 1], "end": [291, 2], "filename": "src/http/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/http/mod.rs:244`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set the URL
