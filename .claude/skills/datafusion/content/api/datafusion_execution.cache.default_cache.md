# `datafusion_execution::cache::default_cache`

Crate `datafusion-execution` · 3 public items · structured records in [`model/datafusion_execution.cache.default_cache.json`](../model/datafusion_execution.cache.default_cache.json)

## DefaultCache

`struct` · `datafusion_execution::cache::default_cache::DefaultCache`

```rust
struct DefaultCache<K: CacheKey, V: CacheValue>
```

**Implements**: `datafusion_execution::cache::Cache`

**Methods** (5)

```rust
fn memory_used(&self) -> usize
fn new(memory_limit: usize) -> Self
fn new_with_ttl(memory_limit: usize, ttl: Option<Duration>) -> Self
fn with_name(self, name: impl Into<String>) -> Self
fn with_time_provider(self, provider: Arc<dyn TimeProvider>) -> Self
```

**via `datafusion_execution::cache::Cache`**

```rust
fn cache_limit(&self) -> usize
fn cache_ttl(&self) -> Option<Duration>
fn clear(&self)
fn contains_key(&self, k: &K) -> bool
fn drop_table_entries(&self, table_ref: &TableReference) -> Result<()>
fn get(&self, key: &K) -> Option<V>
fn len(&self) -> usize
fn list_entries(&self) -> HashMap<K, CacheEntryInfo<V>>
fn name(&self) -> String
fn put(&self, key: &K, value: V) -> Option<V>
fn remove(&self, k: &K) -> Option<V>
fn update_cache_limit(&self, limit: usize)
fn update_cache_ttl(&self, ttl: Option<Duration>)
```

[Full member, field, variant and typed contracts](../operations/datafusion_execution.cache.default_cache.DefaultCache.md).


In-memory [`Cache`] with an LRU eviction policy, byte-based memory limit,
and optional per-entry TTL.

Entries are evicted in least-recently-used order whenever an insert would
push `memory_used` above `memory_limit`. Inserts whose own size exceeds the
limit are rejected (and any prior entry under the same key is removed).
When a TTL is configured, the expiration is stamped onto each entry at
insertion time and checked lazily on access. Entries with size 0 are rejected.

---

## SystemTimeProvider

`struct` · `datafusion_execution::cache::default_cache::SystemTimeProvider`

```rust
struct SystemTimeProvider
```

**Implements**: `datafusion_execution::cache::default_cache::TimeProvider`

**Derives**: Debug, Default

**via `datafusion_execution::cache::default_cache::TimeProvider`**

```rust
fn now(&self) -> Instant
```

[Full member, field, variant and typed contracts](../operations/datafusion_execution.cache.default_cache.SystemTimeProvider.md).


[`TimeProvider`] backed by [`Instant::now`].

This is the default time source used by [`DefaultCache`]

---

## TimeProvider

`trait` · `datafusion_execution::cache::default_cache::TimeProvider`

```rust
trait TimeProvider: Send + Sync
```

**Implementors** (1)

- `datafusion_execution::cache::default_cache::SystemTimeProvider`

**Methods** (1)

```rust
fn now(&self) -> Instant
```

[Full member, field, variant and typed contracts](../operations/datafusion_execution.cache.default_cache.TimeProvider.md).


Source of the current time used by a [`DefaultCache`] when applying TTLs.

---
