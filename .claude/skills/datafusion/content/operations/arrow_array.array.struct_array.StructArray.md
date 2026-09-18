# `arrow_array::array::struct_array::StructArray`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.struct_array.StructArray.json).

<a id="op-994d82c95af72fa0e3aa6207"></a>
## StructArray

`struct` · `arrow_array::array::struct_array::StructArray` · arrow-array 59.3.0

```rust
struct StructArray
```

Source: `src/array/struct_array.rs:77`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

An array of [structs](https://arrow.apache.org/docs/format/Columnar.html#struct-layout)

Each child (called *field*) is represented by a separate array.

# Comparison with [RecordBatch](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)

Both [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) and [`StructArray`](../operations/arrow_array.array.struct_array.StructArray.md#op-994d82c95af72fa0e3aa6207) represent a collection of columns / arrays with the
same length.

However, there are a couple of key differences:

* [`StructArray`](../operations/arrow_array.array.struct_array.StructArray.md#op-994d82c95af72fa0e3aa6207) can be nested within other [`Array`](../operations/arrow_array.array.Array.md#op-cabb9fba5e3e968cee823d21), including itself
* [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) can contain top-level metadata on its associated [`Schema`][arrow_schema::Schema](../operations/arrow_schema.schema.Schema.md#op-144709aa539d6163483c2050)
* [`StructArray`](../operations/arrow_array.array.struct_array.StructArray.md#op-994d82c95af72fa0e3aa6207) can contain top-level nulls, i.e. `null`
* [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) can only represent nulls in its child columns, i.e. `{"field": null}`

[`StructArray`](../operations/arrow_array.array.struct_array.StructArray.md#op-994d82c95af72fa0e3aa6207) is therefore a more general data container than [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34), and as such
code that needs to handle both will typically share an implementation in terms of
[`StructArray`](../operations/arrow_array.array.struct_array.StructArray.md#op-994d82c95af72fa0e3aa6207) and convert to/from [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) as necessary.

[`From`] implementations are provided to facilitate this conversion, however, converting
from a [`StructArray`](../operations/arrow_array.array.struct_array.StructArray.md#op-994d82c95af72fa0e3aa6207) containing top-level nulls to a [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) will panic, as there
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

Unresolved upstream links (retained, not inferred): ``From``.

<a id="op-fb64bddfb5f3ae140250994d"></a>
## Error

`assoc_type` · `arrow_array::array::struct_array::StructArray::Error` · arrow-array 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::struct_array::StructArray", "path": "StructArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [463, 1], "end": [480, 2], "filename": "src/array/struct_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"tuple": [{"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"primitive": "str"}}}, {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"dyn_trait": {"lifetime": null, "traits": [{"generic_params": [], "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}}]}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}]}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/array/struct_array.rs:464`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aa1cda78aa719610f9e26a43"></a>
## Output

`assoc_type` · `arrow_array::array::struct_array::StructArray::Output` · arrow-array 59.3.0

```rust
Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::struct_array::StructArray", "path": "StructArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [610, 1], "end": [625, 2], "filename": "src/array/struct_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"primitive": "str"}}}}], "constraints": []}}, "id": "core::ops::index::Index", "path": "Index"}, "trait_path": "core::ops::index::Index"}`

Source: `src/array/struct_array.rs:611`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e6bf9cafed0dd16557707b05"></a>
## as_any

`function` · `arrow_array::array::struct_array::StructArray::as_any` · arrow-array 59.3.0

```rust
fn as_any(&self) -> &dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::struct_array::StructArray", "path": "StructArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [558, 2], "filename": "src/array/struct_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/struct_array.rs:484`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a159ce660562bfa9ccea61c7"></a>
## claim

`function` · `arrow_array::array::struct_array::StructArray::claim` · arrow-array 59.3.0

```rust
fn claim(&self, pool: &dyn arrow_buffer::MemoryPool)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::struct_array::StructArray", "path": "StructArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [558, 2], "filename": "src/array/struct_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/struct_array.rs:550`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2d5a59b3870dfc56d4f205fb"></a>
## clone

`function` · `arrow_array::array::struct_array::StructArray::clone` · arrow-array 59.3.0

```rust
fn clone(&self) -> StructArray
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::struct_array::StructArray", "path": "StructArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [76, 10], "end": [76, 15], "filename": "src/array/struct_array.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/array/struct_array.rs:76`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fe721cc7a325d22a5ce0808c"></a>
## column

`function` · `arrow_array::array::struct_array::StructArray::column` · arrow-array 59.3.0

```rust
fn column(&self, pos: usize) -> &ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::struct_array::StructArray", "path": "StructArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 1], "end": [423, 2], "filename": "src/array/struct_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/struct_array.rs:285`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the field at `pos`.

<a id="op-6df0ef165495d067973277b2"></a>
## column_by_name

`function` · `arrow_array::array::struct_array::StructArray::column_by_name` · arrow-array 59.3.0

```rust
fn column_by_name(&self, column_name: &str) -> Option<&ArrayRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::struct_array::StructArray", "path": "StructArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 1], "end": [423, 2], "filename": "src/array/struct_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/struct_array.rs:323`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Return child array whose field name equals to column_name

Note: A schema can currently have duplicate field names, in which case
the first field will always be selected.
This issue will be addressed in [#9205](https://github.com/apache/arrow-rs/issues/9205)

<a id="op-845747a7094fbca9accb1cf1"></a>
## column_names

`function` · `arrow_array::array::struct_array::StructArray::column_names` · arrow-array 59.3.0

```rust
fn column_names(&self) -> Vec<&str>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::struct_array::StructArray", "path": "StructArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 1], "end": [423, 2], "filename": "src/array/struct_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/struct_array.rs:300`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Return field names in this struct array

<a id="op-e2dd0e713630c89b9573f2d8"></a>
## columns

`function` · `arrow_array::array::struct_array::StructArray::columns` · arrow-array 59.3.0

```rust
fn columns(&self) -> &[ArrayRef]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::struct_array::StructArray", "path": "StructArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 1], "end": [423, 2], "filename": "src/array/struct_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/struct_array.rs:295`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the fields of the struct array

<a id="op-c645210530cfd167da44e12f"></a>
## data_type

`function` · `arrow_array::array::struct_array::StructArray::data_type` · arrow-array 59.3.0

```rust
fn data_type(&self) -> &DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::struct_array::StructArray", "path": "StructArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [558, 2], "filename": "src/array/struct_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/struct_array.rs:496`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-42248f40370467b91b215951"></a>
## eq

`function` · `arrow_array::array::struct_array::StructArray::eq` · arrow-array 59.3.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::struct_array::StructArray", "path": "StructArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [842, 1], "end": [846, 2], "filename": "src/array/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/array/mod.rs:843`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4e48bd23867874333d932bfa"></a>
## field

`function` · `arrow_array::array::struct_array::StructArray::field` · arrow-array 59.3.0

```rust
fn field(&self, pos: usize) -> &FieldRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::struct_array::StructArray", "path": "StructArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 1], "end": [423, 2], "filename": "src/array/struct_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/struct_array.rs:330`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the [`FieldRef`](../operations/arrow_schema.field.FieldRef.md#op-a0fdaf7a91a3563923566542) at `pos`.

<a id="op-f2d1d895d089963d38b3f179"></a>
## field_by_name

`function` · `arrow_array::array::struct_array::StructArray::field_by_name` · arrow-array 59.3.0

```rust
fn field_by_name(&self, field_name: &str) -> Option<&FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::struct_array::StructArray", "path": "StructArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 1], "end": [423, 2], "filename": "src/array/struct_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/struct_array.rs:339`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Return the [`FieldRef`](../operations/arrow_schema.field.FieldRef.md#op-a0fdaf7a91a3563923566542) whose name equals to `field_name`

Note: A schema can currently have duplicate field names, in which case
the first field will always be selected.
This issue will be addressed in [#9205](https://github.com/apache/arrow-rs/issues/9205)

<a id="op-54978b1fc8189f90560da2a6"></a>
## fields

`function` · `arrow_array::array::struct_array::StructArray::fields` · arrow-array 59.3.0

```rust
fn fields(&self) -> &Fields
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::struct_array::StructArray", "path": "StructArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 1], "end": [423, 2], "filename": "src/array/struct_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/struct_array.rs:311`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the [`Fields`](../operations/arrow_schema.fields.Fields.md#op-8db4115e48dc67eeb9be6faa) of this [`StructArray`](../operations/arrow_array.array.struct_array.StructArray.md#op-994d82c95af72fa0e3aa6207)

<a id="op-8cc7a75901600d5317e82a8b"></a>
## flatten

`function` · `arrow_array::array::struct_array::StructArray::flatten` · arrow-array 59.3.0

```rust
fn flatten(&self) -> (Fields, Vec<ArrayRef>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::struct_array::StructArray", "path": "StructArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 1], "end": [423, 2], "filename": "src/array/struct_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/struct_array.rs:389`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the children of this [`StructArray`](../operations/arrow_array.array.struct_array.StructArray.md#op-994d82c95af72fa0e3aa6207) with the struct's validity
bitmap AND'd into each child's validity bitmap.

This ensures that positions where the struct itself is null are also
null in each returned child array. Fields that were non-nullable are
marked nullable in the returned [`Fields`](../operations/arrow_schema.fields.Fields.md#op-8db4115e48dc67eeb9be6faa) when the struct has nulls.

If the struct has no nulls, children and fields are returned as-is.

This mirrors the semantics of C++ Arrow's `StructArray::Flatten`.

# Example

```
# use std::sync::Arc;
# use arrow_array::{Array, ArrayRef, Int32Array, StructArray};
# use arrow_buffer::{BooleanBuffer, NullBuffer};
# use arrow_schema::{DataType, Field, Fields};
let child = Arc::new(Int32Array::from(vec![1, 2, 3])) as ArrayRef;
let struct_nulls = NullBuffer::new(BooleanBuffer::from(vec![true, false, true]));
let sa = StructArray::new(
    Fields::from(vec![Field::new("a", DataType::Int32, false)]),
    vec![child],
    Some(struct_nulls),
);
let (fields, columns) = sa.flatten();
assert!(fields[0].is_nullable());
assert!(columns[0].is_null(1));
```

<a id="op-2a57c4cc801a784e7ca75391"></a>
## fmt

`function` · `arrow_array::array::struct_array::StructArray::fmt` · arrow-array 59.3.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::struct_array::StructArray", "path": "StructArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [567, 1], "end": [588, 2], "filename": "src/array/struct_array.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/array/struct_array.rs:568`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1ad55851f1b24686a8e68e76"></a>
## from

`function` · `arrow_array::array::struct_array::StructArray::from` · arrow-array 59.3.0

```rust
fn from(data: ArrayData) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::struct_array::StructArray", "path": "StructArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [425, 1], "end": [450, 2], "filename": "src/array/struct_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayData", "path": "ArrayData"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/struct_array.rs:426`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1d7148f0ee760c9d3c7ae405"></a>
## from

`function` · `arrow_array::array::struct_array::StructArray::from` · arrow-array 59.3.0

```rust
fn from(v: Vec<(FieldRef, ArrayRef)>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::struct_array::StructArray", "path": "StructArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [560, 1], "end": [565, 2], "filename": "src/array/struct_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"tuple": [{"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}, {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"dyn_trait": {"lifetime": null, "traits": [{"generic_params": [], "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}}]}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}]}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/struct_array.rs:561`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-64d42b7372ab1c00c8daa4ed"></a>
## from

`function` · `arrow_array::array::struct_array::StructArray::from` · arrow-array 59.3.0

```rust
fn from(value: RecordBatch) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::struct_array::StructArray", "path": "StructArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [599, 1], "end": [608, 2], "filename": "src/array/struct_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatch", "path": "RecordBatch"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/struct_array.rs:600`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-79a1f8ba143780f2be8358e2"></a>
## from

`function` · `arrow_array::array::struct_array::StructArray::from` · arrow-array 59.3.0

```rust
fn from(pair: (Vec<(FieldRef, ArrayRef)>, Buffer)) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::struct_array::StructArray", "path": "StructArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [590, 1], "end": [597, 2], "filename": "src/array/struct_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"tuple": [{"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"tuple": [{"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}, {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"dyn_trait": {"lifetime": null, "traits": [{"generic_params": [], "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}}]}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}]}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}, {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::immutable::Buffer", "path": "Buffer"}}]}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/struct_array.rs:591`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3b61816b2ce71cf084454f5a"></a>
## get_array_memory_size

`function` · `arrow_array::array::struct_array::StructArray::get_array_memory_size` · arrow-array 59.3.0

```rust
fn get_array_memory_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::struct_array::StructArray", "path": "StructArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [558, 2], "filename": "src/array/struct_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/struct_array.rs:540`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1186e5b0f79240e6b23f6d67"></a>
## get_buffer_memory_size

`function` · `arrow_array::array::struct_array::StructArray::get_buffer_memory_size` · arrow-array 59.3.0

```rust
fn get_buffer_memory_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::struct_array::StructArray", "path": "StructArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [558, 2], "filename": "src/array/struct_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/struct_array.rs:532`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d5fad82f14720f8037e78928"></a>
## index

`function` · `arrow_array::array::struct_array::StructArray::index` · arrow-array 59.3.0

```rust
fn index(&self, name: &str) -> &Self::Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::struct_array::StructArray", "path": "StructArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [610, 1], "end": [625, 2], "filename": "src/array/struct_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"primitive": "str"}}}}], "constraints": []}}, "id": "core::ops::index::Index", "path": "Index"}, "trait_path": "core::ops::index::Index"}`

Source: `src/array/struct_array.rs:622`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Get a reference to a column's array by name.

Note: A schema can currently have duplicate field names, in which case
the first field will always be selected.
This issue will be addressed in [ARROW-11178](https://issues.apache.org/jira/browse/ARROW-11178)

# Panics

Panics if the name is not in the schema.

<a id="op-6350c6b817b44ca020445533"></a>
## into_data

`function` · `arrow_array::array::struct_array::StructArray::into_data` · arrow-array 59.3.0

```rust
fn into_data(self) -> ArrayData
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::struct_array::StructArray", "path": "StructArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [558, 2], "filename": "src/array/struct_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/struct_array.rs:492`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-927381b38ff5e9a84052fc01"></a>
## into_parts

`function` · `arrow_array::array::struct_array::StructArray::into_parts` · arrow-array 59.3.0

```rust
fn into_parts(self) -> (Fields, Vec<ArrayRef>, Option<NullBuffer>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::struct_array::StructArray", "path": "StructArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 1], "end": [423, 2], "filename": "src/array/struct_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/struct_array.rs:276`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Deconstruct this array into its constituent parts

<a id="op-e55a4fac22cb2694fc101ec7"></a>
## is_empty

`function` · `arrow_array::array::struct_array::StructArray::is_empty` · arrow-array 59.3.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::struct_array::StructArray", "path": "StructArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [558, 2], "filename": "src/array/struct_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/struct_array.rs:508`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-99762001abb5314082850635"></a>
## len

`function` · `arrow_array::array::struct_array::StructArray::len` · arrow-array 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::struct_array::StructArray", "path": "StructArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [558, 2], "filename": "src/array/struct_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/struct_array.rs:504`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-334ec35a7e748d42b822700e"></a>
## logical_null_count

`function` · `arrow_array::array::struct_array::StructArray::logical_null_count` · arrow-array 59.3.0

```rust
fn logical_null_count(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::struct_array::StructArray", "path": "StructArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [558, 2], "filename": "src/array/struct_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/struct_array.rs:527`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2598e406029f896b6ca7050e"></a>
## new

`function` · `arrow_array::array::struct_array::StructArray::new` · arrow-array 59.3.0

```rust
fn new(fields: Fields, arrays: Vec<ArrayRef>, nulls: Option<NullBuffer>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::struct_array::StructArray", "path": "StructArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 1], "end": [423, 2], "filename": "src/array/struct_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/struct_array.rs:90`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`StructArray`](../operations/arrow_array.array.struct_array.StructArray.md#op-994d82c95af72fa0e3aa6207) from the provided parts, panicking on failure

# Panics

Panics if [`Self::try_new`](../operations/arrow_array.array.struct_array.StructArray.md#op-2c2d2d017833ba5435783b13) returns an error

<a id="op-5d5726b2a80ad09e7af956d1"></a>
## new_empty_fields

`function` · `arrow_array::array::struct_array::StructArray::new_empty_fields` · arrow-array 59.3.0

```rust
fn new_empty_fields(len: usize, nulls: Option<NullBuffer>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::struct_array::StructArray", "path": "StructArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 1], "end": [423, 2], "filename": "src/array/struct_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/struct_array.rs:263`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`StructArray`](../operations/arrow_array.array.struct_array.StructArray.md#op-994d82c95af72fa0e3aa6207) containing no fields

# Panics

If `len != nulls.len()`

<a id="op-4a4ae52b743caaf47b9a7028"></a>
## new_null

`function` · `arrow_array::array::struct_array::StructArray::new_null` · arrow-array 59.3.0

```rust
fn new_null(fields: Fields, len: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::struct_array::StructArray", "path": "StructArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 1], "end": [423, 2], "filename": "src/array/struct_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/struct_array.rs:192`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`StructArray`](../operations/arrow_array.array.struct_array.StructArray.md#op-994d82c95af72fa0e3aa6207) of length `len` where all values are null

<a id="op-042055dd490587e0415c43bb"></a>
## new_unchecked

`function` · `arrow_array::array::struct_array::StructArray::new_unchecked` · arrow-array 59.3.0

```rust
unsafe fn new_unchecked(fields: Fields, arrays: Vec<ArrayRef>, nulls: Option<NullBuffer>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::struct_array::StructArray", "path": "StructArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 1], "end": [423, 2], "filename": "src/array/struct_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/struct_array.rs:215`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`StructArray`](../operations/arrow_array.array.struct_array.StructArray.md#op-994d82c95af72fa0e3aa6207) from the provided parts without validation

The length will be inferred from the length of the child arrays.  Panics if there are no
child arrays.  Consider using [`Self::new_unchecked_with_length`](../operations/arrow_array.array.struct_array.StructArray.md#op-80745b6bfb265c5308535e7d) if the length is known
to avoid this.

# Safety

Safe if [`Self::new`](../operations/arrow_array.array.struct_array.StructArray.md#op-2598e406029f896b6ca7050e) would not panic with the given arguments

<a id="op-80745b6bfb265c5308535e7d"></a>
## new_unchecked_with_length

`function` · `arrow_array::array::struct_array::StructArray::new_unchecked_with_length` · arrow-array 59.3.0

```rust
unsafe fn new_unchecked_with_length(fields: Fields, arrays: Vec<ArrayRef>, nulls: Option<NullBuffer>, len: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::struct_array::StructArray", "path": "StructArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 1], "end": [423, 2], "filename": "src/array/struct_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/struct_array.rs:240`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`StructArray`](../operations/arrow_array.array.struct_array.StructArray.md#op-994d82c95af72fa0e3aa6207) from the provided parts without validation

# Safety

Safe if [`Self::new`](../operations/arrow_array.array.struct_array.StructArray.md#op-2598e406029f896b6ca7050e) would not panic with the given arguments

<a id="op-f614077c2500cfa6774f3368"></a>
## nulls

`function` · `arrow_array::array::struct_array::StructArray::nulls` · arrow-array 59.3.0

```rust
fn nulls(&self) -> Option<&NullBuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::struct_array::StructArray", "path": "StructArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [558, 2], "filename": "src/array/struct_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/struct_array.rs:523`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-28766bac5e49ca083d7a6b10"></a>
## num_columns

`function` · `arrow_array::array::struct_array::StructArray::num_columns` · arrow-array 59.3.0

```rust
fn num_columns(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::struct_array::StructArray", "path": "StructArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 1], "end": [423, 2], "filename": "src/array/struct_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/struct_array.rs:290`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Return the number of fields in this struct array

<a id="op-d14f1163eee4d9b3f495a52f"></a>
## offset

`function` · `arrow_array::array::struct_array::StructArray::offset` · arrow-array 59.3.0

```rust
fn offset(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::struct_array::StructArray", "path": "StructArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [558, 2], "filename": "src/array/struct_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/struct_array.rs:519`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9ea5e7378595ab40d1992ce6"></a>
## shrink_to_fit

`function` · `arrow_array::array::struct_array::StructArray::shrink_to_fit` · arrow-array 59.3.0

```rust
fn shrink_to_fit(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::struct_array::StructArray", "path": "StructArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [558, 2], "filename": "src/array/struct_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/struct_array.rs:512`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1bbde1c8f853112d9d7d759f"></a>
## slice

`function` · `arrow_array::array::struct_array::StructArray::slice` · arrow-array 59.3.0

```rust
fn slice(&self, offset: usize, len: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::struct_array::StructArray", "path": "StructArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 1], "end": [423, 2], "filename": "src/array/struct_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/struct_array.rs:344`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a zero-copy slice of this array with the indicated offset and length.

<a id="op-8ea74e94ee7f8a57f73fa5aa"></a>
## slice

`function` · `arrow_array::array::struct_array::StructArray::slice` · arrow-array 59.3.0

```rust
fn slice(&self, offset: usize, length: usize) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::struct_array::StructArray", "path": "StructArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [558, 2], "filename": "src/array/struct_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/struct_array.rs:500`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5e79994e2461534a49e04bed"></a>
## to_data

`function` · `arrow_array::array::struct_array::StructArray::to_data` · arrow-array 59.3.0

```rust
fn to_data(&self) -> ArrayData
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::struct_array::StructArray", "path": "StructArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [558, 2], "filename": "src/array/struct_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/struct_array.rs:488`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d288fdc6051a6abc3c1823dc"></a>
## try_from

`function` · `arrow_array::array::struct_array::StructArray::try_from` · arrow-array 59.3.0

```rust
fn try_from(values: Vec<(&str, ArrayRef)>) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::struct_array::StructArray", "path": "StructArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [463, 1], "end": [480, 2], "filename": "src/array/struct_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"tuple": [{"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"primitive": "str"}}}, {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"dyn_trait": {"lifetime": null, "traits": [{"generic_params": [], "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}}]}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}]}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/array/struct_array.rs:467`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

builds a StructArray from a vector of names and arrays.

<a id="op-2c2d2d017833ba5435783b13"></a>
## try_new

`function` · `arrow_array::array::struct_array::StructArray::try_new` · arrow-array 59.3.0

```rust
fn try_new(fields: Fields, arrays: Vec<ArrayRef>, nulls: Option<NullBuffer>) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::struct_array::StructArray", "path": "StructArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 1], "end": [423, 2], "filename": "src/array/struct_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/struct_array.rs:106`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`StructArray`](../operations/arrow_array.array.struct_array.StructArray.md#op-994d82c95af72fa0e3aa6207) from the provided parts, returning an error on failure

The length will be inferred from the length of the child arrays.  Returns an error if
there are no child arrays.  Consider using [`Self::try_new_with_length`](../operations/arrow_array.array.struct_array.StructArray.md#op-bcbc5179b72b93429d309bc5) if the length
is known to avoid this.

# Errors

Errors if

* `fields.len() == 0`
* Any reason that [`Self::try_new_with_length`](../operations/arrow_array.array.struct_array.StructArray.md#op-bcbc5179b72b93429d309bc5) would error

<a id="op-bcbc5179b72b93429d309bc5"></a>
## try_new_with_length

`function` · `arrow_array::array::struct_array::StructArray::try_new_with_length` · arrow-array 59.3.0

```rust
fn try_new_with_length(fields: Fields, arrays: Vec<ArrayRef>, nulls: Option<NullBuffer>, len: usize) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::struct_array::StructArray", "path": "StructArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 1], "end": [423, 2], "filename": "src/array/struct_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/struct_array.rs:127`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`StructArray`](../operations/arrow_array.array.struct_array.StructArray.md#op-994d82c95af72fa0e3aa6207) from the provided parts, returning an error on failure

# Errors

Errors if

* `fields.len() != arrays.len()`
* `fields[i].data_type() != arrays[i].data_type()`
* `arrays[i].len() != arrays[j].len()`
* `arrays[i].len() != nulls.len()`
* `!fields[i].is_nullable() && !nulls.contains(arrays[i].nulls())`
