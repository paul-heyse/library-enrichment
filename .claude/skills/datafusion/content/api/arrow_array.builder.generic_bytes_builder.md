# `arrow_array::builder::generic_bytes_builder`

Crate `arrow-array` · 5 public items · structured records in [`model/arrow_array.builder.generic_bytes_builder.json`](../model/arrow_array.builder.generic_bytes_builder.json)

## GenericByteBuilder

`struct` · `arrow_array::builder::generic_bytes_builder::GenericByteBuilder`

```rust
struct GenericByteBuilder<T: ByteArrayType>
```

**Implements**: `arrow_array::builder::ArrayBuilder`, `core::iter::traits::collect::Extend`

**Derives**: Debug, Default

**Methods** (18)

```rust
fn append_array(&mut self, array: &GenericByteArray<T>) -> Result<(), ArrowError>
fn append_null(&mut self)
fn append_nulls(&mut self, n: usize)
fn append_option(&mut self, value: Option<impl AsRef<T::Native>>)
fn append_value(&mut self, value: impl AsRef<T::Native>)
fn append_value_n(&mut self, value: impl AsRef<T::Native>, n: usize)
fn finish(&mut self) -> GenericByteArray<T>
fn finish_cloned(&self) -> GenericByteArray<T>
fn new() -> Self
unsafe fn new_from_buffer(offsets_buffer: MutableBuffer, value_buffer: MutableBuffer, null_buffer: Option<MutableBuffer>) -> Self
fn offsets_capacity(&self) -> usize
fn offsets_slice(&self) -> &[T::Offset]
fn validity_capacity(&self) -> usize
fn validity_slice(&self) -> Option<&[u8]>
fn validity_slice_mut(&mut self) -> Option<&mut [u8]>
fn values_capacity(&self) -> usize
fn values_slice(&self) -> &[u8]
fn with_capacity(item_capacity: usize, data_capacity: usize) -> Self
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

Builder for [`GenericByteArray`]

For building strings, see docs on [`GenericStringBuilder`].
For building binary, see docs on [`GenericBinaryBuilder`].

---

## BinaryLikeArrayBuilder

`trait` · `arrow_array::builder::generic_bytes_builder::BinaryLikeArrayBuilder`

```rust
trait BinaryLikeArrayBuilder: ArrayBuilder
```

**Implementors** (2)

- `arrow_array::builder::generic_bytes_builder::GenericBinaryBuilder`
- `arrow_array::builder::generic_bytes_view_builder::BinaryViewBuilder`

**Methods** (4)

```rust
fn append_null(&mut self)
fn append_value(&mut self, value: &[u8])
fn type_name() -> &'static str
fn with_capacity(capacity: usize) -> Self
```

Trait for binary-like array builders

This trait provides unified interface for builders that append binary-like data
such as [`GenericBinaryBuilder<O>`] and [`crate::builder::BinaryViewBuilder`]

---

## StringLikeArrayBuilder

`trait` · `arrow_array::builder::generic_bytes_builder::StringLikeArrayBuilder`

```rust
trait StringLikeArrayBuilder: ArrayBuilder
```

**Implementors** (2)

- `arrow_array::builder::generic_bytes_builder::GenericStringBuilder`
- `arrow_array::builder::generic_bytes_view_builder::StringViewBuilder`

**Methods** (4)

```rust
fn append_null(&mut self)
fn append_value(&mut self, value: &str)
fn type_name() -> &'static str
fn with_capacity(capacity: usize) -> Self
```

Trait for string-like array builders

This trait provides unified interface for builders that append string-like data
such as [`GenericStringBuilder<O>`] and [`crate::builder::StringViewBuilder`]

---

## GenericBinaryBuilder

`type_alias` · `arrow_array::builder::generic_bytes_builder::GenericBinaryBuilder`

```rust
type GenericBinaryBuilder<O> = GenericByteBuilder<types::GenericBinaryType<O>>
```

**Implements**: `arrow_array::builder::generic_bytes_builder::BinaryLikeArrayBuilder`

**via `arrow_array::builder::generic_bytes_builder::BinaryLikeArrayBuilder`**

```rust
fn append_null(&mut self)
fn append_value(&mut self, value: &[u8])
fn type_name() -> &'static str
fn with_capacity(capacity: usize) -> Self
```

 Array builder for [`GenericBinaryArray`][crate::GenericBinaryArray]

Values can be appended using [`GenericByteBuilder::append_value`], and nulls with
[`GenericByteBuilder::append_null`].

# Example
```
# use arrow_array::builder::GenericBinaryBuilder;
let mut builder = GenericBinaryBuilder::<i32>::new();

// Write data
builder.append_value("foo");

// Write second value
builder.append_value(&[0,1,2]);

let array = builder.finish();
// binary values
assert_eq!(array.value(0), b"foo");
assert_eq!(array.value(1), b"\x00\x01\x02");
```

# Example incrementally writing bytes with `write_bytes`

```
# use std::io::Write;
# use arrow_array::builder::GenericBinaryBuilder;
let mut builder = GenericBinaryBuilder::<i32>::new();

// Write data in multiple `write_bytes` calls
write!(builder, "foo").unwrap();
write!(builder, "bar").unwrap();
// The next call to append_value finishes the current string
// including all previously written strings.
builder.append_value("baz");

// Write second value with a single write call
write!(builder, "v2").unwrap();
// finish the value by calling append_value with an empty string
builder.append_value("");

let array = builder.finish();
assert_eq!(array.value(0), "foobarbaz".as_bytes());
assert_eq!(array.value(1), "v2".as_bytes());
```

---

## GenericStringBuilder

`type_alias` · `arrow_array::builder::generic_bytes_builder::GenericStringBuilder`

```rust
type GenericStringBuilder<O> = GenericByteBuilder<types::GenericStringType<O>>
```

**Implements**: `arrow_array::builder::generic_bytes_builder::StringLikeArrayBuilder`

**via `arrow_array::builder::generic_bytes_builder::StringLikeArrayBuilder`**

```rust
fn append_null(&mut self)
fn append_value(&mut self, value: &str)
fn type_name() -> &'static str
fn with_capacity(capacity: usize) -> Self
```

Array builder for [`GenericStringArray`][crate::GenericStringArray]

Values can be appended using [`GenericByteBuilder::append_value`], and nulls with
[`GenericByteBuilder::append_null`].

This builder also implements [`std::fmt::Write`] with any written data
included in the next appended value. This allows using [`std::fmt::Display`]
with standard Rust idioms like `write!` and `writeln!` to write data
directly to the builder without intermediate allocations.

# Example writing strings with `append_value`
```
# use arrow_array::builder::GenericStringBuilder;
let mut builder = GenericStringBuilder::<i32>::new();

// Write one string value
builder.append_value("foobarbaz");

// Write a second string
builder.append_value("v2");

let array = builder.finish();
assert_eq!(array.value(0), "foobarbaz");
assert_eq!(array.value(1), "v2");
```

# Example incrementally writing strings with `std::fmt::Write`

```
# use std::fmt::Write;
# use arrow_array::builder::GenericStringBuilder;
let mut builder = GenericStringBuilder::<i32>::new();

// Write data in multiple `write!` calls
write!(builder, "foo").unwrap();
write!(builder, "bar").unwrap();
// The next call to append_value finishes the current string
// including all previously written strings.
builder.append_value("baz");

// Write second value with a single write call
write!(builder, "v2").unwrap();
// finish the value by calling append_value with an empty string
builder.append_value("");

let array = builder.finish();
assert_eq!(array.value(0), "foobarbaz");
assert_eq!(array.value(1), "v2");
```

---
