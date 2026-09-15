# `arrow_buffer::buffer::boolean`

Crate `arrow-buffer` · 1 public items · structured records in [`model/arrow_buffer.buffer.boolean.json`](../model/arrow_buffer.buffer.boolean.json)

## BooleanBuffer

`struct` · `arrow_buffer::buffer::boolean::BooleanBuffer`

```rust
struct BooleanBuffer
```

**Implements**: `core::convert::From`, `core::iter::traits::collect::FromIterator`, `core::ops::bit::BitAndAssign`, `core::ops::bit::BitOrAssign`, `core::ops::bit::BitXorAssign`

**Derives**: Clone, Debug, Eq, PartialEq

**Methods** (29)

```rust
fn bit_chunks(&self) -> BitChunks<'_>
fn claim(&self, pool: &dyn MemoryPool)
fn collect_bool<F: FnMut(usize) -> bool>(len: usize, f: F) -> Self
fn count_set_bits(&self) -> usize
fn find_nth_set_bit_position(&self, start: usize, n: usize) -> usize
fn from_bits(src: impl AsRef<[u8]>, offset_in_bits: usize, len_in_bits: usize) -> Self
fn from_bitwise_binary_op<F>(left: impl AsRef<[u8]>, left_offset_in_bits: usize, right: impl AsRef<[u8]>, right_offset_in_bits: usize, len_in_bits: usize, op: F) -> Self where F: FnMut(u64, u64) -> u64
fn from_bitwise_unary_op<F>(src: impl AsRef<[u8]>, offset_in_bits: usize, len_in_bits: usize, op: F) -> Self where F: FnMut(u64) -> u64
fn has_false(&self) -> bool
fn has_true(&self) -> bool
fn inner(&self) -> &Buffer
fn into_inner(self) -> Buffer
fn is_empty(&self) -> bool
fn iter(&self) -> BitIterator<'_>
fn len(&self) -> usize
fn new(buffer: Buffer, bit_offset: usize, bit_len: usize) -> Self
fn new_set(length: usize) -> Self
fn new_unset(length: usize) -> Self
fn offset(&self) -> usize
fn ptr_eq(&self, other: &Self) -> bool
fn set_indices(&self) -> BitIndexIterator<'_>
fn set_indices_u32(&self) -> BitIndexU32Iterator<'_>
fn set_slices(&self) -> BitSliceIterator<'_>
fn shrink_to_fit(&mut self)
fn slice(&self, offset: usize, len: usize) -> Self
fn sliced(&self) -> Buffer
fn value(&self, idx: usize) -> bool
unsafe fn value_unchecked(&self, i: usize) -> bool
fn values(&self) -> &[u8]
```

**via `core::convert::From`**

```rust
fn from(value: &[bool]) -> Self
fn from(value: Vec<bool>) -> Self
fn from(builder: BooleanBufferBuilder) -> Self
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<T: IntoIterator<Item = bool>>(iter: T) -> Self
```

**via `core::ops::bit::BitAndAssign`**

```rust
fn bitand_assign(&mut self, rhs: &BooleanBuffer)
```

**via `core::ops::bit::BitOrAssign`**

```rust
fn bitor_assign(&mut self, rhs: &BooleanBuffer)
```

**via `core::ops::bit::BitXorAssign`**

```rust
fn bitxor_assign(&mut self, rhs: &BooleanBuffer)
```

A slice-able [`Buffer`] containing bit-packed booleans

This structure represents a sequence of boolean values packed into a
byte-aligned [`Buffer`]. Both the offset and length are represented in bits.

# Layout

The values are represented as little endian bit-packed values, where the
least significant bit of each byte represents the first boolean value and
then proceeding to the most significant bit.

For example, the 10 bit bitmask `0b0111001101` has length 10, and is
represented using 2 bytes with offset 0 like this:

```text
       ┌─────────────────────────────────┐    ┌─────────────────────────────────┐
       │┌───┬───┬───┬───┬───┬───┬───┬───┐│    │┌───┬───┬───┬───┬───┬───┬───┬───┐│
       ││ 1 │ 0 │ 1 │ 1 │ 0 │ 0 │ 1 │ 1 ││    ││ 1 │ 0 │ ? │ ? │ ? │ ? │ ? │ ? ││
       │└───┴───┴───┴───┴───┴───┴───┴───┘│    │└───┴───┴───┴───┴───┴───┴───┴───┘│
bit    └─────────────────────────────────┘    └─────────────────────────────────┘
offset  0             Byte 0             7    0              Byte 1            7

        length = 10 bits, offset = 0
```

The same bitmask with length 10 and offset 3 would be represented using 2
bytes like this:

```text
      ┌─────────────────────────────────┐    ┌─────────────────────────────────┐
      │┌───┬───┬───┬───┬───┬───┬───┬───┐│    │┌───┬───┬───┬───┬───┬───┬───┬───┐│
      ││ ? │ ? │ ? │ 1 │ 0 │ 1 │ 1 │ 0 ││    ││ 0 │ 1 │ 1 │ 1 │ 0 │ ? │ ? │ ? ││
      │└───┴───┴───┴───┴───┴───┴───┴───┘│    │└───┴───┴───┴───┴───┴───┴───┴───┘│
bit   └─────────────────────────────────┘    └─────────────────────────────────┘
offset 0             Byte 0             7    0              Byte 1            7

       length = 10 bits, offset = 3
```

Note that the bits marked `?` are not logically part of the mask and may
contain either `0` or `1`

# Bitwise Operations

`BooleanBuffer` implements the standard bitwise traits for creating a new
buffer ([`BitAnd`], [`BitOr`], [`BitXor`], [`Not`]) as well as the assign variants
for updating an existing buffer in place when possible ([`BitAndAssign`],
[`BitOrAssign`], [`BitXorAssign`]).

```
# use arrow_buffer::BooleanBuffer;
let mut left = BooleanBuffer::from(&[true, false, true, true] as &[bool]);
let right = BooleanBuffer::from(&[true, true, false, true] as &[bool]);

// Create a new buffer by applying bitwise AND
let anded = &left & &right;
assert_eq!(anded, BooleanBuffer::from(&[true, false, false, true] as &[bool]));

// Update `left` in place by applying bitwise AND in place
left &= &right;
assert_eq!(left, BooleanBuffer::from(&[true, false, false, true] as &[bool]));
```

# See Also
* [`BooleanBufferBuilder`] for building [`BooleanBuffer`] instances
* [`NullBuffer`] for representing null values in Arrow arrays

[`NullBuffer`]: crate::NullBuffer

---
