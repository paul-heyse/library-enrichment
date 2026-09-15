# `datafusion_execution::cache`

Crate `datafusion-execution` · 6 public items · structured records in [`model/datafusion_execution.cache.json`](../model/datafusion_execution.cache.json)

## CacheEntryInfo

`struct` · `datafusion_execution::cache::CacheEntryInfo`

```rust
struct CacheEntryInfo<V>
```

**Fields**: `value`, `size_bytes`, `hits`, `expires`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

---

## SchemaFingerprint

`struct` · `datafusion_execution::cache::SchemaFingerprint`

Also reachable as `datafusion_execution::cache::cache_manager::SchemaFingerprint`

```rust
struct SchemaFingerprint
```

**Implements**: `datafusion_common::heap_size::DFHeapSize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (1)

```rust
fn from_schema(file_schema: &Schema) -> Self
```

**via `datafusion_common::heap_size::DFHeapSize`**

```rust
fn heap_size(&self, ctx: &mut DFHeapSizeCtx) -> usize
```

A fingerprint of the `file_schema` used to compute a file's statistics.

Captures exactly the attributes that determine the layout and meaning of
`Statistics::column_statistics`: each column's name, data type and
nullability, in order. It deliberately excludes field/schema metadata, which
cannot affect statistics — including it would needlessly fragment the cache.

---

## TableScopedPath

`struct` · `datafusion_execution::cache::TableScopedPath`

Also reachable as `datafusion_execution::cache::cache_manager::TableScopedPath`

```rust
struct TableScopedPath
```

**Fields**: `table`, `path`

**Implements**: `core::fmt::Display`, `datafusion_common::heap_size::DFHeapSize`, `datafusion_execution::cache::CacheKey`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `datafusion_common::heap_size::DFHeapSize`**

```rust
fn heap_size(&self, ctx: &mut DFHeapSizeCtx) -> usize
```

**via `datafusion_execution::cache::CacheKey`**

```rust
fn size(&self) -> usize
fn table_ref(&self) -> Option<&TableReference>
```

Each entry is scoped to its use within a specific table so that the cache
can differentiate between identical paths in different tables, and
table-level cache invalidation.

---

## Cache

`trait` · `datafusion_execution::cache::Cache`

Also reachable as `datafusion_execution::cache::cache_manager::Cache`

```rust
trait Cache<K: CacheKey, V: CacheValue>: Send + Sync
```

**Implementors** (1)

- `datafusion_execution::cache::default_cache::DefaultCache`

**Methods** (14)

```rust
fn cache_limit(&self) -> usize
fn cache_ttl(&self) -> Option<Duration>
fn clear(&self)
fn contains_key(&self, k: &K) -> bool
fn drop_table_entries(&self, table_ref: &TableReference) -> datafusion_common::Result<()>
fn get(&self, key: &K) -> Option<V>
fn is_empty(&self) -> bool
fn len(&self) -> usize
fn list_entries(&self) -> HashMap<K, CacheEntryInfo<V>>
fn name(&self) -> String
fn put(&self, key: &K, value: V) -> Option<V>
fn remove(&self, k: &K) -> Option<V>
fn update_cache_limit(&self, limit: usize)
fn update_cache_ttl(&self, _ttl: Option<Duration>)
```

Base trait for cache implementations with common operations.

This trait provides the fundamental cache operations (`get`, `put`, `remove`, etc.)
that all cache types share.

## Thread Safety

Implementations must handle their own locking via internal mutability, as methods do not
take mutable references and may be accessed by multiple concurrent queries.

---

## CacheKey

`trait` · `datafusion_execution::cache::CacheKey`

```rust
trait CacheKey: Clone + Eq + Hash + Send + Sync + Debug
```

**Implementors** (2)

- `datafusion_execution::cache::TableScopedPath`
- `object_store::path::Path`

**Methods** (2)

```rust
fn size(&self) -> usize
fn table_ref(&self) -> Option<&TableReference>
```

Key type for entries stored in a [`Cache`].

---

## CacheValue

`trait` · `datafusion_execution::cache::CacheValue`

Also reachable as `datafusion_execution::cache::cache_manager::CacheValue`

```rust
trait CacheValue: Clone + Send + Sync
```

**Implementors** (3)

- `datafusion_execution::cache::cache_manager::CachedFileList`
- `datafusion_execution::cache::cache_manager::CachedFileMetadata`
- `datafusion_execution::cache::cache_manager::CachedFileMetadataEntry`

**Methods** (1)

```rust
fn size(&self) -> usize
```

Value type for entries stored in a [`Cache`].

---
