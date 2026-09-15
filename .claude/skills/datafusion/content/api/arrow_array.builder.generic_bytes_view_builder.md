# `arrow_array::builder::generic_bytes_view_builder`

Crate `arrow-array` · 4 public items · structured records in [`model/arrow_array.builder.generic_bytes_view_builder.json`](../model/arrow_array.builder.generic_bytes_view_builder.json)

## make_view

`function` · `arrow_array::builder::generic_bytes_view_builder::make_view`

```rust
fn make_view(data: &[u8], block_id: u32, offset: u32) -> u128
```

Create a view based on the given data, block id and offset.

Note that the code below is carefully examined with x86_64 assembly code: <https://godbolt.org/z/685YPsd5G>
The goal is to avoid calling into `ptr::copy_non_interleave`, which makes function call (i.e., not inlined),
which slows down things.

---

## GenericByteViewBuilder

`struct` · `arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder`

```rust
struct GenericByteViewBuilder<T: ByteViewType + ?Sized>
```

**Implements**: `arrow_array::builder::ArrayBuilder`, `core::iter::traits::collect::Extend`

**Derives**: Debug, Default

**Methods** (19)

```rust
fn allocated_size(&self) -> usize
fn append_array(&mut self, array: &GenericByteViewArray<T>)
fn append_block(&mut self, buffer: Buffer) -> u32
fn append_null(&mut self)
fn append_option(&mut self, value: Option<impl AsRef<T::Native>>)
fn append_value(&mut self, value: impl AsRef<T::Native>)
unsafe fn append_view_unchecked(&mut self, block: u32, offset: u32, len: u32)
fn finish(&mut self) -> GenericByteViewArray<T>
fn finish_cloned(&self) -> GenericByteViewArray<T>
fn get_value(&self, index: usize) -> &[u8]
fn new() -> Self
fn try_append_value(&mut self, value: impl AsRef<T::Native>) -> Result<(), ArrowError>
fn try_append_value_n(&mut self, value: impl AsRef<T::Native>, n: usize) -> Result<(), ArrowError>
fn try_append_view(&mut self, block: u32, offset: u32, len: u32) -> Result<(), ArrowError>
fn validity_slice(&self) -> Option<&[u8]>
fn with_capacity(capacity: usize) -> Self
fn with_deduplicate_strings(self) -> Self
fn with_fixed_block_size(self, block_size: u32) -> Self
fn with_max_deduplication_len(self, max_deduplication_len: u32) -> Self
```

**via `arrow_array::builder::ArrayBuilder`**

```rust
fn as_any(&self) -> &dyn Any
fn as_any_mut(&mut self) -> &mut dyn Any
fn finish(&mut self) -> ArrayRef
fn finish_cloned(&self) -> ArrayRef
fn into_box_any(Box<self>) -> Box<dyn Any>
fn len(&self) -> usize
```

**via `core::iter::traits::collect::Extend`**

```rust
fn extend<I: IntoIterator<Item = Option<V>>>(&mut self, iter: I)
```

A builder for [`GenericByteViewArray`]

A [`GenericByteViewArray`] consists of a list of data blocks containing string data,
and a list of views into those buffers.

See examples on [`StringViewBuilder`] and [`BinaryViewBuilder`]

This builder can be used in two ways

# Append Values

To avoid bump allocating, this builder allocates data in fixed size blocks, configurable
using [`GenericByteViewBuilder::with_fixed_block_size`]. [`GenericByteViewBuilder::append_value`]
writes values larger than [`MAX_INLINE_VIEW_LEN`] bytes to the current in-progress block, with values smaller
than [`MAX_INLINE_VIEW_LEN`] bytes inlined into the views. If a value is appended that will not fit in the
in-progress block, it will be closed, and a new block of sufficient size allocated

# Append Views

Some use-cases may wish to reuse an existing allocation containing string data, for example,
when parsing data from a parquet data page. In such a case entire blocks can be appended
using [`GenericByteViewBuilder::append_block`] and then views into this block appended
using [`GenericByteViewBuilder::try_append_view`]

---

## BinaryViewBuilder

`type_alias` · `arrow_array::builder::generic_bytes_view_builder::BinaryViewBuilder`

```rust
type BinaryViewBuilder = GenericByteViewBuilder<types::BinaryViewType>
```

**Implements**: `arrow_array::builder::generic_bytes_builder::BinaryLikeArrayBuilder`

**via `arrow_array::builder::generic_bytes_builder::BinaryLikeArrayBuilder`**

```rust
fn append_null(&mut self)
fn append_value(&mut self, value: &[u8])
fn type_name() -> &'static str
fn with_capacity(capacity: usize) -> Self
```

 Array builder for [`BinaryViewArray`][crate::BinaryViewArray]

Values can be appended using [`GenericByteViewBuilder::append_value`], and nulls with
[`GenericByteViewBuilder::append_null`] as normal.

# Example
```
# use arrow_array::builder::BinaryViewBuilder;
use arrow_array::BinaryViewArray;
let mut builder = BinaryViewBuilder::new();
builder.append_value("hello");
builder.append_null();
builder.append_value("world");
let array = builder.finish();

let expected: Vec<Option<&[u8]>> = vec![Some(b"hello"), None, Some(b"world")];
let actual: Vec<_> = array.iter().collect();
assert_eq!(expected, actual);
```

---

## StringViewBuilder

`type_alias` · `arrow_array::builder::generic_bytes_view_builder::StringViewBuilder`

```rust
type StringViewBuilder = GenericByteViewBuilder<types::StringViewType>
```

**Implements**: `arrow_array::builder::generic_bytes_builder::StringLikeArrayBuilder`

**via `arrow_array::builder::generic_bytes_builder::StringLikeArrayBuilder`**

```rust
fn append_null(&mut self)
fn append_value(&mut self, value: &str)
fn type_name() -> &'static str
fn with_capacity(capacity: usize) -> Self
```

Array builder for [`StringViewArray`][crate::StringViewArray]

Values can be appended using [`GenericByteViewBuilder::append_value`], and nulls with
[`GenericByteViewBuilder::append_null`] as normal.

# Example
```
# use arrow_array::builder::StringViewBuilder;
# use arrow_array::StringViewArray;
let mut builder = StringViewBuilder::new();
builder.append_value("hello");
builder.append_null();
builder.append_value("world");
let array = builder.finish();

let expected = vec![Some("hello"), None, Some("world")];
let actual: Vec<_> = array.iter().collect();
assert_eq!(expected, actual);
```

---
