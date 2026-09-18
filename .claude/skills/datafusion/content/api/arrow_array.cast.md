# `arrow_array::cast`

Crate `arrow-array` · 17 public items · structured records in [`model/arrow_array.cast.json`](../model/arrow_array.cast.json)

## as_boolean_array

`function` · `arrow_array::cast::as_boolean_array`

Also reachable as `arrow::array::as_boolean_array`

```rust
fn as_boolean_array(arr: &dyn Array) -> &BooleanArray
```

[Full member, field, variant and typed contracts](../operations/arrow_array.cast.as_boolean_array.md).


Force downcast of an [`Array`], such as an [`ArrayRef`] to
[`BooleanArray`], panicking on failure.

# Example

```
# use std::sync::Arc;
# use arrow_array::{ArrayRef, BooleanArray};
# use arrow_array::cast::as_boolean_array;

let arr: ArrayRef = Arc::new(BooleanArray::from_iter(vec![Some(true)]));
let boolean_array = as_boolean_array(&arr);
```

---

## as_dictionary_array

`function` · `arrow_array::cast::as_dictionary_array`

Also reachable as `arrow::array::as_dictionary_array`

```rust
fn as_dictionary_array<T>(arr: &dyn Array) -> &DictionaryArray<T> where T: ArrowDictionaryKeyType
```

[Full member, field, variant and typed contracts](../operations/arrow_array.cast.as_dictionary_array.md).


Force downcast of an [`Array`], such as an [`ArrayRef`] to
[`DictionaryArray<T>`], panic'ing on failure.

# Example

```
# use arrow_array::{ArrayRef, DictionaryArray};
# use arrow_array::cast::as_dictionary_array;
# use arrow_array::types::Int32Type;

let arr: DictionaryArray<Int32Type> = vec![Some("foo")].into_iter().collect();
let arr: ArrayRef = std::sync::Arc::new(arr);
let dict_array: &DictionaryArray<Int32Type> = as_dictionary_array::<Int32Type>(&arr);
```

---

## as_fixed_size_list_array

`function` · `arrow_array::cast::as_fixed_size_list_array`

Also reachable as `arrow::array::as_fixed_size_list_array`

```rust
fn as_fixed_size_list_array(arr: &dyn Array) -> &FixedSizeListArray
```

[Full member, field, variant and typed contracts](../operations/arrow_array.cast.as_fixed_size_list_array.md).


Force downcast of an [`Array`], such as an [`ArrayRef`] to
[`FixedSizeListArray`], panicking on failure.

---

## as_generic_binary_array

`function` · `arrow_array::cast::as_generic_binary_array`

Also reachable as `arrow::array::as_generic_binary_array`

```rust
fn as_generic_binary_array<S: OffsetSizeTrait>(arr: &dyn Array) -> &GenericBinaryArray<S>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.cast.as_generic_binary_array.md).


Force downcast of an [`Array`], such as an [`ArrayRef`] to
[`GenericBinaryArray<S>`], panicking on failure.

---

## as_generic_list_array

`function` · `arrow_array::cast::as_generic_list_array`

Also reachable as `arrow::array::as_generic_list_array`

```rust
fn as_generic_list_array<S: OffsetSizeTrait>(arr: &dyn Array) -> &GenericListArray<S>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.cast.as_generic_list_array.md).


Force downcast of an [`Array`], such as an [`ArrayRef`] to
[`GenericListArray<T>`], panicking on failure.

---

## as_large_list_array

`function` · `arrow_array::cast::as_large_list_array`

Also reachable as `arrow::array::as_large_list_array`

```rust
fn as_large_list_array(arr: &dyn Array) -> &LargeListArray
```

[Full member, field, variant and typed contracts](../operations/arrow_array.cast.as_large_list_array.md).


Force downcast of an [`Array`], such as an [`ArrayRef`] to
[`LargeListArray`], panicking on failure.

---

## as_largestring_array

`function` · `arrow_array::cast::as_largestring_array`

Also reachable as `arrow::array::as_largestring_array`

```rust
fn as_largestring_array(arr: &dyn Array) -> &LargeStringArray
```

[Full member, field, variant and typed contracts](../operations/arrow_array.cast.as_largestring_array.md).


Force downcast of an [`Array`], such as an [`ArrayRef`] to 
[`LargeStringArray`], panicking on failure.

---

## as_list_array

`function` · `arrow_array::cast::as_list_array`

Also reachable as `arrow::array::as_list_array`

```rust
fn as_list_array(arr: &dyn Array) -> &ListArray
```

[Full member, field, variant and typed contracts](../operations/arrow_array.cast.as_list_array.md).


Force downcast of an [`Array`], such as an [`ArrayRef`] to
[`ListArray`], panicking on failure.

---

## as_map_array

`function` · `arrow_array::cast::as_map_array`

Also reachable as `arrow::array::as_map_array`

```rust
fn as_map_array(arr: &dyn Array) -> &MapArray
```

[Full member, field, variant and typed contracts](../operations/arrow_array.cast.as_map_array.md).


Force downcast of an [`Array`], such as an [`ArrayRef`] to 
[`MapArray`], panicking on failure.

---

## as_null_array

`function` · `arrow_array::cast::as_null_array`

Also reachable as `arrow::array::as_null_array`

```rust
fn as_null_array(arr: &dyn Array) -> &NullArray
```

[Full member, field, variant and typed contracts](../operations/arrow_array.cast.as_null_array.md).


Force downcast of an [`Array`], such as an [`ArrayRef`] to 
[`NullArray`], panicking on failure.

---

## as_primitive_array

`function` · `arrow_array::cast::as_primitive_array`

Also reachable as `arrow::array::as_primitive_array`

```rust
fn as_primitive_array<T>(arr: &dyn Array) -> &PrimitiveArray<T> where T: ArrowPrimitiveType
```

[Full member, field, variant and typed contracts](../operations/arrow_array.cast.as_primitive_array.md).


Force downcast of an [`Array`], such as an [`ArrayRef`], to
[`PrimitiveArray<T>`], panic'ing on failure.

# Example

```
# use std::sync::Arc;
# use arrow_array::{ArrayRef, Int32Array};
# use arrow_array::cast::as_primitive_array;
# use arrow_array::types::Int32Type;

let arr: ArrayRef = Arc::new(Int32Array::from(vec![Some(1)]));

// Downcast an `ArrayRef` to Int32Array / PrimitiveArray<Int32>:
let primitive_array: &Int32Array = as_primitive_array(&arr);

// Equivalently:
let primitive_array = as_primitive_array::<Int32Type>(&arr);

// This is the equivalent of:
let primitive_array = arr
    .as_any()
    .downcast_ref::<Int32Array>()
    .unwrap();
```

---

## as_run_array

`function` · `arrow_array::cast::as_run_array`

Also reachable as `arrow::array::as_run_array`

```rust
fn as_run_array<T>(arr: &dyn Array) -> &RunArray<T> where T: RunEndIndexType
```

[Full member, field, variant and typed contracts](../operations/arrow_array.cast.as_run_array.md).


Force downcast of an [`Array`], such as an [`ArrayRef`] to
[`RunArray<T>`], panic'ing on failure.

# Example

```
# use arrow_array::{ArrayRef, RunArray};
# use arrow_array::cast::as_run_array;
# use arrow_array::types::Int32Type;

let arr: RunArray<Int32Type> = vec![Some("foo")].into_iter().collect();
let arr: ArrayRef = std::sync::Arc::new(arr);
let run_array: &RunArray<Int32Type> = as_run_array::<Int32Type>(&arr);
```

---

## as_string_array

`function` · `arrow_array::cast::as_string_array`

Also reachable as `arrow::array::as_string_array`

```rust
fn as_string_array(arr: &dyn Array) -> &StringArray
```

[Full member, field, variant and typed contracts](../operations/arrow_array.cast.as_string_array.md).


Force downcast of an [`Array`], such as an [`ArrayRef`] to
[`StringArray`], panicking on failure.

# Example

```
# use std::sync::Arc;
# use arrow_array::cast::as_string_array;
# use arrow_array::{ArrayRef, StringArray};

let arr: ArrayRef = Arc::new(StringArray::from_iter(vec![Some("foo")]));
let string_array = as_string_array(&arr);
```

---

## as_struct_array

`function` · `arrow_array::cast::as_struct_array`

Also reachable as `arrow::array::as_struct_array`

```rust
fn as_struct_array(arr: &dyn Array) -> &StructArray
```

[Full member, field, variant and typed contracts](../operations/arrow_array.cast.as_struct_array.md).


Force downcast of an [`Array`], such as an [`ArrayRef`] to 
[`StructArray`], panicking on failure.

---

## as_union_array

`function` · `arrow_array::cast::as_union_array`

Also reachable as `arrow::array::as_union_array`

```rust
fn as_union_array(arr: &dyn Array) -> &UnionArray
```

[Full member, field, variant and typed contracts](../operations/arrow_array.cast.as_union_array.md).


Force downcast of an [`Array`], such as an [`ArrayRef`] to 
[`UnionArray`], panicking on failure.

---

## downcast_array

`function` · `arrow_array::cast::downcast_array`

Also reachable as `arrow::array::downcast_array`

```rust
fn downcast_array<T>(array: &dyn Array) -> T where T: From<arrow_data::ArrayData>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.cast.downcast_array.md).


Downcasts a `dyn Array` to a concrete type

```
# use arrow_array::{BooleanArray, Int32Array, RecordBatch, StringArray};
# use arrow_array::cast::downcast_array;
struct ConcreteBatch {
    col1: Int32Array,
    col2: BooleanArray,
    col3: StringArray,
}

impl ConcreteBatch {
    fn new(batch: &RecordBatch) -> Self {
        Self {
            col1: downcast_array(batch.column(0).as_ref()),
            col2: downcast_array(batch.column(1).as_ref()),
            col3: downcast_array(batch.column(2).as_ref()),
        }
    }
}
```

# Panics

Panics if array is not of the correct data type

---

## AsArray

`trait` · `arrow_array::cast::AsArray`

Also reachable as `arrow::array::AsArray`

```rust
trait AsArray: private::Sealed
```

**Implementors** (1)

- `arrow_array::array::ArrayRef`

**Methods** (38)

```rust
fn as_any_dictionary(&self) -> &dyn AnyDictionaryArray
fn as_any_dictionary_opt(&self) -> Option<&dyn AnyDictionaryArray>
fn as_any_ree(&self) -> &dyn AnyRunEndArray
fn as_any_ree_opt(&self) -> Option<&dyn AnyRunEndArray>
fn as_binary<O: OffsetSizeTrait>(&self) -> &GenericBinaryArray<O>
fn as_binary_opt<O: OffsetSizeTrait>(&self) -> Option<&GenericBinaryArray<O>>
fn as_binary_view(&self) -> &BinaryViewArray
fn as_binary_view_opt(&self) -> Option<&BinaryViewArray>
fn as_boolean(&self) -> &BooleanArray
fn as_boolean_opt(&self) -> Option<&BooleanArray>
fn as_byte_view<T: ByteViewType>(&self) -> &GenericByteViewArray<T>
fn as_byte_view_opt<T: ByteViewType>(&self) -> Option<&GenericByteViewArray<T>>
fn as_bytes<T: ByteArrayType>(&self) -> &GenericByteArray<T>
fn as_bytes_opt<T: ByteArrayType>(&self) -> Option<&GenericByteArray<T>>
fn as_dictionary<K: ArrowDictionaryKeyType>(&self) -> &DictionaryArray<K>
fn as_dictionary_opt<K: ArrowDictionaryKeyType>(&self) -> Option<&DictionaryArray<K>>
fn as_fixed_size_binary(&self) -> &FixedSizeBinaryArray
fn as_fixed_size_binary_opt(&self) -> Option<&FixedSizeBinaryArray>
fn as_fixed_size_list(&self) -> &FixedSizeListArray
fn as_fixed_size_list_opt(&self) -> Option<&FixedSizeListArray>
fn as_list<O: OffsetSizeTrait>(&self) -> &GenericListArray<O>
fn as_list_opt<O: OffsetSizeTrait>(&self) -> Option<&GenericListArray<O>>
fn as_list_view<O: OffsetSizeTrait>(&self) -> &GenericListViewArray<O>
fn as_list_view_opt<O: OffsetSizeTrait>(&self) -> Option<&GenericListViewArray<O>>
fn as_map(&self) -> &MapArray
fn as_map_opt(&self) -> Option<&MapArray>
fn as_primitive<T: ArrowPrimitiveType>(&self) -> &PrimitiveArray<T>
fn as_primitive_opt<T: ArrowPrimitiveType>(&self) -> Option<&PrimitiveArray<T>>
fn as_run<K: RunEndIndexType>(&self) -> &RunArray<K>
fn as_run_opt<K: RunEndIndexType>(&self) -> Option<&RunArray<K>>
fn as_string<O: OffsetSizeTrait>(&self) -> &GenericStringArray<O>
fn as_string_opt<O: OffsetSizeTrait>(&self) -> Option<&GenericStringArray<O>>
fn as_string_view(&self) -> &StringViewArray
fn as_string_view_opt(&self) -> Option<&StringViewArray>
fn as_struct(&self) -> &StructArray
fn as_struct_opt(&self) -> Option<&StructArray>
fn as_union(&self) -> &UnionArray
fn as_union_opt(&self) -> Option<&UnionArray>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.cast.AsArray.md).


An extension trait for `dyn Array` that provides ergonomic downcasting

```
# use std::sync::Arc;
# use arrow_array::{ArrayRef, Int32Array};
# use arrow_array::cast::AsArray;
# use arrow_array::types::Int32Type;
let col = Arc::new(Int32Array::from(vec![1, 2, 3])) as ArrayRef;
assert_eq!(col.as_primitive::<Int32Type>().values(), &[1, 2, 3]);
```

---
