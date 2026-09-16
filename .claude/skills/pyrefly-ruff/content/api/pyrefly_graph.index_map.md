# `pyrefly_graph::index_map`

Crate `pyrefly_graph` · 1 public items · structured records in [`model/pyrefly_graph.index_map.json`](../model/pyrefly_graph.index_map.json)

## IndexMap

`struct` · `pyrefly_graph::index_map::IndexMap`

```rust
struct IndexMap<K, V>
```

**Derives**: Clone, Debug, Default

**Methods** (11)

```rust
fn get(&self, key: Idx<K>) -> Option<&V>
fn get_exists(&self, key: Idx<K>) -> &V
fn get_mut(&mut self, key: Idx<K>) -> Option<&mut V>
fn insert(&mut self, key: Idx<K>, value: V) -> Option<V>
fn insert_if_missing(&mut self, key: Idx<K>, f: impl Fn() -> V) -> &mut V
fn insert_once(&mut self, key: Idx<K>, value: V)
fn is_empty(&self) -> bool
fn iter(&self) -> impl Iterator<Item = (Idx<K>, &V)>
fn len(&self) -> usize
fn new() -> Self
fn reserve(&mut self, capacity: usize)
```

A mapping from `Idx<K>` to `V`.
All the `Idx` values must be obtained from the same `Index`.

---
