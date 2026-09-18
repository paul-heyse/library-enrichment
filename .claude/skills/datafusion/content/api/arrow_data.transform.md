# `arrow_data::transform`

Crate `arrow-data` · 2 public items · structured records in [`model/arrow_data.transform.json`](../model/arrow_data.transform.json)

## Capacities

`enum` · `arrow_data::transform::Capacities`

Also reachable as `arrow::array::Capacities`

```rust
enum Capacities
```

**Variants**: `Binary`, `List`, `Struct`, `Dictionary`, `Array`

**Derives**: Clone, Debug

[Full member, field, variant and typed contracts](../operations/arrow_data.transform.Capacities.md).


Define capacities to pre-allocate for child data or data buffers.

---

## MutableArrayData

`struct` · `arrow_data::transform::MutableArrayData`

Also reachable as `arrow::array::MutableArrayData`

```rust
struct MutableArrayData<'a>
```

**Derives**: Debug

**Methods** (11)

```rust
fn extend(&mut self, index: usize, start: usize, end: usize)
fn extend_nulls(&mut self, len: usize)
fn freeze(self) -> ArrayData
fn into_builder(self) -> ArrayDataBuilder
fn is_empty(&self) -> bool
fn len(&self) -> usize
fn new(arrays: Vec<&'a ArrayData>, use_nulls: bool, capacity: usize) -> Self
fn null_count(&self) -> usize
fn try_extend(&mut self, index: usize, start: usize, end: usize) -> Result<(), ArrowError>
fn try_extend_nulls(&mut self, len: usize) -> Result<(), ArrowError>
fn with_capacities(arrays: Vec<&'a ArrayData>, use_nulls: bool, capacities: Capacities) -> Self
```

[Full member, field, variant and typed contracts](../operations/arrow_data.transform.MutableArrayData.md).


Efficiently create an [ArrayData] from one or more existing [ArrayData]s by
copying chunks.

The main use case of this struct is to perform unary operations to arrays of
arbitrary types, such as `filter` and `take`.

# Example
```
use arrow_buffer::Buffer;
use arrow_data::ArrayData;
use arrow_data::transform::MutableArrayData;
use arrow_schema::DataType;
fn i32_array(values: &[i32]) -> ArrayData {
  ArrayData::try_new(DataType::Int32, values.len(), None, 0, vec![Buffer::from_slice_ref(values)], vec![]).unwrap()
}
let arr1  = i32_array(&[1, 2, 3, 4, 5]);
let arr2  = i32_array(&[6, 7, 8, 9, 10]);
// Create a mutable array for copying values from arr1 and arr2, with a capacity for 6 elements
let capacity = 3 * std::mem::size_of::<i32>();
let mut mutable = MutableArrayData::new(vec![&arr1, &arr2], false, 10);
// Copy the first 3 elements from arr1
mutable.extend(0, 0, 3);
// Copy the last 3 elements from arr2
mutable.extend(1, 2, 5);
// Complete the MutableArrayData into a new ArrayData
let frozen = mutable.freeze();
assert_eq!(frozen, i32_array(&[1, 2, 3, 8, 9, 10]));
```

---
