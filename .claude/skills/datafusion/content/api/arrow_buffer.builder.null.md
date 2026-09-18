# `arrow_buffer::builder::null`

Crate `arrow-buffer` · 1 public items · structured records in [`model/arrow_buffer.builder.null.json`](../model/arrow_buffer.builder.null.json)

## NullBufferBuilder

`struct` · `arrow_buffer::builder::null::NullBufferBuilder`

Also reachable as `arrow::array::NullBufferBuilder`, `arrow_array::builder::NullBufferBuilder`

```rust
struct NullBufferBuilder
```

**Derives**: Debug

**Methods** (21)

```rust
fn allocated_size(&self) -> usize
fn append(&mut self, not_null: bool)
fn append_buffer(&mut self, buffer: &NullBuffer)
fn append_n_non_nulls(&mut self, n: usize)
fn append_n_nulls(&mut self, n: usize)
fn append_non_null(&mut self)
fn append_null(&mut self)
fn append_slice(&mut self, slice: &[bool])
fn as_slice(&self) -> Option<&[u8]>
fn as_slice_mut(&mut self) -> Option<&mut [u8]>
fn build(self) -> Option<NullBuffer>
fn finish(&mut self) -> Option<NullBuffer>
fn finish_cloned(&self) -> Option<NullBuffer>
fn is_empty(&self) -> bool
fn is_valid(&self, index: usize) -> bool
fn len(&self) -> usize
fn new(capacity: usize) -> Self
fn new_from_buffer(buffer: MutableBuffer, len: usize) -> Self
fn new_with_len(len: usize) -> Self
fn set_bit(&mut self, index: usize, v: bool)
fn truncate(&mut self, len: usize)
```

[Full member, field, variant and typed contracts](../operations/arrow_buffer.builder.null.NullBufferBuilder.md).


Builder for creating [`NullBuffer`]s (bitmaps indicating validity/nulls).

# See also
* [`BooleanBufferBuilder`] for a lower-level bitmap builder.
* [`Self::allocated_size`] for the current memory allocated by the builder.

# Performance

This builder only materializes the buffer when null values (`false`) are
appended. If you only append non-null, (`true`) to the builder, no buffer is
allocated and [`build`](#method.build) or [`finish`](#method.finish) return
`None`.

This optimization is **very** important for the performance as it avoids
allocating memory for the null buffer when there are no nulls.

# Example
```
# use arrow_buffer::NullBufferBuilder;
let mut builder = NullBufferBuilder::new(8);
builder.append_n_non_nulls(8);
// If no non null values are appended, the null buffer is not created
let buffer = builder.finish();
assert!(buffer.is_none());
// however, if a null value is appended, the null buffer is created
let mut builder = NullBufferBuilder::new(8);
builder.append_n_non_nulls(7);
builder.append_null();
let buffer = builder.finish().unwrap();
assert_eq!(buffer.len(), 8);
assert_eq!(buffer.iter().collect::<Vec<_>>(), vec![true, true, true, true, true, true, true, false]);
```

---
