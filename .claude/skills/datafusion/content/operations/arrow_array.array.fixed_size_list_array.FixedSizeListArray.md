# `arrow_array::array::fixed_size_list_array::FixedSizeListArray`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.fixed_size_list_array.FixedSizeListArray.json).

<a id="op-4772cb4965341d875c1269ee"></a>
## FixedSizeListArray

`struct` · `arrow_array::array::fixed_size_list_array::FixedSizeListArray` · arrow-array 59.3.0

```rust
struct FixedSizeListArray
```

Source: `src/array/fixed_size_list_array.rs:119`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

An array of [fixed length lists], similar to JSON arrays
(e.g. `["A", "B"]`).

Lists are represented using a `values` child
array where each list has a fixed size of `value_length`.

Use [`FixedSizeListBuilder`](../operations/arrow_array.builder.fixed_size_list_builder.FixedSizeListBuilder.md#op-f08edffd6af5d83e255a4d2a) to construct a [`FixedSizeListArray`](../operations/arrow_array.array.fixed_size_list_array.FixedSizeListArray.md#op-4772cb4965341d875c1269ee).

# Representation

A [`FixedSizeListArray`](../operations/arrow_array.array.fixed_size_list_array.FixedSizeListArray.md#op-4772cb4965341d875c1269ee) can represent a list of values of any other
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

<a id="op-44d689106aa23720478734d6"></a>
## Item

`assoc_type` · `arrow_array::array::fixed_size_list_array::FixedSizeListArray::Item` · arrow-array 59.3.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_list_array::FixedSizeListArray", "path": "FixedSizeListArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [579, 1], "end": [589, 2], "filename": "src/array/fixed_size_list_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::ArrayAccessor", "path": "ArrayAccessor"}, "trait_path": "arrow_array::array::ArrayAccessor"}`

Source: `src/array/fixed_size_list_array.rs:580`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-82fe935f4e3a699a41177800"></a>
## as_any

`function` · `arrow_array::array::fixed_size_list_array::FixedSizeListArray::as_any` · arrow-array 59.3.0

```rust
fn as_any(&self) -> &dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_list_array::FixedSizeListArray", "path": "FixedSizeListArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [493, 1], "end": [565, 2], "filename": "src/array/fixed_size_list_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/fixed_size_list_array.rs:494`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eebbf2ee65269a3d72315c97"></a>
## claim

`function` · `arrow_array::array::fixed_size_list_array::FixedSizeListArray::claim` · arrow-array 59.3.0

```rust
fn claim(&self, pool: &dyn arrow_buffer::MemoryPool)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_list_array::FixedSizeListArray", "path": "FixedSizeListArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [493, 1], "end": [565, 2], "filename": "src/array/fixed_size_list_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/fixed_size_list_array.rs:559`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5118e8597d9a3470bb7c8a09"></a>
## clone

`function` · `arrow_array::array::fixed_size_list_array::FixedSizeListArray::clone` · arrow-array 59.3.0

```rust
fn clone(&self) -> FixedSizeListArray
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_list_array::FixedSizeListArray", "path": "FixedSizeListArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 10], "end": [118, 15], "filename": "src/array/fixed_size_list_array.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/array/fixed_size_list_array.rs:118`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-70a7bdb87874991e218c5d36"></a>
## data_type

`function` · `arrow_array::array::fixed_size_list_array::FixedSizeListArray::data_type` · arrow-array 59.3.0

```rust
fn data_type(&self) -> &DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_list_array::FixedSizeListArray", "path": "FixedSizeListArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [493, 1], "end": [565, 2], "filename": "src/array/fixed_size_list_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/fixed_size_list_array.rs:506`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-20a667cc1b371f8c06440c42"></a>
## element_range

`function` · `arrow_array::array::fixed_size_list_array::FixedSizeListArray::element_range` · arrow-array 59.3.0

```rust
fn element_range(&self, index: usize) -> std::ops::Range<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_list_array::FixedSizeListArray", "path": "FixedSizeListArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [567, 1], "end": [577, 2], "filename": "src/array/fixed_size_list_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::ListLikeArray", "path": "ListLikeArray"}, "trait_path": "arrow_array::array::ListLikeArray"}`

Source: `src/array/fixed_size_list_array.rs:572`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-28aaa96c3e13c2b142076dbf"></a>
## eq

`function` · `arrow_array::array::fixed_size_list_array::FixedSizeListArray::eq` · arrow-array 59.3.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_list_array::FixedSizeListArray", "path": "FixedSizeListArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [836, 1], "end": [840, 2], "filename": "src/array/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/array/mod.rs:837`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1ee495db6ccf3bc74a0ea3af"></a>
## fmt

`function` · `arrow_array::array::fixed_size_list_array::FixedSizeListArray::fmt` · arrow-array 59.3.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_list_array::FixedSizeListArray", "path": "FixedSizeListArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [591, 1], "end": [599, 2], "filename": "src/array/fixed_size_list_array.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/array/fixed_size_list_array.rs:592`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-82311c0d356f08fbc6769dea"></a>
## from

`function` · `arrow_array::array::fixed_size_list_array::FixedSizeListArray::from` · arrow-array 59.3.0

```rust
fn from(data: ArrayData) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_list_array::FixedSizeListArray", "path": "FixedSizeListArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [456, 1], "end": [479, 2], "filename": "src/array/fixed_size_list_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayData", "path": "ArrayData"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/fixed_size_list_array.rs:457`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8c493f9e7bdea71a2defe3d9"></a>
## from_iter_primitive

`function` · `arrow_array::array::fixed_size_list_array::FixedSizeListArray::from_iter_primitive` · arrow-array 59.3.0

```rust
fn from_iter_primitive<T, P, I>(iter: I, length: i32) -> Self where T: ArrowPrimitiveType, P: IntoIterator<Item = Option<<T as ArrowPrimitiveType>::Native>>, I: IntoIterator<Item = Option<P>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_list_array::FixedSizeListArray", "path": "FixedSizeListArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 1], "end": [454, 2], "filename": "src/array/fixed_size_list_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/fixed_size_list_array.rs:418`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a [`FixedSizeListArray`](../operations/arrow_array.array.fixed_size_list_array.FixedSizeListArray.md#op-4772cb4965341d875c1269ee) from an iterator of primitive values
# Example
```
# use arrow_array::FixedSizeListArray;
# use arrow_array::types::Int32Type;

let data = vec![
   Some(vec![Some(0), Some(1), Some(2)]),
   None,
   Some(vec![Some(3), None, Some(5)]),
   Some(vec![Some(6), Some(7), Some(45)]),
];
let list_array = FixedSizeListArray::from_iter_primitive::<Int32Type, _, _>(data, 3);
println!("{:?}", list_array);
```

<a id="op-2dc30c8511e8b6e986a3c0b9"></a>
## get_array_memory_size

`function` · `arrow_array::array::fixed_size_list_array::FixedSizeListArray::get_array_memory_size` · arrow-array 59.3.0

```rust
fn get_array_memory_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_list_array::FixedSizeListArray", "path": "FixedSizeListArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [493, 1], "end": [565, 2], "filename": "src/array/fixed_size_list_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/fixed_size_list_array.rs:550`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f9697311d83c07a51d3087d6"></a>
## get_buffer_memory_size

`function` · `arrow_array::array::fixed_size_list_array::FixedSizeListArray::get_buffer_memory_size` · arrow-array 59.3.0

```rust
fn get_buffer_memory_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_list_array::FixedSizeListArray", "path": "FixedSizeListArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [493, 1], "end": [565, 2], "filename": "src/array/fixed_size_list_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/fixed_size_list_array.rs:542`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e14ce8ed97ed7015fc040cfd"></a>
## into_data

`function` · `arrow_array::array::fixed_size_list_array::FixedSizeListArray::into_data` · arrow-array 59.3.0

```rust
fn into_data(self) -> ArrayData
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_list_array::FixedSizeListArray", "path": "FixedSizeListArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [493, 1], "end": [565, 2], "filename": "src/array/fixed_size_list_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/fixed_size_list_array.rs:502`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e7d39bdd211d29f739e16402"></a>
## into_parts

`function` · `arrow_array::array::fixed_size_list_array::FixedSizeListArray::into_parts` · arrow-array 59.3.0

```rust
fn into_parts(self) -> (FieldRef, i32, ArrayRef, Option<NullBuffer>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_list_array::FixedSizeListArray", "path": "FixedSizeListArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 1], "end": [454, 2], "filename": "src/array/fixed_size_list_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/fixed_size_list_array.rs:335`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Deconstruct this array into its constituent parts

<a id="op-ea8b25442942e5b296b40b7e"></a>
## is_empty

`function` · `arrow_array::array::fixed_size_list_array::FixedSizeListArray::is_empty` · arrow-array 59.3.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_list_array::FixedSizeListArray", "path": "FixedSizeListArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [493, 1], "end": [565, 2], "filename": "src/array/fixed_size_list_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/fixed_size_list_array.rs:518`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1fffa9fb165e176f067603e7"></a>
## iter

`function` · `arrow_array::array::fixed_size_list_array::FixedSizeListArray::iter` · arrow-array 59.3.0

```rust
fn iter(&self) -> FixedSizeListIter<'_>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_list_array::FixedSizeListArray", "path": "FixedSizeListArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 1], "end": [454, 2], "filename": "src/array/fixed_size_list_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/fixed_size_list_array.rs:451`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

constructs a new iterator

<a id="op-0aa47bc6cbdace23f12bc549"></a>
## len

`function` · `arrow_array::array::fixed_size_list_array::FixedSizeListArray::len` · arrow-array 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_list_array::FixedSizeListArray", "path": "FixedSizeListArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [493, 1], "end": [565, 2], "filename": "src/array/fixed_size_list_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/fixed_size_list_array.rs:514`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9985fbfd609e279975826507"></a>
## logical_null_count

`function` · `arrow_array::array::fixed_size_list_array::FixedSizeListArray::logical_null_count` · arrow-array 59.3.0

```rust
fn logical_null_count(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_list_array::FixedSizeListArray", "path": "FixedSizeListArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [493, 1], "end": [565, 2], "filename": "src/array/fixed_size_list_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/fixed_size_list_array.rs:537`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-16fc84a521d8d5b7badac796"></a>
## new

`function` · `arrow_array::array::fixed_size_list_array::FixedSizeListArray::new` · arrow-array 59.3.0

```rust
fn new(field: FieldRef, size: i32, values: ArrayRef, nulls: Option<NullBuffer>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_list_array::FixedSizeListArray", "path": "FixedSizeListArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 1], "end": [454, 2], "filename": "src/array/fixed_size_list_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/fixed_size_list_array.rs:141`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`FixedSizeListArray`](../operations/arrow_array.array.fixed_size_list_array.FixedSizeListArray.md#op-4772cb4965341d875c1269ee) with `size` element size, panicking on failure.

Note that if `size == 0` and `nulls` is `None` (a degenerate, non-nullable
`FixedSizeListArray`), this function will set the length of the array to 0.

If you would like to have a degenerate, non-nullable `FixedSizeListArray` with arbitrary
length, use the [`try_new_with_length()`] constructor.

[`try_new_with_length()`]: Self::try_new_with_length

# Panics

Panics if [`Self::try_new`](../operations/arrow_array.array.fixed_size_list_array.FixedSizeListArray.md#op-e164731462b6ed4d06b49312) returns an error

<a id="op-1eaeb5819cd472f72a6bd566"></a>
## new_null

`function` · `arrow_array::array::fixed_size_list_array::FixedSizeListArray::new_null` · arrow-array 59.3.0

```rust
fn new_null(field: FieldRef, size: i32, len: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_list_array::FixedSizeListArray", "path": "FixedSizeListArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 1], "end": [454, 2], "filename": "src/array/fixed_size_list_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/fixed_size_list_array.rs:323`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`FixedSizeListArray`](../operations/arrow_array.array.fixed_size_list_array.FixedSizeListArray.md#op-4772cb4965341d875c1269ee) of length `len` where all values are null

# Panics

Panics if

* `size < 0`
* `size * len` would overflow `usize`

<a id="op-067409bc983726b7f04c9894"></a>
## new_unchecked

`function` · `arrow_array::array::fixed_size_list_array::FixedSizeListArray::new_unchecked` · arrow-array 59.3.0

```rust
unsafe fn new_unchecked(field: FieldRef, size: i32, values: ArrayRef, nulls: Option<NullBuffer>, len: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_list_array::FixedSizeListArray", "path": "FixedSizeListArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 1], "end": [454, 2], "filename": "src/array/fixed_size_list_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/fixed_size_list_array.rs:152`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`FixedSizeListArray`](../operations/arrow_array.array.fixed_size_list_array.FixedSizeListArray.md#op-4772cb4965341d875c1269ee) from the provided parts without validation.

# Safety
- `size >= 0`
- `values.len() == len * size as usize`
- `nulls.len() == len` if `nulls` is `Some`
- `field.data_type() == values.data_type()`

<a id="op-6b13c1f2c2fe762ef10adf59"></a>
## nulls

`function` · `arrow_array::array::fixed_size_list_array::FixedSizeListArray::nulls` · arrow-array 59.3.0

```rust
fn nulls(&self) -> Option<&NullBuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_list_array::FixedSizeListArray", "path": "FixedSizeListArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [493, 1], "end": [565, 2], "filename": "src/array/fixed_size_list_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/fixed_size_list_array.rs:533`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0007053870288da1ff1a5f23"></a>
## offset

`function` · `arrow_array::array::fixed_size_list_array::FixedSizeListArray::offset` · arrow-array 59.3.0

```rust
fn offset(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_list_array::FixedSizeListArray", "path": "FixedSizeListArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [493, 1], "end": [565, 2], "filename": "src/array/fixed_size_list_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/fixed_size_list_array.rs:529`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3902a41da6eb4d6b63baa99f"></a>
## shrink_to_fit

`function` · `arrow_array::array::fixed_size_list_array::FixedSizeListArray::shrink_to_fit` · arrow-array 59.3.0

```rust
fn shrink_to_fit(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_list_array::FixedSizeListArray", "path": "FixedSizeListArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [493, 1], "end": [565, 2], "filename": "src/array/fixed_size_list_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/fixed_size_list_array.rs:522`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e6e1510f0f75c3fc31b70f39"></a>
## slice

`function` · `arrow_array::array::fixed_size_list_array::FixedSizeListArray::slice` · arrow-array 59.3.0

```rust
fn slice(&self, offset: usize, len: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_list_array::FixedSizeListArray", "path": "FixedSizeListArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 1], "end": [454, 2], "filename": "src/array/fixed_size_list_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/fixed_size_list_array.rs:387`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a zero-copy slice of this array with the indicated offset and length.

<a id="op-f65ce6e54f59a68c9665d978"></a>
## slice

`function` · `arrow_array::array::fixed_size_list_array::FixedSizeListArray::slice` · arrow-array 59.3.0

```rust
fn slice(&self, offset: usize, length: usize) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_list_array::FixedSizeListArray", "path": "FixedSizeListArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [493, 1], "end": [565, 2], "filename": "src/array/fixed_size_list_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/fixed_size_list_array.rs:510`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a53e17a5e5da53e4e9552797"></a>
## to_data

`function` · `arrow_array::array::fixed_size_list_array::FixedSizeListArray::to_data` · arrow-array 59.3.0

```rust
fn to_data(&self) -> ArrayData
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_list_array::FixedSizeListArray", "path": "FixedSizeListArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [493, 1], "end": [565, 2], "filename": "src/array/fixed_size_list_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/fixed_size_list_array.rs:498`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e164731462b6ed4d06b49312"></a>
## try_new

`function` · `arrow_array::array::fixed_size_list_array::FixedSizeListArray::try_new` · arrow-array 59.3.0

```rust
fn try_new(field: FieldRef, size: i32, values: ArrayRef, nulls: Option<NullBuffer>) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_list_array::FixedSizeListArray", "path": "FixedSizeListArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 1], "end": [454, 2], "filename": "src/array/fixed_size_list_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/fixed_size_list_array.rs:187`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`FixedSizeListArray`](../operations/arrow_array.array.fixed_size_list_array.FixedSizeListArray.md#op-4772cb4965341d875c1269ee) from the provided parts, returning an error on failure.

Note that if `size == 0` and `nulls` is `None` (a degenerate, non-nullable
`FixedSizeListArray`), this function will set the length of the array to 0.

If you would like to have a degenerate, non-nullable `FixedSizeListArray` with arbitrary
length, use the [`try_new_with_length()`] constructor.

[`try_new_with_length()`]: Self::try_new_with_length

# Errors

* `size < 0`
* `values.len() != nulls.len() * size` if `nulls` is `Some`
* `values.data_type() != field.data_type()`
* `!field.is_nullable() && !nulls.expand(size).contains(values.logical_nulls())`

<a id="op-ecc3e3b2100fb7f6ba0eab23"></a>
## try_new_with_length

`function` · `arrow_array::array::fixed_size_list_array::FixedSizeListArray::try_new_with_length` · arrow-array 59.3.0

```rust
fn try_new_with_length(field: FieldRef, size: i32, values: ArrayRef, nulls: Option<NullBuffer>, len: usize) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_list_array::FixedSizeListArray", "path": "FixedSizeListArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 1], "end": [454, 2], "filename": "src/array/fixed_size_list_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/fixed_size_list_array.rs:245`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`FixedSizeListArray`](../operations/arrow_array.array.fixed_size_list_array.FixedSizeListArray.md#op-4772cb4965341d875c1269ee) from the provided parts, returning an error on failure.

This method exists to allow the construction of arbitrary length degenerate (`size == 0`)
and non-nullable `FixedSizeListArray`s. If you want a nullable `FixedSizeListArray`, then
you can use [`try_new()`] instead.

[`try_new()`]: Self::try_new

# Errors

* `size < 0`
* `nulls.len() != len` if `nulls` is `Some`
* `values.len() != len * size`
* `values.data_type() != field.data_type()`
* `!field.is_nullable() && !nulls.expand(size).contains(values.logical_nulls())`

<a id="op-5eb0b0ae110f4af1517aeca7"></a>
## value

`function` · `arrow_array::array::fixed_size_list_array::FixedSizeListArray::value` · arrow-array 59.3.0

```rust
fn value(&self, i: usize) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_list_array::FixedSizeListArray", "path": "FixedSizeListArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 1], "end": [454, 2], "filename": "src/array/fixed_size_list_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/fixed_size_list_array.rs:360`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns ith value of this list array.

Note: This method does not check for nulls and the value is arbitrary
(but still well-defined) if [`is_null`](Self::is_null) returns true for the index.

# Panics
Panics if index `i` is out of bounds

<a id="op-7b1028b75d2ea530643038b2"></a>
## value

`function` · `arrow_array::array::fixed_size_list_array::FixedSizeListArray::value` · arrow-array 59.3.0

```rust
fn value(&self, index: usize) -> Self::Item
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_list_array::FixedSizeListArray", "path": "FixedSizeListArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [579, 1], "end": [589, 2], "filename": "src/array/fixed_size_list_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::ArrayAccessor", "path": "ArrayAccessor"}, "trait_path": "arrow_array::array::ArrayAccessor"}`

Source: `src/array/fixed_size_list_array.rs:582`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2e556ba74a00871d01f266ba"></a>
## value_length

`function` · `arrow_array::array::fixed_size_list_array::FixedSizeListArray::value_length` · arrow-array 59.3.0

```rust
const fn value_length(&self) -> i32
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_list_array::FixedSizeListArray", "path": "FixedSizeListArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 1], "end": [454, 2], "filename": "src/array/fixed_size_list_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/fixed_size_list_array.rs:377`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the length for an element.

All elements have the same length as the array is a fixed size.

<a id="op-a723ee0ec82a228c8ea08a60"></a>
## value_offset

`function` · `arrow_array::array::fixed_size_list_array::FixedSizeListArray::value_offset` · arrow-array 59.3.0

```rust
fn value_offset(&self, i: usize) -> i32
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_list_array::FixedSizeListArray", "path": "FixedSizeListArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 1], "end": [454, 2], "filename": "src/array/fixed_size_list_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/fixed_size_list_array.rs:369`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the offset for value at index `i`.

Note this doesn't do any bound checking, for performance reason.

<a id="op-44f4bf1fd8c4f21fbfaf1cea"></a>
## value_type

`function` · `arrow_array::array::fixed_size_list_array::FixedSizeListArray::value_type` · arrow-array 59.3.0

```rust
fn value_type(&self) -> DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_list_array::FixedSizeListArray", "path": "FixedSizeListArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 1], "end": [454, 2], "filename": "src/array/fixed_size_list_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/fixed_size_list_array.rs:349`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a clone of the value type of this list.

<a id="op-5554545315910c1e21a62b7c"></a>
## value_unchecked

`function` · `arrow_array::array::fixed_size_list_array::FixedSizeListArray::value_unchecked` · arrow-array 59.3.0

```rust
unsafe fn value_unchecked(&self, index: usize) -> Self::Item
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_list_array::FixedSizeListArray", "path": "FixedSizeListArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [579, 1], "end": [589, 2], "filename": "src/array/fixed_size_list_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::ArrayAccessor", "path": "ArrayAccessor"}, "trait_path": "arrow_array::array::ArrayAccessor"}`

Source: `src/array/fixed_size_list_array.rs:586`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-14e15f3a59737608462c032f"></a>
## values

`function` · `arrow_array::array::fixed_size_list_array::FixedSizeListArray::values` · arrow-array 59.3.0

```rust
fn values(&self) -> &ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_list_array::FixedSizeListArray", "path": "FixedSizeListArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [567, 1], "end": [577, 2], "filename": "src/array/fixed_size_list_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::ListLikeArray", "path": "ListLikeArray"}, "trait_path": "arrow_array::array::ListLikeArray"}`

Source: `src/array/fixed_size_list_array.rs:568`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9177638322bf3cb1f691ed77"></a>
## values

`function` · `arrow_array::array::fixed_size_list_array::FixedSizeListArray::values` · arrow-array 59.3.0

```rust
fn values(&self) -> &ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_list_array::FixedSizeListArray", "path": "FixedSizeListArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 1], "end": [454, 2], "filename": "src/array/fixed_size_list_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/fixed_size_list_array.rs:344`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a reference to the values of this list.
