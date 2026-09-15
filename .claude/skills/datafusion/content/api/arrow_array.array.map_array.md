# `arrow_array::array::map_array`

Crate `arrow-array` · 1 public items · structured records in [`model/arrow_array.array.map_array.json`](../model/arrow_array.array.map_array.json)

## MapArray

`struct` · `arrow_array::array::map_array::MapArray`

```rust
struct MapArray
```

**Implements**: `arrow_array::array::Array`, `core::convert::From`, `datafusion_common::heap_size::DFHeapSize`

**Derives**: Clone, Debug, PartialEq

**Methods** (19)

```rust
fn entries(&self) -> &StructArray
fn entries_fields(&self) -> (&Field, &Field)
fn from_vec_of_maps<KeyArray, ValueArray, K, V>(input: Vec<Option<Vec<(K, Option<V>)>>>, ordered: bool) -> Self where KeyArray: Array + 'static, ValueArray: Array + 'static, Vec<K>: Into<KeyArray>, Vec<Option<V>>: Into<ValueArray>
fn into_parts(self) -> (FieldRef, OffsetBuffer<i32>, StructArray, Option<NullBuffer>, bool)
fn iter(&self) -> MapArrayIter<'_>
fn key_type(&self) -> &DataType
fn keys(&self) -> &ArrayRef
fn new(field: FieldRef, offsets: OffsetBuffer<i32>, entries: StructArray, nulls: Option<NullBuffer>, ordered: bool) -> Self
fn new_from_strings<'a>(keys: impl Iterator<Item = &'a str>, values: &dyn Array, entry_offsets: &[u32]) -> Result<Self, ArrowError>
unsafe fn new_unchecked(field: FieldRef, offsets: OffsetBuffer<i32>, entries: StructArray, nulls: Option<NullBuffer>, ordered: bool) -> Self
fn offsets(&self) -> &OffsetBuffer<i32>
fn slice(&self, offset: usize, length: usize) -> Self
fn try_new(field: FieldRef, offsets: OffsetBuffer<i32>, entries: StructArray, nulls: Option<NullBuffer>, ordered: bool) -> Result<Self, ArrowError>
fn value(&self, i: usize) -> StructArray
fn value_length(&self, i: usize) -> i32
fn value_offsets(&self) -> &[i32]
fn value_type(&self) -> &DataType
unsafe fn value_unchecked(&self, i: usize) -> StructArray
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

**via `core::convert::From`**

```rust
fn from(data: ArrayData) -> Self
```

An array of key-value maps

Keys should always be non-null, but values can be null.

[`MapArray`] is physically a [`ListArray`] of key values pairs stored as an `entries`
[`StructArray`] with 2 child fields.

# See also
* [`MapBuilder`](crate::builder::MapBuilder) for how to construct a [`MapArray`]
* [`Self::from_vec_of_maps`] for ergonomically creating maps for testing

---
