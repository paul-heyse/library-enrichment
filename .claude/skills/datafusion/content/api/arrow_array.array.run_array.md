# `arrow_array::array::run_array`

Crate `arrow-array` · 6 public items · structured records in [`model/arrow_array.array.run_array.json`](../model/arrow_array.array.run_array.json)

## RunArray

`struct` · `arrow_array::array::run_array::RunArray`

```rust
struct RunArray<R: RunEndIndexType>
```

**Implements**: `arrow_array::array::Array`, `arrow_array::array::run_array::AnyRunEndArray`, `core::convert::From`, `core::iter::traits::collect::FromIterator`

**Derives**: Clone, Debug, PartialEq

**Methods** (14)

```rust
fn downcast<V: 'static>(&self) -> Option<TypedRunArray<'_, R, V>>
fn get_end_physical_index(&self) -> usize
fn get_physical_index(&self, logical_index: usize) -> usize
fn get_physical_indices<I>(&self, logical_indices: &[I]) -> Result<Vec<usize>, ArrowError> where I: ArrowNativeType
fn get_start_physical_index(&self) -> usize
fn into_parts(self) -> (DataType, RunEndBuffer<R::Native>, ArrayRef)
fn logical_len(run_ends: &PrimitiveArray<R>) -> usize
unsafe fn new_unchecked(data_type: DataType, run_ends: RunEndBuffer<R::Native>, values: ArrayRef) -> Self
fn run_ends(&self) -> &RunEndBuffer<R::Native>
fn slice(&self, offset: usize, length: usize) -> Self
fn try_new(run_ends: &PrimitiveArray<R>, values: &dyn Array) -> Result<Self, ArrowError>
fn values(&self) -> &ArrayRef
fn values_slice(&self) -> ArrayRef
fn with_values(&self, values: ArrayRef) -> Self
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

**via `arrow_array::array::run_array::AnyRunEndArray`**

```rust
fn values(&self) -> &Arc<dyn Array>
fn with_values(&self, values: ArrayRef) -> ArrayRef
```

**via `core::convert::From`**

```rust
fn from(data: ArrayData) -> Self
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<I: IntoIterator<Item = Option<&'a str>>>(iter: I) -> Self
fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Self
```

An array of [run-end encoded values].

This encoding is variation on [run-length encoding (RLE)] and is good for representing
data containing the same values repeated consecutively.

A [`RunArray`] consists of a `run_ends` buffer and a `values` array of equivalent
lengths. The `run_ends` buffer stores the indexes at which the run ends. The
`values` array stores the corresponding value of each run. The below example
illustrates how a logical array is represented by a [`RunArray`]:

```text
┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─┐
  ┌─────────────────┐  ┌─────────┐       ┌─────────────────┐
│ │        A        │  │    2    │ │     │        A        │
  ├─────────────────┤  ├─────────┤       ├─────────────────┤
│ │        D        │  │    3    │ │     │        A        │    run length of 'A' = runs_ends[0] - 0 = 2
  ├─────────────────┤  ├─────────┤       ├─────────────────┤
│ │        B        │  │    6    │ │     │        D        │    run length of 'D' = run_ends[1] - run_ends[0] = 1
  └─────────────────┘  └─────────┘       ├─────────────────┤
│        values          run_ends  │     │        B        │
                                         ├─────────────────┤
└ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─┘     │        B        │
                                         ├─────────────────┤
               RunArray                  │        B        │    run length of 'B' = run_ends[2] - run_ends[1] = 3
              length = 3                 └─────────────────┘

                                            Logical array
                                               Contents
```

[run-end encoded values]: https://arrow.apache.org/docs/format/Columnar.html#run-end-encoded-layout
[run-length encoding (RLE)]: https://en.wikipedia.org/wiki/Run-length_encoding

---

## TypedRunArray

`struct` · `arrow_array::array::run_array::TypedRunArray`

```rust
struct TypedRunArray<'a, R: RunEndIndexType, V>
```

**Implements**: `arrow_array::array::Array`, `arrow_array::array::ArrayAccessor`, `core::iter::traits::collect::IntoIterator`

**Derives**: Clone, Copy, Debug

**Methods** (3)

```rust
fn run_array(&self) -> &'a RunArray<R>
fn run_ends(&self) -> &'a RunEndBuffer<R::Native>
fn values(&self) -> &'a V
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
fn logical_null_count(&self) -> usize
fn logical_nulls(&self) -> Option<NullBuffer>
fn nulls(&self) -> Option<&NullBuffer>
fn offset(&self) -> usize
fn slice(&self, offset: usize, length: usize) -> ArrayRef
fn to_data(&self) -> ArrayData
```

**via `arrow_array::array::ArrayAccessor`**

```rust
fn value(&self, logical_index: usize) -> Self::Item
unsafe fn value_unchecked(&self, logical_index: usize) -> Self::Item
```

**via `core::iter::traits::collect::IntoIterator`**

```rust
fn into_iter(self) -> Self::IntoIter
```

A [`RunArray`] typed typed on its child values array

Implements [`ArrayAccessor`] and [`IntoIterator`] allowing fast access to its elements

```
use arrow_array::{RunArray, StringArray, types::Int32Type};

let orig = ["a", "b", "a", "b"];
let ree_array = RunArray::<Int32Type>::from_iter(orig);

// `TypedRunArray` allows you to access the values directly
let typed = ree_array.downcast::<StringArray>().unwrap();

for (maybe_val, orig) in typed.into_iter().zip(orig) {
    assert_eq!(maybe_val.unwrap(), orig)
}
```

---

## AnyRunEndArray

`trait` · `arrow_array::array::run_array::AnyRunEndArray`

```rust
trait AnyRunEndArray: Array
```

**Implementors** (1)

- `arrow_array::array::run_array::RunArray`

**Methods** (2)

```rust
fn values(&self) -> &Arc<dyn Array>
fn with_values(&self, values: ArrayRef) -> ArrayRef
```

An array that can be downcast to a [`RunArray`] of any run end type and any value type.

This can be used to efficiently implement kernels for all possible run end
types without needing to create specialized implementations for each key type.

---

## Int16RunArray

`type_alias` · `arrow_array::array::run_array::Int16RunArray`

```rust
type Int16RunArray = RunArray<types::Int16Type>
```


A [`RunArray`] with `i16` run ends

# Example: Using `collect`
```
# use arrow_array::{Array, Int16RunArray, Int16Array, StringArray};
# use std::sync::Arc;

let array: Int16RunArray = vec!["a", "a", "b", "c", "c"].into_iter().collect();
let values: Arc<dyn Array> = Arc::new(StringArray::from(vec!["a", "b", "c"]));
assert_eq!(array.run_ends().values(), &[2, 3, 5]);
assert_eq!(array.values(), &values);
```

---

## Int32RunArray

`type_alias` · `arrow_array::array::run_array::Int32RunArray`

```rust
type Int32RunArray = RunArray<types::Int32Type>
```


A [`RunArray`] with `i32` run ends

# Example: Using `collect`
```
# use arrow_array::{Array, Int32RunArray, Int32Array, StringArray};
# use std::sync::Arc;

let array: Int32RunArray = vec!["a", "a", "b", "c", "c"].into_iter().collect();
let values: Arc<dyn Array> = Arc::new(StringArray::from(vec!["a", "b", "c"]));
assert_eq!(array.run_ends().values(), &[2, 3, 5]);
assert_eq!(array.values(), &values);
```

---

## Int64RunArray

`type_alias` · `arrow_array::array::run_array::Int64RunArray`

```rust
type Int64RunArray = RunArray<types::Int64Type>
```


A [`RunArray`] with `i64` run ends

# Example: Using `collect`
```
# use arrow_array::{Array, Int64RunArray, Int64Array, StringArray};
# use std::sync::Arc;

let array: Int64RunArray = vec!["a", "a", "b", "c", "c"].into_iter().collect();
let values: Arc<dyn Array> = Arc::new(StringArray::from(vec!["a", "b", "c"]));
assert_eq!(array.run_ends().values(), &[2, 3, 5]);
assert_eq!(array.values(), &values);
```

---
