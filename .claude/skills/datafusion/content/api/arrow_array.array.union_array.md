# `arrow_array::array::union_array`

Crate `arrow-array` · 1 public items · structured records in [`model/arrow_array.array.union_array.json`](../model/arrow_array.array.union_array.json)

## UnionArray

`struct` · `arrow_array::array::union_array::UnionArray`

```rust
struct UnionArray
```

**Implements**: `arrow_array::array::Array`, `core::convert::From`

**Derives**: Clone, Debug

**Methods** (13)

```rust
fn child(&self, type_id: i8) -> &ArrayRef
fn fields(&self) -> &UnionFields
fn into_parts(self) -> (UnionFields, ScalarBuffer<i8>, Option<ScalarBuffer<i32>>, Vec<ArrayRef>)
fn is_dense(&self) -> bool
unsafe fn new_unchecked(fields: UnionFields, type_ids: ScalarBuffer<i8>, offsets: Option<ScalarBuffer<i32>>, children: Vec<ArrayRef>) -> Self
fn offsets(&self) -> Option<&ScalarBuffer<i32>>
fn slice(&self, offset: usize, length: usize) -> Self
fn try_new(fields: UnionFields, type_ids: ScalarBuffer<i8>, offsets: Option<ScalarBuffer<i32>>, children: Vec<ArrayRef>) -> Result<Self, ArrowError>
fn type_id(&self, index: usize) -> i8
fn type_ids(&self) -> &ScalarBuffer<i8>
fn type_names(&self) -> Vec<&str>
fn value(&self, i: usize) -> ArrayRef
fn value_offset(&self, index: usize) -> usize
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
fn is_nullable(&self) -> bool
fn len(&self) -> usize
fn logical_nulls(&self) -> Option<NullBuffer>
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

An array of [values of varying types](https://arrow.apache.org/docs/format/Columnar.html#union-layout)

Each slot in a [UnionArray] can have a value chosen from a number
of types.  Each of the possible types are named like the fields of
a [`StructArray`](crate::StructArray).  A `UnionArray` can
have two possible memory layouts, "dense" or "sparse".  For more
information on please see the
[specification](https://arrow.apache.org/docs/format/Columnar.html#union-layout).

[UnionBuilder](crate::builder::UnionBuilder) can be used to
create [UnionArray]'s of primitive types. `UnionArray`'s of nested
types are also supported but not via `UnionBuilder`, see the tests
for examples.

# Examples
## Create a dense UnionArray `[1, 3.2, 34]`
```
use arrow_buffer::ScalarBuffer;
use arrow_schema::*;
use std::sync::Arc;
use arrow_array::{Array, Int32Array, Float64Array, UnionArray};

let int_array = Int32Array::from(vec![1, 34]);
let float_array = Float64Array::from(vec![3.2]);
let type_ids = [0, 1, 0].into_iter().collect::<ScalarBuffer<i8>>();
let offsets = [0, 0, 1].into_iter().collect::<ScalarBuffer<i32>>();

let union_fields = [
    (0, Arc::new(Field::new("A", DataType::Int32, false))),
    (1, Arc::new(Field::new("B", DataType::Float64, false))),
].into_iter().collect::<UnionFields>();

let children = vec![
    Arc::new(int_array) as Arc<dyn Array>,
    Arc::new(float_array),
];

let array = UnionArray::try_new(
    union_fields,
    type_ids,
    Some(offsets),
    children,
).unwrap();

let value = array.value(0).as_any().downcast_ref::<Int32Array>().unwrap().value(0);
assert_eq!(1, value);

let value = array.value(1).as_any().downcast_ref::<Float64Array>().unwrap().value(0);
assert!(3.2 - value < f64::EPSILON);

let value = array.value(2).as_any().downcast_ref::<Int32Array>().unwrap().value(0);
assert_eq!(34, value);
```

## Create a sparse UnionArray `[1, 3.2, 34]`
```
use arrow_buffer::ScalarBuffer;
use arrow_schema::*;
use std::sync::Arc;
use arrow_array::{Array, Int32Array, Float64Array, UnionArray};

let int_array = Int32Array::from(vec![Some(1), None, Some(34)]);
let float_array = Float64Array::from(vec![None, Some(3.2), None]);
let type_ids = [0_i8, 1, 0].into_iter().collect::<ScalarBuffer<i8>>();

let union_fields = [
    (0, Arc::new(Field::new("A", DataType::Int32, false))),
    (1, Arc::new(Field::new("B", DataType::Float64, false))),
].into_iter().collect::<UnionFields>();

let children = vec![
    Arc::new(int_array) as Arc<dyn Array>,
    Arc::new(float_array),
];

let array = UnionArray::try_new(
    union_fields,
    type_ids,
    None,
    children,
).unwrap();

let value = array.value(0).as_any().downcast_ref::<Int32Array>().unwrap().value(0);
assert_eq!(1, value);

let value = array.value(1).as_any().downcast_ref::<Float64Array>().unwrap().value(0);
assert!(3.2 - value < f64::EPSILON);

let value = array.value(2).as_any().downcast_ref::<Int32Array>().unwrap().value(0);
assert_eq!(34, value);
```

---
