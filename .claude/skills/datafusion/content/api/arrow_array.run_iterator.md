# `arrow_array::run_iterator`

Crate `arrow-array` · 1 public items · structured records in [`model/arrow_array.run_iterator.json`](../model/arrow_array.run_iterator.json)

## RunArrayIter

`struct` · `arrow_array::run_iterator::RunArrayIter`

```rust
struct RunArrayIter<'a, R, V> where R: RunEndIndexType, V: Sync + Send, &'a V: ArrayAccessor, <&'a V as ArrayAccessor>::Item: Default
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`

**Derives**: Debug

**Methods** (1)

```rust
fn new(array: TypedRunArray<'a, R, V>) -> Self
```

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> Option<Self::Item>
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
fn size_hint(&self) -> (usize, Option<usize>)
```

[Full member, field, variant and typed contracts](../operations/arrow_array.run_iterator.RunArrayIter.md).


The [`RunArrayIter`] provides an idiomatic way to iterate over the run array.
It returns Some(T) if there is a value or None if the value is null.

The iterator comes with a cost as it has to iterate over three arrays to determine
the value to be returned. The run_ends array is used to determine the index of the value.
The nulls array is used to determine if the value is null and the values array is used to
get the value.

Unlike other iterators in this crate, [`RunArrayIter`] does not use [`ArrayAccessor`]
because the run array accessor does binary search to access each value which is too slow.
The run array iterator can determine the next value in constant time.

---
