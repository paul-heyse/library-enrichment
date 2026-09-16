# `pyrefly_graph::index`

Crate `pyrefly_graph` · 2 public items · structured records in [`model/pyrefly_graph.index.json`](../model/pyrefly_graph.index.json)

## Idx

`struct` · `pyrefly_graph::index::Idx`

```rust
struct Idx<K>
```

**Implements**: `dupe::Dupe`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (2)

```rust
fn idx(self) -> usize
fn new(idx: usize) -> Self
```

---

## Index

`struct` · `pyrefly_graph::index::Index`

```rust
struct Index<K>
```

**Derives**: Clone, Debug, Default

**Methods** (6)

```rust
fn idx_to_key(&self, idx: Idx<K>) -> &K
fn insert(&mut self, k: K) -> Idx<K>
fn items(&self) -> impl ExactSizeIterator<Item = (Idx<K>, &K)>
fn key_to_idx(&self, k: &K) -> Option<Idx<K>>
fn key_to_idx_hashed(&self, k: Hashed<&K>) -> Option<Idx<K>>
fn new() -> Self
```

---
