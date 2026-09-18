# `arrow_array::array::union_array::UnionArray`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.union_array.UnionArray.json).

<a id="op-39e2f188616dc0298ba644ac"></a>
## UnionArray

`struct` · `arrow_array::array::union_array::UnionArray` · arrow-array 59.3.0

```rust
struct UnionArray
```

Source: `src/array/union_array.rs:123`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

An array of [values of varying types](https://arrow.apache.org/docs/format/Columnar.html#union-layout)

Each slot in a [UnionArray](../operations/arrow_array.array.union_array.UnionArray.md#op-39e2f188616dc0298ba644ac) can have a value chosen from a number
of types.  Each of the possible types are named like the fields of
a [`StructArray`](crate::StructArray).  A `UnionArray` can
have two possible memory layouts, "dense" or "sparse".  For more
information on please see the
[specification](https://arrow.apache.org/docs/format/Columnar.html#union-layout).

[UnionBuilder](crate::builder::UnionBuilder) can be used to
create [UnionArray](../operations/arrow_array.array.union_array.UnionArray.md#op-39e2f188616dc0298ba644ac)'s of primitive types. `UnionArray`'s of nested
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

<a id="op-c50cf8ca077f83951e06f2bc"></a>
## as_any

`function` · `arrow_array::array::union_array::UnionArray::as_any` · arrow-array 59.3.0

```rust
fn as_any(&self) -> &dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::union_array::UnionArray", "path": "UnionArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [746, 1], "end": [960, 2], "filename": "src/array/union_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/union_array.rs:747`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-11fde666ceb29e594dae6e59"></a>
## child

`function` · `arrow_array::array::union_array::UnionArray::child` · arrow-array 59.3.0

```rust
fn child(&self, type_id: i8) -> &ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::union_array::UnionArray", "path": "UnionArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [679, 2], "filename": "src/array/union_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/union_array.rs:250`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Accesses the child array for `type_id`.

# Panics

Panics if the `type_id` provided is not present in the array's DataType
in the `Union`.

<a id="op-a7bdebe00e23e326bcaeeb54"></a>
## claim

`function` · `arrow_array::array::union_array::UnionArray::claim` · arrow-array 59.3.0

```rust
fn claim(&self, pool: &dyn arrow_buffer::MemoryPool)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::union_array::UnionArray", "path": "UnionArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [746, 1], "end": [960, 2], "filename": "src/array/union_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/union_array.rs:951`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b41e0262509123b20ec389b9"></a>
## clone

`function` · `arrow_array::array::union_array::UnionArray::clone` · arrow-array 59.3.0

```rust
fn clone(&self) -> UnionArray
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::union_array::UnionArray", "path": "UnionArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 10], "end": [122, 15], "filename": "src/array/union_array.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/array/union_array.rs:122`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c995f8792739ca808e6b211"></a>
## data_type

`function` · `arrow_array::array::union_array::UnionArray::data_type` · arrow-array 59.3.0

```rust
fn data_type(&self) -> &DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::union_array::UnionArray", "path": "UnionArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [746, 1], "end": [960, 2], "filename": "src/array/union_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/union_array.rs:759`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1904fe0c7d68d5de40ff6b84"></a>
## fields

`function` · `arrow_array::array::union_array::UnionArray::fields` · arrow-array 59.3.0

```rust
fn fields(&self) -> &UnionFields
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::union_array::UnionArray", "path": "UnionArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [679, 2], "filename": "src/array/union_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/union_array.rs:315`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the [`UnionFields`](../operations/arrow_schema.fields.UnionFields.md#op-daca7b2fe864f8176f562862) for the union.

<a id="op-0c5a49cd52907c2f1c50218e"></a>
## fmt

`function` · `arrow_array::array::union_array::UnionArray::fmt` · arrow-array 59.3.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::union_array::UnionArray", "path": "UnionArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [962, 1], "end": [998, 2], "filename": "src/array/union_array.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/array/union_array.rs:963`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8fd2c6f498f7c920ecf5441c"></a>
## from

`function` · `arrow_array::array::union_array::UnionArray::from` · arrow-array 59.3.0

```rust
fn from(data: ArrayData) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::union_array::UnionArray", "path": "UnionArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [681, 1], "end": [718, 2], "filename": "src/array/union_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayData", "path": "ArrayData"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/union_array.rs:682`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d66dbf242ab392d5c1c0f4f5"></a>
## get_array_memory_size

`function` · `arrow_array::array::union_array::UnionArray::get_array_memory_size` · arrow-array 59.3.0

```rust
fn get_array_memory_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::union_array::UnionArray", "path": "UnionArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [746, 1], "end": [960, 2], "filename": "src/array/union_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/union_array.rs:936`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-db7dfa9abe5e0821375e45c6"></a>
## get_buffer_memory_size

`function` · `arrow_array::array::union_array::UnionArray::get_buffer_memory_size` · arrow-array 59.3.0

```rust
fn get_buffer_memory_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::union_array::UnionArray", "path": "UnionArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [746, 1], "end": [960, 2], "filename": "src/array/union_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/union_array.rs:924`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3fcabc83480ab3b6bae76cab"></a>
## into_data

`function` · `arrow_array::array::union_array::UnionArray::into_data` · arrow-array 59.3.0

```rust
fn into_data(self) -> ArrayData
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::union_array::UnionArray", "path": "UnionArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [746, 1], "end": [960, 2], "filename": "src/array/union_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/union_array.rs:755`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e78b8c853e10c64db7305897"></a>
## into_parts

`function` · `arrow_array::array::union_array::UnionArray::into_parts` · arrow-array 59.3.0

```rust
fn into_parts(self) -> (UnionFields, ScalarBuffer<i8>, Option<ScalarBuffer<i32>>, Vec<ArrayRef>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::union_array::UnionArray", "path": "UnionArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [679, 2], "filename": "src/array/union_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/union_array.rs:382`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Deconstruct this array into its constituent parts

# Example

```
# use arrow_array::array::UnionArray;
# use arrow_array::types::Int32Type;
# use arrow_array::builder::UnionBuilder;
# use arrow_buffer::ScalarBuffer;
# fn main() -> Result<(), arrow_schema::ArrowError> {
let mut builder = UnionBuilder::new_dense();
builder.append::<Int32Type>("a", 1).unwrap();
let union_array = builder.build()?;

// Deconstruct into parts
let (union_fields, type_ids, offsets, children) = union_array.into_parts();

// Reconstruct from parts
let union_array = UnionArray::try_new(
    union_fields,
    type_ids,
    offsets,
    children,
);
# Ok(())
# }
```

<a id="op-bbb6e7a01ea746c03d1740c6"></a>
## is_dense

`function` · `arrow_array::array::union_array::UnionArray::is_dense` · arrow-array 59.3.0

```rust
fn is_dense(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::union_array::UnionArray", "path": "UnionArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [679, 2], "filename": "src/array/union_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/union_array.rs:323`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns whether the `UnionArray` is dense (or sparse if `false`).

<a id="op-2531bd1751a3ee4fb42426b9"></a>
## is_empty

`function` · `arrow_array::array::union_array::UnionArray::is_empty` · arrow-array 59.3.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::union_array::UnionArray", "path": "UnionArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [746, 1], "end": [960, 2], "filename": "src/array/union_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/union_array.rs:771`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5f56d523c937ce91160a03aa"></a>
## is_nullable

`function` · `arrow_array::array::union_array::UnionArray::is_nullable` · arrow-array 59.3.0

```rust
fn is_nullable(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::union_array::UnionArray", "path": "UnionArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [746, 1], "end": [960, 2], "filename": "src/array/union_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/union_array.rs:917`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ae6b887af236ed2057718ad8"></a>
## len

`function` · `arrow_array::array::union_array::UnionArray::len` · arrow-array 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::union_array::UnionArray", "path": "UnionArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [746, 1], "end": [960, 2], "filename": "src/array/union_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/union_array.rs:767`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b964f66267980a82e93b54f2"></a>
## logical_nulls

`function` · `arrow_array::array::union_array::UnionArray::logical_nulls` · arrow-array 59.3.0

```rust
fn logical_nulls(&self) -> Option<NullBuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::union_array::UnionArray", "path": "UnionArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [746, 1], "end": [960, 2], "filename": "src/array/union_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/union_array.rs:794`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fdfda96ba12732c1d59876a7"></a>
## new_unchecked

`function` · `arrow_array::array::union_array::UnionArray::new_unchecked` · arrow-array 59.3.0

```rust
unsafe fn new_unchecked(fields: UnionFields, type_ids: ScalarBuffer<i8>, offsets: Option<ScalarBuffer<i32>>, children: Vec<ArrayRef>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::union_array::UnionArray", "path": "UnionArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [679, 2], "filename": "src/array/union_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/union_array.rs:149`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new `UnionArray`.

Accepts type ids, child arrays and optionally offsets (for dense unions) to create
a new `UnionArray`.  This method makes no attempt to validate the data provided by the
caller and assumes that each of the components are correct and consistent with each other.
See `try_new` for an alternative that validates the data provided.

# Safety

The `type_ids` values should be non-negative and must match one of the type ids of the fields provided in `fields`.
These values are used to index into the `children` arrays.

The `offsets` is provided in the case of a dense union, sparse unions should use `None`.
If provided the `offsets` values should be non-negative and must be less than the length of the
corresponding array.

In both cases above we use signed integer types to maintain compatibility with other
Arrow implementations.

<a id="op-c464101585be5791dfbf43bd"></a>
## nulls

`function` · `arrow_array::array::union_array::UnionArray::nulls` · arrow-array 59.3.0

```rust
fn nulls(&self) -> Option<&NullBuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::union_array::UnionArray", "path": "UnionArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [746, 1], "end": [960, 2], "filename": "src/array/union_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/union_array.rs:790`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c67723117176d41df3a855d9"></a>
## offset

`function` · `arrow_array::array::union_array::UnionArray::offset` · arrow-array 59.3.0

```rust
fn offset(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::union_array::UnionArray", "path": "UnionArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [746, 1], "end": [960, 2], "filename": "src/array/union_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/union_array.rs:786`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-34d3587ee53af3fcb54b95f6"></a>
## offsets

`function` · `arrow_array::array::union_array::UnionArray::offsets` · arrow-array 59.3.0

```rust
fn offsets(&self) -> Option<&ScalarBuffer<i32>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::union_array::UnionArray", "path": "UnionArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [679, 2], "filename": "src/array/union_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/union_array.rs:272`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the `offsets` buffer if this is a dense array

<a id="op-f47cbe39060f595f8e3fb97e"></a>
## shrink_to_fit

`function` · `arrow_array::array::union_array::UnionArray::shrink_to_fit` · arrow-array 59.3.0

```rust
fn shrink_to_fit(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::union_array::UnionArray", "path": "UnionArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [746, 1], "end": [960, 2], "filename": "src/array/union_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/union_array.rs:775`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-16a92234230e52fc5561083e"></a>
## slice

`function` · `arrow_array::array::union_array::UnionArray::slice` · arrow-array 59.3.0

```rust
fn slice(&self, offset: usize, length: usize) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::union_array::UnionArray", "path": "UnionArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [746, 1], "end": [960, 2], "filename": "src/array/union_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/union_array.rs:763`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e895abb4cd31bca1e0792e09"></a>
## slice

`function` · `arrow_array::array::union_array::UnionArray::slice` · arrow-array 59.3.0

```rust
fn slice(&self, offset: usize, length: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::union_array::UnionArray", "path": "UnionArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [679, 2], "filename": "src/array/union_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/union_array.rs:331`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a zero-copy slice of this array with the indicated offset and length.

<a id="op-d1ed89fff52b21186663869c"></a>
## to_data

`function` · `arrow_array::array::union_array::UnionArray::to_data` · arrow-array 59.3.0

```rust
fn to_data(&self) -> ArrayData
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::union_array::UnionArray", "path": "UnionArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [746, 1], "end": [960, 2], "filename": "src/array/union_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/union_array.rs:751`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b41936bc5e65c51c4b23a490"></a>
## try_new

`function` · `arrow_array::array::union_array::UnionArray::try_new` · arrow-array 59.3.0

```rust
fn try_new(fields: UnionFields, type_ids: ScalarBuffer<i8>, offsets: Option<ScalarBuffer<i32>>, children: Vec<ArrayRef>) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::union_array::UnionArray", "path": "UnionArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [679, 2], "filename": "src/array/union_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/union_array.rs:177`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Attempts to create a new `UnionArray`, validating the inputs provided.

The order of child arrays child array order must match the fields order

<a id="op-d1288f4dc0f68d9573ae716a"></a>
## type_id

`function` · `arrow_array::array::union_array::UnionArray::type_id` · arrow-array 59.3.0

```rust
fn type_id(&self, index: usize) -> i8
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::union_array::UnionArray", "path": "UnionArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [679, 2], "filename": "src/array/union_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/union_array.rs:261`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the `type_id` for the array slot at `index`.

# Panics

Panics if `index` is greater than or equal to the number of child arrays

<a id="op-9ae99b7ece335c69081e6d00"></a>
## type_ids

`function` · `arrow_array::array::union_array::UnionArray::type_ids` · arrow-array 59.3.0

```rust
fn type_ids(&self) -> &ScalarBuffer<i8>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::union_array::UnionArray", "path": "UnionArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [679, 2], "filename": "src/array/union_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/union_array.rs:267`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the `type_ids` buffer for this array

<a id="op-8d3778beaf14eb0bd11874b0"></a>
## type_names

`function` · `arrow_array::array::union_array::UnionArray::type_names` · arrow-array 59.3.0

```rust
fn type_names(&self) -> Vec<&str>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::union_array::UnionArray", "path": "UnionArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [679, 2], "filename": "src/array/union_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/union_array.rs:304`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the names of the types in the union.

<a id="op-c371c6af9049f2382fd15626"></a>
## value

`function` · `arrow_array::array::union_array::UnionArray::value` · arrow-array 59.3.0

```rust
fn value(&self, i: usize) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::union_array::UnionArray", "path": "UnionArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [679, 2], "filename": "src/array/union_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/union_array.rs:296`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the array's value at index `i`.

Note: This method does not check for nulls and the value is arbitrary
(but still well-defined) if [`is_null`](Self::is_null) returns true for the index.

# Panics
Panics if index `i` is out of bounds

<a id="op-fd28a39f5f1c4caac5c73dee"></a>
## value_offset

`function` · `arrow_array::array::union_array::UnionArray::value_offset` · arrow-array 59.3.0

```rust
fn value_offset(&self, index: usize) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::union_array::UnionArray", "path": "UnionArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [679, 2], "filename": "src/array/union_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/union_array.rs:281`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the offset into the underlying values array for the array slot at `index`.

# Panics

Panics if `index` is greater than or equal the length of the array.
