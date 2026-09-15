# `arrow_array::array::null_array`

Crate `arrow-array` · 1 public items · structured records in [`model/arrow_array.array.null_array.json`](../model/arrow_array.array.null_array.json)

## NullArray

`struct` · `arrow_array::array::null_array::NullArray`

```rust
struct NullArray
```

**Implements**: `arrow_array::array::Array`, `core::convert::From`

**Derives**: Clone, Debug, PartialEq

**Methods** (3)

```rust
fn builder(_capacity: usize) -> NullBuilder
fn new(length: usize) -> Self
fn slice(&self, offset: usize, len: usize) -> Self
```

**via `arrow_array::array::Array`**

```rust
fn as_any(&self) -> &dyn Any
fn claim(&self, _pool: &dyn arrow_buffer::MemoryPool)
fn data_type(&self) -> &DataType
fn get_array_memory_size(&self) -> usize
fn get_buffer_memory_size(&self) -> usize
fn into_data(self) -> ArrayData
fn is_empty(&self) -> bool
fn is_nullable(&self) -> bool
fn len(&self) -> usize
fn logical_null_count(&self) -> usize
fn logical_nulls(&self) -> Option<NullBuffer>
fn nulls(&self) -> Option<&NullBuffer>
fn offset(&self) -> usize
fn slice(&self, offset: usize, length: usize) -> ArrayRef
fn to_data(&self) -> ArrayData
```

**via `core::convert::From`**

```rust
fn from(data: ArrayData) -> Self
```

An array of [null values](https://arrow.apache.org/docs/format/Columnar.html#null-layout)

A `NullArray` is a simplified array where all values are null.

# Example: Create an array

```
use arrow_array::{Array, NullArray};

let array = NullArray::new(10);

assert!(array.is_nullable());
assert_eq!(array.len(), 10);
assert_eq!(array.null_count(), 0);
assert_eq!(array.logical_null_count(), 10);
assert_eq!(array.logical_nulls().unwrap().null_count(), 10);
```

---
