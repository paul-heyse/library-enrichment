# `arrow_array::array::Array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.Array.json).

<a id="op-cabb9fba5e3e968cee823d21"></a>
## Array

`trait` · `arrow_array::array::Array` · arrow-array 59.3.0

```rust
unsafe trait Array: std::fmt::Debug + Send + Sync
```

Source: `src/array/mod.rs:100`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

An array in the [Arrow Columnar Format](https://arrow.apache.org/docs/format/Columnar.html)

# Safety

Implementations of this trait must ensure that all methods implementations comply with
the Arrow specification. No safety guards are placed and failing to comply with it can
translate into panics or undefined behavior. For example, a value computed based on `len`
may be used as a direct index into memory regions without checks.

Note that it is likely impossible to correctly implement the trait for a
third party type, as substantial arrow-rs functionality is based on the
return values of [`Array::data_type`](../operations/arrow_array.array.Array.md#op-005cf83513cfb4acafff8b2d) and third party types cannot extend
the [`DataType`](../operations/arrow_schema.datatype.DataType.md#op-bf69df5b14436e006d3a531c) enum. So any code that attempts casting based on data type
(including internal arrow library code) risks a panic or undefined behavior.
See [this discussion] for more details.

This trait might be sealed in the future. Use at your own risk.

[this discussion]: https://github.com/apache/arrow-rs/pull/9234#pullrequestreview-3708950936

<a id="op-4eeed326f6b4eb03d3f2c4d9"></a>
## as_any

`function` · `arrow_array::array::Array::as_any` · arrow-array 59.3.0

```rust
fn as_any(&self) -> &dyn Any
```

Source: `src/array/mod.rs:123`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the array as [`Any`] so that it can be
downcasted to a specific implementation.

# Example:

```
# use std::sync::Arc;
# use arrow_array::{Int32Array, RecordBatch};
# use arrow_schema::{Schema, Field, DataType, ArrowError};

let id = Int32Array::from(vec![1, 2, 3, 4, 5]);
let batch = RecordBatch::try_new(
    Arc::new(Schema::new(vec![Field::new("id", DataType::Int32, false)])),
    vec![Arc::new(id)]
).unwrap();

let int32array = batch
    .column(0)
    .as_any()
    .downcast_ref::<Int32Array>()
    .expect("Failed to downcast");
```

Unresolved upstream links (retained, not inferred): ``Any``.

<a id="op-c1a11e99277502289a9534ee"></a>
## claim

`function` · `arrow_array::array::Array::claim` · arrow-array 59.3.0

```rust
fn claim(&self, pool: &dyn arrow_buffer::MemoryPool)
```

Source: `src/array/mod.rs:423`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Claim memory used by this array in the provided memory pool.

This recursively claims memory for:
- All data buffers in this array
- All child arrays (for nested types like List, Struct, etc.)
- The null bitmap buffer if present

This method guarantees that the memory pool will only compute occupied memory
exactly once. For example, if this array is derived from operations like `slice`,
calling `claim` on it would not change the memory pool's usage if the underlying buffers
are already counted before.

# Example
```
# use arrow_array::{Int32Array, Array};
# use arrow_buffer::TrackingMemoryPool;
# use arrow_buffer::MemoryPool;

let pool = TrackingMemoryPool::default();

let small_array = Int32Array::from(vec![1, 2, 3, 4, 5]);
let small_array_size = small_array.get_buffer_memory_size();

// Claim the array's memory in the pool
small_array.claim(&pool);

// Create and claim slices of `small_array`; should not increase memory usage
let slice1 = small_array.slice(0, 2);
let slice2 = small_array.slice(2, 2);
slice1.claim(&pool);
slice2.claim(&pool);

assert_eq!(pool.used(), small_array_size);

// Create a `large_array` which does not derive from the original `small_array`

let large_array = Int32Array::from((0..1000).collect::<Vec<i32>>());
let large_array_size = large_array.get_buffer_memory_size();

large_array.claim(&pool);

// Trying to claim more than once is a no-op
large_array.claim(&pool);
large_array.claim(&pool);

assert_eq!(pool.used(), small_array_size + large_array_size);

let sum_of_all_sizes = small_array_size + large_array_size + slice1.get_buffer_memory_size() + slice2.get_buffer_memory_size();

// `get_buffer_memory_size` works independently of the memory pool, so a sum of all the
// arrays in scope will always be >= the memory used reported by the memory pool.
assert_ne!(pool.used(), sum_of_all_sizes);

// Until the final claim is dropped the buffer size remains accounted for
drop(small_array);
drop(slice1);

assert_eq!(pool.used(), small_array_size + large_array_size);

// Dropping this finally releases the buffer that was backing `small_array`
drop(slice2);

assert_eq!(pool.used(), large_array_size);
```

<a id="op-005cf83513cfb4acafff8b2d"></a>
## data_type

`function` · `arrow_array::array::Array::data_type` · arrow-array 59.3.0

```rust
fn data_type(&self) -> &DataType
```

Source: `src/array/mod.rs:145`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a reference to the [`DataType`](../operations/arrow_schema.datatype.DataType.md#op-bf69df5b14436e006d3a531c) of this array.

# Example:

```
use arrow_schema::DataType;
use arrow_array::{Array, Int32Array};

let array = Int32Array::from(vec![1, 2, 3, 4, 5]);

assert_eq!(*array.data_type(), DataType::Int32);
```

<a id="op-754c7a1a47b1bd1b23d6a1fe"></a>
## get_array_memory_size

`function` · `arrow_array::array::Array::get_array_memory_size` · arrow-array 59.3.0

```rust
fn get_array_memory_size(&self) -> usize
```

Source: `src/array/mod.rs:356`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the total number of bytes of memory occupied physically by this array.
This value will always be greater than returned by `get_buffer_memory_size()` and
includes the overhead of the data structures that contain the pointers to the various buffers.

<a id="op-2cc1974e7020b3d257bd7b7e"></a>
## get_buffer_memory_size

`function` · `arrow_array::array::Array::get_buffer_memory_size` · arrow-array 59.3.0

```rust
fn get_buffer_memory_size(&self) -> usize
```

Source: `src/array/mod.rs:351`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the total number of bytes of memory pointed to by this array.
The buffers store bytes in the Arrow memory format, and include the data as well as the validity map.
Note that this does not always correspond to the exact memory usage of an array,
since multiple arrays can share the same buffers or slices thereof.

<a id="op-8df616008a80e7536953c6c5"></a>
## into_data

`function` · `arrow_array::array::Array::into_data` · arrow-array 59.3.0

```rust
fn into_data(self) -> ArrayData
```

Source: `src/array/mod.rs:131`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the underlying data of this array

Unlike [`Array::to_data`](../operations/arrow_array.array.Array.md#op-4e74a91e9c7ba552d3a09ac1) this consumes self, allowing it avoid unnecessary clones

<a id="op-ae35d27b38e0abdc2508bf1d"></a>
## is_empty

`function` · `arrow_array::array::Array::is_empty` · arrow-array 59.3.0

```rust
fn is_empty(&self) -> bool
```

Source: `src/array/mod.rs:186`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns whether this array is empty.

# Example:

```
use arrow_array::{Array, Int32Array};

let array = Int32Array::from(vec![1, 2, 3, 4, 5]);

assert_eq!(array.is_empty(), false);
```

<a id="op-fc6aff0e0247c3255575b6e1"></a>
## is_null

`function` · `arrow_array::array::Array::is_null` · arrow-array 59.3.0

```rust
fn is_null(&self, index: usize) -> bool
```

Source: `src/array/mod.rs:268`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns whether the element at `index` is null according to [`Array::nulls`](../operations/arrow_array.array.Array.md#op-2ca0e82b745cdf33d4a99d40)

Note: For performance reasons, this method returns nullability solely as determined by the
null buffer. This difference can lead to surprising results, for example, [`NullArray::is_null`](../operations/arrow_array.array.Array.md#op-fc6aff0e0247c3255575b6e1) always
returns `false` as the array lacks a null buffer. Similarly [`DictionaryArray`](../operations/arrow_array.array.dictionary_array.DictionaryArray.md#op-761ab0c11d727f026bdc2e47), [`RunArray`](../operations/arrow_array.array.run_array.RunArray.md#op-f0742b2d9e045922a076478f) and [`UnionArray`](../operations/arrow_array.array.union_array.UnionArray.md#op-39e2f188616dc0298ba644ac) may
encode nullability in their children. See [`Self::logical_nulls`](../operations/arrow_array.array.Array.md#op-79c309d5bb1bab4867eba124) for more information.

# Example:

```
use arrow_array::{Array, Int32Array, NullArray};

let array = Int32Array::from(vec![Some(1), None]);
assert_eq!(array.is_null(0), false);
assert_eq!(array.is_null(1), true);

// NullArrays do not have a null buffer, and therefore always
// return false for is_null.
let array = NullArray::new(1);
assert_eq!(array.is_null(0), false);
```

<a id="op-adbf0c156bb6243f7208522f"></a>
## is_nullable

`function` · `arrow_array::array::Array::is_nullable` · arrow-array 59.3.0

```rust
fn is_nullable(&self) -> bool
```

Source: `src/array/mod.rs:343`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns `false` if the array is guaranteed to not contain any logical nulls

This is generally equivalent to `Array::logical_null_count() != 0` unless determining
the logical nulls is expensive, in which case this method can return true even for an
array without nulls.

This is also generally equivalent to `Array::null_count() != 0` but may differ in the
presence of logical nullability, see [`Array::logical_null_count`](../operations/arrow_array.array.Array.md#op-a5dcc91ae5f1b77d3f999bc9) and [`Array::null_count`](../operations/arrow_array.array.Array.md#op-debf27e013374fc870d82c7e).

Implementations will return `true` unless they can cheaply prove no logical nulls
are present. For example a [`DictionaryArray`](../operations/arrow_array.array.dictionary_array.DictionaryArray.md#op-761ab0c11d727f026bdc2e47) with nullable values will still return true,
even if the nulls present in [`DictionaryArray::values`](../operations/arrow_array.array.dictionary_array.DictionaryArray.md#op-e04615315cfe741e7c23ed24) are not referenced by any key,
and therefore would not appear in [`Array::logical_nulls`](../operations/arrow_array.array.Array.md#op-79c309d5bb1bab4867eba124).

<a id="op-f1d3cd3c05fd377ecf2d4bbf"></a>
## is_valid

`function` · `arrow_array::array::Array::is_valid` · arrow-array 59.3.0

```rust
fn is_valid(&self, index: usize) -> bool
```

Source: `src/array/mod.rs:285`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns whether the element at `index` is *not* null, the
opposite of [`Self::is_null`](../operations/arrow_array.array.Array.md#op-fc6aff0e0247c3255575b6e1).

# Example:

```
use arrow_array::{Array, Int32Array};

let array = Int32Array::from(vec![Some(1), None]);

assert_eq!(array.is_valid(0), true);
assert_eq!(array.is_valid(1), false);
```

<a id="op-f46bc94a0dc5945d9a631899"></a>
## len

`function` · `arrow_array::array::Array::len` · arrow-array 59.3.0

```rust
fn len(&self) -> usize
```

Source: `src/array/mod.rs:173`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the length (i.e., number of elements) of this array.

# Example:

```
use arrow_array::{Array, Int32Array};

let array = Int32Array::from(vec![1, 2, 3, 4, 5]);

assert_eq!(array.len(), 5);
```

<a id="op-a5dcc91ae5f1b77d3f999bc9"></a>
## logical_null_count

`function` · `arrow_array::array::Array::logical_null_count` · arrow-array 59.3.0

```rust
fn logical_null_count(&self) -> usize
```

Source: `src/array/mod.rs:324`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the total number of logical null values in this array.

Note: this method returns the logical null count, i.e. that encoded in
[`Array::logical_nulls`](../operations/arrow_array.array.Array.md#op-79c309d5bb1bab4867eba124). In general this is equivalent to [`Array::null_count`](../operations/arrow_array.array.Array.md#op-debf27e013374fc870d82c7e) but may differ in the
presence of logical nullability, see [`Array::nulls`](../operations/arrow_array.array.Array.md#op-2ca0e82b745cdf33d4a99d40) and [`Array::logical_nulls`](../operations/arrow_array.array.Array.md#op-79c309d5bb1bab4867eba124).

# Example:

```
use arrow_array::{Array, Int32Array};

// Construct an array with values [1, NULL, NULL]
let array = Int32Array::from(vec![Some(1), None, None]);

assert_eq!(array.logical_null_count(), 2);
```

<a id="op-79c309d5bb1bab4867eba124"></a>
## logical_nulls

`function` · `arrow_array::array::Array::logical_nulls` · arrow-array 59.3.0

```rust
fn logical_nulls(&self) -> Option<NullBuffer>
```

Source: `src/array/mod.rs:243`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a potentially computed [`NullBuffer`](../operations/arrow_buffer.buffer.null.NullBuffer.md#op-d8209d291b86c5deb0050e48) that represents the logical
null values of this array, if any.

Logical nulls represent the values that are null in the array,
regardless of the underlying physical arrow representation.

For most array types, this is equivalent to the "physical" nulls
returned by [`Array::nulls`](../operations/arrow_array.array.Array.md#op-2ca0e82b745cdf33d4a99d40). It is different for the following cases, because which
elements are null is not encoded in a single null buffer:

* [`DictionaryArray`](../operations/arrow_array.array.dictionary_array.DictionaryArray.md#op-761ab0c11d727f026bdc2e47) where [`DictionaryArray::values`](../operations/arrow_array.array.dictionary_array.DictionaryArray.md#op-e04615315cfe741e7c23ed24) contains nulls
* [`RunArray`](../operations/arrow_array.array.run_array.RunArray.md#op-f0742b2d9e045922a076478f) where [`RunArray::values`](../operations/arrow_array.array.run_array.RunArray.md#op-3d6722c72c8815acd53335f6) contains nulls
* [`NullArray`](../operations/arrow_array.array.null_array.NullArray.md#op-eccd805771d0b7ea8e20fabd) where all indices are nulls
* [`UnionArray`](../operations/arrow_array.array.union_array.UnionArray.md#op-39e2f188616dc0298ba644ac) where the selected values contains nulls

In these cases a logical [`NullBuffer`](../operations/arrow_buffer.buffer.null.NullBuffer.md#op-d8209d291b86c5deb0050e48) will be computed, encoding the
logical nullability of these arrays, beyond what is encoded in
[`Array::nulls`](../operations/arrow_array.array.Array.md#op-2ca0e82b745cdf33d4a99d40)

<a id="op-debf27e013374fc870d82c7e"></a>
## null_count

`function` · `arrow_array::array::Array::null_count` · arrow-array 59.3.0

```rust
fn null_count(&self) -> usize
```

Source: `src/array/mod.rs:304`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the total number of physical null values in this array.

Note: this method returns the physical null count, i.e. that encoded in [`Array::nulls`](../operations/arrow_array.array.Array.md#op-2ca0e82b745cdf33d4a99d40),
see [`Array::logical_nulls`](../operations/arrow_array.array.Array.md#op-79c309d5bb1bab4867eba124) for logical nullability

# Example:

```
use arrow_array::{Array, Int32Array};

// Construct an array with values [1, NULL, NULL]
let array = Int32Array::from(vec![Some(1), None, None]);

assert_eq!(array.null_count(), 2);
```

<a id="op-2ca0e82b745cdf33d4a99d40"></a>
## nulls

`function` · `arrow_array::array::Array::nulls` · arrow-array 59.3.0

```rust
fn nulls(&self) -> Option<&NullBuffer>
```

Source: `src/array/mod.rs:223`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the null buffer of this array if any.

The null buffer contains the "physical" nulls of an array, that is how
the nulls are represented in the underlying arrow format.

The physical representation is efficient, but is sometimes non intuitive
for certain array types such as those with nullable child arrays like
[`DictionaryArray::values`](../operations/arrow_array.array.dictionary_array.DictionaryArray.md#op-e04615315cfe741e7c23ed24), [`RunArray::values`](../operations/arrow_array.array.run_array.RunArray.md#op-3d6722c72c8815acd53335f6) or [`UnionArray`](../operations/arrow_array.array.union_array.UnionArray.md#op-39e2f188616dc0298ba644ac), or without a
null buffer, such as [`NullArray`](../operations/arrow_array.array.null_array.NullArray.md#op-eccd805771d0b7ea8e20fabd).

To determine if each element of such an array is "logically" null,
use the slower [`Array::logical_nulls`](../operations/arrow_array.array.Array.md#op-79c309d5bb1bab4867eba124) to obtain a computed mask.

<a id="op-0c6ed779945165ff26ca4fdf"></a>
## offset

`function` · `arrow_array::array::Array::offset` · arrow-array 59.3.0

```rust
fn offset(&self) -> usize
```

Source: `src/array/mod.rs:209`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the offset into the underlying data used by this array(-slice).
Note that the underlying data can be shared by many arrays.
This defaults to `0`.

# Example:

```
use arrow_array::{Array, BooleanArray};

let array = BooleanArray::from(vec![false, false, true, true]);
let array_slice = array.slice(1, 3);

assert_eq!(array.offset(), 0);
assert_eq!(array_slice.offset(), 1);
```

<a id="op-21f3d4fbc7197d62da449405"></a>
## shrink_to_fit

`function` · `arrow_array::array::Array::shrink_to_fit` · arrow-array 59.3.0

```rust
fn shrink_to_fit(&mut self)
```

Source: `src/array/mod.rs:192`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Shrinks the capacity of any exclusively owned buffer as much as possible

Shared or externally allocated buffers will be ignored, and
any buffer offsets will be preserved.

<a id="op-de5c5ee117de6e0858673954"></a>
## slice

`function` · `arrow_array::array::Array::slice` · arrow-array 59.3.0

```rust
fn slice(&self, offset: usize, length: usize) -> ArrayRef
```

Source: `src/array/mod.rs:160`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a zero-copy slice of this array with the indicated offset and length.

# Example:

```
use arrow_array::{Array, Int32Array};

let array = Int32Array::from(vec![1, 2, 3, 4, 5]);
// Make slice over the values [2, 3, 4]
let array_slice = array.slice(1, 3);

assert_eq!(&array_slice, &Int32Array::from(vec![2, 3, 4]));
```

<a id="op-4e74a91e9c7ba552d3a09ac1"></a>
## to_data

`function` · `arrow_array::array::Array::to_data` · arrow-array 59.3.0

```rust
fn to_data(&self) -> ArrayData
```

Source: `src/array/mod.rs:126`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the underlying data of this array
