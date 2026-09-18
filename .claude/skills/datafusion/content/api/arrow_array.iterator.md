# `arrow_array::iterator`

Crate `arrow-array` · 10 public items · structured records in [`model/arrow_array.iterator.json`](../model/arrow_array.iterator.json)

## ArrayIter

`struct` · `arrow_array::iterator::ArrayIter`

Also reachable as `arrow::array::ArrayIter`

```rust
struct ArrayIter<T: ArrayAccessor>
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn new(array: T) -> Self
```

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> Option<Self::Item>
fn nth_back(&mut self, n: usize) -> Option<Self::Item>
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn count(self) -> usize where Self: Sized
fn last(self) -> Option<Self::Item>
fn next(&mut self) -> Option<Self::Item>
fn nth(&mut self, n: usize) -> Option<Self::Item>
fn size_hint(&self) -> (usize, Option<usize>)
```

[Full member, field, variant and typed contracts](../operations/arrow_array.iterator.ArrayIter.md).


An iterator that returns Some(T) or None, that can be used on any [`ArrayAccessor`]

# Performance

[`ArrayIter`] provides an idiomatic way to iterate over an array, however, this
comes at the cost of performance. In particular the interleaved handling of
the null mask is often sub-optimal.

If performing an infallible operation, it is typically faster to perform the operation
on every index of the array, and handle the null mask separately. For [`PrimitiveArray`]
this functionality is provided by [`compute::unary`]

If performing a fallible operation, it isn't possible to perform the operation independently
of the null mask, as this might result in a spurious failure on a null index. However,
there are more efficient ways to iterate over just the non-null indices, this functionality
is provided by [`compute::try_unary`]

[`PrimitiveArray`]: crate::PrimitiveArray
[`compute::unary`]: https://docs.rs/arrow/latest/arrow/compute/fn.unary.html
[`compute::try_unary`]: https://docs.rs/arrow/latest/arrow/compute/fn.try_unary.html

---

## BooleanIter

`type_alias` · `arrow_array::iterator::BooleanIter`

Also reachable as `arrow::array::BooleanIter`

```rust
type BooleanIter<'a> = ArrayIter<&'a array::BooleanArray>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.iterator.BooleanIter.md).


an iterator that returns Some(T) or None, that can be used on any BooleanArray

---

## FixedSizeBinaryIter

`type_alias` · `arrow_array::iterator::FixedSizeBinaryIter`

Also reachable as `arrow::array::FixedSizeBinaryIter`

```rust
type FixedSizeBinaryIter<'a> = ArrayIter<&'a array::FixedSizeBinaryArray>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.iterator.FixedSizeBinaryIter.md).


an iterator that returns Some(T) or None, that can be used on any FixedSizeBinaryArray

---

## FixedSizeListIter

`type_alias` · `arrow_array::iterator::FixedSizeListIter`

Also reachable as `arrow::array::FixedSizeListIter`

```rust
type FixedSizeListIter<'a> = ArrayIter<&'a FixedSizeListArray>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.iterator.FixedSizeListIter.md).


an iterator that returns Some(T) or None, that can be used on any FixedSizeListArray

---

## GenericBinaryIter

`type_alias` · `arrow_array::iterator::GenericBinaryIter`

Also reachable as `arrow::array::GenericBinaryIter`

```rust
type GenericBinaryIter<'a, T> = ArrayIter<&'a array::GenericBinaryArray<T>>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.iterator.GenericBinaryIter.md).


an iterator that returns Some(T) or None, that can be used on any BinaryArray

---

## GenericListArrayIter

`type_alias` · `arrow_array::iterator::GenericListArrayIter`

Also reachable as `arrow::array::GenericListArrayIter`

```rust
type GenericListArrayIter<'a, O> = ArrayIter<&'a array::GenericListArray<O>>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.iterator.GenericListArrayIter.md).


an iterator that returns Some(T) or None, that can be used on any ListArray

---

## GenericListViewArrayIter

`type_alias` · `arrow_array::iterator::GenericListViewArrayIter`

Also reachable as `arrow::array::GenericListViewArrayIter`

```rust
type GenericListViewArrayIter<'a, O> = ArrayIter<&'a GenericListViewArray<O>>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.iterator.GenericListViewArrayIter.md).


an iterator that returns Some(T) or None, that can be used on any ListArray

---

## GenericStringIter

`type_alias` · `arrow_array::iterator::GenericStringIter`

Also reachable as `arrow::array::GenericStringIter`

```rust
type GenericStringIter<'a, T> = ArrayIter<&'a array::GenericStringArray<T>>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.iterator.GenericStringIter.md).


an iterator that returns Some(T) or None, that can be used on any Utf8Array

---

## MapArrayIter

`type_alias` · `arrow_array::iterator::MapArrayIter`

Also reachable as `arrow::array::MapArrayIter`

```rust
type MapArrayIter<'a> = ArrayIter<&'a MapArray>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.iterator.MapArrayIter.md).


an iterator that returns Some(T) or None, that can be used on any MapArray

---

## PrimitiveIter

`type_alias` · `arrow_array::iterator::PrimitiveIter`

Also reachable as `arrow::array::PrimitiveIter`

```rust
type PrimitiveIter<'a, T> = ArrayIter<&'a array::PrimitiveArray<T>>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.iterator.PrimitiveIter.md).


an iterator that returns Some(T) or None, that can be used on any PrimitiveArray

---
