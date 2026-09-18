# `arrow_buffer::buffer::offset`

Crate `arrow-buffer` · 1 public items · structured records in [`model/arrow_buffer.buffer.offset.json`](../model/arrow_buffer.buffer.offset.json)

## OffsetBuffer

`struct` · `arrow_buffer::buffer::offset::OffsetBuffer`

```rust
struct OffsetBuffer<O: ArrowNativeType>
```

**Implements**: `core::convert::AsRef`, `core::convert::From`, `core::ops::deref::Deref`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (15)

```rust
fn claim(&self, pool: &dyn MemoryPool)
fn from_lengths<I>(lengths: I) -> Self where I: IntoIterator<Item = usize>
fn from_repeated_length(length: usize, n: usize) -> Self
fn has_non_empty_nulls(&self, null_buffer: Option<&NullBuffer>) -> bool
fn inner(&self) -> &ScalarBuffer<O>
fn into_inner(self) -> ScalarBuffer<O>
fn lengths(&self) -> impl ExactSizeIterator<Item = usize> + '_
fn new(buffer: ScalarBuffer<O>) -> Self
fn new_empty() -> Self
unsafe fn new_unchecked(buffer: ScalarBuffer<O>) -> Self
fn new_zeroed(len: usize) -> Self
fn ptr_eq(&self, other: &Self) -> bool
fn shrink_to_fit(&mut self)
fn slice(&self, offset: usize, len: usize) -> Self
fn subtract(self, rhs: O) -> Self where O: std::ops::Sub<Output = O> + std::cmp::PartialOrd + num_traits::CheckedSub
```

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &[T]
```

**via `core::convert::From`**

```rust
fn from(value: OffsetBufferBuilder<O>) -> Self
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

[Full member, field, variant and typed contracts](../operations/arrow_buffer.buffer.offset.OffsetBuffer.md).


A non-empty buffer of monotonically increasing, positive integers.

[`OffsetBuffer`] are used to represent ranges of offsets. An
`OffsetBuffer` of `N+1` items contains `N` such ranges. The start
offset for element `i` is `offsets[i]` and the end offset is
`offsets[i+1]`. Equal offsets represent an empty range.

# Example

This example shows how 5 distinct ranges, are represented using a
6 entry `OffsetBuffer`. The first entry `(0, 3)` represents the
three offsets `0, 1, 2`. The entry `(3,3)` represent no offsets
(e.g. an empty list).

```text
  ┌───────┐                ┌───┐
  │ (0,3) │                │ 0 │
  ├───────┤                ├───┤
  │ (3,3) │                │ 3 │
  ├───────┤                ├───┤
  │ (3,4) │                │ 3 │
  ├───────┤                ├───┤
  │ (4,5) │                │ 4 │
  ├───────┤                ├───┤
  │ (5,7) │                │ 5 │
  └───────┘                ├───┤
                           │ 7 │
                           └───┘

                       Offsets Buffer
   Logical
   Offsets

 (offsets[i],
  offsets[i+1])
```

---
