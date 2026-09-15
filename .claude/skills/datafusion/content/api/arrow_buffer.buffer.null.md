# `arrow_buffer::buffer::null`

Crate `arrow-buffer` · 1 public items · structured records in [`model/arrow_buffer.buffer.null.json`](../model/arrow_buffer.buffer.null.json)

## NullBuffer

`struct` · `arrow_buffer::buffer::null::NullBuffer`

```rust
struct NullBuffer
```

**Implements**: `core::convert::From`, `core::iter::traits::collect::FromIterator`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (26)

```rust
fn buffer(&self) -> &Buffer
fn claim(&self, pool: &dyn MemoryPool)
fn contains(&self, other: &NullBuffer) -> bool
fn expand(&self, count: usize) -> Self
fn from_unsliced_buffer(buffer: impl Into<Buffer>, len: usize) -> Option<Self>
fn inner(&self) -> &BooleanBuffer
fn into_inner(self) -> BooleanBuffer
fn is_empty(&self) -> bool
fn is_null(&self, idx: usize) -> bool
fn is_valid(&self, idx: usize) -> bool
fn iter(&self) -> BitIterator<'_>
fn len(&self) -> usize
fn new(buffer: BooleanBuffer) -> Self
fn new_null(len: usize) -> Self
unsafe fn new_unchecked(buffer: BooleanBuffer, null_count: usize) -> Self
fn new_valid(len: usize) -> Self
fn null_count(&self) -> usize
fn offset(&self) -> usize
fn shrink_to_fit(&mut self)
fn slice(&self, offset: usize, len: usize) -> Self
fn try_for_each_valid_idx<E, F: FnMut(usize) -> Result<(), E>>(&self, f: F) -> Result<(), E>
fn union(lhs: Option<&NullBuffer>, rhs: Option<&NullBuffer>) -> Option<NullBuffer>
fn union_many<'a>(nulls: impl IntoIterator<Item = Option<&'a NullBuffer>>) -> Option<NullBuffer>
fn valid_indices(&self) -> BitIndexIterator<'_>
fn valid_slices(&self) -> BitSliceIterator<'_>
fn validity(&self) -> &[u8]
```

**via `core::convert::From`**

```rust
fn from(value: &[bool; N]) -> Self
fn from(value: BooleanBuffer) -> Self
fn from(builder: BooleanBufferBuilder) -> Self
fn from(value: Vec<bool>) -> Self
fn from(value: &[bool]) -> Self
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<T: IntoIterator<Item = bool>>(iter: T) -> Self
```

A [`BooleanBuffer`] used to encode validity (null values) for Arrow arrays

In the [Arrow specification], array validity is encoded in a packed bitmask with a
`true` value indicating the corresponding slot is not null, and `false` indicating
that it is null.

# See also
* [`NullBufferBuilder`] for creating `NullBuffer`s

[Arrow specification]: https://arrow.apache.org/docs/format/Columnar.html#validity-bitmaps
[`NullBufferBuilder`]: crate::NullBufferBuilder

---
