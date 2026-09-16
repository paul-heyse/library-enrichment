# `deltalake_catalog_unity::client::token`

Crate `deltalake-catalog-unity` · 2 public items · structured records in [`model/deltalake_catalog_unity.client.token.json`](../model/deltalake_catalog_unity.client.token.json)

## TemporaryToken

`struct` · `deltalake_catalog_unity::client::token::TemporaryToken`

```rust
struct TemporaryToken<T>
```

**Fields**: `token`, `expiry`

**Derives**: Clone, Debug

A temporary authentication token with an associated expiry

---

## TokenCache

`struct` · `deltalake_catalog_unity::client::token::TokenCache`

```rust
struct TokenCache<T>
```

**Derives**: Debug, Default

**Methods** (1)

```rust
async fn get_or_insert_with<F, Fut, E>(&self, f: F) -> Result<T, E> where F: FnOnce() -> Fut + Send, Fut: Future<Output = Result<TemporaryToken<T>, E>> + Send
```

Provides [`TokenCache::get_or_insert_with`] which can be used to cache a
[`TemporaryToken`] based on its expiry

---
