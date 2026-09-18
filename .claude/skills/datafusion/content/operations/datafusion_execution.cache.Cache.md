# `datafusion_execution::cache::Cache`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.cache.Cache.json).

<a id="op-6f236f740169b7945f9787f8"></a>
## Cache

`trait` · `datafusion_execution::cache::Cache` · datafusion-execution 55.1.0

```rust
trait Cache<K: CacheKey, V: CacheValue>: Send + Sync
```

Source: `src/cache/mod.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Base trait for cache implementations with common operations.

This trait provides the fundamental cache operations (`get`, `put`, `remove`, etc.)
that all cache types share.

## Thread Safety

Implementations must handle their own locking via internal mutability, as methods do not
take mutable references and may be accessed by multiple concurrent queries.


<a id="op-feb7bdebeaa7e4afa6e510dd"></a>
## cache_limit

`function` · `datafusion_execution::cache::Cache::cache_limit` · datafusion-execution 55.1.0

```rust
fn cache_limit(&self) -> usize
```

Source: `src/cache/mod.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Current memory budget, in bytes.

<a id="op-a50b6a51bc8067c591307c8c"></a>
## cache_ttl

`function` · `datafusion_execution::cache::Cache::cache_ttl` · datafusion-execution 55.1.0

```rust
fn cache_ttl(&self) -> Option<Duration>
```

Source: `src/cache/mod.rs:80`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Time-to-live applied to newly inserted entries, or `None` if entries
never expire on their own.

<a id="op-0a94d71bca3633fac0940858"></a>
## clear

`function` · `datafusion_execution::cache::Cache::clear` · datafusion-execution 55.1.0

```rust
fn clear(&self)
```

Source: `src/cache/mod.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Remove all entries from the cache.

<a id="op-758df17480d568a3260fc79e"></a>
## contains_key

`function` · `datafusion_execution::cache::Cache::contains_key` · datafusion-execution 55.1.0

```rust
fn contains_key(&self, k: &K) -> bool
```

Source: `src/cache/mod.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Check if the cache contains a specific key.

<a id="op-6ad7064b935565c50d1a4a46"></a>
## drop_table_entries

`function` · `datafusion_execution::cache::Cache::drop_table_entries` · datafusion-execution 55.1.0

```rust
fn drop_table_entries(&self, table_ref: &TableReference) -> datafusion_common::Result<()>
```

Source: `src/cache/mod.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Invalidate every entry associated with `table_ref`.

<a id="op-9020ac6a8f2077313f5b686d"></a>
## get

`function` · `datafusion_execution::cache::Cache::get` · datafusion-execution 55.1.0

```rust
fn get(&self, key: &K) -> Option<V>
```

Source: `src/cache/mod.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Get a cached entry if it exists.

<a id="op-3d2fc6d3d983b268d274530f"></a>
## is_empty

`function` · `datafusion_execution::cache::Cache::is_empty` · datafusion-execution 55.1.0

```rust
fn is_empty(&self) -> bool
```

Source: `src/cache/mod.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Check if the cache collection is empty.

<a id="op-ccb6b155ab9d62afe07944dd"></a>
## len

`function` · `datafusion_execution::cache::Cache::len` · datafusion-execution 55.1.0

```rust
fn len(&self) -> usize
```

Source: `src/cache/mod.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Fetch the total number of cache entries.

<a id="op-9baad1b51c84461e5b96f7a2"></a>
## list_entries

`function` · `datafusion_execution::cache::Cache::list_entries` · datafusion-execution 55.1.0

```rust
fn list_entries(&self) -> HashMap<K, CacheEntryInfo<V>>
```

Source: `src/cache/mod.rs:93`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Snapshot of all current entries with per-entry metadata (size, hits,
expiration) for diagnostics and observability.

<a id="op-8f5015f1ef4fa922b7f79511"></a>
## name

`function` · `datafusion_execution::cache::Cache::name` · datafusion-execution 55.1.0

```rust
fn name(&self) -> String
```

Source: `src/cache/mod.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Return the cache name.

<a id="op-be701f93593482e2a922470c"></a>
## put

`function` · `datafusion_execution::cache::Cache::put` · datafusion-execution 55.1.0

```rust
fn put(&self, key: &K, value: V) -> Option<V>
```

Source: `src/cache/mod.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Store a value in the cache.

Returns the previous value if one existed.

<a id="op-887600a90681caed86e9ca30"></a>
## remove

`function` · `datafusion_execution::cache::Cache::remove` · datafusion-execution 55.1.0

```rust
fn remove(&self, k: &K) -> Option<V>
```

Source: `src/cache/mod.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Remove an entry from the cache, returning the value if it existed.

<a id="op-9d117d59e2106e3c7a4fd19a"></a>
## update_cache_limit

`function` · `datafusion_execution::cache::Cache::update_cache_limit` · datafusion-execution 55.1.0

```rust
fn update_cache_limit(&self, limit: usize)
```

Source: `src/cache/mod.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Change the memory budget in bytes.

<a id="op-b518562ad149bb75921bf650"></a>
## update_cache_ttl

`function` · `datafusion_execution::cache::Cache::update_cache_ttl` · datafusion-execution 55.1.0

```rust
fn update_cache_ttl(&self, _ttl: Option<Duration>)
```

Source: `src/cache/mod.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Change the TTL applied to subsequent inserts.
