# `object_store::client::retry`

Crate `object_store` · 3 public items · structured records in [`model/object_store.client.retry.json`](../model/object_store.client.retry.json)

## RequestError

`enum` · `object_store::client::retry::RequestError`

```rust
enum RequestError
```

**Variants**: `BareRedirect`, `Status`, `Response`, `Http`

[Full member, field, variant and typed contracts](../operations/object_store.client.retry.RequestError.md).


The reason a request failed

---

## RetryConfig

`struct` · `object_store::client::retry::RetryConfig`

Also reachable as `datafusion::object_store::RetryConfig`, `object_store::RetryConfig`

```rust
struct RetryConfig
```

**Fields**: `backoff`, `max_retries`, `retry_timeout`

**Derives**: Clone, Debug, Default

[Full member, field, variant and typed contracts](../operations/object_store.client.retry.RetryConfig.md).


The configuration for how to respond to request errors

The following categories of error will be retried:

* 5xx server errors
* Connection errors
* Dropped connections
* Timeouts for [safe] / read-only requests

Requests will be retried up to some limit, using exponential
backoff with jitter. See [`BackoffConfig`] for more information

[safe]: https://datatracker.ietf.org/doc/html/rfc7231#section-4.2.1

---

## RetryError

`struct` · `object_store::client::retry::RetryError`

```rust
struct RetryError
```

[Full member, field, variant and typed contracts](../operations/object_store.client.retry.RetryError.md).


Retry request error

---
