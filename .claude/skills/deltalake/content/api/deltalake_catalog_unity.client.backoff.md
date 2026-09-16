# `deltalake_catalog_unity::client::backoff`

Crate `deltalake-catalog-unity` · 2 public items · structured records in [`model/deltalake_catalog_unity.client.backoff.json`](../model/deltalake_catalog_unity.client.backoff.json)

## Backoff

`struct` · `deltalake_catalog_unity::client::backoff::Backoff`

```rust
struct Backoff
```

**Derives**: Debug

**Methods** (3)

```rust
fn new(config: &BackoffConfig) -> Self
fn new_with_rng(config: &BackoffConfig, rng: Option<Box<dyn Rng + Sync + Send>>) -> Self
fn tick(&mut self) -> Duration
```

[`Backoff`] can be created from a [`BackoffConfig`]

Consecutive calls to [`Backoff::tick`] will return the next backoff interval

---

## BackoffConfig

`struct` · `deltalake_catalog_unity::client::backoff::BackoffConfig`

```rust
struct BackoffConfig
```

**Fields**: `init_backoff`, `max_backoff`, `base`

**Derives**: Clone, Debug, Default

Exponential backoff with jitter

See <https://aws.amazon.com/blogs/architecture/exponential-backoff-and-jitter/>

---
