# `arrow_array::array::list_array::GenericListArray`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.list_array.GenericListArray.json).

<a id="op-f5cb16d85a7337f2dafa403b"></a>
## GenericListArray

`struct` · `arrow_array::array::list_array::GenericListArray` · arrow-array 59.3.0

```rust
struct GenericListArray<OffsetSize: OffsetSizeTrait>
```

Source: `src/array/list_array.rs:171`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

An array of [variable length lists], similar to JSON arrays
(e.g. `["A", "B", "C"]`). This struct specifically represents
the [list layout]. Refer to [`GenericListViewArray`] for the
[list-view layout].

Lists are represented using `offsets` into a `values` child
array. Offsets are stored in two adjacent entries of an
[`OffsetBuffer`](../operations/arrow_buffer.buffer.offset.OffsetBuffer.md#op-a91a6a87515fb5ae378e13fc).

Arrow defines [`ListArray`](../operations/arrow_array.array.list_array.ListArray.md#op-fbfbbaae30c81192ec1e4456) with `i32` offsets and
[`LargeListArray`](../operations/arrow_array.array.list_array.LargeListArray.md#op-e34a694617d66e91221407db) with `i64` offsets.

Use [`GenericListBuilder`](../operations/arrow_array.builder.generic_list_builder.GenericListBuilder.md#op-97b03adbae152fb71b763a5d) to construct a [`GenericListArray`](../operations/arrow_array.array.list_array.GenericListArray.md#op-f5cb16d85a7337f2dafa403b).

# Representation

A [`ListArray`](../operations/arrow_array.array.list_array.ListArray.md#op-fbfbbaae30c81192ec1e4456) can represent a list of values of any other
supported Arrow type. Each element of the `ListArray` itself is
a list which may be empty, may contain NULL and non-null values,
or may itself be NULL.

For example, the `ListArray` shown in the following diagram stores
lists of strings. Note that `[]` represents an empty (length
0), but non NULL list.

```text
┌─────────────┐
│   [A,B,C]   │
├─────────────┤
│     []      │
├─────────────┤
│    NULL     │
├─────────────┤
│     [D]     │
├─────────────┤
│  [NULL, F]  │
└─────────────┘
```

The `values` are stored in a child [`StringArray`] and the offsets
are stored in an [`OffsetBuffer`](../operations/arrow_buffer.buffer.offset.OffsetBuffer.md#op-a91a6a87515fb5ae378e13fc) as shown in the following
diagram. The logical values and offsets are shown on the left, and
the actual `ListArray` encoding on the right.

```text
                                        ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
                                                                ┌ ─ ─ ─ ─ ─ ─ ┐    │
 ┌─────────────┐  ┌───────┐             │     ┌───┐   ┌───┐       ┌───┐ ┌───┐
 │   [A,B,C]   │  │ (0,3) │                   │ 1 │   │ 0 │     │ │ 1 │ │ A │ │ 0  │
 ├─────────────┤  ├───────┤             │     ├───┤   ├───┤       ├───┤ ├───┤
 │ [] (empty)  │  │ (3,3) │                   │ 1 │   │ 3 │     │ │ 1 │ │ B │ │ 1  │
 ├─────────────┤  ├───────┤             │     ├───┤   ├───┤       ├───┤ ├───┤
 │    NULL     │  │ (3,3) │                   │ 0 │   │ 3 │     │ │ 1 │ │ C │ │ 2  │
 ├─────────────┤  ├───────┤             │     ├───┤   ├───┤       ├───┤ ├───┤
 │     [D]     │  │ (3,4) │                   │ 1 │   │ 3 │     │ │ 1 │ │ D │ │ 3  │
 ├─────────────┤  ├───────┤             │     ├───┤   ├───┤       ├───┤ ├───┤
 │  [NULL, F]  │  │ (4,6) │                   │ 1 │   │ 4 │     │ │ 0 │ │ ? │ │ 4  │
 └─────────────┘  └───────┘             │     └───┘   ├───┤       ├───┤ ├───┤
                                                      │ 6 │     │ │ 1 │ │ F │ │ 5  │
                                        │  Validity   └───┘       └───┘ └───┘
    Logical       Logical                  (nulls)   Offsets    │    Values   │    │
     Values       Offsets               │                           (Array)
                                                                └ ─ ─ ─ ─ ─ ─ ┘    │
                (offsets[i],            │   ListArray
               offsets[i+1])                                                       │
                                        └ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
```

# Slicing

Slicing a `ListArray` creates a new `ListArray` without copying any data,
but this means the [`Self::values`](../operations/arrow_array.array.list_array.GenericListArray.md#op-13bd6eeaf01add795b79bc57) and [`Self::offsets`](../operations/arrow_array.array.list_array.GenericListArray.md#op-9ec7b023ae1581b417d98555) may have "unused" data

For example, calling `slice(1, 3)` on the `ListArray` in the above example
would result in the following. Note

1. `Values` array is unchanged
2. `Offsets` do not start at `0`, nor cover all values in the Values array.

```text
                                ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
                                                        ┌ ─ ─ ─ ─ ─ ─ ┐    │  ╔═══╗
                                │                         ╔═══╗ ╔═══╗         ║   ║  Not used
                                                        │ ║ 1 ║ ║ A ║ │ 0  │  ╚═══╝
 ┌─────────────┐  ┌───────┐     │     ┌───┐   ┌───┐       ╠═══╣ ╠═══╣
 │ [] (empty)  │  │ (3,3) │           │ 1 │   │ 3 │     │ ║ 1 ║ ║ B ║ │ 1  │
 ├─────────────┤  ├───────┤     │     ├───┤   ├───┤       ╠═══╣ ╠═══╣
 │    NULL     │  │ (3,3) │           │ 0 │   │ 3 │     │ ║ 1 ║ ║ C ║ │ 2  │
 ├─────────────┤  ├───────┤     │     ├───┤   ├───┤       ╚═══╝ ╚═══╝
 │     [D]     │  │ (3,4) │           │ 1 │   │ 3 │     │ │ 1 │ │ D │ │ 3  │
 └─────────────┘  └───────┘     │     └───┘   ├───┤       ╔═══╗ ╔═══╗
                                              │ 4 │     │ ║ 0 ║ ║ ? ║ │ 4  │
                                │             └───┘       ╠═══╣ ╠═══╣
                                                        │ ║ 1 ║ ║ F ║ │ 5  │
                                │  Validity               ╚═══╝ ╚═══╝
    Logical       Logical          (nulls)   Offsets    │    Values   │    │
     Values       Offsets       │                           (Array)
                                                        └ ─ ─ ─ ─ ─ ─ ┘    │
                (offsets[i],    │   ListArray
               offsets[i+1])                                               │
                                └ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
```

[`StringArray`]: crate::array::StringArray
[`GenericListViewArray`]: crate::array::GenericListViewArray
[variable length lists]: https://arrow.apache.org/docs/format/Columnar.html#variable-size-list-layout
[list layout]: https://arrow.apache.org/docs/format/Columnar.html#list-layout
[list-view layout]: https://arrow.apache.org/docs/format/Columnar.html#listview-layout

<a id="op-91c08fa37bd7649e753cb25e"></a>
## DATA_TYPE_CONSTRUCTOR

`assoc_const` · `arrow_array::array::list_array::GenericListArray::DATA_TYPE_CONSTRUCTOR` · arrow-array 59.3.0

```rust
DATA_TYPE_CONSTRUCTOR
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_array::GenericListArray", "path": "GenericListArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [189, 1], "end": [488, 2], "filename": "src/array/list_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/list_array.rs:193`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

The data type constructor of list array.
The input is the schema of the child array and
the output is the [`DataType`](../operations/arrow_schema.datatype.DataType.md#op-bf69df5b14436e006d3a531c), List or LargeList.

<a id="op-328d13d70af58aced6e6a0bf"></a>
## as_any

`function` · `arrow_array::array::list_array::GenericListArray::as_any` · arrow-array 59.3.0

```rust
fn as_any(&self) -> &dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_array::GenericListArray", "path": "GenericListArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [579, 1], "end": [655, 2], "filename": "src/array/list_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/list_array.rs:580`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e7f1f0f30ed17bba9ea9dacd"></a>
## claim

`function` · `arrow_array::array::list_array::GenericListArray::claim` · arrow-array 59.3.0

```rust
fn claim(&self, pool: &dyn arrow_buffer::MemoryPool)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_array::GenericListArray", "path": "GenericListArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [579, 1], "end": [655, 2], "filename": "src/array/list_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/list_array.rs:648`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f9b87913207e45487672122c"></a>
## clone

`function` · `arrow_array::array::list_array::GenericListArray::clone` · arrow-array 59.3.0

```rust
fn clone(&self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_array::GenericListArray", "path": "GenericListArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [178, 1], "end": [187, 2], "filename": "src/array/list_array.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/array/list_array.rs:179`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c18d7515f77ce5f7ef92805"></a>
## data_type

`function` · `arrow_array::array::list_array::GenericListArray::data_type` · arrow-array 59.3.0

```rust
fn data_type(&self) -> &DataType
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_array::GenericListArray", "path": "GenericListArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [579, 1], "end": [655, 2], "filename": "src/array/list_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/list_array.rs:592`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4ae39964b1bb780348960b09"></a>
## element_range

`function` · `arrow_array::array::list_array::GenericListArray::element_range` · arrow-array 59.3.0

```rust
fn element_range(&self, index: usize) -> std::ops::Range<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_array::GenericListArray", "path": "GenericListArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [657, 1], "end": [668, 2], "filename": "src/array/list_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::ListLikeArray", "path": "ListLikeArray"}, "trait_path": "arrow_array::array::ListLikeArray"}`

Source: `src/array/list_array.rs:662`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d9920028cbabc0d9a6899fef"></a>
## eq

`function` · `arrow_array::array::list_array::GenericListArray::eq` · arrow-array 59.3.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_array::GenericListArray", "path": "GenericListArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [818, 1], "end": [822, 2], "filename": "src/array/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/array/mod.rs:819`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f441a6027d8bb101c477008b"></a>
## fmt

`function` · `arrow_array::array::list_array::GenericListArray::fmt` · arrow-array 59.3.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_array::GenericListArray", "path": "GenericListArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [682, 1], "end": [692, 2], "filename": "src/array/list_array.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/array/list_array.rs:683`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-44c226f2d9c1c064a6ae4db1"></a>
## from

`function` · `arrow_array::array::list_array::GenericListArray::from` · arrow-array 59.3.0

```rust
fn from(value: FixedSizeListArray) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_array::GenericListArray", "path": "GenericListArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [510, 1], "end": [526, 2], "filename": "src/array/list_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_list_array::FixedSizeListArray", "path": "FixedSizeListArray"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/list_array.rs:511`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5d9ed18b68fed0298cf9f400"></a>
## from

`function` · `arrow_array::array::list_array::GenericListArray::from` · arrow-array 59.3.0

```rust
fn from(data: ArrayData) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_array::GenericListArray", "path": "GenericListArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [490, 1], "end": [495, 2], "filename": "src/array/list_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayData", "path": "ArrayData"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/list_array.rs:491`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-12c2c028b31508510f12422e"></a>
## from_iter_primitive

`function` · `arrow_array::array::list_array::GenericListArray::from_iter_primitive` · arrow-array 59.3.0

```rust
fn from_iter_primitive<T, P, I>(iter: I) -> Self where T: ArrowPrimitiveType, P: IntoIterator<Item = Option<<T as ArrowPrimitiveType>::Native>>, I: IntoIterator<Item = Option<P>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_array::GenericListArray", "path": "GenericListArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [189, 1], "end": [488, 2], "filename": "src/array/list_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/list_array.rs:439`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a [`GenericListArray`](../operations/arrow_array.array.list_array.GenericListArray.md#op-f5cb16d85a7337f2dafa403b) from an iterator of primitive values
# Example
```
# use arrow_array::ListArray;
# use arrow_array::types::Int32Type;

let data = vec![
   Some(vec![Some(0), Some(1), Some(2)]),
   None,
   Some(vec![Some(3), None, Some(5)]),
   Some(vec![Some(6), Some(7)]),
];
let list_array = ListArray::from_iter_primitive::<Int32Type, _, _>(data);
println!("{:?}", list_array);
```

<a id="op-fd7e6429bc0bee66c2ca7437"></a>
## from_nested_iter

`function` · `arrow_array::array::list_array::GenericListArray::from_nested_iter` · arrow-array 59.3.0

```rust
fn from_nested_iter<B, T, P, I>(iter: I) -> Self where B: ArrayBuilder + Default + Extend<Option<T>>, P: IntoIterator<Item = Option<T>>, I: IntoIterator<Item = Option<P>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_array::GenericListArray", "path": "GenericListArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [189, 1], "end": [488, 2], "filename": "src/array/list_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/list_array.rs:467`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a [`GenericListArray`](../operations/arrow_array.array.list_array.GenericListArray.md#op-f5cb16d85a7337f2dafa403b) from a nested iterator of values.
This method works for any values type that has a corresponding builder that implements the
`Extend` trait. That includes all numeric types, booleans, binary and string types and also
dictionary encoded binary and strings.

# Example
```
# use arrow_array::ListArray;
# use arrow_array::types::Int32Type;
# use arrow_array::builder::StringDictionaryBuilder;
let data = vec![
   Some(vec![Some("foo"), Some("bar"), Some("baz")]),
   None,
   Some(vec![Some("bar"), None, Some("foo")]),
   Some(vec![]),
];
let list_array = ListArray::from_nested_iter::<StringDictionaryBuilder<Int32Type>, _, _, _>(data);
println!("{:?}", list_array);
```

<a id="op-b178b82c3d15c8ac3b32f972"></a>
## get_array_memory_size

`function` · `arrow_array::array::list_array::GenericListArray::get_array_memory_size` · arrow-array 59.3.0

```rust
fn get_array_memory_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_array::GenericListArray", "path": "GenericListArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [579, 1], "end": [655, 2], "filename": "src/array/list_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/list_array.rs:638`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-40434dc9a8fd727cb889abbe"></a>
## get_buffer_memory_size

`function` · `arrow_array::array::list_array::GenericListArray::get_buffer_memory_size` · arrow-array 59.3.0

```rust
fn get_buffer_memory_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_array::GenericListArray", "path": "GenericListArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [579, 1], "end": [655, 2], "filename": "src/array/list_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/list_array.rs:629`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-79122675c80adbb6b0733ed6"></a>
## into_data

`function` · `arrow_array::array::list_array::GenericListArray::into_data` · arrow-array 59.3.0

```rust
fn into_data(self) -> ArrayData
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_array::GenericListArray", "path": "GenericListArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [579, 1], "end": [655, 2], "filename": "src/array/list_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/list_array.rs:588`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b348d2355dd624dded691c0a"></a>
## into_parts

`function` · `arrow_array::array::list_array::GenericListArray::into_parts` · arrow-array 59.3.0

```rust
fn into_parts(self) -> (FieldRef, OffsetBuffer<OffsetSize>, ArrayRef, Option<NullBuffer>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_array::GenericListArray", "path": "GenericListArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [189, 1], "end": [488, 2], "filename": "src/array/list_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/list_array.rs:310`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Deconstruct this array into its constituent parts

<a id="op-6180695d9060f51e44241b3a"></a>
## is_empty

`function` · `arrow_array::array::list_array::GenericListArray::is_empty` · arrow-array 59.3.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_array::GenericListArray", "path": "GenericListArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [579, 1], "end": [655, 2], "filename": "src/array/list_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/list_array.rs:604`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5dfd2ae5e24fbb7b26ee70c6"></a>
## iter

`function` · `arrow_array::array::list_array::GenericListArray::iter` · arrow-array 59.3.0

```rust
fn iter<'a>(&'a self) -> GenericListArrayIter<'a, OffsetSize>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_array::GenericListArray", "path": "GenericListArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [189, 1], "end": [488, 2], "filename": "src/array/list_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/list_array.rs:396`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

constructs a new iterator

<a id="op-85544b1a2ca707e01132078e"></a>
## len

`function` · `arrow_array::array::list_array::GenericListArray::len` · arrow-array 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_array::GenericListArray", "path": "GenericListArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [579, 1], "end": [655, 2], "filename": "src/array/list_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/list_array.rs:600`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d58763e824f75e863b793b59"></a>
## logical_null_count

`function` · `arrow_array::array::list_array::GenericListArray::logical_null_count` · arrow-array 59.3.0

```rust
fn logical_null_count(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_array::GenericListArray", "path": "GenericListArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [579, 1], "end": [655, 2], "filename": "src/array/list_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/list_array.rs:624`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b9bece8511b73316b0612d22"></a>
## new

`function` · `arrow_array::array::list_array::GenericListArray::new` · arrow-array 59.3.0

```rust
fn new(field: FieldRef, offsets: OffsetBuffer<OffsetSize>, values: ArrayRef, nulls: Option<NullBuffer>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_array::GenericListArray", "path": "GenericListArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [189, 1], "end": [488, 2], "filename": "src/array/list_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/list_array.rs:266`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`GenericListArray`](../operations/arrow_array.array.list_array.GenericListArray.md#op-f5cb16d85a7337f2dafa403b) from the provided parts

# Panics

Panics if [`Self::try_new`](../operations/arrow_array.array.list_array.GenericListArray.md#op-a8fcab958704ce1093cba4ea) returns an error

<a id="op-a3176a4b2b76b760d13d0a51"></a>
## new_null

`function` · `arrow_array::array::list_array::GenericListArray::new_null` · arrow-array 59.3.0

```rust
fn new_null(field: FieldRef, len: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_array::GenericListArray", "path": "GenericListArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [189, 1], "end": [488, 2], "filename": "src/array/list_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/list_array.rs:299`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`GenericListArray`](../operations/arrow_array.array.list_array.GenericListArray.md#op-f5cb16d85a7337f2dafa403b) of length `len` where all values are null

<a id="op-886af35ad75bcda026bf35f5"></a>
## new_unchecked

`function` · `arrow_array::array::list_array::GenericListArray::new_unchecked` · arrow-array 59.3.0

```rust
unsafe fn new_unchecked(field: FieldRef, offsets: OffsetBuffer<OffsetSize>, values: ArrayRef, nulls: Option<NullBuffer>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_array::GenericListArray", "path": "GenericListArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [189, 1], "end": [488, 2], "filename": "src/array/list_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/list_array.rs:281`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`GenericListArray`](../operations/arrow_array.array.list_array.GenericListArray.md#op-f5cb16d85a7337f2dafa403b) from the provided parts without validation.

# Safety
- `offsets.len() - 1 == nulls.len()` if `nulls` is `Some`
- `offsets.last() <= values.len()`
- `field.data_type() == values.data_type()`

<a id="op-0e814db815500a00eeba00f0"></a>
## nulls

`function` · `arrow_array::array::list_array::GenericListArray::nulls` · arrow-array 59.3.0

```rust
fn nulls(&self) -> Option<&NullBuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_array::GenericListArray", "path": "GenericListArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [579, 1], "end": [655, 2], "filename": "src/array/list_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/list_array.rs:620`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-842c95dea57894713200a91e"></a>
## offset

`function` · `arrow_array::array::list_array::GenericListArray::offset` · arrow-array 59.3.0

```rust
fn offset(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_array::GenericListArray", "path": "GenericListArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [579, 1], "end": [655, 2], "filename": "src/array/list_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/list_array.rs:616`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9ec7b023ae1581b417d98555"></a>
## offsets

`function` · `arrow_array::array::list_array::GenericListArray::offsets` · arrow-array 59.3.0

```rust
fn offsets(&self) -> &OffsetBuffer<OffsetSize>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_array::GenericListArray", "path": "GenericListArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [189, 1], "end": [488, 2], "filename": "src/array/list_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/list_array.rs:334`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a reference to the offsets of this list

Unlike [`Self::value_offsets`](../operations/arrow_array.array.list_array.GenericListArray.md#op-4fa661aee32d428ce6647420) this returns the [`OffsetBuffer`](../operations/arrow_buffer.buffer.offset.OffsetBuffer.md#op-a91a6a87515fb5ae378e13fc)
allowing for zero-copy cloning.

Notes: The `offsets` may not start at 0 and may not cover all values in
[`Self::values`](../operations/arrow_array.array.list_array.GenericListArray.md#op-13bd6eeaf01add795b79bc57). This can happen when the list array was sliced via
[`Self::slice`](../operations/arrow_array.array.list_array.GenericListArray.md#op-30b14f6423262794fba8c9b5). See documentation for [`Self`](../operations/arrow_array.array.list_array.GenericListArray.md#op-f5cb16d85a7337f2dafa403b) for more details.

<a id="op-d5981d41fd934e4d37f864d3"></a>
## shrink_to_fit

`function` · `arrow_array::array::list_array::GenericListArray::shrink_to_fit` · arrow-array 59.3.0

```rust
fn shrink_to_fit(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_array::GenericListArray", "path": "GenericListArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [579, 1], "end": [655, 2], "filename": "src/array/list_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/list_array.rs:608`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-30b14f6423262794fba8c9b5"></a>
## slice

`function` · `arrow_array::array::list_array::GenericListArray::slice` · arrow-array 59.3.0

```rust
fn slice(&self, offset: usize, length: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_array::GenericListArray", "path": "GenericListArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [189, 1], "end": [488, 2], "filename": "src/array/list_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/list_array.rs:415`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a zero-copy slice of this array with the indicated offset and length.

Notes: this method does *NOT* slice the underlying values array or modify
the values in the offsets buffer. See [`Self::values`](../operations/arrow_array.array.list_array.GenericListArray.md#op-13bd6eeaf01add795b79bc57) and
[`Self::offsets`](../operations/arrow_array.array.list_array.GenericListArray.md#op-9ec7b023ae1581b417d98555) for more information.

<a id="op-b1d2b307dd276038e5b22037"></a>
## slice

`function` · `arrow_array::array::list_array::GenericListArray::slice` · arrow-array 59.3.0

```rust
fn slice(&self, offset: usize, length: usize) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_array::GenericListArray", "path": "GenericListArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [579, 1], "end": [655, 2], "filename": "src/array/list_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/list_array.rs:596`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6d7b55f257cea695454d1d38"></a>
## to_data

`function` · `arrow_array::array::list_array::GenericListArray::to_data` · arrow-array 59.3.0

```rust
fn to_data(&self) -> ArrayData
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_array::GenericListArray", "path": "GenericListArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [579, 1], "end": [655, 2], "filename": "src/array/list_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/list_array.rs:584`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a8fcab958704ce1093cba4ea"></a>
## try_new

`function` · `arrow_array::array::list_array::GenericListArray::try_new` · arrow-array 59.3.0

```rust
fn try_new(field: FieldRef, offsets: OffsetBuffer<OffsetSize>, values: ArrayRef, nulls: Option<NullBuffer>) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_array::GenericListArray", "path": "GenericListArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [189, 1], "end": [488, 2], "filename": "src/array/list_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/list_array.rs:209`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`GenericListArray`](../operations/arrow_array.array.list_array.GenericListArray.md#op-f5cb16d85a7337f2dafa403b) from the provided parts

# Errors

Errors if

* `offsets.len() - 1 != nulls.len()`
* `offsets.last() > values.len()`
* `!field.is_nullable() && values.is_nullable()`
* `field.data_type() != values.data_type()`

<a id="op-c2af1bc5d4ddac6410b791d3"></a>
## value

`function` · `arrow_array::array::list_array::GenericListArray::value` · arrow-array 59.3.0

```rust
fn value(&self, i: usize) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_array::GenericListArray", "path": "GenericListArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [189, 1], "end": [488, 2], "filename": "src/array/list_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/list_array.rs:374`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns ith value of this list array.

Note: This method does not check for nulls and the value is arbitrary
(but still well-defined) if [`is_null`](Self::is_null) returns true for the index.

# Panics
Panics if index `i` is out of bounds

<a id="op-f4a93da69119a3aabf81988a"></a>
## value_length

`function` · `arrow_array::array::list_array::GenericListArray::value_length` · arrow-array 59.3.0

```rust
fn value_length(&self, i: usize) -> OffsetSize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_array::GenericListArray", "path": "GenericListArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [189, 1], "end": [488, 2], "filename": "src/array/list_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/list_array.rs:390`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the length for value at index `i`.

<a id="op-4fa661aee32d428ce6647420"></a>
## value_offsets

`function` · `arrow_array::array::list_array::GenericListArray::value_offsets` · arrow-array 59.3.0

```rust
fn value_offsets(&self) -> &[OffsetSize]
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_array::GenericListArray", "path": "GenericListArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [189, 1], "end": [488, 2], "filename": "src/array/list_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/list_array.rs:384`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the offset values in the offsets buffer.

See [`Self::offsets`](../operations/arrow_array.array.list_array.GenericListArray.md#op-9ec7b023ae1581b417d98555) for more details.

<a id="op-213096c0ac06f04ea7cf0a3f"></a>
## value_type

`function` · `arrow_array::array::list_array::GenericListArray::value_type` · arrow-array 59.3.0

```rust
fn value_type(&self) -> DataType
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_array::GenericListArray", "path": "GenericListArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [189, 1], "end": [488, 2], "filename": "src/array/list_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/list_array.rs:350`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a clone of the value type of this list.

<a id="op-35955aaddbbc040c0ce493be"></a>
## value_unchecked

`function` · `arrow_array::array::list_array::GenericListArray::value_unchecked` · arrow-array 59.3.0

```rust
unsafe fn value_unchecked(&self, i: usize) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_array::GenericListArray", "path": "GenericListArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [189, 1], "end": [488, 2], "filename": "src/array/list_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/list_array.rs:361`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns ith value of this list array.

Note: This method does not check for nulls and the value is arbitrary
if [`is_null`](Self::is_null) returns true for the index.

# Safety
Caller must ensure that the index is within the array bounds

<a id="op-13bd6eeaf01add795b79bc57"></a>
## values

`function` · `arrow_array::array::list_array::GenericListArray::values` · arrow-array 59.3.0

```rust
fn values(&self) -> &ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_array::GenericListArray", "path": "GenericListArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [189, 1], "end": [488, 2], "filename": "src/array/list_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/list_array.rs:345`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a reference to the values of this list

Note: The list array may not refer to all values in the `values` array.
For example if the list array was sliced via [`Self::slice`](../operations/arrow_array.array.list_array.GenericListArray.md#op-30b14f6423262794fba8c9b5) values will
still contain values both before and after the slice. See documentation
for [`Self`](../operations/arrow_array.array.list_array.GenericListArray.md#op-f5cb16d85a7337f2dafa403b) for more details.

<a id="op-5f51be00a9f5b2784c86782d"></a>
## values

`function` · `arrow_array::array::list_array::GenericListArray::values` · arrow-array 59.3.0

```rust
fn values(&self) -> &ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_array::GenericListArray", "path": "GenericListArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [657, 1], "end": [668, 2], "filename": "src/array/list_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::ListLikeArray", "path": "ListLikeArray"}, "trait_path": "arrow_array::array::ListLikeArray"}`

Source: `src/array/list_array.rs:658`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
