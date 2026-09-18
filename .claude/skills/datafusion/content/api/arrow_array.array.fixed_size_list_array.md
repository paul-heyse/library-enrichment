# `arrow_array::array::fixed_size_list_array`

Crate `arrow-array` · 1 public items · structured records in [`model/arrow_array.array.fixed_size_list_array.json`](../model/arrow_array.array.fixed_size_list_array.json)

## FixedSizeListArray

`struct` · `arrow_array::array::fixed_size_list_array::FixedSizeListArray`

```rust
struct FixedSizeListArray
```

**Implements**: `arrow_array::array::Array`, `arrow_array::array::ArrayAccessor`, `arrow_array::array::ListLikeArray`, `core::convert::From`, `datafusion_common::heap_size::DFHeapSize`

**Derives**: Clone, Debug, PartialEq

**Methods** (14)

```rust
fn from_iter_primitive<T, P, I>(iter: I, length: i32) -> Self where T: ArrowPrimitiveType, P: IntoIterator<Item = Option<<T as ArrowPrimitiveType>::Native>>, I: IntoIterator<Item = Option<P>>
fn into_parts(self) -> (FieldRef, i32, ArrayRef, Option<NullBuffer>)
fn iter(&self) -> FixedSizeListIter<'_>
fn new(field: FieldRef, size: i32, values: ArrayRef, nulls: Option<NullBuffer>) -> Self
fn new_null(field: FieldRef, size: i32, len: usize) -> Self
unsafe fn new_unchecked(field: FieldRef, size: i32, values: ArrayRef, nulls: Option<NullBuffer>, len: usize) -> Self
fn slice(&self, offset: usize, len: usize) -> Self
fn try_new(field: FieldRef, size: i32, values: ArrayRef, nulls: Option<NullBuffer>) -> Result<Self, ArrowError>
fn try_new_with_length(field: FieldRef, size: i32, values: ArrayRef, nulls: Option<NullBuffer>, len: usize) -> Result<Self, ArrowError>
fn value(&self, i: usize) -> ArrayRef
const fn value_length(&self) -> i32
fn value_offset(&self, i: usize) -> i32
fn value_type(&self) -> DataType
fn values(&self) -> &ArrayRef
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

**via `arrow_array::array::ArrayAccessor`**

```rust
fn value(&self, index: usize) -> Self::Item
unsafe fn value_unchecked(&self, index: usize) -> Self::Item
```

**via `arrow_array::array::ListLikeArray`**

```rust
fn element_range(&self, index: usize) -> std::ops::Range<usize>
fn values(&self) -> &ArrayRef
```

**via `core::convert::From`**

```rust
fn from(data: ArrayData) -> Self
```

[Full member, field, variant and typed contracts](../operations/arrow_array.array.fixed_size_list_array.FixedSizeListArray.md).


An array of [fixed length lists], similar to JSON arrays
(e.g. `["A", "B"]`).

Lists are represented using a `values` child
array where each list has a fixed size of `value_length`.

Use [`FixedSizeListBuilder`] to construct a [`FixedSizeListArray`].

# Representation

A [`FixedSizeListArray`] can represent a list of values of any other
supported Arrow type. Each element of the `FixedSizeListArray` itself is
a list which may contain NULL and non-null values,
or may itself be NULL.

For example, this `FixedSizeListArray` stores lists of strings:

```text
┌─────────────┐
│    [A,B]    │
├─────────────┤
│    NULL     │
├─────────────┤
│   [C,NULL]  │
└─────────────┘
```

The `values` of this `FixedSizeListArray`s are stored in a child
[`StringArray`] where logical null values take up `values_length` slots in the array
as shown in the following diagram. The logical values
are shown on the left, and the actual `FixedSizeListArray` encoding on the right

```text
                                ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐
                                                        ┌ ─ ─ ─ ─ ─ ─ ─ ─┐
 ┌─────────────┐                │     ┌───┐               ┌───┐ ┌──────┐      │
 │   [A,B]     │                      │ 1 │             │ │ 1 │ │  A   │ │ 0
 ├─────────────┤                │     ├───┤               ├───┤ ├──────┤      │
 │    NULL     │                      │ 0 │             │ │ 1 │ │  B   │ │ 1
 ├─────────────┤                │     ├───┤               ├───┤ ├──────┤      │
 │  [C,NULL]   │                      │ 1 │             │ │ 0 │ │ ???? │ │ 2
 └─────────────┘                │     └───┘               ├───┤ ├──────┤      │
                                                        | │ 0 │ │ ???? │ │ 3
 Logical Values                 │   Validity              ├───┤ ├──────┤      │
                                    (nulls)             │ │ 1 │ │  C   │ │ 4
                                │                         ├───┤ ├──────┤      │
                                                        │ │ 0 │ │ ???? │ │ 5
                                │                         └───┘ └──────┘      │
                                                        │     Values     │
                                │   FixedSizeListArray        (Array)         │
                                                        └ ─ ─ ─ ─ ─ ─ ─ ─┘
                                └ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┘
```

# Example

```
# use std::sync::Arc;
# use arrow_array::{Array, FixedSizeListArray, Int32Array};
# use arrow_data::ArrayData;
# use arrow_schema::{DataType, Field};
# use arrow_buffer::Buffer;
// Construct a value array
let value_data = ArrayData::builder(DataType::Int32)
    .len(9)
    .add_buffer(Buffer::from_slice_ref(&[0, 1, 2, 3, 4, 5, 6, 7, 8]))
    .build()
    .unwrap();
let list_data_type = DataType::FixedSizeList(
    Arc::new(Field::new_list_field(DataType::Int32, false)),
    3,
);
let list_data = ArrayData::builder(list_data_type.clone())
    .len(3)
    .add_child_data(value_data.clone())
    .build()
    .unwrap();
let list_array = FixedSizeListArray::from(list_data);
let list0 = list_array.value(0);
let list1 = list_array.value(1);
let list2 = list_array.value(2);

assert_eq!( &[0, 1, 2], list0.as_any().downcast_ref::<Int32Array>().unwrap().values());
assert_eq!( &[3, 4, 5], list1.as_any().downcast_ref::<Int32Array>().unwrap().values());
assert_eq!( &[6, 7, 8], list2.as_any().downcast_ref::<Int32Array>().unwrap().values());
```

[`StringArray`]: crate::array::StringArray
[fixed length lists]: https://arrow.apache.org/docs/format/Columnar.html#fixed-size-list-layout

---
