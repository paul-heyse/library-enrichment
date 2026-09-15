# `arrow_array::array::fixed_size_binary_array`

Crate `arrow-array` · 1 public items · structured records in [`model/arrow_array.array.fixed_size_binary_array.json`](../model/arrow_array.array.fixed_size_binary_array.json)

## FixedSizeBinaryArray

`struct` · `arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray`

```rust
struct FixedSizeBinaryArray
```

**Implements**: `arrow_array::array::Array`, `core::convert::From`, `core::convert::TryFrom`

**Derives**: Clone, Debug, PartialEq

**Methods** (19)

```rust
fn into_parts(self) -> (i32, Buffer, Option<NullBuffer>)
fn iter(&self) -> FixedSizeBinaryIter<'_>
fn new(value_length: i32, values: Buffer, nulls: Option<NullBuffer>) -> Self
fn new_null(value_length: i32, len: usize) -> Self
fn new_scalar(value: impl AsRef<[u8]>) -> Scalar<Self>
unsafe fn new_unchecked(value_length: i32, values: Buffer, nulls: Option<NullBuffer>, len: usize) -> Self
fn slice(&self, offset: usize, len: usize) -> Self
fn try_from_iter<T, U>(iter: T) -> Result<Self, ArrowError> where T: Iterator<Item = U>, U: AsRef<[u8]>
fn try_from_sparse_iter<T, U>(iter: T) -> Result<Self, ArrowError> where T: Iterator<Item = Option<U>>, U: AsRef<[u8]>
fn try_from_sparse_iter_with_size<T, U>(iter: T, value_length: i32) -> Result<Self, ArrowError> where T: Iterator<Item = Option<U>>, U: AsRef<[u8]>
fn try_new(value_length: i32, values: Buffer, nulls: Option<NullBuffer>) -> Result<Self, ArrowError>
fn try_new_with_len(value_length: i32, values: Buffer, nulls: Option<NullBuffer>, len: usize) -> Result<Self, ArrowError>
fn value(&self, i: usize) -> &[u8]
fn value_data(&self) -> &[u8]
fn value_length(&self) -> i32
fn value_offset(&self, i: usize) -> i32
fn value_size(&self) -> usize
unsafe fn value_unchecked(&self, i: usize) -> &[u8]
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
fn from(v: FixedSizeListArray) -> Self
fn from(data: ArrayData) -> Self
```

**via `core::convert::TryFrom`**

```rust
fn try_from(v: Vec<Option<&[u8]>>) -> Result<Self, Self::Error>
fn try_from(v: Vec<&[u8]>) -> Result<Self, Self::Error>
fn try_from(v: Vec<Option<&[u8; N]>>) -> Result<Self, Self::Error>
fn try_from(v: Vec<&[u8; N]>) -> Result<Self, Self::Error>
```

An array of [fixed-size binary values](https://arrow.apache.org/docs/format/Columnar.html#fixed-size-primitive-layout)

Each element in a [`FixedSizeBinaryArray`] has `value_length` bytes, where
`value_length` is defined by the schema.

This array type is useful for storing fixed-length values such as 16-byte
UUIDs (`value_length = 16`).

# Layout

Values in a [`FixedSizeBinaryArray`] are stored contiguously in a single
buffer. The byte offset for the `i`-th element can be calculated as
`i * value_length`.

Nulls are stored in a standard optional Arrow [`NullBuffer`].

For example, a 100-value [`FixedSizeBinaryArray`] with `value_length = 12`
is shown below.

```text
┌──────────────────────────────────────────┐
│ Computed byte offsets                    │
│          ┌──────────────────────┐ ┌────┐ │
│          │┌────────────────────┐│ │    │ │
│       0  ││value 0  (12 bytes) ││ │ 1  │ │
│          │├────────────────────┤│ │    │ │
│       12 ││value 1  (12 bytes) ││ │ 0  │ │
│          │├────────────────────┤│ │    │ │
│       24 ││value 2  (12 bytes) ││ │ 1  │ │
│          │└────────────────────┘│ │    │ │
│          │         ...          │ │... │ │
│          │┌───────────────────┐ │ │    │ │
│     1188 ││value 99 (12 bytes)│ │ │ 1  │ │
│          │└───────────────────┘ │ │    │ │
│          └──────────────────────┘ └────┘ │
│           value_data              nulls  │
└──────────────────────────────────────────┘
```

# Examples

Create an array from an iterable argument of byte slices.

```
   use arrow_array::{Array, FixedSizeBinaryArray};
   let input_arg = vec![ vec![1, 2], vec![3, 4], vec![5, 6] ];
   let arr = FixedSizeBinaryArray::try_from_iter(input_arg.into_iter()).unwrap();

   assert_eq!(3, arr.len());

```
Create an array from an iterable argument of sparse byte slices.
Sparsity means that the input argument can contain `None` items.
```
   use arrow_array::{Array, FixedSizeBinaryArray};
   let input_arg = vec![ None, Some(vec![7, 8]), Some(vec![9, 10]), None, Some(vec![13, 14]) ];
   let arr = FixedSizeBinaryArray::try_from_sparse_iter_with_size(input_arg.into_iter(), 2).unwrap();
   assert_eq!(5, arr.len())

```

---
