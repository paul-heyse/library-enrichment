# `arrow_array::array::run_array::TypedRunArray`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.run_array.TypedRunArray.json).

<a id="op-507451b8b0e62718e6f7a05e"></a>
## TypedRunArray

`struct` · `arrow_array::array::run_array::TypedRunArray` · arrow-array 59.3.0

```rust
struct TypedRunArray<'a, R: RunEndIndexType, V>
```

Source: `src/array/run_array.rs:646`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A [`RunArray`](../operations/arrow_array.array.run_array.RunArray.md#op-f0742b2d9e045922a076478f) typed typed on its child values array

Implements [`ArrayAccessor`](../operations/arrow_array.array.ArrayAccessor.md#op-0f7f2e64730382bf2e8d3393) and [`IntoIterator`] allowing fast access to its elements

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

Unresolved upstream links (retained, not inferred): ``IntoIterator``.

<a id="op-20fbb918644ec9f60d5a95e1"></a>
## IntoIter

`assoc_type` · `arrow_array::array::run_array::TypedRunArray::IntoIter` · arrow-array 59.3.0

```rust
IntoIter
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::array::run_array::TypedRunArray", "path": "TypedRunArray"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "generic_params": [], "type": {"generic": "R"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "generic_params": [], "type": {"generic": "V"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::ArrayAccessor", "path": "ArrayAccessor"}}}], "generic_params": [], "type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"generic": "V"}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::default::Default", "path": "Default"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Item", "self_type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"generic": "V"}}}, "trait": {"args": null, "id": "arrow_array::array::ArrayAccessor", "path": "ArrayAccessor"}}}}}]}, "is_negative": false, "span": {"begin": [777, 1], "end": [790, 2], "filename": "src/array/run_array.rs"}, "trait": {"args": null, "id": "core::iter::traits::collect::IntoIterator", "path": "IntoIterator"}, "trait_path": "core::iter::traits::collect::IntoIterator"}`

Source: `src/array/run_array.rs:785`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0c3d6d85173faf91443fb1e7"></a>
## Item

`assoc_type` · `arrow_array::array::run_array::TypedRunArray::Item` · arrow-array 59.3.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::array::run_array::TypedRunArray", "path": "TypedRunArray"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "generic_params": [], "type": {"generic": "R"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "generic_params": [], "type": {"generic": "V"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::ArrayAccessor", "path": "ArrayAccessor"}}}], "generic_params": [], "type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"generic": "V"}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::default::Default", "path": "Default"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Item", "self_type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"generic": "V"}}}, "trait": {"args": null, "id": "arrow_array::array::ArrayAccessor", "path": "ArrayAccessor"}}}}}]}, "is_negative": false, "span": {"begin": [752, 1], "end": [775, 2], "filename": "src/array/run_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::ArrayAccessor", "path": "ArrayAccessor"}, "trait_path": "arrow_array::array::ArrayAccessor"}`

Source: `src/array/run_array.rs:759`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2397a9368affd1b497e1b963"></a>
## Item

`assoc_type` · `arrow_array::array::run_array::TypedRunArray::Item` · arrow-array 59.3.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::array::run_array::TypedRunArray", "path": "TypedRunArray"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "generic_params": [], "type": {"generic": "R"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "generic_params": [], "type": {"generic": "V"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::ArrayAccessor", "path": "ArrayAccessor"}}}], "generic_params": [], "type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"generic": "V"}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::default::Default", "path": "Default"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Item", "self_type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"generic": "V"}}}, "trait": {"args": null, "id": "arrow_array::array::ArrayAccessor", "path": "ArrayAccessor"}}}}}]}, "is_negative": false, "span": {"begin": [777, 1], "end": [790, 2], "filename": "src/array/run_array.rs"}, "trait": {"args": null, "id": "core::iter::traits::collect::IntoIterator", "path": "IntoIterator"}, "trait_path": "core::iter::traits::collect::IntoIterator"}`

Source: `src/array/run_array.rs:784`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d839b6533db61c159bde2076"></a>
## as_any

`function` · `arrow_array::array::run_array::TypedRunArray::as_any` · arrow-array 59.3.0

```rust
fn as_any(&self) -> &dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::array::run_array::TypedRunArray", "path": "TypedRunArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [687, 1], "end": [748, 2], "filename": "src/array/run_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/run_array.rs:688`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-45bd3b7848744a561a30c58a"></a>
## claim

`function` · `arrow_array::array::run_array::TypedRunArray::claim` · arrow-array 59.3.0

```rust
fn claim(&self, pool: &dyn arrow_buffer::MemoryPool)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::array::run_array::TypedRunArray", "path": "TypedRunArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [687, 1], "end": [748, 2], "filename": "src/array/run_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/run_array.rs:745`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-792f4e06f924ec6d40b879f6"></a>
## clone

`function` · `arrow_array::array::run_array::TypedRunArray::clone` · arrow-array 59.3.0

```rust
fn clone(&self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::array::run_array::TypedRunArray", "path": "TypedRunArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [655, 1], "end": [659, 2], "filename": "src/array/run_array.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/array/run_array.rs:656`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-88d5e0cadf75de124b8855f7"></a>
## data_type

`function` · `arrow_array::array::run_array::TypedRunArray::data_type` · arrow-array 59.3.0

```rust
fn data_type(&self) -> &DataType
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::array::run_array::TypedRunArray", "path": "TypedRunArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [687, 1], "end": [748, 2], "filename": "src/array/run_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/run_array.rs:700`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-31e7e3c4fe5e6a8725cc4981"></a>
## fmt

`function` · `arrow_array::array::run_array::TypedRunArray::fmt` · arrow-array 59.3.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::array::run_array::TypedRunArray", "path": "TypedRunArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [663, 1], "end": [667, 2], "filename": "src/array/run_array.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/array/run_array.rs:664`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-91492816108ddf23bf9c1949"></a>
## get_array_memory_size

`function` · `arrow_array::array::run_array::TypedRunArray::get_array_memory_size` · arrow-array 59.3.0

```rust
fn get_array_memory_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::array::run_array::TypedRunArray", "path": "TypedRunArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [687, 1], "end": [748, 2], "filename": "src/array/run_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/run_array.rs:740`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-de5be6a9cba8079ef8337e7c"></a>
## get_buffer_memory_size

`function` · `arrow_array::array::run_array::TypedRunArray::get_buffer_memory_size` · arrow-array 59.3.0

```rust
fn get_buffer_memory_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::array::run_array::TypedRunArray", "path": "TypedRunArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [687, 1], "end": [748, 2], "filename": "src/array/run_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/run_array.rs:736`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-10e3e23bb116f43c8188aea9"></a>
## into_data

`function` · `arrow_array::array::run_array::TypedRunArray::into_data` · arrow-array 59.3.0

```rust
fn into_data(self) -> ArrayData
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::array::run_array::TypedRunArray", "path": "TypedRunArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [687, 1], "end": [748, 2], "filename": "src/array/run_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/run_array.rs:696`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-55c4802981b2e58fe6b76735"></a>
## into_iter

`function` · `arrow_array::array::run_array::TypedRunArray::into_iter` · arrow-array 59.3.0

```rust
fn into_iter(self) -> Self::IntoIter
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::array::run_array::TypedRunArray", "path": "TypedRunArray"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "generic_params": [], "type": {"generic": "R"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "generic_params": [], "type": {"generic": "V"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::ArrayAccessor", "path": "ArrayAccessor"}}}], "generic_params": [], "type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"generic": "V"}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::default::Default", "path": "Default"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Item", "self_type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"generic": "V"}}}, "trait": {"args": null, "id": "arrow_array::array::ArrayAccessor", "path": "ArrayAccessor"}}}}}]}, "is_negative": false, "span": {"begin": [777, 1], "end": [790, 2], "filename": "src/array/run_array.rs"}, "trait": {"args": null, "id": "core::iter::traits::collect::IntoIterator", "path": "IntoIterator"}, "trait_path": "core::iter::traits::collect::IntoIterator"}`

Source: `src/array/run_array.rs:787`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f388f12eb2ff52f7179956c4"></a>
## is_empty

`function` · `arrow_array::array::run_array::TypedRunArray::is_empty` · arrow-array 59.3.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::array::run_array::TypedRunArray", "path": "TypedRunArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [687, 1], "end": [748, 2], "filename": "src/array/run_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/run_array.rs:712`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9b40169abcfd4f3d972bb794"></a>
## is_nullable

`function` · `arrow_array::array::run_array::TypedRunArray::is_nullable` · arrow-array 59.3.0

```rust
fn is_nullable(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::array::run_array::TypedRunArray", "path": "TypedRunArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [687, 1], "end": [748, 2], "filename": "src/array/run_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/run_array.rs:732`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-090233c3bb334d21c6b5f5f4"></a>
## len

`function` · `arrow_array::array::run_array::TypedRunArray::len` · arrow-array 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::array::run_array::TypedRunArray", "path": "TypedRunArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [687, 1], "end": [748, 2], "filename": "src/array/run_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/run_array.rs:708`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5a5652647d4b8692e1be9a2c"></a>
## logical_null_count

`function` · `arrow_array::array::run_array::TypedRunArray::logical_null_count` · arrow-array 59.3.0

```rust
fn logical_null_count(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::array::run_array::TypedRunArray", "path": "TypedRunArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [687, 1], "end": [748, 2], "filename": "src/array/run_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/run_array.rs:728`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea607e026ec9a1309fb554cd"></a>
## logical_nulls

`function` · `arrow_array::array::run_array::TypedRunArray::logical_nulls` · arrow-array 59.3.0

```rust
fn logical_nulls(&self) -> Option<NullBuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::array::run_array::TypedRunArray", "path": "TypedRunArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [687, 1], "end": [748, 2], "filename": "src/array/run_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/run_array.rs:724`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ab72b309af41ff0d8bd29bfd"></a>
## nulls

`function` · `arrow_array::array::run_array::TypedRunArray::nulls` · arrow-array 59.3.0

```rust
fn nulls(&self) -> Option<&NullBuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::array::run_array::TypedRunArray", "path": "TypedRunArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [687, 1], "end": [748, 2], "filename": "src/array/run_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/run_array.rs:720`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3b88808868d30fee4312c71e"></a>
## offset

`function` · `arrow_array::array::run_array::TypedRunArray::offset` · arrow-array 59.3.0

```rust
fn offset(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::array::run_array::TypedRunArray", "path": "TypedRunArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [687, 1], "end": [748, 2], "filename": "src/array/run_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/run_array.rs:716`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-220545d959655e92c44eda5e"></a>
## run_array

`function` · `arrow_array::array::run_array::TypedRunArray::run_array` · arrow-array 59.3.0

```rust
fn run_array(&self) -> &'a RunArray<R>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::array::run_array::TypedRunArray", "path": "TypedRunArray"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [669, 1], "end": [684, 2], "filename": "src/array/run_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/run_array.rs:681`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the run array of this [`TypedRunArray`](../operations/arrow_array.array.run_array.TypedRunArray.md#op-507451b8b0e62718e6f7a05e)

<a id="op-afb4a14397ab79d7ba91a5f1"></a>
## run_ends

`function` · `arrow_array::array::run_array::TypedRunArray::run_ends` · arrow-array 59.3.0

```rust
fn run_ends(&self) -> &'a RunEndBuffer<R::Native>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::array::run_array::TypedRunArray", "path": "TypedRunArray"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [669, 1], "end": [684, 2], "filename": "src/array/run_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/run_array.rs:671`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the run_ends of this [`TypedRunArray`](../operations/arrow_array.array.run_array.TypedRunArray.md#op-507451b8b0e62718e6f7a05e)

<a id="op-a6d03f497e83ad4fec7a6431"></a>
## slice

`function` · `arrow_array::array::run_array::TypedRunArray::slice` · arrow-array 59.3.0

```rust
fn slice(&self, offset: usize, length: usize) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::array::run_array::TypedRunArray", "path": "TypedRunArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [687, 1], "end": [748, 2], "filename": "src/array/run_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/run_array.rs:704`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-70ea2b9470cc300e2cd1cfad"></a>
## to_data

`function` · `arrow_array::array::run_array::TypedRunArray::to_data` · arrow-array 59.3.0

```rust
fn to_data(&self) -> ArrayData
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::array::run_array::TypedRunArray", "path": "TypedRunArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [687, 1], "end": [748, 2], "filename": "src/array/run_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/run_array.rs:692`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-66300db1010063467ede0161"></a>
## value

`function` · `arrow_array::array::run_array::TypedRunArray::value` · arrow-array 59.3.0

```rust
fn value(&self, logical_index: usize) -> Self::Item
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::array::run_array::TypedRunArray", "path": "TypedRunArray"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "generic_params": [], "type": {"generic": "R"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "generic_params": [], "type": {"generic": "V"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::ArrayAccessor", "path": "ArrayAccessor"}}}], "generic_params": [], "type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"generic": "V"}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::default::Default", "path": "Default"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Item", "self_type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"generic": "V"}}}, "trait": {"args": null, "id": "arrow_array::array::ArrayAccessor", "path": "ArrayAccessor"}}}}}]}, "is_negative": false, "span": {"begin": [752, 1], "end": [775, 2], "filename": "src/array/run_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::ArrayAccessor", "path": "ArrayAccessor"}, "trait_path": "arrow_array::array::ArrayAccessor"}`

Source: `src/array/run_array.rs:761`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4578a34bc5acc8c3e03cbe71"></a>
## value_unchecked

`function` · `arrow_array::array::run_array::TypedRunArray::value_unchecked` · arrow-array 59.3.0

```rust
unsafe fn value_unchecked(&self, logical_index: usize) -> Self::Item
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::array::run_array::TypedRunArray", "path": "TypedRunArray"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "generic_params": [], "type": {"generic": "R"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "generic_params": [], "type": {"generic": "V"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::ArrayAccessor", "path": "ArrayAccessor"}}}], "generic_params": [], "type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"generic": "V"}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::default::Default", "path": "Default"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Item", "self_type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"generic": "V"}}}, "trait": {"args": null, "id": "arrow_array::array::ArrayAccessor", "path": "ArrayAccessor"}}}}}]}, "is_negative": false, "span": {"begin": [752, 1], "end": [775, 2], "filename": "src/array/run_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::ArrayAccessor", "path": "ArrayAccessor"}, "trait_path": "arrow_array::array::ArrayAccessor"}`

Source: `src/array/run_array.rs:771`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2aaa2248b0653d8ecc424403"></a>
## values

`function` · `arrow_array::array::run_array::TypedRunArray::values` · arrow-array 59.3.0

```rust
fn values(&self) -> &'a V
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::array::run_array::TypedRunArray", "path": "TypedRunArray"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [669, 1], "end": [684, 2], "filename": "src/array/run_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/run_array.rs:676`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the values of this [`TypedRunArray`](../operations/arrow_array.array.run_array.TypedRunArray.md#op-507451b8b0e62718e6f7a05e)
