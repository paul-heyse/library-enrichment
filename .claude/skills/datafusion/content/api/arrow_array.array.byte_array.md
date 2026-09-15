# `arrow_array::array::byte_array`

Crate `arrow-array` · 1 public items · structured records in [`model/arrow_array.array.byte_array.json`](../model/arrow_array.array.byte_array.json)

## GenericByteArray

`struct` · `arrow_array::array::byte_array::GenericByteArray`

```rust
struct GenericByteArray<T: ByteArrayType>
```

**Implements**: `arrow_array::array::Array`, `core::convert::From`, `core::iter::traits::collect::FromIterator`

**Derives**: Clone, Debug

**Methods** (27)

```rust
fn from_iter_values<Ptr, I>(iter: I) -> Self where Ptr: AsRef<T::Native>, I: IntoIterator<Item = Ptr>
fn from_opt_vec(v: Vec<Option<&[u8]>>) -> Self
fn from_vec(v: Vec<&[u8]>) -> Self
fn into_builder(self) -> Result<GenericByteBuilder<T>, Self>
fn into_parts(self) -> (OffsetBuffer<T::Offset>, Buffer, Option<NullBuffer>)
fn is_ascii(&self) -> bool
fn iter(&self) -> ArrayIter<&Self>
fn new(offsets: OffsetBuffer<T::Offset>, values: Buffer, nulls: Option<NullBuffer>) -> Self
fn new_null(len: usize) -> Self
fn new_repeated(value: impl AsRef<T::Native>, repeat_count: usize) -> Self
fn new_scalar(value: impl AsRef<T::Native>) -> Scalar<Self>
unsafe fn new_unchecked(offsets: OffsetBuffer<T::Offset>, values: Buffer, nulls: Option<NullBuffer>) -> Self
fn num_chars(&self, i: usize) -> usize
fn offsets(&self) -> &OffsetBuffer<T::Offset>
fn slice(&self, offset: usize, length: usize) -> Self
fn take_iter<'a>(&'a self, indexes: impl Iterator<Item = Option<usize>> + 'a) -> impl Iterator<Item = Option<&'a [u8]>>
fn take_iter<'a>(&'a self, indexes: impl Iterator<Item = Option<usize>> + 'a) -> impl Iterator<Item = Option<&'a str>>
unsafe fn take_iter_unchecked<'a>(&'a self, indexes: impl Iterator<Item = Option<usize>> + 'a) -> impl Iterator<Item = Option<&'a [u8]>>
unsafe fn take_iter_unchecked<'a>(&'a self, indexes: impl Iterator<Item = Option<usize>> + 'a) -> impl Iterator<Item = Option<&'a str>>
fn try_from_binary(v: GenericBinaryArray<OffsetSize>) -> Result<Self, ArrowError>
fn try_new(offsets: OffsetBuffer<T::Offset>, values: Buffer, nulls: Option<NullBuffer>) -> Result<Self, ArrowError>
fn value(&self, i: usize) -> &T::Native
fn value_data(&self) -> &[u8]
fn value_length(&self, i: usize) -> T::Offset
fn value_offsets(&self) -> &[T::Offset]
unsafe fn value_unchecked(&self, i: usize) -> &T::Native
fn values(&self) -> &Buffer
```

**via `arrow_array::array::Array`**

```rust
fn as_any(&self) -> &dyn Any
fn claim(&self, pool: &dyn arrow_buffer::MemoryPool)
fn data_type(&self) -> &DataType
fn get_array_memory_size(&self) -> usize
fn get_buffer_memory_size(&self) -> usize
fn into_data(self) -> ArrayData
fn is_empty(&self) -> bool
fn len(&self) -> usize
fn logical_null_count(&self) -> usize
fn nulls(&self) -> Option<&NullBuffer>
fn offset(&self) -> usize
fn shrink_to_fit(&mut self)
fn slice(&self, offset: usize, length: usize) -> ArrayRef
fn to_data(&self) -> ArrayData
```

**via `core::convert::From`**

```rust
fn from(data: ArrayData) -> Self
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<I: IntoIterator<Item = &'a Option<Ptr>>>(iter: I) -> Self
fn from_iter<I: IntoIterator<Item = Option<Ptr>>>(iter: I) -> Self
```

An array of [variable length byte arrays](https://arrow.apache.org/docs/format/Columnar.html#variable-size-binary-layout)

See [`StringArray`] and [`LargeStringArray`] for storing utf8 encoded string data

See [`BinaryArray`] and [`LargeBinaryArray`] for storing arbitrary bytes

# Example: From a Vec

```
# use arrow_array::{Array, GenericByteArray, types::Utf8Type};
let arr: GenericByteArray<Utf8Type> = vec!["hello", "world", ""].into();
assert_eq!(arr.value_data(), b"helloworld");
assert_eq!(arr.value_offsets(), &[0, 5, 10, 10]);
let values: Vec<_> = arr.iter().collect();
assert_eq!(values, &[Some("hello"), Some("world"), Some("")]);
```

# Example: From an optional Vec

```
# use arrow_array::{Array, GenericByteArray, types::Utf8Type};
let arr: GenericByteArray<Utf8Type> = vec![Some("hello"), Some("world"), Some(""), None].into();
assert_eq!(arr.value_data(), b"helloworld");
assert_eq!(arr.value_offsets(), &[0, 5, 10, 10, 10]);
let values: Vec<_> = arr.iter().collect();
assert_eq!(values, &[Some("hello"), Some("world"), Some(""), None]);
```

# Example: From an iterator of option

```
# use arrow_array::{Array, GenericByteArray, types::Utf8Type};
let arr: GenericByteArray<Utf8Type> = (0..5).map(|x| (x % 2 == 0).then(|| x.to_string())).collect();
let values: Vec<_> = arr.iter().collect();
assert_eq!(values, &[Some("0"), None, Some("2"), None, Some("4")]);
```

# Example: Using Builder

```
# use arrow_array::Array;
# use arrow_array::builder::GenericByteBuilder;
# use arrow_array::types::Utf8Type;
let mut builder = GenericByteBuilder::<Utf8Type>::new();
builder.append_value("hello");
builder.append_null();
builder.append_value("world");
let array = builder.finish();
let values: Vec<_> = array.iter().collect();
assert_eq!(values, &[Some("hello"), None, Some("world")]);
```

[`StringArray`]: crate::StringArray
[`LargeStringArray`]: crate::LargeStringArray
[`BinaryArray`]: crate::BinaryArray
[`LargeBinaryArray`]: crate::LargeBinaryArray

---
