# `deltalake_catalog_unity::client::retry`

Crate `deltalake-catalog-unity` · 3 public items · structured records in [`model/deltalake_catalog_unity.client.retry.json`](../model/deltalake_catalog_unity.client.retry.json)

## RetryConfig

`struct` · `deltalake_catalog_unity::client::retry::RetryConfig`
[Full member contracts, output types and access classification](../operations/deltalake_catalog_unity.client.retry.RetryConfig.md)

```rust
struct RetryConfig
```

**Fields**: `backoff`, `max_retries`, `retry_timeout`

**Derives**: Clone, Debug, Default

Contains the configuration for how to respond to server errors

By default, they will be retried up to some limit, using exponential
backoff with jitter. See [`BackoffConfig`] for more information

---

## RetryError

`struct` · `deltalake_catalog_unity::client::retry::RetryError`
[Full member contracts, output types and access classification](../operations/deltalake_catalog_unity.client.retry.RetryError.md)

```rust
struct RetryError
```

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**Methods** (1)

```rust
fn status(&self) -> Option<StatusCode>
```

**via `core::error::Error`**

```rust
fn source(&self) -> Option<&dyn std::error::Error + 'static>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Retry request error

---

## Result

`type_alias` · `deltalake_catalog_unity::client::retry::Result`
[Full member contracts, output types and access classification](../operations/deltalake_catalog_unity.client.retry.Result.md)

```rust
type Result<T, E = RetryError> = std::result::Result<T, E>
```

Error retrying http requests

---
