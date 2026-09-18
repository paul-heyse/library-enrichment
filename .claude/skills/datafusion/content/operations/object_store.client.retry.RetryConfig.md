# `object_store::client::retry::RetryConfig`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.client.retry.RetryConfig.json).

<a id="op-584b62486ba676a5f7aeab66"></a>
## RetryConfig

`struct` · `object_store::client::retry::RetryConfig` · object_store 0.13.2

```rust
struct RetryConfig
```

Source: `src/client/retry.rs:229`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

The configuration for how to respond to request errors

The following categories of error will be retried:

* 5xx server errors
* Connection errors
* Dropped connections
* Timeouts for [safe] / read-only requests

Requests will be retried up to some limit, using exponential
backoff with jitter. See [`BackoffConfig`](../operations/object_store.client.backoff.BackoffConfig.md#op-34b26a88c595bed8eca98825) for more information

[safe]: https://datatracker.ietf.org/doc/html/rfc7231#section-4.2.1

<a id="op-60b1e6411f0823b22bbd8fee"></a>
## backoff

`struct_field` · `object_store::client::retry::RetryConfig::backoff` · object_store 0.13.2

```rust
backoff: client::backoff::BackoffConfig
```

Source: `src/client/retry.rs:231`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

The backoff configuration

<a id="op-998d3c8dea0f044378a49cbf"></a>
## clone

`function` · `object_store::client::retry::RetryConfig::clone` · object_store 0.13.2

```rust
fn clone(&self) -> RetryConfig
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::retry::RetryConfig", "path": "RetryConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [228, 17], "end": [228, 22], "filename": "src/client/retry.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/client/retry.rs:228`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4811d23a9dac2158ac36afa1"></a>
## default

`function` · `object_store::client::retry::RetryConfig::default` · object_store 0.13.2

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::retry::RetryConfig", "path": "RetryConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [252, 1], "end": [260, 2], "filename": "src/client/retry.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/client/retry.rs:253`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1472932f6d3653d92dcb727a"></a>
## fmt

`function` · `object_store::client::retry::RetryConfig::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::retry::RetryConfig", "path": "RetryConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [228, 10], "end": [228, 15], "filename": "src/client/retry.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/client/retry.rs:228`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-de891cb0bbd83379bc227892"></a>
## max_retries

`struct_field` · `object_store::client::retry::RetryConfig::max_retries` · object_store 0.13.2

```rust
max_retries: usize
```

Source: `src/client/retry.rs:236`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

The maximum number of times to retry a request

Set to 0 to disable retries

<a id="op-5e637d0a3fab4349f5cd2cec"></a>
## retry_timeout

`struct_field` · `object_store::client::retry::RetryConfig::retry_timeout` · object_store 0.13.2

```rust
retry_timeout: std::time::Duration
```

Source: `src/client/retry.rs:249`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

The maximum length of time from the initial request
after which no further retries will be attempted

This not only bounds the length of time before a server
error will be surfaced to the application, but also bounds
the length of time a request's credentials must remain valid.

As requests are retried without renewing credentials or
regenerating request payloads, this number should be kept
below 5 minutes to avoid errors due to expired credentials
and/or request payloads
