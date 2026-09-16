# `pyrefly_util::with_hash`

Crate `pyrefly_util` · 1 public items · structured records in [`model/pyrefly_util.with_hash.json`](../model/pyrefly_util.with_hash.json)

## WithHash

`struct` · `pyrefly_util::with_hash::WithHash`

```rust
struct WithHash<K>
```

**Implements**: `core::fmt::Display`, `core::ops::deref::Deref`, `dupe::Dupe`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (10)

```rust
fn as_ref(&self) -> WithHash<&K>
fn cloned(self) -> WithHash<K> where K: Clone
fn copied(self) -> WithHash<K> where K: Copy
fn hash(&self) -> u64
fn into_key(self) -> K
fn key(&self) -> &K
fn key_mut(&mut self) -> &mut K
fn new(key: K) -> Self where K: Hash
fn new_unchecked(hash: u64, key: K) -> Self
fn owned<T>(self) -> WithHash<T> where K: Equivalent<T> + ToOwned<Owned = T>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

A key and its hash.

---
