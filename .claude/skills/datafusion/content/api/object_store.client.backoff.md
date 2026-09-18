# `object_store::client::backoff`

Crate `object_store` · 1 public items · structured records in [`model/object_store.client.backoff.json`](../model/object_store.client.backoff.json)

## BackoffConfig

`struct` · `object_store::client::backoff::BackoffConfig`

Also reachable as `datafusion::object_store::BackoffConfig`, `object_store::BackoffConfig`

```rust
struct BackoffConfig
```

**Fields**: `init_backoff`, `max_backoff`, `base`

**Derives**: Clone, Debug, Default

[Full member, field, variant and typed contracts](../operations/object_store.client.backoff.BackoffConfig.md).


Exponential backoff with decorrelated jitter algorithm

The first backoff will always be `init_backoff`.

Subsequent backoffs will pick a random value between `init_backoff` and
`base * previous` where `previous` is the duration of the previous backoff

See <https://aws.amazon.com/blogs/architecture/exponential-backoff-and-jitter/>

---
