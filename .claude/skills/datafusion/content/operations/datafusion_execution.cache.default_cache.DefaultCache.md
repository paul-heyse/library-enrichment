# `datafusion_execution::cache::default_cache::DefaultCache`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.cache.default_cache.DefaultCache.json).

<a id="op-0505d2d12e8de5ca44498485"></a>
## DefaultCache

`struct` · `datafusion_execution::cache::default_cache::DefaultCache` · datafusion-execution 55.1.0

```rust
struct DefaultCache<K: CacheKey, V: CacheValue>
```

Source: `src/cache/default_cache.rs:169`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

In-memory [`Cache`](../operations/datafusion_execution.cache.Cache.md#op-6f236f740169b7945f9787f8) with an LRU eviction policy, byte-based memory limit,
and optional per-entry TTL.

Entries are evicted in least-recently-used order whenever an insert would
push `memory_used` above `memory_limit`. Inserts whose own size exceeds the
limit are rejected (and any prior entry under the same key is removed).
When a TTL is configured, the expiration is stamped onto each entry at
insertion time and checked lazily on access. Entries with size 0 are rejected.

<a id="op-eaead62e9cadbb34326c0fb6"></a>
## cache_limit

`function` · `datafusion_execution::cache::default_cache::DefaultCache::cache_limit` · datafusion-execution 55.1.0

```rust
fn cache_limit(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_execution::cache::default_cache::DefaultCache", "path": "DefaultCache"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_execution::cache::CacheKey", "path": "CacheKey"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_execution::cache::CacheValue", "path": "CacheValue"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [209, 1], "end": [296, 2], "filename": "src/cache/default_cache.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_execution::cache::Cache", "path": "Cache"}, "trait_path": "datafusion_execution::cache::Cache"}`

Source: `src/cache/default_cache.rs:245`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7bff6e83f8281d8294b1ae0d"></a>
## cache_ttl

`function` · `datafusion_execution::cache::default_cache::DefaultCache::cache_ttl` · datafusion-execution 55.1.0

```rust
fn cache_ttl(&self) -> Option<Duration>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_execution::cache::default_cache::DefaultCache", "path": "DefaultCache"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_execution::cache::CacheKey", "path": "CacheKey"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_execution::cache::CacheValue", "path": "CacheValue"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [209, 1], "end": [296, 2], "filename": "src/cache/default_cache.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_execution::cache::Cache", "path": "Cache"}, "trait_path": "datafusion_execution::cache::Cache"}`

Source: `src/cache/default_cache.rs:255`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7e65085a75738734ba35015e"></a>
## clear

`function` · `datafusion_execution::cache::default_cache::DefaultCache::clear` · datafusion-execution 55.1.0

```rust
fn clear(&self)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_execution::cache::default_cache::DefaultCache", "path": "DefaultCache"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_execution::cache::CacheKey", "path": "CacheKey"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_execution::cache::CacheValue", "path": "CacheValue"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [209, 1], "end": [296, 2], "filename": "src/cache/default_cache.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_execution::cache::Cache", "path": "Cache"}, "trait_path": "datafusion_execution::cache::Cache"}`

Source: `src/cache/default_cache.rs:237`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e9fd021906ba5c1be013451a"></a>
## contains_key

`function` · `datafusion_execution::cache::default_cache::DefaultCache::contains_key` · datafusion-execution 55.1.0

```rust
fn contains_key(&self, k: &K) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_execution::cache::default_cache::DefaultCache", "path": "DefaultCache"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_execution::cache::CacheKey", "path": "CacheKey"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_execution::cache::CacheValue", "path": "CacheValue"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [209, 1], "end": [296, 2], "filename": "src/cache/default_cache.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_execution::cache::Cache", "path": "Cache"}, "trait_path": "datafusion_execution::cache::Cache"}`

Source: `src/cache/default_cache.rs:227`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-faf86c7f4ae0c7a865a904d0"></a>
## drop_table_entries

`function` · `datafusion_execution::cache::default_cache::DefaultCache::drop_table_entries` · datafusion-execution 55.1.0

```rust
fn drop_table_entries(&self, table_ref: &TableReference) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_execution::cache::default_cache::DefaultCache", "path": "DefaultCache"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_execution::cache::CacheKey", "path": "CacheKey"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_execution::cache::CacheValue", "path": "CacheValue"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [209, 1], "end": [296, 2], "filename": "src/cache/default_cache.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_execution::cache::Cache", "path": "Cache"}, "trait_path": "datafusion_execution::cache::Cache"}`

Source: `src/cache/default_cache.rs:264`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2fe82f5b3bdec32caa51d3e9"></a>
## get

`function` · `datafusion_execution::cache::default_cache::DefaultCache::get` · datafusion-execution 55.1.0

```rust
fn get(&self, key: &K) -> Option<V>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_execution::cache::default_cache::DefaultCache", "path": "DefaultCache"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_execution::cache::CacheKey", "path": "CacheKey"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_execution::cache::CacheValue", "path": "CacheValue"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [209, 1], "end": [296, 2], "filename": "src/cache/default_cache.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_execution::cache::Cache", "path": "Cache"}, "trait_path": "datafusion_execution::cache::Cache"}`

Source: `src/cache/default_cache.rs:210`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eddcf829223e4a1b4fc2da02"></a>
## len

`function` · `datafusion_execution::cache::default_cache::DefaultCache::len` · datafusion-execution 55.1.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_execution::cache::default_cache::DefaultCache", "path": "DefaultCache"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_execution::cache::CacheKey", "path": "CacheKey"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_execution::cache::CacheValue", "path": "CacheValue"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [209, 1], "end": [296, 2], "filename": "src/cache/default_cache.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_execution::cache::Cache", "path": "Cache"}, "trait_path": "datafusion_execution::cache::Cache"}`

Source: `src/cache/default_cache.rs:233`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4136877acc251cd1b8dedf64"></a>
## list_entries

`function` · `datafusion_execution::cache::default_cache::DefaultCache::list_entries` · datafusion-execution 55.1.0

```rust
fn list_entries(&self) -> HashMap<K, CacheEntryInfo<V>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_execution::cache::default_cache::DefaultCache", "path": "DefaultCache"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_execution::cache::CacheKey", "path": "CacheKey"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_execution::cache::CacheValue", "path": "CacheValue"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [209, 1], "end": [296, 2], "filename": "src/cache/default_cache.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_execution::cache::Cache", "path": "Cache"}, "trait_path": "datafusion_execution::cache::Cache"}`

Source: `src/cache/default_cache.rs:278`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6ecf637db99c8bdb2a6cff42"></a>
## memory_used

`function` · `datafusion_execution::cache::default_cache::DefaultCache::memory_used` · datafusion-execution 55.1.0

```rust
fn memory_used(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_execution::cache::default_cache::DefaultCache", "path": "DefaultCache"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_execution::cache::CacheKey", "path": "CacheKey"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_execution::cache::CacheValue", "path": "CacheValue"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [175, 1], "end": [207, 2], "filename": "src/cache/default_cache.rs"}, "trait": null, "trait_path": null}`

Source: `src/cache/default_cache.rs:204`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Number of bytes currently accounted for by live entries.

<a id="op-94aca05c18b1ae9a545a4cac"></a>
## name

`function` · `datafusion_execution::cache::default_cache::DefaultCache::name` · datafusion-execution 55.1.0

```rust
fn name(&self) -> String
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_execution::cache::default_cache::DefaultCache", "path": "DefaultCache"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_execution::cache::CacheKey", "path": "CacheKey"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_execution::cache::CacheValue", "path": "CacheValue"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [209, 1], "end": [296, 2], "filename": "src/cache/default_cache.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_execution::cache::Cache", "path": "Cache"}, "trait_path": "datafusion_execution::cache::Cache"}`

Source: `src/cache/default_cache.rs:242`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-74f65ac4f47d51bad83d5c24"></a>
## new

`function` · `datafusion_execution::cache::default_cache::DefaultCache::new` · datafusion-execution 55.1.0

```rust
fn new(memory_limit: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_execution::cache::default_cache::DefaultCache", "path": "DefaultCache"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_execution::cache::CacheKey", "path": "CacheKey"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_execution::cache::CacheValue", "path": "CacheValue"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [175, 1], "end": [207, 2], "filename": "src/cache/default_cache.rs"}, "trait": null, "trait_path": null}`

Source: `src/cache/default_cache.rs:177`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Create a cache with the given memory budget in bytes and no TTL.

<a id="op-2db62082dd313cd276aca2c7"></a>
## new_with_ttl

`function` · `datafusion_execution::cache::default_cache::DefaultCache::new_with_ttl` · datafusion-execution 55.1.0

```rust
fn new_with_ttl(memory_limit: usize, ttl: Option<Duration>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_execution::cache::default_cache::DefaultCache", "path": "DefaultCache"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_execution::cache::CacheKey", "path": "CacheKey"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_execution::cache::CacheValue", "path": "CacheValue"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [175, 1], "end": [207, 2], "filename": "src/cache/default_cache.rs"}, "trait": null, "trait_path": null}`

Source: `src/cache/default_cache.rs:183`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Create a cache with the given memory budget in bytes and an optional
TTL applied to every newly inserted entry.

<a id="op-6e0742067bea23868abfba22"></a>
## put

`function` · `datafusion_execution::cache::default_cache::DefaultCache::put` · datafusion-execution 55.1.0

```rust
fn put(&self, key: &K, value: V) -> Option<V>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_execution::cache::default_cache::DefaultCache", "path": "DefaultCache"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_execution::cache::CacheKey", "path": "CacheKey"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_execution::cache::CacheValue", "path": "CacheValue"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [209, 1], "end": [296, 2], "filename": "src/cache/default_cache.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_execution::cache::Cache", "path": "Cache"}, "trait_path": "datafusion_execution::cache::Cache"}`

Source: `src/cache/default_cache.rs:216`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-766e5e5826487bcae290d092"></a>
## remove

`function` · `datafusion_execution::cache::default_cache::DefaultCache::remove` · datafusion-execution 55.1.0

```rust
fn remove(&self, k: &K) -> Option<V>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_execution::cache::default_cache::DefaultCache", "path": "DefaultCache"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_execution::cache::CacheKey", "path": "CacheKey"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_execution::cache::CacheValue", "path": "CacheValue"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [209, 1], "end": [296, 2], "filename": "src/cache/default_cache.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_execution::cache::Cache", "path": "Cache"}, "trait_path": "datafusion_execution::cache::Cache"}`

Source: `src/cache/default_cache.rs:222`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ce3620159e2e6ac705a0dd4f"></a>
## update_cache_limit

`function` · `datafusion_execution::cache::default_cache::DefaultCache::update_cache_limit` · datafusion-execution 55.1.0

```rust
fn update_cache_limit(&self, limit: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_execution::cache::default_cache::DefaultCache", "path": "DefaultCache"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_execution::cache::CacheKey", "path": "CacheKey"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_execution::cache::CacheValue", "path": "CacheValue"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [209, 1], "end": [296, 2], "filename": "src/cache/default_cache.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_execution::cache::Cache", "path": "Cache"}, "trait_path": "datafusion_execution::cache::Cache"}`

Source: `src/cache/default_cache.rs:249`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-976b65e27c89e8f0280ab91a"></a>
## update_cache_ttl

`function` · `datafusion_execution::cache::default_cache::DefaultCache::update_cache_ttl` · datafusion-execution 55.1.0

```rust
fn update_cache_ttl(&self, ttl: Option<Duration>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_execution::cache::default_cache::DefaultCache", "path": "DefaultCache"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_execution::cache::CacheKey", "path": "CacheKey"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_execution::cache::CacheValue", "path": "CacheValue"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [209, 1], "end": [296, 2], "filename": "src/cache/default_cache.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_execution::cache::Cache", "path": "Cache"}, "trait_path": "datafusion_execution::cache::Cache"}`

Source: `src/cache/default_cache.rs:259`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-044ddbde8ec0fc7c1d771adf"></a>
## with_name

`function` · `datafusion_execution::cache::default_cache::DefaultCache::with_name` · datafusion-execution 55.1.0

```rust
fn with_name(self, name: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_execution::cache::default_cache::DefaultCache", "path": "DefaultCache"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_execution::cache::CacheKey", "path": "CacheKey"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_execution::cache::CacheValue", "path": "CacheValue"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [175, 1], "end": [207, 2], "filename": "src/cache/default_cache.rs"}, "trait": null, "trait_path": null}`

Source: `src/cache/default_cache.rs:192`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Override the cache name.

<a id="op-e3ed4836e1f8a09e03784a9d"></a>
## with_time_provider

`function` · `datafusion_execution::cache::default_cache::DefaultCache::with_time_provider` · datafusion-execution 55.1.0

```rust
fn with_time_provider(self, provider: Arc<dyn TimeProvider>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_execution::cache::default_cache::DefaultCache", "path": "DefaultCache"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_execution::cache::CacheKey", "path": "CacheKey"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_execution::cache::CacheValue", "path": "CacheValue"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [175, 1], "end": [207, 2], "filename": "src/cache/default_cache.rs"}, "trait": null, "trait_path": null}`

Source: `src/cache/default_cache.rs:198`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Override the time source used to stamp and check TTLs.
