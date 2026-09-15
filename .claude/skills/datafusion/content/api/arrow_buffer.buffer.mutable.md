# `arrow_buffer::buffer::mutable`

Crate `arrow-buffer` · 1 public items · structured records in [`model/arrow_buffer.buffer.mutable.json`](../model/arrow_buffer.buffer.mutable.json)

## MutableBuffer

`struct` · `arrow_buffer::buffer::mutable::MutableBuffer`

```rust
struct MutableBuffer
```

**Implements**: `core::convert::From`, `core::iter::traits::collect::Extend`, `core::iter::traits::collect::FromIterator`, `core::ops::deref::Deref`, `core::ops::deref::DerefMut`, `core::ops::drop::Drop`

**Derives**: Debug, Default, PartialEq, Send, Sync

**Methods** (32)

```rust
fn as_mut_ptr(&mut self) -> *mut u8
const fn as_ptr(&self) -> *const u8
fn as_slice(&self) -> &[u8]
fn as_slice_mut(&mut self) -> &mut [u8]
const fn capacity(&self) -> usize
fn claim(&self, pool: &dyn MemoryPool)
fn clear(&mut self)
fn collect_bool<F: FnMut(usize) -> bool>(len: usize, f: F) -> Self
unsafe fn extend_bool_trusted_len<I: Iterator<Item = bool>>(&mut self, iter: I, offset: usize)
fn extend_from_slice<T: ArrowNativeType>(&mut self, items: &[T])
fn extend_zeros(&mut self, additional: usize)
fn from_len_zeroed(len: usize) -> Self
unsafe fn from_trusted_len_iter<T: ArrowNativeType, I: Iterator<Item = T>>(iterator: I) -> Self
unsafe fn from_trusted_len_iter_bool<I: Iterator<Item = bool>>(iterator: I) -> Self
const fn is_empty(&self) -> bool
const fn len(&self) -> usize
fn new(capacity: usize) -> Self
fn new_null(len: usize) -> Self
fn push<T: ToByteSlice>(&mut self, item: T)
unsafe fn push_unchecked<T: ToByteSlice>(&mut self, item: T)
fn repeat_slice_n_times<T: ArrowNativeType>(&mut self, slice_to_repeat: &[T], repeat_count: usize)
fn reserve(&mut self, additional: usize)
fn resize(&mut self, new_len: usize, value: u8)
unsafe fn set_len(&mut self, len: usize)
fn set_null_bits(&mut self, start: usize, count: usize)
fn shrink_to_fit(&mut self)
fn truncate(&mut self, len: usize)
unsafe fn try_from_trusted_len_iter<E, T: ArrowNativeType, I: Iterator<Item = Result<T, E>>>(iterator: I) -> Result<Self, E>
fn typed_data<T: ArrowNativeType>(&self) -> &[T]
fn typed_data_mut<T: ArrowNativeType>(&mut self) -> &mut [T]
fn with_bitset(self, end: usize, val: bool) -> Self
fn with_capacity(capacity: usize) -> Self
```

**via `core::convert::From`**

```rust
fn from(value: Vec<T>) -> Self
```

**via `core::iter::traits::collect::Extend`**

```rust
fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T)
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<I>(iter: I) -> Self where I: IntoIterator<Item = bool>
fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &[u8]
```

**via `core::ops::deref::DerefMut`**

```rust
fn deref_mut(&mut self) -> &mut [u8]
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

A [`MutableBuffer`] is a wrapper over memory regions, used to build
[`Buffer`]s out of items or slices of items.

[`Buffer`]s created from [`MutableBuffer`] (via `into`) are guaranteed to be
aligned along cache lines and in multiples of 64 bytes.

Use [MutableBuffer::push] to insert an item, [MutableBuffer::extend_from_slice]
to insert many items, and `into` to convert it to [`Buffer`]. For typed data,
it is often more efficient to use [`Vec`] and convert it to [`Buffer`] rather
than using [`MutableBuffer`] (see examples below).

# See Also
* For a safe, strongly typed API consider using [`Vec`] and [`ScalarBuffer`](crate::ScalarBuffer)
* To apply bitwise operations, see [`apply_bitwise_binary_op`] and [`apply_bitwise_unary_op`]

[`apply_bitwise_binary_op`]: crate::bit_util::apply_bitwise_binary_op
[`apply_bitwise_unary_op`]: crate::bit_util::apply_bitwise_unary_op

# Example: Creating a [`Buffer`] from a [`MutableBuffer`]
```
# use arrow_buffer::buffer::{Buffer, MutableBuffer};
let mut buffer = MutableBuffer::new(0);
buffer.push(256u32);
buffer.extend_from_slice(&[1u32]);
let buffer = Buffer::from(buffer);
assert_eq!(buffer.as_slice(), &[0u8, 1, 0, 0, 1, 0, 0, 0])
```

The same can be achieved more efficiently by using a `Vec<u32>`
```
# use arrow_buffer::buffer::Buffer;
let mut vec = Vec::new();
vec.push(256u32);
vec.extend_from_slice(&[1u32]);
let buffer = Buffer::from(vec);
assert_eq!(buffer.as_slice(), &[0u8, 1, 0, 0, 1, 0, 0, 0]);
```

# Example: Creating a [`MutableBuffer`] from a `Vec<T>`
```
# use arrow_buffer::buffer::MutableBuffer;
let vec = vec![1u32, 2, 3];
let mutable_buffer = MutableBuffer::from(vec); // reuses the allocation from vec
assert_eq!(mutable_buffer.len(), 12); // 3 * 4 bytes
```

# Example: Creating a [`MutableBuffer`] from a [`Buffer`]
```
# use arrow_buffer::buffer::{Buffer, MutableBuffer};
let buffer: Buffer = Buffer::from(&[1u8, 2, 3, 4][..]);
// Only possible to convert a Buffer into a MutableBuffer if uniquely owned
// (i.e., there are no other references to it).
let mut mutable_buffer = match buffer.into_mutable() {
   Ok(mutable) => mutable,
   Err(orig_buffer) => {
     panic!("buffer was not uniquely owned");
   }
};
mutable_buffer.push(5u8);
let buffer = Buffer::from(mutable_buffer);
assert_eq!(buffer.as_slice(), &[1u8, 2, 3, 4, 5])
```

---
