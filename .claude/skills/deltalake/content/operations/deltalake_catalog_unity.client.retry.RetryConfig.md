# `deltalake_catalog_unity::client::retry::RetryConfig`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_catalog_unity.client.retry.RetryConfig.json).

<a id="op-253b88b3de5db39d1cfe20d1"></a>
## RetryConfig

`struct` · `deltalake_catalog_unity::client::retry::RetryConfig` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct RetryConfig
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/retry.rs#L78).

Source: `crates/catalog-unity/src/client/retry.rs:78`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Contains the configuration for how to respond to server errors

By default, they will be retried up to some limit, using exponential
backoff with jitter. See [`BackoffConfig`](../operations/deltalake_catalog_unity.client.backoff.BackoffConfig.md#op-2ad37cade38b3ae6f9d5b87b) for more information


<a id="op-76a81293cfeee0618fca3cf7"></a>
## backoff

`struct_field` · `deltalake_catalog_unity::client::retry::RetryConfig::backoff` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
backoff: super::backoff::BackoffConfig
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/retry.rs#L80).

Source: `crates/catalog-unity/src/client/retry.rs:80`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The backoff configuration

<a id="op-f76c3263d9ec549fc9879657"></a>
## clone

`function` · `deltalake_catalog_unity::client::retry::RetryConfig::clone` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> RetryConfig
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/retry.rs#L77).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::client::retry::RetryConfig", "path": "RetryConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 17], "end": [77, 22], "filename": "crates/catalog-unity/src/client/retry.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/catalog-unity/src/client/retry.rs:77`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-41fcb6140b57c91f8d643496"></a>
## default

`function` · `deltalake_catalog_unity::client::retry::RetryConfig::default` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/retry.rs#L102).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::client::retry::RetryConfig", "path": "RetryConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [101, 1], "end": [109, 2], "filename": "crates/catalog-unity/src/client/retry.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/catalog-unity/src/client/retry.rs:102`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e0cdf1f2f46ccf37a2c18ea0"></a>
## fmt

`function` · `deltalake_catalog_unity::client::retry::RetryConfig::fmt` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/retry.rs#L77).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::client::retry::RetryConfig", "path": "RetryConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 10], "end": [77, 15], "filename": "crates/catalog-unity/src/client/retry.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/catalog-unity/src/client/retry.rs:77`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3e28c7724fe585b4925d72fb"></a>
## max_retries

`struct_field` · `deltalake_catalog_unity::client::retry::RetryConfig::max_retries` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
max_retries: usize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/retry.rs#L85).

Source: `crates/catalog-unity/src/client/retry.rs:85`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The maximum number of times to retry a request

Set to 0 to disable retries

<a id="op-e9a06527b0ef17294fcda30c"></a>
## retry_timeout

`struct_field` · `deltalake_catalog_unity::client::retry::RetryConfig::retry_timeout` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
retry_timeout: std::time::Duration
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/retry.rs#L98).

Source: `crates/catalog-unity/src/client/retry.rs:98`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The maximum length of time from the initial request
after which no further retries will be attempted

This not only bounds the length of time before a server
error will be surfaced to the application, but also bounds
the length of time a request's credentials must remain valid.

As requests are retried without renewing credentials or
regenerating request payloads, this number should be kept
below 5 minutes to avoid errors due to expired credentials
and/or request payloads
