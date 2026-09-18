# `deltalake_catalog_unity::client::token::TokenCache`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_catalog_unity.client.token.TokenCache.json).

<a id="op-aabcf65fa6fbf413d0b15ad1"></a>
## TokenCache

`struct` · `deltalake_catalog_unity::client::token::TokenCache` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct TokenCache<T>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/token.rs#L20).

Source: `crates/catalog-unity/src/client/token.rs:20`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Provides [`TokenCache::get_or_insert_with`](../operations/deltalake_catalog_unity.client.token.TokenCache.md#op-4ad20dfd8d0681f400c10620) which can be used to cache a
[`TemporaryToken`](../operations/deltalake_catalog_unity.client.token.TemporaryToken.md#op-5bd01ed00906a0658fc6b82b) based on its expiry

<a id="op-f228d165923dc819c71257ae"></a>
## default

`function` · `deltalake_catalog_unity::client::token::TokenCache::default` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/token.rs#L25).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "deltalake_catalog_unity::client::token::TokenCache", "path": "TokenCache"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [24, 1], "end": [30, 2], "filename": "crates/catalog-unity/src/client/token.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/catalog-unity/src/client/token.rs:25`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-797ba3e68bcc8004ab430274"></a>
## fmt

`function` · `deltalake_catalog_unity::client::token::TokenCache::fmt` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/token.rs#L19).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "deltalake_catalog_unity::client::token::TokenCache", "path": "TokenCache"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [19, 10], "end": [19, 15], "filename": "crates/catalog-unity/src/client/token.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/catalog-unity/src/client/token.rs:19`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4ad20dfd8d0681f400c10620"></a>
## get_or_insert_with

`function` · `deltalake_catalog_unity::client::token::TokenCache::get_or_insert_with` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn get_or_insert_with<F, Fut, E>(&self, f: F) -> Result<T, E> where F: FnOnce() -> Fut + Send, Fut: Future<Output = Result<TemporaryToken<T>, E>> + Send
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/token.rs#L34).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "deltalake_catalog_unity::client::token::TokenCache", "path": "TokenCache"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 1], "end": [64, 2], "filename": "crates/catalog-unity/src/client/token.rs"}, "trait": null, "trait_path": null}`

Source: `crates/catalog-unity/src/client/token.rs:34`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Get current token or update with a given closure

<a id="op-087ecaf8b99d624a4c4dc42d"></a>
## cache

`struct_field` · `deltalake_catalog_unity::client::token::TokenCache::cache` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
cache: tokio::sync::Mutex<Option<TemporaryToken<T>>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/token.rs#L21).

Source: `crates/catalog-unity/src/client/token.rs:21`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
