# `pyrefly_util::locked_map`

Crate `pyrefly_util` · 2 public items · structured records in [`model/pyrefly_util.locked_map.json`](../model/pyrefly_util.locked_map.json)

## IntoIter

`struct` · `pyrefly_util::locked_map::IntoIter`

```rust
struct IntoIter<K, V>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
```

Consuming iterator over entries in a `LockedMap`.

---

## LockedMap

`struct` · `pyrefly_util::locked_map::LockedMap`

```rust
struct LockedMap<K, V>
```

**Implements**: `core::iter::traits::collect::IntoIterator`

**Derives**: Debug, Default

**Methods** (9)

```rust
fn ensure(&self, key: &K, value: impl FnOnce() -> V) -> (&V, bool) where K: Dupe
fn get(&self, key: &K) -> Option<&V>
fn insert(&self, key: K, value: V) -> Option<V>
fn is_empty(&self) -> bool
fn iter_unordered(&self) -> impl Iterator<Item = (&K, &V)>
fn keys(&self) -> impl Iterator<Item = &K>
fn len(&self) -> usize
fn new() -> Self
fn values(&self) -> impl Iterator<Item = &V>
```

**via `core::iter::traits::collect::IntoIterator`**

```rust
fn into_iter(self) -> IntoIter<K, V>
```

---
