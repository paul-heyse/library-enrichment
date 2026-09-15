# `arrow_buffer::buffer::scalar`

Crate `arrow-buffer` · 1 public items · structured records in [`model/arrow_buffer.buffer.scalar.json`](../model/arrow_buffer.buffer.scalar.json)

## ScalarBuffer

`struct` · `arrow_buffer::buffer::scalar::ScalarBuffer`

```rust
struct ScalarBuffer<T: ArrowNativeType>
```

**Implements**: `core::convert::AsRef`, `core::convert::From`, `core::iter::traits::collect::FromIterator`, `core::ops::deref::Deref`

**Derives**: Clone, Debug, Default, Eq, PartialEq

**Methods** (10)

```rust
fn claim(&self, pool: &dyn MemoryPool)
fn inner(&self) -> &Buffer
fn into_inner(self) -> Buffer
fn is_empty(&self) -> bool
fn len(&self) -> usize
fn new(buffer: Buffer, offset: usize, len: usize) -> Self
unsafe fn new_unchecked(buffer: Buffer) -> Self
fn ptr_eq(&self, other: &Self) -> bool
fn shrink_to_fit(&mut self)
fn slice(&self, offset: usize, len: usize) -> Self
```

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &[T]
```

**via `core::convert::From`**

```rust
fn from(value: OffsetBuffer<T>) -> Self
fn from(value: BufferBuilder<T>) -> Self
fn from(value: Vec<T>) -> Self
fn from(value: MutableBuffer) -> Self
fn from(buffer: Buffer) -> Self
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

A strongly-typed [`Buffer`] supporting zero-copy cloning and slicing

The easiest way to think about `ScalarBuffer<T>` is being equivalent to a `Arc<Vec<T>>`,
with the following differences:

- slicing and cloning is O(1).
- support for external allocated memory (e.g. via FFI).

See [`Buffer`] for more low-level memory management details.

# Example: Convert to/from Vec (without copies)

(See [`Buffer::from_vec`] and [`Buffer::into_vec`] for a lower level API)
```
# use arrow_buffer::ScalarBuffer;
// Zero-copy conversion from Vec
let buffer = ScalarBuffer::from(vec![1, 2, 3]);
assert_eq!(&buffer, &[1, 2, 3]);
// convert the buffer back to Vec without copy assuming:
// 1. the inner buffer is not sliced
// 2. the inner buffer uses standard allocation
// 3. there are no other references to the inner buffer
let vec: Vec<i32> = buffer.into();
assert_eq!(&vec, &[1, 2, 3]);
```

# Example: Zero copy slicing
```
# use arrow_buffer::ScalarBuffer;
let buffer = ScalarBuffer::from(vec![1, 2, 3]);
assert_eq!(&buffer, &[1, 2, 3]);
// Zero-copy slicing
let sliced = buffer.slice(1, 2);
assert_eq!(&sliced, &[2, 3]);
// Original buffer is unchanged
assert_eq!(&buffer, &[1, 2, 3]);
// converting the sliced buffer back to Vec incurs a copy
let vec: Vec<i32> = sliced.into();
```

---
