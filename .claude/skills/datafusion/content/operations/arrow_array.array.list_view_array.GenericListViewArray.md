# `arrow_array::array::list_view_array::GenericListViewArray`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.list_view_array.GenericListViewArray.json).

<a id="op-e77399dcef65864e81be3a9a"></a>
## GenericListViewArray

`struct` · `arrow_array::array::list_view_array::GenericListViewArray` · arrow-array 59.3.0

```rust
struct GenericListViewArray<OffsetSize: OffsetSizeTrait>
```

Source: `src/array/list_view_array.rs:108`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

An array of [variable length lists], specifically in the [list-view layout].

Differs from [`GenericListArray`] (which represents the [list layout]) in that
the sizes of the child arrays are explicitly encoded in a separate buffer, instead
of being derived from the difference between subsequent offsets in the offset buffer.

This allows the offsets (and subsequently child data) to be out of order. It also
allows take / filter operations to be implemented without copying the underlying data.

# Representation

Given the same example array from [`GenericListArray`], it would be represented
as such via a list-view layout array:

```text
                                        ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
                                                                        ┌ ─ ─ ─ ─ ─ ─ ┐    │
 ┌─────────────┐  ┌───────┐             │     ┌───┐   ┌───┐   ┌───┐       ┌───┐ ┌───┐
 │   [A,B,C]   │  │ (0,3) │                   │ 1 │   │ 0 │   │ 3 │     │ │ 1 │ │ A │ │ 0  │
 ├─────────────┤  ├───────┤             │     ├───┤   ├───┤   ├───┤       ├───┤ ├───┤
 │      []     │  │ (3,0) │                   │ 1 │   │ 3 │   │ 0 │     │ │ 1 │ │ B │ │ 1  │
 ├─────────────┤  ├───────┤             │     ├───┤   ├───┤   ├───┤       ├───┤ ├───┤
 │    NULL     │  │ (?,?) │                   │ 0 │   │ ? │   │ ? │     │ │ 1 │ │ C │ │ 2  │
 ├─────────────┤  ├───────┤             │     ├───┤   ├───┤   ├───┤       ├───┤ ├───┤
 │     [D]     │  │ (4,1) │                   │ 1 │   │ 4 │   │ 1 │     │ │ ? │ │ ? │ │ 3  │
 ├─────────────┤  ├───────┤             │     ├───┤   ├───┤   ├───┤       ├───┤ ├───┤
 │  [NULL, F]  │  │ (5,2) │                   │ 1 │   │ 5 │   │ 2 │     │ │ 1 │ │ D │ │ 4  │
 └─────────────┘  └───────┘             │     └───┘   └───┘   └───┘       ├───┤ ├───┤
                                                                        │ │ 0 │ │ ? │ │ 5  │
    Logical       Logical               │  Validity  Offsets  Sizes       ├───┤ ├───┤
     Values       Offset                   (nulls)                      │ │ 1 │ │ F │ │ 6  │
                  & Size                │                                 └───┘ └───┘
                                                                        │    Values   │    │
                (offsets[i],            │   ListViewArray                   (Array)
                 sizes[i])                                              └ ─ ─ ─ ─ ─ ─ ┘    │
                                        └ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
```

Another way of representing the same array but taking advantage of the offsets being out of order:

```text
                                        ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
                                                                        ┌ ─ ─ ─ ─ ─ ─ ┐    │
 ┌─────────────┐  ┌───────┐             │     ┌───┐   ┌───┐   ┌───┐       ┌───┐ ┌───┐
 │   [A,B,C]   │  │ (2,3) │                   │ 1 │   │ 2 │   │ 3 │     │ │ 0 │ │ ? │ │ 0  │
 ├─────────────┤  ├───────┤             │     ├───┤   ├───┤   ├───┤       ├───┤ ├───┤
 │      []     │  │ (0,0) │                   │ 1 │   │ 0 │   │ 0 │     │ │ 1 │ │ F │ │ 1  │
 ├─────────────┤  ├───────┤             │     ├───┤   ├───┤   ├───┤       ├───┤ ├───┤
 │    NULL     │  │ (?,?) │                   │ 0 │   │ ? │   │ ? │     │ │ 1 │ │ A │ │ 2  │
 ├─────────────┤  ├───────┤             │     ├───┤   ├───┤   ├───┤       ├───┤ ├───┤
 │     [D]     │  │ (5,1) │                   │ 1 │   │ 5 │   │ 1 │     │ │ 1 │ │ B │ │ 3  │
 ├─────────────┤  ├───────┤             │     ├───┤   ├───┤   ├───┤       ├───┤ ├───┤
 │  [NULL, F]  │  │ (0,2) │                   │ 1 │   │ 0 │   │ 2 │     │ │ 1 │ │ C │ │ 4  │
 └─────────────┘  └───────┘             │     └───┘   └───┘   └───┘       ├───┤ ├───┤
                                                                        │ │ 1 │ │ D │ │ 5  │
    Logical       Logical               │  Validity  Offsets  Sizes       └───┘ └───┘
     Values       Offset                   (nulls)                      │    Values   │    │
                  & Size                │                                   (Array)
                                                                        └ ─ ─ ─ ─ ─ ─ ┘    │
                (offsets[i],            │   ListViewArray
                 sizes[i])                                                                 │
                                        └ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
```

[`GenericListArray`]: crate::array::GenericListArray
[variable length lists]: https://arrow.apache.org/docs/format/Columnar.html#variable-size-list-layout
[list layout]: https://arrow.apache.org/docs/format/Columnar.html#list-layout
[list-view layout]: https://arrow.apache.org/docs/format/Columnar.html#listview-layout

<a id="op-bdb1844ee3de1ec65749a568"></a>
## DATA_TYPE_CONSTRUCTOR

`assoc_const` · `arrow_array::array::list_view_array::GenericListViewArray::DATA_TYPE_CONSTRUCTOR` · arrow-array 59.3.0

```rust
DATA_TYPE_CONSTRUCTOR
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_view_array::GenericListViewArray", "path": "GenericListViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [433, 2], "filename": "src/array/list_view_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/list_view_array.rs:122`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

The data type constructor of listview array.
The input is the schema of the child array and
the output is the [`DataType`](../operations/arrow_schema.datatype.DataType.md#op-bf69df5b14436e006d3a531c), ListView or LargeListView.

<a id="op-5c6c9edb36e444f4ae97a08d"></a>
## as_any

`function` · `arrow_array::array::list_view_array::GenericListViewArray::as_any` · arrow-array 59.3.0

```rust
fn as_any(&self) -> &dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_view_array::GenericListViewArray", "path": "GenericListViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [448, 1], "end": [528, 2], "filename": "src/array/list_view_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/list_view_array.rs:449`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e7445da7d1a01a1ef155c999"></a>
## claim

`function` · `arrow_array::array::list_view_array::GenericListViewArray::claim` · arrow-array 59.3.0

```rust
fn claim(&self, pool: &dyn arrow_buffer::MemoryPool)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_view_array::GenericListViewArray", "path": "GenericListViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [448, 1], "end": [528, 2], "filename": "src/array/list_view_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/list_view_array.rs:520`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-37c008db9ec0c000b4ab323a"></a>
## clone

`function` · `arrow_array::array::list_view_array::GenericListViewArray::clone` · arrow-array 59.3.0

```rust
fn clone(&self) -> GenericListViewArray<OffsetSize>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_view_array::GenericListViewArray", "path": "GenericListViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "$crate::clone::Clone"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 10], "end": [107, 15], "filename": "src/array/list_view_array.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/array/list_view_array.rs:107`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c9150dfb8471cdb80ebcd992"></a>
## data_type

`function` · `arrow_array::array::list_view_array::GenericListViewArray::data_type` · arrow-array 59.3.0

```rust
fn data_type(&self) -> &DataType
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_view_array::GenericListViewArray", "path": "GenericListViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [448, 1], "end": [528, 2], "filename": "src/array/list_view_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/list_view_array.rs:461`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bfa4742f04876178393ed4c6"></a>
## element_range

`function` · `arrow_array::array::list_view_array::GenericListViewArray::element_range` · arrow-array 59.3.0

```rust
fn element_range(&self, index: usize) -> std::ops::Range<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_view_array::GenericListViewArray", "path": "GenericListViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [530, 1], "end": [540, 2], "filename": "src/array/list_view_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::ListLikeArray", "path": "ListLikeArray"}, "trait_path": "arrow_array::array::ListLikeArray"}`

Source: `src/array/list_view_array.rs:535`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-867a209f94bd68413b501dc3"></a>
## eq

`function` · `arrow_array::array::list_view_array::GenericListViewArray::eq` · arrow-array 59.3.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_view_array::GenericListViewArray", "path": "GenericListViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [824, 1], "end": [828, 2], "filename": "src/array/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/array/mod.rs:825`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-48d6d2a438ca7cf6388896f2"></a>
## fmt

`function` · `arrow_array::array::list_view_array::GenericListViewArray::fmt` · arrow-array 59.3.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_view_array::GenericListViewArray", "path": "GenericListViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [542, 1], "end": [551, 2], "filename": "src/array/list_view_array.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/array/list_view_array.rs:543`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3d42e338792f66b3caa6db9b"></a>
## from

`function` · `arrow_array::array::list_view_array::GenericListViewArray::from` · arrow-array 59.3.0

```rust
fn from(value: FixedSizeListArray) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_view_array::GenericListViewArray", "path": "GenericListViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [599, 1], "end": [625, 2], "filename": "src/array/list_view_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_list_array::FixedSizeListArray", "path": "FixedSizeListArray"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/list_view_array.rs:600`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9f24c898fc6f6bdcb2270664"></a>
## from

`function` · `arrow_array::array::list_view_array::GenericListViewArray::from` · arrow-array 59.3.0

```rust
fn from(value: GenericListArray<OffsetSize>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_view_array::GenericListViewArray", "path": "GenericListViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [553, 1], "end": [574, 2], "filename": "src/array/list_view_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_array::GenericListArray", "path": "GenericListArray"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/list_view_array.rs:556`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d0cbe64d1214bc4d5f224ab3"></a>
## from

`function` · `arrow_array::array::list_view_array::GenericListViewArray::from` · arrow-array 59.3.0

```rust
fn from(data: ArrayData) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_view_array::GenericListViewArray", "path": "GenericListViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [592, 1], "end": [597, 2], "filename": "src/array/list_view_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayData", "path": "ArrayData"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/list_view_array.rs:593`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-501d7467fb956dcb510c8e94"></a>
## from_iter_primitive

`function` · `arrow_array::array::list_view_array::GenericListViewArray::from_iter_primitive` · arrow-array 59.3.0

```rust
fn from_iter_primitive<T, P, I>(iter: I) -> Self where T: ArrowPrimitiveType, P: IntoIterator<Item = Option<<T as ArrowPrimitiveType>::Native>>, I: IntoIterator<Item = Option<P>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_view_array::GenericListViewArray", "path": "GenericListViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [433, 2], "filename": "src/array/list_view_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/list_view_array.rs:409`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a [`GenericListViewArray`](../operations/arrow_array.array.list_view_array.GenericListViewArray.md#op-e77399dcef65864e81be3a9a) from an iterator of primitive values
# Example
```
# use arrow_array::ListViewArray;
# use arrow_array::types::Int32Type;

let data = vec![
   Some(vec![Some(0), Some(1), Some(2)]),
   None,
   Some(vec![Some(3), None, Some(5)]),
   Some(vec![Some(6), Some(7)]),
];
let list_array = ListViewArray::from_iter_primitive::<Int32Type, _, _>(data);
println!("{:?}", list_array);
```

<a id="op-931961b8ea0e797b76479adb"></a>
## get_array_memory_size

`function` · `arrow_array::array::list_view_array::GenericListViewArray::get_array_memory_size` · arrow-array 59.3.0

```rust
fn get_array_memory_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_view_array::GenericListViewArray", "path": "GenericListViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [448, 1], "end": [528, 2], "filename": "src/array/list_view_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/list_view_array.rs:509`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-568e287c6d897c6296a0243a"></a>
## get_buffer_memory_size

`function` · `arrow_array::array::list_view_array::GenericListViewArray::get_buffer_memory_size` · arrow-array 59.3.0

```rust
fn get_buffer_memory_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_view_array::GenericListViewArray", "path": "GenericListViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [448, 1], "end": [528, 2], "filename": "src/array/list_view_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/list_view_array.rs:499`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-13151008fb5d74e5f6b3c448"></a>
## into_data

`function` · `arrow_array::array::list_view_array::GenericListViewArray::into_data` · arrow-array 59.3.0

```rust
fn into_data(self) -> ArrayData
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_view_array::GenericListViewArray", "path": "GenericListViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [448, 1], "end": [528, 2], "filename": "src/array/list_view_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/list_view_array.rs:457`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-47d7f80c307bc28e800dff62"></a>
## into_parts

`function` · `arrow_array::array::list_view_array::GenericListViewArray::into_parts` · arrow-array 59.3.0

```rust
fn into_parts(self) -> (FieldRef, ScalarBuffer<OffsetSize>, ScalarBuffer<OffsetSize>, ArrayRef, Option<NullBuffer>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_view_array::GenericListViewArray", "path": "GenericListViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [433, 2], "filename": "src/array/list_view_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/list_view_array.rs:268`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Deconstruct this array into its constituent parts

<a id="op-a337d445baac10781bf90e74"></a>
## is_empty

`function` · `arrow_array::array::list_view_array::GenericListViewArray::is_empty` · arrow-array 59.3.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_view_array::GenericListViewArray", "path": "GenericListViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [448, 1], "end": [528, 2], "filename": "src/array/list_view_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/list_view_array.rs:473`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f916266afc38593692a07727"></a>
## iter

`function` · `arrow_array::array::list_view_array::GenericListViewArray::iter` · arrow-array 59.3.0

```rust
fn iter(&self) -> GenericListViewArrayIter<'_, OffsetSize>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_view_array::GenericListViewArray", "path": "GenericListViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [433, 2], "filename": "src/array/list_view_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/list_view_array.rs:369`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Constructs a new iterator

<a id="op-78f2957c90775e0d7f521aab"></a>
## len

`function` · `arrow_array::array::list_view_array::GenericListViewArray::len` · arrow-array 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_view_array::GenericListViewArray", "path": "GenericListViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [448, 1], "end": [528, 2], "filename": "src/array/list_view_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/list_view_array.rs:469`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7ba68eaaaa996bd74d7d3050"></a>
## logical_null_count

`function` · `arrow_array::array::list_view_array::GenericListViewArray::logical_null_count` · arrow-array 59.3.0

```rust
fn logical_null_count(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_view_array::GenericListViewArray", "path": "GenericListViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [448, 1], "end": [528, 2], "filename": "src/array/list_view_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/list_view_array.rs:494`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bcb9da7cedb09aa041547b8c"></a>
## new

`function` · `arrow_array::array::list_view_array::GenericListViewArray::new` · arrow-array 59.3.0

```rust
fn new(field: FieldRef, offsets: ScalarBuffer<OffsetSize>, sizes: ScalarBuffer<OffsetSize>, values: ArrayRef, nulls: Option<NullBuffer>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_view_array::GenericListViewArray", "path": "GenericListViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [433, 2], "filename": "src/array/list_view_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/list_view_array.rs:216`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`GenericListViewArray`](../operations/arrow_array.array.list_view_array.GenericListViewArray.md#op-e77399dcef65864e81be3a9a) from the provided parts

# Panics

Panics if [`Self::try_new`](../operations/arrow_array.array.list_view_array.GenericListViewArray.md#op-b02818869e9b1bb44d6108af) returns an error

<a id="op-335707a14d69395897b36af8"></a>
## new_null

`function` · `arrow_array::array::list_view_array::GenericListViewArray::new_null` · arrow-array 59.3.0

```rust
fn new_null(field: FieldRef, len: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_view_array::GenericListViewArray", "path": "GenericListViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [433, 2], "filename": "src/array/list_view_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/list_view_array.rs:256`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`GenericListViewArray`](../operations/arrow_array.array.list_view_array.GenericListViewArray.md#op-e77399dcef65864e81be3a9a) of length `len` where all values are null

<a id="op-7fb3694cc105ee289ebd1388"></a>
## new_unchecked

`function` · `arrow_array::array::list_view_array::GenericListViewArray::new_unchecked` · arrow-array 59.3.0

```rust
unsafe fn new_unchecked(field: FieldRef, offsets: ScalarBuffer<OffsetSize>, sizes: ScalarBuffer<OffsetSize>, values: ArrayRef, nulls: Option<NullBuffer>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_view_array::GenericListViewArray", "path": "GenericListViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [433, 2], "filename": "src/array/list_view_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/list_view_array.rs:235`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`GenericListViewArray`](../operations/arrow_array.array.list_view_array.GenericListViewArray.md#op-e77399dcef65864e81be3a9a) from the provided parts without validation

See [`Self::try_new`](../operations/arrow_array.array.list_view_array.GenericListViewArray.md#op-b02818869e9b1bb44d6108af) for the checked version of this function, and the
documentation of that function for the invariants that must be upheld.

# Safety

The parts must form a valid [`ListViewArray`](../operations/arrow_array.array.list_view_array.ListViewArray.md#op-f804ab17e2d4b64e9d2e4b55) or [`LargeListViewArray`](../operations/arrow_array.array.list_view_array.LargeListViewArray.md#op-184699cca31c6f96df389081) according
to the Arrow spec.

<a id="op-16a4f3e169b7f3d9300b4716"></a>
## nulls

`function` · `arrow_array::array::list_view_array::GenericListViewArray::nulls` · arrow-array 59.3.0

```rust
fn nulls(&self) -> Option<&NullBuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_view_array::GenericListViewArray", "path": "GenericListViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [448, 1], "end": [528, 2], "filename": "src/array/list_view_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/list_view_array.rs:490`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-61ffcb68a565b87c884fe47c"></a>
## offset

`function` · `arrow_array::array::list_view_array::GenericListViewArray::offset` · arrow-array 59.3.0

```rust
fn offset(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_view_array::GenericListViewArray", "path": "GenericListViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [448, 1], "end": [528, 2], "filename": "src/array/list_view_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/list_view_array.rs:486`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-37ebfed1dce22fc443bec2f3"></a>
## offsets

`function` · `arrow_array::array::list_view_array::GenericListViewArray::offsets` · arrow-array 59.3.0

```rust
fn offsets(&self) -> &ScalarBuffer<OffsetSize>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_view_array::GenericListViewArray", "path": "GenericListViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [433, 2], "filename": "src/array/list_view_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/list_view_array.rs:295`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a reference to the offsets of this list

Unlike [`Self::value_offsets`](../operations/arrow_array.array.list_view_array.GenericListViewArray.md#op-5efbec5634e05b9fe856cb24) this returns the [`ScalarBuffer`](../operations/arrow_buffer.buffer.scalar.ScalarBuffer.md#op-3edbfa8eafafb64c0414f247)
allowing for zero-copy cloning

<a id="op-bdcaf755fa568821d5e39342"></a>
## shrink_to_fit

`function` · `arrow_array::array::list_view_array::GenericListViewArray::shrink_to_fit` · arrow-array 59.3.0

```rust
fn shrink_to_fit(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_view_array::GenericListViewArray", "path": "GenericListViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [448, 1], "end": [528, 2], "filename": "src/array/list_view_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/list_view_array.rs:477`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-65039e5b9a8159d289f3d338"></a>
## sizes

`function` · `arrow_array::array::list_view_array::GenericListViewArray::sizes` · arrow-array 59.3.0

```rust
fn sizes(&self) -> &ScalarBuffer<OffsetSize>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_view_array::GenericListViewArray", "path": "GenericListViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [433, 2], "filename": "src/array/list_view_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/list_view_array.rs:310`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a reference to the sizes of this list

Unlike [`Self::value_sizes`](../operations/arrow_array.array.list_view_array.GenericListViewArray.md#op-8aeba42a62b9f6a64498e879) this returns the [`ScalarBuffer`](../operations/arrow_buffer.buffer.scalar.ScalarBuffer.md#op-3edbfa8eafafb64c0414f247)
allowing for zero-copy cloning

<a id="op-53d8a88f009c686e304b15d8"></a>
## slice

`function` · `arrow_array::array::list_view_array::GenericListViewArray::slice` · arrow-array 59.3.0

```rust
fn slice(&self, offset: usize, length: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_view_array::GenericListViewArray", "path": "GenericListViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [433, 2], "filename": "src/array/list_view_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/list_view_array.rs:384`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a zero-copy slice of this array with the indicated offset and length.

<a id="op-bd38720081ccb8cb0a69d533"></a>
## slice

`function` · `arrow_array::array::list_view_array::GenericListViewArray::slice` · arrow-array 59.3.0

```rust
fn slice(&self, offset: usize, length: usize) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_view_array::GenericListViewArray", "path": "GenericListViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [448, 1], "end": [528, 2], "filename": "src/array/list_view_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/list_view_array.rs:465`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-03dcf02061b1b8d4793a9438"></a>
## to_data

`function` · `arrow_array::array::list_view_array::GenericListViewArray::to_data` · arrow-array 59.3.0

```rust
fn to_data(&self) -> ArrayData
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_view_array::GenericListViewArray", "path": "GenericListViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [448, 1], "end": [528, 2], "filename": "src/array/list_view_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/list_view_array.rs:453`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b02818869e9b1bb44d6108af"></a>
## try_new

`function` · `arrow_array::array::list_view_array::GenericListViewArray::try_new` · arrow-array 59.3.0

```rust
fn try_new(field: FieldRef, offsets: ScalarBuffer<OffsetSize>, sizes: ScalarBuffer<OffsetSize>, values: ArrayRef, nulls: Option<NullBuffer>) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_view_array::GenericListViewArray", "path": "GenericListViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [433, 2], "filename": "src/array/list_view_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/list_view_array.rs:141`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`GenericListViewArray`](../operations/arrow_array.array.list_view_array.GenericListViewArray.md#op-e77399dcef65864e81be3a9a) from the provided parts

# Errors

Errors if

* `offsets.len() != sizes.len()`
* `offsets.len() != nulls.len()`
* `offsets[i] > values.len()`
* `!field.is_nullable() && values.is_nullable()`
* `field.data_type() != values.data_type()`
* `0 <= offsets[i] <= length of the child array`
* `0 <= offsets[i] + size[i] <= length of the child array`

<a id="op-94529d9e668a4b3945e33a31"></a>
## value

`function` · `arrow_array::array::list_view_array::GenericListViewArray::value` · arrow-array 59.3.0

```rust
fn value(&self, i: usize) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_view_array::GenericListViewArray", "path": "GenericListViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [433, 2], "filename": "src/array/list_view_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/list_view_array.rs:339`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns ith value of this list view array.

Note: This method does not check for nulls and the value is arbitrary
(but still well-defined) if [`is_null`](Self::is_null) returns true for the index.

# Panics
Panics if the index is out of bounds

<a id="op-ee2a555c1c6dd48da5172ac7"></a>
## value_offset

`function` · `arrow_array::array::list_view_array::GenericListViewArray::value_offset` · arrow-array 59.3.0

```rust
fn value_offset(&self, i: usize) -> OffsetSize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_view_array::GenericListViewArray", "path": "GenericListViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [433, 2], "filename": "src/array/list_view_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/list_view_array.rs:364`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the offset for value at index `i`.

<a id="op-5efbec5634e05b9fe856cb24"></a>
## value_offsets

`function` · `arrow_array::array::list_view_array::GenericListViewArray::value_offsets` · arrow-array 59.3.0

```rust
fn value_offsets(&self) -> &[OffsetSize]
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_view_array::GenericListViewArray", "path": "GenericListViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [433, 2], "filename": "src/array/list_view_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/list_view_array.rs:347`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the offset values in the offsets buffer

<a id="op-c2229251bb6369889990f3bf"></a>
## value_size

`function` · `arrow_array::array::list_view_array::GenericListViewArray::value_size` · arrow-array 59.3.0

```rust
fn value_size(&self, i: usize) -> OffsetSize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_view_array::GenericListViewArray", "path": "GenericListViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [433, 2], "filename": "src/array/list_view_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/list_view_array.rs:359`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the size for value at index `i`.

<a id="op-8aeba42a62b9f6a64498e879"></a>
## value_sizes

`function` · `arrow_array::array::list_view_array::GenericListViewArray::value_sizes` · arrow-array 59.3.0

```rust
fn value_sizes(&self) -> &[OffsetSize]
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_view_array::GenericListViewArray", "path": "GenericListViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [433, 2], "filename": "src/array/list_view_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/list_view_array.rs:353`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the sizes values in the offsets buffer

<a id="op-853cdd643ce514d1e1e7dcc6"></a>
## value_type

`function` · `arrow_array::array::list_view_array::GenericListViewArray::value_type` · arrow-array 59.3.0

```rust
fn value_type(&self) -> DataType
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_view_array::GenericListViewArray", "path": "GenericListViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [433, 2], "filename": "src/array/list_view_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/list_view_array.rs:315`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a clone of the value type of this list.

<a id="op-f54b485ff78af5e118ea1cc1"></a>
## value_unchecked

`function` · `arrow_array::array::list_view_array::GenericListViewArray::value_unchecked` · arrow-array 59.3.0

```rust
unsafe fn value_unchecked(&self, i: usize) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_view_array::GenericListViewArray", "path": "GenericListViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [433, 2], "filename": "src/array/list_view_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/list_view_array.rs:326`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns ith value of this list view array.

Note: This method does not check for nulls and the value is arbitrary
if [`is_null`](Self::is_null) returns true for the index.

# Safety
Caller must ensure that the index is within the array bounds

<a id="op-bd6bcf3f2ecf74a9a285812e"></a>
## values

`function` · `arrow_array::array::list_view_array::GenericListViewArray::values` · arrow-array 59.3.0

```rust
fn values(&self) -> &ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_view_array::GenericListViewArray", "path": "GenericListViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [530, 1], "end": [540, 2], "filename": "src/array/list_view_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::ListLikeArray", "path": "ListLikeArray"}, "trait_path": "arrow_array::array::ListLikeArray"}`

Source: `src/array/list_view_array.rs:531`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c0ab9e1bdd9119afa10ac7bb"></a>
## values

`function` · `arrow_array::array::list_view_array::GenericListViewArray::values` · arrow-array 59.3.0

```rust
fn values(&self) -> &ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_view_array::GenericListViewArray", "path": "GenericListViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [433, 2], "filename": "src/array/list_view_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/list_view_array.rs:301`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a reference to the values of this list
