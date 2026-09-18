# `arrow_buffer::buffer::immutable`

Crate `arrow-buffer` · 1 public items · structured records in [`model/arrow_buffer.buffer.immutable.json`](../model/arrow_buffer.buffer.immutable.json)

## Buffer

`struct` · `arrow_buffer::buffer::immutable::Buffer`

```rust
struct Buffer
```

**Implements**: `core::convert::From`, `core::iter::traits::collect::FromIterator`, `core::ops::deref::Deref`

**Derives**: Clone, Debug, Default, Eq, PartialEq, Send, Sync

**Methods** (25)

```rust
fn advance(&mut self, offset: usize)
fn as_ptr(&self) -> *const u8
fn as_slice(&self) -> &[u8]
fn bit_chunks(&self, offset: usize, len: usize) -> BitChunks<'_>
fn bit_slice(&self, offset: usize, len: usize) -> Self
fn capacity(&self) -> usize
fn claim(&self, pool: &dyn MemoryPool)
fn count_set_bits_offset(&self, offset: usize, len: usize) -> usize
fn data_ptr(&self) -> NonNull<u8>
unsafe fn from_custom_allocation(ptr: NonNull<u8>, len: usize, owner: Arc<dyn Allocation>) -> Self
fn from_slice_ref<U: ArrowNativeType, T: AsRef<[U]>>(items: T) -> Self
unsafe fn from_trusted_len_iter<T: ArrowNativeType, I: Iterator<Item = T>>(iterator: I) -> Self
fn from_vec<T: ArrowNativeType>(vec: Vec<T>) -> Self
fn into_mutable(self) -> Result<MutableBuffer, Self>
fn into_vec<T: ArrowNativeType>(self) -> Result<Vec<T>, Self>
fn is_empty(&self) -> bool
fn len(&self) -> usize
fn ptr_eq(&self, other: &Self) -> bool
fn ptr_offset(&self) -> usize
fn shrink_to_fit(&mut self)
fn slice(&self, offset: usize) -> Self
fn slice_with_length(&self, offset: usize, length: usize) -> Self
fn strong_count(&self) -> usize
unsafe fn try_from_trusted_len_iter<E, T: ArrowNativeType, I: Iterator<Item = Result<T, E>>>(iterator: I) -> Result<Self, E>
fn typed_data<T: ArrowNativeType>(&self) -> &[T]
```

**via `core::convert::From`**

```rust
fn from(bytes: bytes::Bytes) -> Self
fn from(value: Vec<T>) -> Self
fn from(p: [u8; N]) -> Self
fn from(buffer: MutableBuffer) -> Self
fn from(value: ScalarBuffer<T>) -> Self
fn from(p: &[u8; N]) -> Self
fn from(p: &[u8]) -> Self
fn from(value: BufferBuilder<T>) -> Self
fn from(builder: BooleanBufferBuilder) -> Self
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self
fn from_iter<I>(iter: I) -> Self where I: IntoIterator<Item = bool>
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &[u8]
```

[Full member, field, variant and typed contracts](../operations/arrow_buffer.buffer.immutable.Buffer.md).


 A contiguous memory region that can be shared with other buffers and across
 thread boundaries that stores Arrow data.

 `Buffer`s can be sliced and cloned without copying the underlying data and can
 be created from memory allocated by non-Rust sources such as C/C++.

 # Example: Create a `Buffer` from a `Vec` (without copying)
 ```
 # use arrow_buffer::Buffer;
 let vec: Vec<u32> = vec![1, 2, 3];
 let buffer = Buffer::from(vec);
 ```

 # Example: Convert a `Buffer` to a `Vec` (without copying)

 Use [`Self::into_vec`] to convert a `Buffer` back into a `Vec` if there are
 no other references and the types are aligned correctly.
 ```
 # use arrow_buffer::Buffer;
 # let vec: Vec<u32> = vec![1, 2, 3];
 # let buffer = Buffer::from(vec);
 // convert the buffer back into a Vec of u32
 // note this will fail if the buffer is shared or not aligned correctly
 let vec: Vec<u32> = buffer.into_vec().unwrap();
 ```

 # Example: Create a `Buffer` from a [`bytes::Bytes`] (without copying)

 [`bytes::Bytes`] is a common type in the Rust ecosystem for shared memory
 regions. You can create a buffer from a `Bytes` instance using the `From`
 implementation, also without copying.

 ```
 # use arrow_buffer::Buffer;
 let bytes = bytes::Bytes::from("hello");
 let buffer = Buffer::from(bytes);
```

---
