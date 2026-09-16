# `ruff_index::slice`

Crate `ruff_index` · 1 public items · structured records in [`model/ruff_index.slice.json`](../model/ruff_index.slice.json)

## IndexSlice

`struct` · `ruff_index::slice::IndexSlice`

Also reachable as `ruff_index::IndexSlice`

```rust
struct IndexSlice<I, T>
```

**Fields**: `raw`

**Implements**: `alloc::borrow::ToOwned`, `core::ops::index::Index`, `core::ops::index::IndexMut`

**Derives**: Debug, Eq, Hash, PartialEq, Send, StructuralPartialEq

**Methods** (18)

```rust
fn binary_search(&self, value: &T) -> Result<I, I> where T: Ord
const fn empty() -> &'static Self
const fn first(&self) -> Option<&T>
const fn from_raw(raw: &[T]) -> &Self
fn from_raw_mut(raw: &mut [T]) -> &mut Self
fn get(&self, index: I) -> Option<&T>
fn get_mut(&mut self, index: I) -> Option<&mut T>
fn indices(&self) -> impl DoubleEndedIterator<Item = I> + ExactSizeIterator + Clone + 'static
const fn is_empty(&self) -> bool
fn iter(&self) -> std::slice::Iter<'_, T>
fn iter_enumerated(&self) -> impl DoubleEndedIterator<Item = (I, &T)> + ExactSizeIterator + '_
fn iter_mut(&mut self) -> std::slice::IterMut<'_, T>
fn iter_mut_enumerated(&mut self) -> impl DoubleEndedIterator<Item = (I, &mut T)> + ExactSizeIterator + '_
const fn last(&self) -> Option<&T>
fn last_index(&self) -> Option<I>
const fn last_mut(&mut self) -> Option<&mut T>
const fn len(&self) -> usize
fn swap(&mut self, a: I, b: I)
```

**via `alloc::borrow::ToOwned`**

```rust
fn clone_into(&self, target: &mut IndexVec<I, T>)
fn to_owned(&self) -> IndexVec<I, T>
```

**via `core::ops::index::Index`**

```rust
fn index(&self, range: Range<I>) -> &[T]
fn index(&self, index: I) -> &T
```

**via `core::ops::index::IndexMut`**

```rust
fn index_mut(&mut self, range: Range<I>) -> &mut [T]
fn index_mut(&mut self, index: I) -> &mut T
```

A view into contiguous `T`s, indexed by `I` rather than by `usize`.

---
