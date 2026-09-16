# `pyrefly_util::small_map1`

Crate `pyrefly_util` · 1 public items · structured records in [`model/pyrefly_util.small_map1.json`](../model/pyrefly_util.small_map1.json)

## SmallMap1

`struct` · `pyrefly_util::small_map1::SmallMap1`

```rust
struct SmallMap1<K, V>
```

**Derives**: Clone, Debug

**Methods** (7)

```rust
fn first(&self) -> (&K, &V)
fn first_mut(&mut self) -> (&K, &mut V)
fn get(&self, key: &K) -> Option<&V> where K: Hash + Eq
fn get_mut(&mut self, key: &K) -> Option<&mut V> where K: Hash + Eq
fn insert(&mut self, key: K, value: V) -> Option<V> where K: Hash + Eq
fn iter_keys(&self) -> impl Iterator<Item = &K>
fn new(key: K, value: V) -> Self
```

SmallMap but with at least one element.
If only one element is inserted, we won't ever hash it.

---
