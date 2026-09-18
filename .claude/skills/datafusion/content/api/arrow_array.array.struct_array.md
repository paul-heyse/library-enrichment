# `arrow_array::array::struct_array`

Crate `arrow-array` · 1 public items · structured records in [`model/arrow_array.array.struct_array.json`](../model/arrow_array.array.struct_array.json)

## StructArray

`struct` · `arrow_array::array::struct_array::StructArray`

```rust
struct StructArray
```

**Implements**: `arrow_array::array::Array`, `core::convert::From`, `core::convert::TryFrom`, `core::ops::index::Index`, `datafusion_common::heap_size::DFHeapSize`

**Derives**: Clone, Debug, PartialEq

**Methods** (18)

```rust
fn column(&self, pos: usize) -> &ArrayRef
fn column_by_name(&self, column_name: &str) -> Option<&ArrayRef>
fn column_names(&self) -> Vec<&str>
fn columns(&self) -> &[ArrayRef]
fn field(&self, pos: usize) -> &FieldRef
fn field_by_name(&self, field_name: &str) -> Option<&FieldRef>
fn fields(&self) -> &Fields
fn flatten(&self) -> (Fields, Vec<ArrayRef>)
fn into_parts(self) -> (Fields, Vec<ArrayRef>, Option<NullBuffer>)
fn new(fields: Fields, arrays: Vec<ArrayRef>, nulls: Option<NullBuffer>) -> Self
fn new_empty_fields(len: usize, nulls: Option<NullBuffer>) -> Self
fn new_null(fields: Fields, len: usize) -> Self
unsafe fn new_unchecked(fields: Fields, arrays: Vec<ArrayRef>, nulls: Option<NullBuffer>) -> Self
unsafe fn new_unchecked_with_length(fields: Fields, arrays: Vec<ArrayRef>, nulls: Option<NullBuffer>, len: usize) -> Self
fn num_columns(&self) -> usize
fn slice(&self, offset: usize, len: usize) -> Self
fn try_new(fields: Fields, arrays: Vec<ArrayRef>, nulls: Option<NullBuffer>) -> Result<Self, ArrowError>
fn try_new_with_length(fields: Fields, arrays: Vec<ArrayRef>, nulls: Option<NullBuffer>, len: usize) -> Result<Self, ArrowError>
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
fn from(pair: (Vec<(FieldRef, ArrayRef)>, Buffer)) -> Self
fn from(data: ArrayData) -> Self
fn from(v: Vec<(FieldRef, ArrayRef)>) -> Self
fn from(value: RecordBatch) -> Self
```

**via `core::convert::TryFrom`**

```rust
fn try_from(values: Vec<(&str, ArrayRef)>) -> Result<Self, ArrowError>
```

**via `core::ops::index::Index`**

```rust
fn index(&self, name: &str) -> &Self::Output
```

[Full member, field, variant and typed contracts](../operations/arrow_array.array.struct_array.StructArray.md).


An array of [structs](https://arrow.apache.org/docs/format/Columnar.html#struct-layout)

Each child (called *field*) is represented by a separate array.

# Comparison with [RecordBatch]

Both [`RecordBatch`] and [`StructArray`] represent a collection of columns / arrays with the
same length.

However, there are a couple of key differences:

* [`StructArray`] can be nested within other [`Array`], including itself
* [`RecordBatch`] can contain top-level metadata on its associated [`Schema`][arrow_schema::Schema]
* [`StructArray`] can contain top-level nulls, i.e. `null`
* [`RecordBatch`] can only represent nulls in its child columns, i.e. `{"field": null}`

[`StructArray`] is therefore a more general data container than [`RecordBatch`], and as such
code that needs to handle both will typically share an implementation in terms of
[`StructArray`] and convert to/from [`RecordBatch`] as necessary.

[`From`] implementations are provided to facilitate this conversion, however, converting
from a [`StructArray`] containing top-level nulls to a [`RecordBatch`] will panic, as there
is no way to preserve them.

# Example: Create an array from a vector of fields

```
use std::sync::Arc;
use arrow_array::{Array, ArrayRef, BooleanArray, Int32Array, StructArray};
use arrow_schema::{DataType, Field};

let boolean = Arc::new(BooleanArray::from(vec![false, false, true, true]));
let int = Arc::new(Int32Array::from(vec![42, 28, 19, 31]));

let struct_array = StructArray::from(vec![
    (
        Arc::new(Field::new("b", DataType::Boolean, false)),
        boolean.clone() as ArrayRef,
    ),
    (
        Arc::new(Field::new("c", DataType::Int32, false)),
        int.clone() as ArrayRef,
    ),
]);
assert_eq!(struct_array.column(0).as_ref(), boolean.as_ref());
assert_eq!(struct_array.column(1).as_ref(), int.as_ref());
assert_eq!(4, struct_array.len());
assert_eq!(0, struct_array.null_count());
assert_eq!(0, struct_array.offset());
```

---
