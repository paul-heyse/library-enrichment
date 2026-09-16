# `ruff_index::frozen`

Crate `ruff_index` · 1 public items · structured records in [`model/ruff_index.frozen.json`](../model/ruff_index.frozen.json)

## FrozenIndexVec

`struct` · `ruff_index::frozen::FrozenIndexVec`

Also reachable as `ruff_index::FrozenIndexVec`

```rust
struct FrozenIndexVec<I, T>
```

**Implements**: `core::convert::From`, `core::iter::traits::collect::FromIterator`, `core::ops::deref::Deref`, `core::ops::deref::DerefMut`, `get_size2::GetSize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, Send, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(vec: IndexVec<I, T>) -> Self
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<Iter: IntoIterator<Item = T>>(iter: Iter) -> Self
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

**via `core::ops::deref::DerefMut`**

```rust
fn deref_mut(&mut self) -> &mut Self::Target
```

**via `get_size2::GetSize`**

```rust
fn get_heap_size(&self) -> usize
fn get_heap_size_with_tracker<TRACKER: ::get_size2::GetSizeTracker>(&self, tracker: TRACKER) -> (usize, TRACKER)
```

A structurally immutable sequence of `T` indexed by `I`.

---
