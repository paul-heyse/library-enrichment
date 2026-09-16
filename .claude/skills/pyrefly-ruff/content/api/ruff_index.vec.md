# `ruff_index::vec`

Crate `ruff_index` · 1 public items · structured records in [`model/ruff_index.vec.json`](../model/ruff_index.vec.json)

## IndexVec

`struct` · `ruff_index::vec::IndexVec`

Also reachable as `ruff_index::IndexVec`

```rust
struct IndexVec<I, T>
```

**Fields**: `raw`

**Implements**: `core::borrow::Borrow`, `core::borrow::BorrowMut`, `core::convert::From`, `core::iter::traits::collect::Extend`, `core::iter::traits::collect::FromIterator`, `core::iter::traits::collect::IntoIterator`, `core::ops::deref::Deref`, `core::ops::deref::DerefMut`, `get_size2::GetSize`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, Send, StructuralPartialEq

**Methods** (11)

```rust
fn as_mut_slice(&mut self) -> &mut IndexSlice<I, T>
fn as_slice(&self) -> &IndexSlice<I, T>
fn drain<R: RangeBounds<usize>>(&mut self, range: R) -> impl Iterator<Item = T> + '_
fn from_raw(raw: Vec<T>) -> Self
fn new() -> Self
fn next_index(&self) -> I
fn push(&mut self, data: T) -> I
fn resize(&mut self, new_len: usize, value: T) where T: Clone
fn shrink_to_fit(&mut self)
fn truncate(&mut self, a: usize)
fn with_capacity(capacity: usize) -> Self
```

**via `core::borrow::Borrow`**

```rust
fn borrow(&self) -> &IndexSlice<I, T>
```

**via `core::borrow::BorrowMut`**

```rust
fn borrow_mut(&mut self) -> &mut IndexSlice<I, T>
```

**via `core::convert::From`**

```rust
fn from(array: [T; N]) -> Self
```

**via `core::iter::traits::collect::Extend`**

```rust
fn extend<Iter: IntoIterator<Item = T>>(&mut self, iter: Iter)
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<Iter: IntoIterator<Item = T>>(iter: Iter) -> Self
```

**via `core::iter::traits::collect::IntoIterator`**

```rust
fn into_iter(self) -> std::vec::IntoIter<T>
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

An owned sequence of `T` indexed by `I`

---
