# `datafusion_execution::cache::CacheKey`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.cache.CacheKey.json).

<a id="op-86319069fc8dd373cc7ca39c"></a>
## CacheKey

`trait` · `datafusion_execution::cache::CacheKey` · datafusion-execution 55.1.0

```rust
trait CacheKey: Clone + Eq + Hash + Send + Sync + Debug
```

Source: `src/cache/mod.rs:97`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Key type for entries stored in a [`Cache`](../operations/datafusion_execution.cache.Cache.md#op-6f236f740169b7945f9787f8).

<a id="op-6ce7d296e0c774c7452d7310"></a>
## size

`function` · `datafusion_execution::cache::CacheKey::size` · datafusion-execution 55.1.0

```rust
fn size(&self) -> usize
```

Source: `src/cache/mod.rs:99`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Size of the key in bytes, used for cache memory accounting.

<a id="op-a70e72d8bb555118f180ef69"></a>
## table_ref

`function` · `datafusion_execution::cache::CacheKey::table_ref` · datafusion-execution 55.1.0

```rust
fn table_ref(&self) -> Option<&TableReference>
```

Source: `src/cache/mod.rs:103`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Table this key is associated with, or `None` if the key is not
table-scoped.
