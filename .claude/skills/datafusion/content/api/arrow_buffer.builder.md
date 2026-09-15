# `arrow_buffer::builder`

Crate `arrow-buffer` · 1 public items · structured records in [`model/arrow_buffer.builder.json`](../model/arrow_buffer.builder.json)

## BufferBuilder

`struct` · `arrow_buffer::builder::BufferBuilder`

Also reachable as `arrow_buffer::BufferBuilder`

```rust
struct BufferBuilder<T: ArrowNativeType>
```

**Implements**: `core::convert::From`, `core::iter::traits::collect::Extend`, `core::iter::traits::collect::FromIterator`

**Derives**: Debug, Default

**Methods** (17)

```rust
fn advance(&mut self, i: usize)
fn append(&mut self, v: T)
fn append_n(&mut self, n: usize, v: T)
fn append_n_zeroed(&mut self, n: usize)
fn append_slice(&mut self, slice: &[T])
unsafe fn append_trusted_len_iter(&mut self, iter: impl IntoIterator<Item = T>)
fn as_slice(&self) -> &[T]
fn as_slice_mut(&mut self) -> &mut [T]
fn build(self) -> Buffer
fn capacity(&self) -> usize
fn finish(&mut self) -> Buffer
fn is_empty(&self) -> bool
fn len(&self) -> usize
fn new(capacity: usize) -> Self
unsafe fn new_from_buffer(buffer: MutableBuffer) -> Self
fn reserve(&mut self, n: usize)
fn truncate(&mut self, len: usize)
```

**via `core::convert::From`**

```rust
fn from(value: Vec<T>) -> Self
```

**via `core::iter::traits::collect::Extend`**

```rust
fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I)
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self
```

Builder for creating Arrow [`Buffer`] objects

A [`Buffer`] is the underlying data structure of Arrow's Arrays.

For all supported types, there are type definitions for the
generic version of `BufferBuilder<T>`, e.g. `BufferBuilder`.

**Note it is typically faster to create buffers directly from `Vec`**.
See example on [`Buffer`].

# See Also
* [`BooleanBufferBuilder`]: for packing bits in [`BooleanBuffer`]s
* [`NullBufferBuilder`]: for creating [`NullBuffer`]s of null values

[`BooleanBuffer`]: crate::BooleanBuffer
[`NullBuffer`]: crate::NullBuffer

# Example:

```
# use arrow_buffer::builder::BufferBuilder;
let mut builder = BufferBuilder::<u8>::new(100);
builder.append_slice(&[42, 43, 44]);
builder.append(45);
let buffer = builder.finish();
assert_eq!(unsafe { buffer.typed_data::<u8>() }, &[42, 43, 44, 45]);
```

---
