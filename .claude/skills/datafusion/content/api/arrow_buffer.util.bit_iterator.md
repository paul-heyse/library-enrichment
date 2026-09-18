# `arrow_buffer::util::bit_iterator`

Crate `arrow-buffer` · 5 public items · structured records in [`model/arrow_buffer.util.bit_iterator.json`](../model/arrow_buffer.util.bit_iterator.json)

## try_for_each_valid_idx

`function` · `arrow_buffer::util::bit_iterator::try_for_each_valid_idx`

Also reachable as `arrow::util::bit_iterator::try_for_each_valid_idx`, `arrow_data::bit_iterator::try_for_each_valid_idx`

```rust
fn try_for_each_valid_idx<E, F: FnMut(usize) -> Result<(), E>>(len: usize, offset: usize, null_count: usize, nulls: Option<&[u8]>, f: F) -> Result<(), E>
```

[Full member, field, variant and typed contracts](../operations/arrow_buffer.util.bit_iterator.try_for_each_valid_idx.md).


Calls the provided closure for each index in the provided null mask that is set,
using an adaptive strategy based on the null count

Ideally this would be encapsulated in an [`Iterator`] that would determine the optimal
strategy up front, and then yield indexes based on this.

Unfortunately, external iteration based on the resulting [`Iterator`] would match the strategy
variant on each call to [`Iterator::next`], and LLVM generally cannot eliminate this.

One solution to this might be internal iteration, e.g. [`Iterator::try_fold`], however,
it is currently [not possible] to override this for custom iterators in stable Rust.

As such this is the next best option

[not possible]: https://github.com/rust-lang/rust/issues/69595

---

## BitIndexIterator

`struct` · `arrow_buffer::util::bit_iterator::BitIndexIterator`

Also reachable as `arrow::util::bit_iterator::BitIndexIterator`, `arrow_data::bit_iterator::BitIndexIterator`

```rust
struct BitIndexIterator<'a>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Debug

**Methods** (1)

```rust
fn new(buffer: &'a [u8], offset: usize, len: usize) -> Self
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
```

[Full member, field, variant and typed contracts](../operations/arrow_buffer.util.bit_iterator.BitIndexIterator.md).


An iterator of `usize` whose index in a provided bitmask is true

This provides the best performance on most masks, apart from those which contain
large runs and therefore favour [`BitSliceIterator`]

---

## BitIndexU32Iterator

`struct` · `arrow_buffer::util::bit_iterator::BitIndexU32Iterator`

Also reachable as `arrow::util::bit_iterator::BitIndexU32Iterator`, `arrow_data::bit_iterator::BitIndexU32Iterator`

```rust
struct BitIndexU32Iterator<'a>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Debug

**Methods** (1)

```rust
fn new(buffer: &'a [u8], offset: usize, len: usize) -> Self
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<u32>
```

[Full member, field, variant and typed contracts](../operations/arrow_buffer.util.bit_iterator.BitIndexU32Iterator.md).


An iterator of u32 whose index in a provided bitmask is true
Respects arbitrary offsets and slice lead/trail padding exactly like BitIndexIterator

---

## BitIterator

`struct` · `arrow_buffer::util::bit_iterator::BitIterator`

Also reachable as `arrow::util::bit_iterator::BitIterator`, `arrow_data::bit_iterator::BitIterator`

```rust
struct BitIterator<'a>
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`

**Derives**: Clone

**Methods** (1)

```rust
fn new(buffer: &'a [u8], offset: usize, len: usize) -> Self
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
fn max(self) -> Option<Self::Item> where Self: Sized, Self::Item: Ord
fn next(&mut self) -> Option<Self::Item>
fn nth(&mut self, n: usize) -> Option<Self::Item>
fn size_hint(&self) -> (usize, Option<usize>)
```

[Full member, field, variant and typed contracts](../operations/arrow_buffer.util.bit_iterator.BitIterator.md).


Iterator over the bits within a packed bitmask

To efficiently iterate over just the set bits see [`BitIndexIterator`] and [`BitSliceIterator`]

---

## BitSliceIterator

`struct` · `arrow_buffer::util::bit_iterator::BitSliceIterator`

Also reachable as `arrow::util::bit_iterator::BitSliceIterator`, `arrow_data::bit_iterator::BitSliceIterator`

```rust
struct BitSliceIterator<'a>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Debug

**Methods** (1)

```rust
fn new(buffer: &'a [u8], offset: usize, len: usize) -> Self
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
```

[Full member, field, variant and typed contracts](../operations/arrow_buffer.util.bit_iterator.BitSliceIterator.md).


Iterator of contiguous ranges of set bits within a provided packed bitmask

Returns `(usize, usize)` each representing an interval where the corresponding
bits in the provides mask are set

the first value is the start of the range (inclusive) and the second value is the end of the range (exclusive)

---
