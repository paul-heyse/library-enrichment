# `arrow_buffer::builder::boolean`

Crate `arrow-buffer` · 1 public items · structured records in [`model/arrow_buffer.builder.boolean.json`](../model/arrow_buffer.builder.boolean.json)

## BooleanBufferBuilder

`struct` · `arrow_buffer::builder::boolean::BooleanBufferBuilder`

Also reachable as `arrow::array::BooleanBufferBuilder`, `arrow_array::builder::BooleanBufferBuilder`

```rust
struct BooleanBufferBuilder
```

**Derives**: Debug

**Methods** (23)

```rust
fn advance(&mut self, additional: usize)
fn append(&mut self, v: bool)
fn append_buffer(&mut self, buffer: &BooleanBuffer)
fn append_n(&mut self, additional: usize, v: bool)
fn append_packed_range(&mut self, range: Range<usize>, to_set: &[u8])
fn append_slice(&mut self, slice: &[bool])
fn append_word(&mut self, word: u64, count: usize)
fn as_slice(&self) -> &[u8]
fn as_slice_mut(&mut self) -> &mut [u8]
fn build(self) -> BooleanBuffer
fn capacity(&self) -> usize
unsafe fn extend_trusted_len<I>(&mut self, iterator: I) where I: Iterator<Item = bool>
fn finish(&mut self) -> BooleanBuffer
fn finish_cloned(&self) -> BooleanBuffer
fn get_bit(&self, index: usize) -> bool
fn is_empty(&self) -> bool
fn len(&self) -> usize
fn new(capacity: usize) -> Self
fn new_from_buffer(buffer: MutableBuffer, len: usize) -> Self
fn reserve(&mut self, additional: usize)
fn resize(&mut self, len: usize)
fn set_bit(&mut self, index: usize, v: bool)
fn truncate(&mut self, len: usize)
```

 Builder for [`BooleanBuffer`]

 Builds a packed buffer of bits representing boolean values. Each bit in the
 buffer corresponds to a boolean value,

 # See Also

 * [`NullBufferBuilder`] for building [`BooleanBuffer`]s for representing nulls
 * [`BufferBuilder`] for building [`Buffer`]s

 # Example
 ```
 # use arrow_buffer::builder::BooleanBufferBuilder;
 let mut builder = BooleanBufferBuilder::new(10);
 builder.append(true);
 builder.append(false);
 builder.append_n(3, true); // append 3 trues
 let buffer = builder.build();
 assert_eq!(buffer.len(), 5); // 5 bits appended
 assert_eq!(buffer.values(), &[0b00011101_u8]); // packed bits
```

 [`BufferBuilder`]: crate::builder::BufferBuilder
 [`NullBufferBuilder`]: crate::builder::NullBufferBuilder

---
