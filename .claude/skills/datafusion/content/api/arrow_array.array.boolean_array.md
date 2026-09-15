# `arrow_array::array::boolean_array`

Crate `arrow-array` · 1 public items · structured records in [`model/arrow_array.array.boolean_array.json`](../model/arrow_array.array.boolean_array.json)

## BooleanArray

`struct` · `arrow_array::array::boolean_array::BooleanArray`

```rust
struct BooleanArray
```

**Implements**: `arrow_array::array::Array`, `core::convert::From`, `core::iter::traits::collect::FromIterator`

**Derives**: Clone, Debug, PartialEq

**Methods** (31)

```rust
fn bitwise_bin_op<F>(&self, rhs: &BooleanArray, op: F) -> BooleanArray where F: FnMut(u64, u64) -> u64
fn bitwise_bin_op_mut<F>(self, rhs: &BooleanArray, op: F) -> Result<BooleanArray, BooleanArray> where F: FnMut(u64, u64) -> u64
fn bitwise_bin_op_mut_or_clone<F>(self, rhs: &BooleanArray, op: F) -> BooleanArray where F: FnMut(u64, u64) -> u64
fn bitwise_unary<F>(&self, op: F) -> BooleanArray where F: FnMut(u64) -> u64
fn bitwise_unary_mut<F>(self, op: F) -> Result<BooleanArray, BooleanArray> where F: FnMut(u64) -> u64
fn bitwise_unary_mut_or_clone<F>(self, op: F) -> BooleanArray where F: FnMut(u64) -> u64
fn builder(capacity: usize) -> BooleanBuilder
fn false_count(&self) -> usize
fn from_binary<T: ArrayAccessor, S: ArrayAccessor, F>(left: T, right: S, op: F) -> Self where F: FnMut(T::Item, S::Item) -> bool
unsafe fn from_trusted_len_iter<I, P>(iter: I) -> Self where P: Into<BooleanAdapter>, I: ExactSizeIterator<Item = P>
fn from_unary<T: ArrayAccessor, F>(left: T, op: F) -> Self where F: FnMut(T::Item) -> bool
fn has_false(&self) -> bool
fn has_true(&self) -> bool
fn into_parts(self) -> (BooleanBuffer, Option<NullBuffer>)
fn is_empty(&self) -> bool
fn iter(&'a self) -> BooleanIter<'a>
fn len(&self) -> usize
fn new(values: BooleanBuffer, nulls: Option<NullBuffer>) -> Self
fn new_from_packed(buffer: impl Into<Buffer>, offset: usize, len: usize) -> Self
fn new_from_u8(value: &[u8]) -> Self
fn new_null(len: usize) -> Self
fn new_scalar(value: bool) -> Scalar<Self>
unsafe fn new_unchecked(values: BooleanBuffer, nulls: Option<NullBuffer>) -> Self
fn slice(&self, offset: usize, length: usize) -> Self
fn take_iter<'a>(&'a self, indexes: impl Iterator<Item = Option<usize>> + 'a) -> impl Iterator<Item = Option<bool>> + 'a
unsafe fn take_iter_unchecked<'a>(&'a self, indexes: impl Iterator<Item = Option<usize>> + 'a) -> impl Iterator<Item = Option<bool>> + 'a
fn take_n_true(self, n: usize) -> BooleanArray
fn true_count(&self) -> usize
fn value(&self, i: usize) -> bool
unsafe fn value_unchecked(&self, i: usize) -> bool
fn values(&self) -> &BooleanBuffer
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
fn from(data: Vec<bool>) -> Self
fn from(data: Vec<Option<bool>>) -> Self
fn from(values: BooleanBuffer) -> Self
fn from(data: ArrayData) -> Self
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<I: IntoIterator<Item = Ptr>>(iter: I) -> Self
```

An array of [boolean values](https://arrow.apache.org/docs/format/Columnar.html#fixed-size-primitive-layout)

# Example: From a Vec

```
# use arrow_array::{Array, BooleanArray};
let arr: BooleanArray = vec![true, true, false].into();
```

# Example: From an optional Vec

```
# use arrow_array::{Array, BooleanArray};
let arr: BooleanArray = vec![Some(true), None, Some(false)].into();
```

# Example: From an iterator

```
# use arrow_array::{Array, BooleanArray};
let arr: BooleanArray = (0..5).map(|x| (x % 2 == 0).then(|| x % 3 == 0)).collect();
let values: Vec<_> = arr.iter().collect();
assert_eq!(&values, &[Some(true), None, Some(false), None, Some(false)])
```

# Example: Using Builder

```
# use arrow_array::Array;
# use arrow_array::builder::BooleanBuilder;
let mut builder = BooleanBuilder::new();
builder.append_value(true);
builder.append_null();
builder.append_value(false);
let array = builder.finish();
let values: Vec<_> = array.iter().collect();
assert_eq!(&values, &[Some(true), None, Some(false)])
```

---
