# `arrow_array::array::run_array::RunArray`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.run_array.RunArray.json).

<a id="op-f0742b2d9e045922a076478f"></a>
## RunArray

`struct` · `arrow_array::array::run_array::RunArray` · arrow-array 59.3.0

```rust
struct RunArray<R: RunEndIndexType>
```

Source: `src/array/run_array.rs:84`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

An array of [run-end encoded values].

This encoding is variation on [run-length encoding (RLE)] and is good for representing
data containing the same values repeated consecutively.

A [`RunArray`](../operations/arrow_array.array.run_array.RunArray.md#op-f0742b2d9e045922a076478f) consists of a `run_ends` buffer and a `values` array of equivalent
lengths. The `run_ends` buffer stores the indexes at which the run ends. The
`values` array stores the corresponding value of each run. The below example
illustrates how a logical array is represented by a [`RunArray`](../operations/arrow_array.array.run_array.RunArray.md#op-f0742b2d9e045922a076478f):

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

<a id="op-a8882afb4881e5a450ae10c7"></a>
## as_any

`function` · `arrow_array::array::run_array::RunArray::as_any` · arrow-array 59.3.0

```rust
fn as_any(&self) -> &dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::run_array::RunArray", "path": "RunArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [421, 1], "end": [515, 2], "filename": "src/array/run_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/run_array.rs:422`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0bbe486d28db5e3cef2ab923"></a>
## claim

`function` · `arrow_array::array::run_array::RunArray::claim` · arrow-array 59.3.0

```rust
fn claim(&self, pool: &dyn arrow_buffer::MemoryPool)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::run_array::RunArray", "path": "RunArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [421, 1], "end": [515, 2], "filename": "src/array/run_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/run_array.rs:511`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2fc0acb446c242f77130bdd9"></a>
## clone

`function` · `arrow_array::array::run_array::RunArray::clone` · arrow-array 59.3.0

```rust
fn clone(&self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_array::array::run_array::RunArray", "path": "RunArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [98, 2], "filename": "src/array/run_array.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/array/run_array.rs:91`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-55698d86baa8bcb1175a8d4b"></a>
## data_type

`function` · `arrow_array::array::run_array::RunArray::data_type` · arrow-array 59.3.0

```rust
fn data_type(&self) -> &DataType
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::run_array::RunArray", "path": "RunArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [421, 1], "end": [515, 2], "filename": "src/array/run_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/run_array.rs:434`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-860f9beef6f07aa010e47c03"></a>
## downcast

`function` · `arrow_array::array::run_array::RunArray::downcast` · arrow-array 59.3.0

```rust
fn downcast<V: 'static>(&self) -> Option<TypedRunArray<'_, R, V>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_array::array::run_array::RunArray", "path": "RunArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [100, 1], "end": [354, 2], "filename": "src/array/run_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/run_array.rs:308`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Downcast this [`RunArray`](../operations/arrow_array.array.run_array.RunArray.md#op-f0742b2d9e045922a076478f) to a [`TypedRunArray`](../operations/arrow_array.array.run_array.TypedRunArray.md#op-507451b8b0e62718e6f7a05e)

```
use arrow_array::{Array, ArrayAccessor, RunArray, StringArray, types::Int32Type};

let orig = [Some("a"), Some("b"), None];
let run_array = RunArray::<Int32Type>::from_iter(orig);
let typed = run_array.downcast::<StringArray>().unwrap();
assert_eq!(typed.value(0), "a");
assert_eq!(typed.value(1), "b");
assert!(typed.values().is_null(2));
```

<a id="op-e519390205ab83dd6cf73c69"></a>
## eq

`function` · `arrow_array::array::run_array::RunArray::eq` · arrow-array 59.3.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_array::array::run_array::RunArray", "path": "RunArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [854, 1], "end": [858, 2], "filename": "src/array/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/array/mod.rs:855`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c260d867ca2828033f274aba"></a>
## fmt

`function` · `arrow_array::array::run_array::RunArray::fmt` · arrow-array 59.3.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_array::array::run_array::RunArray", "path": "RunArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [517, 1], "end": [526, 2], "filename": "src/array/run_array.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/array/run_array.rs:518`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1151afbce3921a30a0882ba0"></a>
## from

`function` · `arrow_array::array::run_array::RunArray::from` · arrow-array 59.3.0

```rust
fn from(data: ArrayData) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_array::array::run_array::RunArray", "path": "RunArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [356, 1], "end": [398, 2], "filename": "src/array/run_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayData", "path": "ArrayData"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/run_array.rs:358`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f84f62f4ba07131eabb44b0f"></a>
## from_iter

`function` · `arrow_array::array::run_array::RunArray::from_iter` · arrow-array 59.3.0

```rust
fn from_iter<I: IntoIterator<Item = Option<&'a str>>>(iter: I) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::run_array::RunArray", "path": "RunArray"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [544, 1], "end": [555, 2], "filename": "src/array/run_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"primitive": "str"}}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "core::iter::traits::collect::FromIterator", "path": "FromIterator"}, "trait_path": "core::iter::traits::collect::FromIterator"}`

Source: `src/array/run_array.rs:545`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-faccf7645408480cbf2f8972"></a>
## from_iter

`function` · `arrow_array::array::run_array::RunArray::from_iter` · arrow-array 59.3.0

```rust
fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::run_array::RunArray", "path": "RunArray"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [571, 1], "end": [582, 2], "filename": "src/array/run_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"primitive": "str"}}}}], "constraints": []}}, "id": "core::iter::traits::collect::FromIterator", "path": "FromIterator"}, "trait_path": "core::iter::traits::collect::FromIterator"}`

Source: `src/array/run_array.rs:572`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-605832fde335cea26b0bb996"></a>
## get_array_memory_size

`function` · `arrow_array::array::run_array::RunArray::get_array_memory_size` · arrow-array 59.3.0

```rust
fn get_array_memory_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::run_array::RunArray", "path": "RunArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [421, 1], "end": [515, 2], "filename": "src/array/run_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/run_array.rs:504`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ec242b413efb744a3d262ff2"></a>
## get_buffer_memory_size

`function` · `arrow_array::array::run_array::RunArray::get_buffer_memory_size` · arrow-array 59.3.0

```rust
fn get_buffer_memory_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::run_array::RunArray", "path": "RunArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [421, 1], "end": [515, 2], "filename": "src/array/run_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/run_array.rs:500`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9949c944780b56f1707f0c94"></a>
## get_end_physical_index

`function` · `arrow_array::array::run_array::RunArray::get_end_physical_index` · arrow-array 59.3.0

```rust
fn get_end_physical_index(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_array::array::run_array::RunArray", "path": "RunArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [100, 1], "end": [354, 2], "filename": "src/array/run_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/run_array.rs:292`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the physical index at which the array slice ends.

See [`RunEndBuffer::get_end_physical_index`].

Unresolved upstream links (retained, not inferred): ``RunEndBuffer::get_end_physical_index``.

<a id="op-41161b3a1b63ade6e0274585"></a>
## get_physical_index

`function` · `arrow_array::array::run_array::RunArray::get_physical_index` · arrow-array 59.3.0

```rust
fn get_physical_index(&self, logical_index: usize) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_array::array::run_array::RunArray", "path": "RunArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [100, 1], "end": [354, 2], "filename": "src/array/run_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/run_array.rs:319`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Calls [`RunEndBuffer::get_physical_index`].

The result is arbitrary if `logical_index >= self.len()`

Unresolved upstream links (retained, not inferred): ``RunEndBuffer::get_physical_index``.

<a id="op-32a8d6fffe095a3d431ae196"></a>
## get_physical_indices

`function` · `arrow_array::array::run_array::RunArray::get_physical_indices` · arrow-array 59.3.0

```rust
fn get_physical_indices<I>(&self, logical_indices: &[I]) -> Result<Vec<usize>, ArrowError> where I: ArrowNativeType
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_array::array::run_array::RunArray", "path": "RunArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [100, 1], "end": [354, 2], "filename": "src/array/run_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/run_array.rs:327`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the physical indices corresponding to the provided logical indices.

See [`RunEndBuffer::get_physical_indices`] for more details.

Unresolved upstream links (retained, not inferred): ``RunEndBuffer::get_physical_indices``.

<a id="op-55784bcb2e11fc39f19d9840"></a>
## get_start_physical_index

`function` · `arrow_array::array::run_array::RunArray::get_start_physical_index` · arrow-array 59.3.0

```rust
fn get_start_physical_index(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_array::array::run_array::RunArray", "path": "RunArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [100, 1], "end": [354, 2], "filename": "src/array/run_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/run_array.rs:285`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the physical index at which the array slice starts.

See [`RunEndBuffer::get_start_physical_index`].

Unresolved upstream links (retained, not inferred): ``RunEndBuffer::get_start_physical_index``.

<a id="op-befccfc179695d83a91c2180"></a>
## into_data

`function` · `arrow_array::array::run_array::RunArray::into_data` · arrow-array 59.3.0

```rust
fn into_data(self) -> ArrayData
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::run_array::RunArray", "path": "RunArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [421, 1], "end": [515, 2], "filename": "src/array/run_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/run_array.rs:430`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-164eee944da8d332be6c4e07"></a>
## into_parts

`function` · `arrow_array::array::run_array::RunArray::into_parts` · arrow-array 59.3.0

```rust
fn into_parts(self) -> (DataType, RunEndBuffer<R::Native>, ArrayRef)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_array::array::run_array::RunArray", "path": "RunArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [100, 1], "end": [354, 2], "filename": "src/array/run_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/run_array.rs:205`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Deconstruct this array into its constituent parts

<a id="op-07e7694cc8b5170b925014b8"></a>
## is_empty

`function` · `arrow_array::array::run_array::RunArray::is_empty` · arrow-array 59.3.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::run_array::RunArray", "path": "RunArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [421, 1], "end": [515, 2], "filename": "src/array/run_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/run_array.rs:446`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-804ba17394b084681cd2434d"></a>
## is_nullable

`function` · `arrow_array::array::run_array::RunArray::is_nullable` · arrow-array 59.3.0

```rust
fn is_nullable(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::run_array::RunArray", "path": "RunArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [421, 1], "end": [515, 2], "filename": "src/array/run_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/run_array.rs:496`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d0fd7f0dce9be2b589fa0f5"></a>
## len

`function` · `arrow_array::array::run_array::RunArray::len` · arrow-array 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::run_array::RunArray", "path": "RunArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [421, 1], "end": [515, 2], "filename": "src/array/run_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/run_array.rs:442`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-04b6833e9d24be27dbb18937"></a>
## logical_len

`function` · `arrow_array::array::run_array::RunArray::logical_len` · arrow-array 59.3.0

```rust
fn logical_len(run_ends: &PrimitiveArray<R>) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_array::array::run_array::RunArray", "path": "RunArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [100, 1], "end": [354, 2], "filename": "src/array/run_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/run_array.rs:103`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Calculates the logical length of the array encoded by treating the `run_ends`
array as if it were a [`RunEndBuffer`](../operations/arrow_buffer.buffer.run.RunEndBuffer.md#op-e35311e9ed6d50fb4e98aafc).

<a id="op-9fb6a20bde26492ee2e36692"></a>
## logical_nulls

`function` · `arrow_array::array::run_array::RunArray::logical_nulls` · arrow-array 59.3.0

```rust
fn logical_nulls(&self) -> Option<NullBuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::run_array::RunArray", "path": "RunArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [421, 1], "end": [515, 2], "filename": "src/array/run_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/run_array.rs:463`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-90ccb4846edbcb993c839fd2"></a>
## new_unchecked

`function` · `arrow_array::array::run_array::RunArray::new_unchecked` · arrow-array 59.3.0

```rust
unsafe fn new_unchecked(data_type: DataType, run_ends: RunEndBuffer<R::Native>, values: ArrayRef) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_array::array::run_array::RunArray", "path": "RunArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [100, 1], "end": [354, 2], "filename": "src/array/run_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/run_array.rs:150`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`RunArray`](../operations/arrow_array.array.run_array.RunArray.md#op-f0742b2d9e045922a076478f) from the provided parts, without validation

# Safety

Safe if [`Self::try_new`](../operations/arrow_array.array.run_array.RunArray.md#op-462c4de0d60717396e0321ca) would not error

<a id="op-55bc17ceb1c83800ad7a07ea"></a>
## nulls

`function` · `arrow_array::array::run_array::RunArray::nulls` · arrow-array 59.3.0

```rust
fn nulls(&self) -> Option<&NullBuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::run_array::RunArray", "path": "RunArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [421, 1], "end": [515, 2], "filename": "src/array/run_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/run_array.rs:459`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9b123365dd7d4693b483041a"></a>
## offset

`function` · `arrow_array::array::run_array::RunArray::offset` · arrow-array 59.3.0

```rust
fn offset(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::run_array::RunArray", "path": "RunArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [421, 1], "end": [515, 2], "filename": "src/array/run_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/run_array.rs:455`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8ac9704b96ca2f079bb5d218"></a>
## run_ends

`function` · `arrow_array::array::run_array::RunArray::run_ends` · arrow-array 59.3.0

```rust
fn run_ends(&self) -> &RunEndBuffer<R::Native>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_array::array::run_array::RunArray", "path": "RunArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [100, 1], "end": [354, 2], "filename": "src/array/run_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/run_array.rs:210`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a reference to the [`RunEndBuffer`](../operations/arrow_buffer.buffer.run.RunEndBuffer.md#op-e35311e9ed6d50fb4e98aafc).

<a id="op-41ae96ce97323ec234a4e599"></a>
## shrink_to_fit

`function` · `arrow_array::array::run_array::RunArray::shrink_to_fit` · arrow-array 59.3.0

```rust
fn shrink_to_fit(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::run_array::RunArray", "path": "RunArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [421, 1], "end": [515, 2], "filename": "src/array/run_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/run_array.rs:450`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e44713f9446dc32f4e0c27ec"></a>
## slice

`function` · `arrow_array::array::run_array::RunArray::slice` · arrow-array 59.3.0

```rust
fn slice(&self, offset: usize, length: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_array::array::run_array::RunArray", "path": "RunArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [100, 1], "end": [354, 2], "filename": "src/array/run_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/run_array.rs:347`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a zero-copy slice of this array with the indicated offset and length.

# Panics

- Specified slice (`offset` + `length`) exceeds existing length

<a id="op-fed977eab944bd7ad54e3cab"></a>
## slice

`function` · `arrow_array::array::run_array::RunArray::slice` · arrow-array 59.3.0

```rust
fn slice(&self, offset: usize, length: usize) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::run_array::RunArray", "path": "RunArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [421, 1], "end": [515, 2], "filename": "src/array/run_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/run_array.rs:438`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6052b9f1cc9ce6d6842e9cf8"></a>
## to_data

`function` · `arrow_array::array::run_array::RunArray::to_data` · arrow-array 59.3.0

```rust
fn to_data(&self) -> ArrayData
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::run_array::RunArray", "path": "RunArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [421, 1], "end": [515, 2], "filename": "src/array/run_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/run_array.rs:426`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-462c4de0d60717396e0321ca"></a>
## try_new

`function` · `arrow_array::array::run_array::RunArray::try_new` · arrow-array 59.3.0

```rust
fn try_new(run_ends: &PrimitiveArray<R>, values: &dyn Array) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_array::array::run_array::RunArray", "path": "RunArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [100, 1], "end": [354, 2], "filename": "src/array/run_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/run_array.rs:118`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Attempts to create a [`RunArray`](../operations/arrow_array.array.run_array.RunArray.md#op-f0742b2d9e045922a076478f) using the given `run_ends` and `values`.

# Errors

- If `run_ends` and `values` have different lengths
- If `run_ends` has any null values
- If `run_ends` doesn't consist of strictly increasing positive integers

<a id="op-3d6722c72c8815acd53335f6"></a>
## values

`function` · `arrow_array::array::run_array::RunArray::values` · arrow-array 59.3.0

```rust
fn values(&self) -> &ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_array::array::run_array::RunArray", "path": "RunArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [100, 1], "end": [354, 2], "filename": "src/array/run_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/run_array.rs:218`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a reference to the values array.

Any slicing of this [`RunArray`](../operations/arrow_array.array.run_array.RunArray.md#op-f0742b2d9e045922a076478f) array is **not** applied to the returned
values here and must be handled separately.

<a id="op-e98a6ec41da0ea0647003854"></a>
## values

`function` · `arrow_array::array::run_array::RunArray::values` · arrow-array 59.3.0

```rust
fn values(&self) -> &Arc<dyn Array>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_array::array::run_array::RunArray", "path": "RunArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [804, 1], "end": [812, 2], "filename": "src/array/run_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::run_array::AnyRunEndArray", "path": "AnyRunEndArray"}, "trait_path": "arrow_array::array::run_array::AnyRunEndArray"}`

Source: `src/array/run_array.rs:805`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-29bd606cdae0317280c8f123"></a>
## values_slice

`function` · `arrow_array::array::run_array::RunArray::values_slice` · arrow-array 59.3.0

```rust
fn values_slice(&self) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_array::array::run_array::RunArray", "path": "RunArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [100, 1], "end": [354, 2], "filename": "src/array/run_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/run_array.rs:273`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Similar to [`values`] but accounts for logical slicing, returning only the values
that are part of the logical slice of this array.

[`values`]: Self::values

<a id="op-896f7c463cee817955ce07a4"></a>
## with_values

`function` · `arrow_array::array::run_array::RunArray::with_values` · arrow-array 59.3.0

```rust
fn with_values(&self, values: ArrayRef) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_array::array::run_array::RunArray", "path": "RunArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [100, 1], "end": [354, 2], "filename": "src/array/run_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/run_array.rs:247`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a new [`RunArray`](../operations/arrow_array.array.run_array.RunArray.md#op-f0742b2d9e045922a076478f) with the same `run_ends` and the supplied `values`.

# Panics

Panics if `values.len()` does not equal `self.values().len()`.

# Example

```
# use std::sync::Arc;
# use arrow_array::{RunArray, Int32Array, StringArray, ArrayRef,Array};
# use arrow_array::types::Int32Type;
// A RunArray logically representing ["a", "a", "b", "c", "c"]
let run_ends = Int32Array::from(vec![2, 3, 5]);
let values: ArrayRef = Arc::new(StringArray::from(vec!["a", "b", "c"]));
let run_array = RunArray::<Int32Type>::try_new(&run_ends, &values).unwrap();

// Swap in new values while keeping the same run pattern.
// The result logically represents ["x", "x", "y", "z", "z"].
let new_values: ArrayRef = Arc::new(StringArray::from(vec!["x", "y", "z"]));
let new_run_array = run_array.with_values(new_values);

assert_eq!(new_run_array.len(), 5);
assert_eq!(new_run_array.run_ends().values(), &[2, 3, 5]);
```

<a id="op-fd7b68d3e406dd665ffc91cd"></a>
## with_values

`function` · `arrow_array::array::run_array::RunArray::with_values` · arrow-array 59.3.0

```rust
fn with_values(&self, values: ArrayRef) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_array::array::run_array::RunArray", "path": "RunArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [804, 1], "end": [812, 2], "filename": "src/array/run_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::run_array::AnyRunEndArray", "path": "AnyRunEndArray"}, "trait_path": "arrow_array::array::run_array::AnyRunEndArray"}`

Source: `src/array/run_array.rs:809`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
