# `arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.fixed_size_binary_array.FixedSizeBinaryArray.json).

<a id="op-9ede10c8076f692a67198c02"></a>
## FixedSizeBinaryArray

`struct` · `arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray` · arrow-array 59.3.0

```rust
struct FixedSizeBinaryArray
```

Source: `src/array/fixed_size_binary_array.rs:90`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

An array of [fixed-size binary values](https://arrow.apache.org/docs/format/Columnar.html#fixed-size-primitive-layout)

Each element in a [`FixedSizeBinaryArray`](../operations/arrow_array.array.fixed_size_binary_array.FixedSizeBinaryArray.md#op-9ede10c8076f692a67198c02) has `value_length` bytes, where
`value_length` is defined by the schema.

This array type is useful for storing fixed-length values such as 16-byte
UUIDs (`value_length = 16`).

# Layout

Values in a [`FixedSizeBinaryArray`](../operations/arrow_array.array.fixed_size_binary_array.FixedSizeBinaryArray.md#op-9ede10c8076f692a67198c02) are stored contiguously in a single
buffer. The byte offset for the `i`-th element can be calculated as
`i * value_length`.

Nulls are stored in a standard optional Arrow [`NullBuffer`](../operations/arrow_buffer.buffer.null.NullBuffer.md#op-d8209d291b86c5deb0050e48).

For example, a 100-value [`FixedSizeBinaryArray`](../operations/arrow_array.array.fixed_size_binary_array.FixedSizeBinaryArray.md#op-9ede10c8076f692a67198c02) with `value_length = 12`
is shown below.

```text
┌──────────────────────────────────────────┐
│ Computed byte offsets                    │
│          ┌──────────────────────┐ ┌────┐ │
│          │┌────────────────────┐│ │    │ │
│       0  ││value 0  (12 bytes) ││ │ 1  │ │
│          │├────────────────────┤│ │    │ │
│       12 ││value 1  (12 bytes) ││ │ 0  │ │
│          │├────────────────────┤│ │    │ │
│       24 ││value 2  (12 bytes) ││ │ 1  │ │
│          │└────────────────────┘│ │    │ │
│          │         ...          │ │... │ │
│          │┌───────────────────┐ │ │    │ │
│     1188 ││value 99 (12 bytes)│ │ │ 1  │ │
│          │└───────────────────┘ │ │    │ │
│          └──────────────────────┘ └────┘ │
│           value_data              nulls  │
└──────────────────────────────────────────┘
```

# Examples

Create an array from an iterable argument of byte slices.

```
   use arrow_array::{Array, FixedSizeBinaryArray};
   let input_arg = vec![ vec![1, 2], vec![3, 4], vec![5, 6] ];
   let arr = FixedSizeBinaryArray::try_from_iter(input_arg.into_iter()).unwrap();

   assert_eq!(3, arr.len());

```
Create an array from an iterable argument of sparse byte slices.
Sparsity means that the input argument can contain `None` items.
```
   use arrow_array::{Array, FixedSizeBinaryArray};
   let input_arg = vec![ None, Some(vec![7, 8]), Some(vec![9, 10]), None, Some(vec![13, 14]) ];
   let arr = FixedSizeBinaryArray::try_from_sparse_iter_with_size(input_arg.into_iter(), 2).unwrap();
   assert_eq!(5, arr.len())

```


<a id="op-b3989dcfef860b6b5c24db9f"></a>
## Error

`assoc_type` · `arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray::Error` · arrow-array 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray", "path": "FixedSizeBinaryArray"}}, "generics": {"params": [{"kind": {"const": {"default": null, "type": {"primitive": "usize"}}}, "name": "N"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [753, 1], "end": [765, 2], "filename": "src/array/fixed_size_binary_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"array": {"len": "N", "type": {"primitive": "u8"}}}}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/array/fixed_size_binary_array.rs:754`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ba88efb9b3617146deb43354"></a>
## Error

`assoc_type` · `arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray::Error` · arrow-array 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray", "path": "FixedSizeBinaryArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [736, 1], "end": [743, 2], "filename": "src/array/fixed_size_binary_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"slice": {"primitive": "u8"}}}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/array/fixed_size_binary_array.rs:737`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ef99265fde86e5547efed4b1"></a>
## Error

`assoc_type` · `arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray::Error` · arrow-array 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray", "path": "FixedSizeBinaryArray"}}, "generics": {"params": [{"kind": {"const": {"default": null, "type": {"primitive": "usize"}}}, "name": "N"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [767, 1], "end": [773, 2], "filename": "src/array/fixed_size_binary_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"array": {"len": "N", "type": {"primitive": "u8"}}}}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/array/fixed_size_binary_array.rs:768`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fa0a2b111f9230b570f309d1"></a>
## Error

`assoc_type` · `arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray::Error` · arrow-array 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray", "path": "FixedSizeBinaryArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [745, 1], "end": [751, 2], "filename": "src/array/fixed_size_binary_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"slice": {"primitive": "u8"}}}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/array/fixed_size_binary_array.rs:746`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9bb85953907c2b79308f6b2e"></a>
## as_any

`function` · `arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray::as_any` · arrow-array 59.3.0

```rust
fn as_any(&self) -> &dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray", "path": "FixedSizeBinaryArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [786, 1], "end": [856, 2], "filename": "src/array/fixed_size_binary_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/fixed_size_binary_array.rs:787`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9de28c44d5b715ca1430b995"></a>
## claim

`function` · `arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray::claim` · arrow-array 59.3.0

```rust
fn claim(&self, pool: &dyn arrow_buffer::MemoryPool)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray", "path": "FixedSizeBinaryArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [786, 1], "end": [856, 2], "filename": "src/array/fixed_size_binary_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/fixed_size_binary_array.rs:850`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-687de78a2b98da0af0d2ce34"></a>
## clone

`function` · `arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray::clone` · arrow-array 59.3.0

```rust
fn clone(&self) -> FixedSizeBinaryArray
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray", "path": "FixedSizeBinaryArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 10], "end": [89, 15], "filename": "src/array/fixed_size_binary_array.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/array/fixed_size_binary_array.rs:89`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-64f360dabd45ae3ed0361365"></a>
## data_type

`function` · `arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray::data_type` · arrow-array 59.3.0

```rust
fn data_type(&self) -> &DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray", "path": "FixedSizeBinaryArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [786, 1], "end": [856, 2], "filename": "src/array/fixed_size_binary_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/fixed_size_binary_array.rs:799`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-47a9f5e26e4b1a7dff94dd18"></a>
## eq

`function` · `arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray::eq` · arrow-array 59.3.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray", "path": "FixedSizeBinaryArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [812, 1], "end": [816, 2], "filename": "src/array/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/array/mod.rs:813`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8ce19df5682c998905c41981"></a>
## fmt

`function` · `arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray::fmt` · arrow-array 59.3.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray", "path": "FixedSizeBinaryArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [775, 1], "end": [783, 2], "filename": "src/array/fixed_size_binary_array.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/array/fixed_size_binary_array.rs:776`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-609e15a9f7c4ebca232eb975"></a>
## from

`function` · `arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray::from` · arrow-array 59.3.0

```rust
fn from(v: FixedSizeListArray) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray", "path": "FixedSizeBinaryArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [696, 1], "end": [734, 2], "filename": "src/array/fixed_size_binary_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_list_array::FixedSizeListArray", "path": "FixedSizeListArray"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/fixed_size_binary_array.rs:697`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-847d3900f0da4468d16c2ee0"></a>
## from

`function` · `arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray::from` · arrow-array 59.3.0

```rust
fn from(data: ArrayData) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray", "path": "FixedSizeBinaryArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [652, 1], "end": [682, 2], "filename": "src/array/fixed_size_binary_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayData", "path": "ArrayData"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/fixed_size_binary_array.rs:653`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-61d389f81706d3cabf32c457"></a>
## get_array_memory_size

`function` · `arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray::get_array_memory_size` · arrow-array 59.3.0

```rust
fn get_array_memory_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray", "path": "FixedSizeBinaryArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [786, 1], "end": [856, 2], "filename": "src/array/fixed_size_binary_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/fixed_size_binary_array.rs:845`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9c328e36e6aa36854d7cb670"></a>
## get_buffer_memory_size

`function` · `arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray::get_buffer_memory_size` · arrow-array 59.3.0

```rust
fn get_buffer_memory_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray", "path": "FixedSizeBinaryArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [786, 1], "end": [856, 2], "filename": "src/array/fixed_size_binary_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/fixed_size_binary_array.rs:837`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5136cc0f0aef82436c4ad664"></a>
## into_data

`function` · `arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray::into_data` · arrow-array 59.3.0

```rust
fn into_data(self) -> ArrayData
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray", "path": "FixedSizeBinaryArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [786, 1], "end": [856, 2], "filename": "src/array/fixed_size_binary_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/fixed_size_binary_array.rs:795`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7db30359be821cfefd5960ba"></a>
## into_parts

`function` · `arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray::into_parts` · arrow-array 59.3.0

```rust
fn into_parts(self) -> (i32, Buffer, Option<NullBuffer>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray", "path": "FixedSizeBinaryArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 1], "end": [650, 2], "filename": "src/array/fixed_size_binary_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/fixed_size_binary_array.rs:264`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Deconstruct this array into its constituent parts

<a id="op-99595abfbe963ac57b8739ea"></a>
## is_empty

`function` · `arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray::is_empty` · arrow-array 59.3.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray", "path": "FixedSizeBinaryArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [786, 1], "end": [856, 2], "filename": "src/array/fixed_size_binary_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/fixed_size_binary_array.rs:811`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d8d7fbb393e2804dc547ddc7"></a>
## iter

`function` · `arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray::iter` · arrow-array 59.3.0

```rust
fn iter(&self) -> FixedSizeBinaryIter<'_>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray", "path": "FixedSizeBinaryArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 1], "end": [650, 2], "filename": "src/array/fixed_size_binary_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/fixed_size_binary_array.rs:647`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

constructs a new iterator

<a id="op-d2feedfc31dd58e1248b12b8"></a>
## len

`function` · `arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray::len` · arrow-array 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray", "path": "FixedSizeBinaryArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [786, 1], "end": [856, 2], "filename": "src/array/fixed_size_binary_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/fixed_size_binary_array.rs:807`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f08d9aab41eb2d9fe68daff4"></a>
## logical_null_count

`function` · `arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray::logical_null_count` · arrow-array 59.3.0

```rust
fn logical_null_count(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray", "path": "FixedSizeBinaryArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [786, 1], "end": [856, 2], "filename": "src/array/fixed_size_binary_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/fixed_size_binary_array.rs:832`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad4ec86ed8d31587dcb22b9a"></a>
## new

`function` · `arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray::new` · arrow-array 59.3.0

```rust
fn new(value_length: i32, values: Buffer, nulls: Option<NullBuffer>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray", "path": "FixedSizeBinaryArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 1], "end": [650, 2], "filename": "src/array/fixed_size_binary_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/fixed_size_binary_array.rs:117`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`FixedSizeBinaryArray`](../operations/arrow_array.array.fixed_size_binary_array.FixedSizeBinaryArray.md#op-9ede10c8076f692a67198c02) with `value_length` bytes per element, panicking on
failure

# Panics

Panics if [`Self::try_new`](../operations/arrow_array.array.fixed_size_binary_array.FixedSizeBinaryArray.md#op-9160dd84fd5a52753ec03c99) returns an error

<a id="op-68f395bba4a8c0743b2944b0"></a>
## new_null

`function` · `arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray::new_null` · arrow-array 59.3.0

```rust
fn new_null(value_length: i32, len: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray", "path": "FixedSizeBinaryArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 1], "end": [650, 2], "filename": "src/array/fixed_size_binary_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/fixed_size_binary_array.rs:249`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`FixedSizeBinaryArray`](../operations/arrow_array.array.fixed_size_binary_array.FixedSizeBinaryArray.md#op-9ede10c8076f692a67198c02) of length `len` where all values are null

# Panics

Panics if

* `value_length < 0`
* `value_length * len` would overflow `usize`
* `value_length * len * 8` would overflow `usize`

<a id="op-0782c2e1023cd86c709c0c78"></a>
## new_scalar

`function` · `arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray::new_scalar` · arrow-array 59.3.0

```rust
fn new_scalar(value: impl AsRef<[u8]>) -> Scalar<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray", "path": "FixedSizeBinaryArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 1], "end": [650, 2], "filename": "src/array/fixed_size_binary_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/fixed_size_binary_array.rs:146`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`Scalar`](../operations/arrow_array.scalar.Scalar.md#op-0ca10f05b262b3afc7077257) from `value`

<a id="op-5373755e99e7aef5fdc9458a"></a>
## new_unchecked

`function` · `arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray::new_unchecked` · arrow-array 59.3.0

```rust
unsafe fn new_unchecked(value_length: i32, values: Buffer, nulls: Option<NullBuffer>, len: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray", "path": "FixedSizeBinaryArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 1], "end": [650, 2], "filename": "src/array/fixed_size_binary_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/fixed_size_binary_array.rs:127`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`FixedSizeBinaryArray`](../operations/arrow_array.array.fixed_size_binary_array.FixedSizeBinaryArray.md#op-9ede10c8076f692a67198c02) from the provided parts without validation.

# Safety
- `value_length >= 0`
- `values.len() == len * value_length as usize`
- `nulls.len() == len` if `nulls` is `Some`

<a id="op-b5bd4366a07b64e52ed9b8ac"></a>
## nulls

`function` · `arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray::nulls` · arrow-array 59.3.0

```rust
fn nulls(&self) -> Option<&NullBuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray", "path": "FixedSizeBinaryArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [786, 1], "end": [856, 2], "filename": "src/array/fixed_size_binary_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/fixed_size_binary_array.rs:828`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-de61731bd643b134fc2d4e82"></a>
## offset

`function` · `arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray::offset` · arrow-array 59.3.0

```rust
fn offset(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray", "path": "FixedSizeBinaryArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [786, 1], "end": [856, 2], "filename": "src/array/fixed_size_binary_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/fixed_size_binary_array.rs:822`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4db0785d9c3fdcbf7d351879"></a>
## shrink_to_fit

`function` · `arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray::shrink_to_fit` · arrow-array 59.3.0

```rust
fn shrink_to_fit(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray", "path": "FixedSizeBinaryArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [786, 1], "end": [856, 2], "filename": "src/array/fixed_size_binary_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/fixed_size_binary_array.rs:815`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-647d17b3ce92262b1491bc52"></a>
## slice

`function` · `arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray::slice` · arrow-array 59.3.0

```rust
fn slice(&self, offset: usize, len: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray", "path": "FixedSizeBinaryArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 1], "end": [650, 2], "filename": "src/array/fixed_size_binary_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/fixed_size_binary_array.rs:355`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a zero-copy slice of this array with the indicated offset and length.

<a id="op-f7dcd83127305474b1fed447"></a>
## slice

`function` · `arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray::slice` · arrow-array 59.3.0

```rust
fn slice(&self, offset: usize, length: usize) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray", "path": "FixedSizeBinaryArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [786, 1], "end": [856, 2], "filename": "src/array/fixed_size_binary_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/fixed_size_binary_array.rs:803`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6d226e33ff994087ae0a82bc"></a>
## to_data

`function` · `arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray::to_data` · arrow-array 59.3.0

```rust
fn to_data(&self) -> ArrayData
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray", "path": "FixedSizeBinaryArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [786, 1], "end": [856, 2], "filename": "src/array/fixed_size_binary_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/fixed_size_binary_array.rs:791`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-48766ead33a1ddeadc2e268c"></a>
## try_from

`function` · `arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray::try_from` · arrow-array 59.3.0

```rust
fn try_from(v: Vec<Option<&[u8; N]>>) -> Result<Self, Self::Error>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray", "path": "FixedSizeBinaryArray"}}, "generics": {"params": [{"kind": {"const": {"default": null, "type": {"primitive": "usize"}}}, "name": "N"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [753, 1], "end": [765, 2], "filename": "src/array/fixed_size_binary_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"array": {"len": "N", "type": {"primitive": "u8"}}}}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/array/fixed_size_binary_array.rs:756`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-51190982d92f9491e3018095"></a>
## try_from

`function` · `arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray::try_from` · arrow-array 59.3.0

```rust
fn try_from(v: Vec<&[u8; N]>) -> Result<Self, Self::Error>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray", "path": "FixedSizeBinaryArray"}}, "generics": {"params": [{"kind": {"const": {"default": null, "type": {"primitive": "usize"}}}, "name": "N"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [767, 1], "end": [773, 2], "filename": "src/array/fixed_size_binary_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"array": {"len": "N", "type": {"primitive": "u8"}}}}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/array/fixed_size_binary_array.rs:770`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c3469dd4c81610db44c5f823"></a>
## try_from

`function` · `arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray::try_from` · arrow-array 59.3.0

```rust
fn try_from(v: Vec<&[u8]>) -> Result<Self, Self::Error>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray", "path": "FixedSizeBinaryArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [745, 1], "end": [751, 2], "filename": "src/array/fixed_size_binary_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"slice": {"primitive": "u8"}}}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/array/fixed_size_binary_array.rs:748`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ebe06db35e634fa5d2bb52ed"></a>
## try_from

`function` · `arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray::try_from` · arrow-array 59.3.0

```rust
fn try_from(v: Vec<Option<&[u8]>>) -> Result<Self, Self::Error>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray", "path": "FixedSizeBinaryArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [736, 1], "end": [743, 2], "filename": "src/array/fixed_size_binary_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"slice": {"primitive": "u8"}}}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/array/fixed_size_binary_array.rs:739`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7595e3a41da1de7f8711d707"></a>
## try_from_iter

`function` · `arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray::try_from_iter` · arrow-array 59.3.0

```rust
fn try_from_iter<T, U>(iter: T) -> Result<Self, ArrowError> where T: Iterator<Item = U>, U: AsRef<[u8]>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray", "path": "FixedSizeBinaryArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 1], "end": [650, 2], "filename": "src/array/fixed_size_binary_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/fixed_size_binary_array.rs:591`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create an array from an iterable argument of byte slices.

# Examples

```
use arrow_array::FixedSizeBinaryArray;
let input_arg = vec![
    vec![1, 2],
    vec![3, 4],
    vec![5, 6],
];
let array = FixedSizeBinaryArray::try_from_iter(input_arg.into_iter()).unwrap();
```

# Errors

Returns error if argument has length zero, or sizes of nested slices don't match.

<a id="op-838a1cfaeab154db9d042691"></a>
## try_from_sparse_iter

`function` · `arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray::try_from_sparse_iter` · arrow-array 59.3.0

```rust
fn try_from_sparse_iter<T, U>(iter: T) -> Result<Self, ArrowError> where T: Iterator<Item = Option<U>>, U: AsRef<[u8]>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray", "path": "FixedSizeBinaryArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 1], "end": [650, 2], "filename": "src/array/fixed_size_binary_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/fixed_size_binary_array.rs:400`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create an array from an iterable argument of sparse byte slices.
Sparsity means that items returned by the iterator are optional, i.e input argument can
contain `None` items.

# Examples

```
use arrow_array::FixedSizeBinaryArray;
let input_arg = vec![
    None,
    Some(vec![7, 8]),
    Some(vec![9, 10]),
    None,
    Some(vec![13, 14]),
    None,
];
let array = FixedSizeBinaryArray::try_from_sparse_iter(input_arg.into_iter()).unwrap();
```

# Errors

Returns error if argument has length zero, or sizes of nested slices don't match.

<a id="op-586aa3aa664dc847f6dec91f"></a>
## try_from_sparse_iter_with_size

`function` · `arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray::try_from_sparse_iter_with_size` · arrow-array 59.3.0

```rust
fn try_from_sparse_iter_with_size<T, U>(iter: T, value_length: i32) -> Result<Self, ArrowError> where T: Iterator<Item = Option<U>>, U: AsRef<[u8]>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray", "path": "FixedSizeBinaryArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 1], "end": [650, 2], "filename": "src/array/fixed_size_binary_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/fixed_size_binary_array.rs:509`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create an array from an iterable argument of sparse byte slices.
Sparsity means that items returned by the iterator are optional, i.e input argument can
contain `None` items. In cases where the iterator returns only `None` values, this
also takes a `value_length` parameter to ensure that a valid
[`FixedSizeBinaryArray`](../operations/arrow_array.array.fixed_size_binary_array.FixedSizeBinaryArray.md#op-9ede10c8076f692a67198c02) is still created.

# Examples

```
use arrow_array::FixedSizeBinaryArray;
let input_arg = vec![
    None,
    Some(vec![7, 8]),
    Some(vec![9, 10]),
    None,
    Some(vec![13, 14]),
    None,
];
let array = FixedSizeBinaryArray::try_from_sparse_iter_with_size(input_arg.into_iter(), 2).unwrap();
```

# Errors

Returns error if argument has length zero, or sizes of nested slices don't match.

<a id="op-9160dd84fd5a52753ec03c99"></a>
## try_new

`function` · `arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray::try_new` · arrow-array 59.3.0

```rust
fn try_new(value_length: i32, values: Buffer, nulls: Option<NullBuffer>) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray", "path": "FixedSizeBinaryArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 1], "end": [650, 2], "filename": "src/array/fixed_size_binary_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/fixed_size_binary_array.rs:164`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`FixedSizeBinaryArray`](../operations/arrow_array.array.fixed_size_binary_array.FixedSizeBinaryArray.md#op-9ede10c8076f692a67198c02) from the provided parts, returning an error on failure

Creating an array with `value_length == 0` will try to get the length from the null
buffer. If no null buffer is provided, the resulting array will have length zero.
You can use [`Self::try_new_with_len`](../operations/arrow_array.array.fixed_size_binary_array.FixedSizeBinaryArray.md#op-f07b958bbfc0a3fd6832b584) to provide the length

# Errors

* `value_length < 0`
* `values.len() / value_length != nulls.len()`
* `value_length == 0 && values.len() != 0`

<a id="op-f07b958bbfc0a3fd6832b584"></a>
## try_new_with_len

`function` · `arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray::try_new_with_len` · arrow-array 59.3.0

```rust
fn try_new_with_len(value_length: i32, values: Buffer, nulls: Option<NullBuffer>, len: usize) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray", "path": "FixedSizeBinaryArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 1], "end": [650, 2], "filename": "src/array/fixed_size_binary_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/fixed_size_binary_array.rs:194`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`FixedSizeBinaryArray`](../operations/arrow_array.array.fixed_size_binary_array.FixedSizeBinaryArray.md#op-9ede10c8076f692a67198c02) from the provided parts and number of elements, returning an error on failure

This is useful when the length cannot be determinated from the provided values (in case of `value_length == 0`) or nulls (`nulls.is_none()`).

# Errors

* `value_length < 0`
* `values.len() / value_length != len`
* `value_length == 0 && values.len() != 0`
* `nulls.len() != len`
* `value_length != 0 && values.len() / value_length != len`

<a id="op-1f931925dc44fcebd07040c6"></a>
## value

`function` · `arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray::value` · arrow-array 59.3.0

```rust
fn value(&self, i: usize) -> &[u8]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray", "path": "FixedSizeBinaryArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 1], "end": [650, 2], "filename": "src/array/fixed_size_binary_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/fixed_size_binary_array.rs:276`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the element at index `i` as a byte slice.

Note: This method does not check for nulls and the value is arbitrary
(but still well-defined) if [`is_null`](Self::is_null) returns true for the index.

# Panics
Panics if index `i` is out of bounds.

<a id="op-43744cac986feb5fbadb3bba"></a>
## value_data

`function` · `arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray::value_data` · arrow-array 59.3.0

```rust
fn value_data(&self) -> &[u8]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray", "path": "FixedSizeBinaryArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 1], "end": [650, 2], "filename": "src/array/fixed_size_binary_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/fixed_size_binary_array.rs:350`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the raw value data.

<a id="op-e0300a900defc70b70ba4f0a"></a>
## value_length

`function` · `arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray::value_length` · arrow-array 59.3.0

```rust
fn value_length(&self) -> i32
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray", "path": "FixedSizeBinaryArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 1], "end": [650, 2], "filename": "src/array/fixed_size_binary_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/fixed_size_binary_array.rs:325`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the length for an element.

All elements have the same length as the array is a fixed size.

Returns an `i32` to be compatible with the Arrow spec.

Use [`Self::value_size`](../operations/arrow_array.array.fixed_size_binary_array.FixedSizeBinaryArray.md#op-3232a6fb6f245b4313406f64) to return a `usize`.

<a id="op-b7b7b57a993eb241a3d08750"></a>
## value_offset

`function` · `arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray::value_offset` · arrow-array 59.3.0

```rust
fn value_offset(&self, i: usize) -> i32
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray", "path": "FixedSizeBinaryArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 1], "end": [650, 2], "filename": "src/array/fixed_size_binary_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/fixed_size_binary_array.rs:313`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the offset for the element at index `i`.

Note this doesn't do any bound checking, for performance reason.

# Panics

Panics if the computed byte offset exceeds `i32::MAX`.

<a id="op-3232a6fb6f245b4313406f64"></a>
## value_size

`function` · `arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray::value_size` · arrow-array 59.3.0

```rust
fn value_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray", "path": "FixedSizeBinaryArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 1], "end": [650, 2], "filename": "src/array/fixed_size_binary_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/fixed_size_binary_array.rs:336`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Return the length for an element, as a usize.

All elements have the same length as the array is a fixed size.

Note: This value will always fit, without overflow, into an i32

<a id="op-ec1003b787ca37e8e1769f98"></a>
## value_unchecked

`function` · `arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray::value_unchecked` · arrow-array 59.3.0

```rust
unsafe fn value_unchecked(&self, i: usize) -> &[u8]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray", "path": "FixedSizeBinaryArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 1], "end": [650, 2], "filename": "src/array/fixed_size_binary_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/fixed_size_binary_array.rs:297`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the element at index `i` as a byte slice.

Note: This method does not check for nulls and the value is arbitrary
if [`is_null`](Self::is_null) returns true for the index.

# Safety

Caller is responsible for ensuring that the index is within the bounds
of the array

<a id="op-0636c11182e57632be1865b9"></a>
## values

`function` · `arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray::values` · arrow-array 59.3.0

```rust
fn values(&self) -> &Buffer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray", "path": "FixedSizeBinaryArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 1], "end": [650, 2], "filename": "src/array/fixed_size_binary_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/fixed_size_binary_array.rs:345`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the values of this array.

Unlike [`Self::value_data`](../operations/arrow_array.array.fixed_size_binary_array.FixedSizeBinaryArray.md#op-43744cac986feb5fbadb3bba) this returns the [`Buffer`](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-f54755e677e7b3529b3edb8b)
allowing for zero-copy cloning.
