# `arrow_array::array::dictionary_array::TypedDictionaryArray`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.dictionary_array.TypedDictionaryArray.json).

<a id="op-177ac4d9426c85e1ab27019d"></a>
## TypedDictionaryArray

`struct` · `arrow_array::array::dictionary_array::TypedDictionaryArray` · arrow-array 59.3.0

```rust
struct TypedDictionaryArray<'a, K: ArrowDictionaryKeyType, V>
```

Source: `src/array/dictionary_array.rs:830`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A [`DictionaryArray`](../operations/arrow_array.array.dictionary_array.DictionaryArray.md#op-761ab0c11d727f026bdc2e47) typed on its child values array

Implements [`ArrayAccessor`](../operations/arrow_array.array.ArrayAccessor.md#op-0f7f2e64730382bf2e8d3393) allowing fast access to its elements

```
use arrow_array::{DictionaryArray, StringArray, types::Int32Type};

let orig = ["a", "b", "a", "b"];
let dictionary = DictionaryArray::<Int32Type>::from_iter(orig);

// `TypedDictionaryArray` allows you to access the values directly
let typed = dictionary.downcast_dict::<StringArray>().unwrap();

for (maybe_val, orig) in typed.into_iter().zip(orig) {
    assert_eq!(maybe_val.unwrap(), orig)
}
```

<a id="op-625b1876e2babb361e694543"></a>
## IntoIter

`assoc_type` · `arrow_array::array::dictionary_array::TypedDictionaryArray::IntoIter` · arrow-array 59.3.0

```rust
IntoIter
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::TypedDictionaryArray", "path": "TypedDictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::ArrayAccessor", "path": "ArrayAccessor"}}}], "generic_params": [], "type": {"generic": "Self"}}}]}, "is_negative": false, "span": {"begin": [927, 1], "end": [938, 2], "filename": "src/array/dictionary_array.rs"}, "trait": {"args": null, "id": "core::iter::traits::collect::IntoIterator", "path": "IntoIterator"}, "trait_path": "core::iter::traits::collect::IntoIterator"}`

Source: `src/array/dictionary_array.rs:933`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-56f9bfd9c5d596e458c9446f"></a>
## Item

`assoc_type` · `arrow_array::array::dictionary_array::TypedDictionaryArray::Item` · arrow-array 59.3.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::TypedDictionaryArray", "path": "TypedDictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::ArrayAccessor", "path": "ArrayAccessor"}}}], "generic_params": [], "type": {"generic": "Self"}}}]}, "is_negative": false, "span": {"begin": [927, 1], "end": [938, 2], "filename": "src/array/dictionary_array.rs"}, "trait": {"args": null, "id": "core::iter::traits::collect::IntoIterator", "path": "IntoIterator"}, "trait_path": "core::iter::traits::collect::IntoIterator"}`

Source: `src/array/dictionary_array.rs:932`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bac3136a84c7b97957822c03"></a>
## Item

`assoc_type` · `arrow_array::array::dictionary_array::TypedDictionaryArray::Item` · arrow-array 59.3.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::TypedDictionaryArray", "path": "TypedDictionaryArray"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "generic_params": [], "type": {"generic": "V"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::ArrayAccessor", "path": "ArrayAccessor"}}}], "generic_params": [], "type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"generic": "V"}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::default::Default", "path": "Default"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Item", "self_type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"generic": "V"}}}, "trait": {"args": null, "id": "arrow_array::array::ArrayAccessor", "path": "ArrayAccessor"}}}}}]}, "is_negative": false, "span": {"begin": [940, 1], "end": [970, 2], "filename": "src/array/dictionary_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::ArrayAccessor", "path": "ArrayAccessor"}, "trait_path": "arrow_array::array::ArrayAccessor"}`

Source: `src/array/dictionary_array.rs:947`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-34c2198f3d5c498fa9b5b09d"></a>
## as_any

`function` · `arrow_array::array::dictionary_array::TypedDictionaryArray::as_any` · arrow-array 59.3.0

```rust
fn as_any(&self) -> &dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::TypedDictionaryArray", "path": "TypedDictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [864, 1], "end": [925, 2], "filename": "src/array/dictionary_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/dictionary_array.rs:865`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-467445bb27b0a10e35befeaf"></a>
## claim

`function` · `arrow_array::array::dictionary_array::TypedDictionaryArray::claim` · arrow-array 59.3.0

```rust
fn claim(&self, pool: &dyn arrow_buffer::MemoryPool)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::TypedDictionaryArray", "path": "TypedDictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [864, 1], "end": [925, 2], "filename": "src/array/dictionary_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/dictionary_array.rs:922`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c91fd636bad18525b02cc355"></a>
## clone

`function` · `arrow_array::array::dictionary_array::TypedDictionaryArray::clone` · arrow-array 59.3.0

```rust
fn clone(&self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::TypedDictionaryArray", "path": "TypedDictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [838, 1], "end": [842, 2], "filename": "src/array/dictionary_array.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/array/dictionary_array.rs:839`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-874c3118ea4a3182f4ea1b83"></a>
## data_type

`function` · `arrow_array::array::dictionary_array::TypedDictionaryArray::data_type` · arrow-array 59.3.0

```rust
fn data_type(&self) -> &DataType
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::TypedDictionaryArray", "path": "TypedDictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [864, 1], "end": [925, 2], "filename": "src/array/dictionary_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/dictionary_array.rs:877`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-61907f71bf85253e8f66f2ad"></a>
## fmt

`function` · `arrow_array::array::dictionary_array::TypedDictionaryArray::fmt` · arrow-array 59.3.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::TypedDictionaryArray", "path": "TypedDictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [846, 1], "end": [850, 2], "filename": "src/array/dictionary_array.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/array/dictionary_array.rs:847`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0e73789fa7ac2ebf1409ca33"></a>
## get_array_memory_size

`function` · `arrow_array::array::dictionary_array::TypedDictionaryArray::get_array_memory_size` · arrow-array 59.3.0

```rust
fn get_array_memory_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::TypedDictionaryArray", "path": "TypedDictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [864, 1], "end": [925, 2], "filename": "src/array/dictionary_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/dictionary_array.rs:917`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2e8702bdd22859af296381b7"></a>
## get_buffer_memory_size

`function` · `arrow_array::array::dictionary_array::TypedDictionaryArray::get_buffer_memory_size` · arrow-array 59.3.0

```rust
fn get_buffer_memory_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::TypedDictionaryArray", "path": "TypedDictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [864, 1], "end": [925, 2], "filename": "src/array/dictionary_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/dictionary_array.rs:913`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8dbda6fe7d039bd5437ccd06"></a>
## into_data

`function` · `arrow_array::array::dictionary_array::TypedDictionaryArray::into_data` · arrow-array 59.3.0

```rust
fn into_data(self) -> ArrayData
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::TypedDictionaryArray", "path": "TypedDictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [864, 1], "end": [925, 2], "filename": "src/array/dictionary_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/dictionary_array.rs:873`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d517aa66c5911ab5de1922cc"></a>
## into_iter

`function` · `arrow_array::array::dictionary_array::TypedDictionaryArray::into_iter` · arrow-array 59.3.0

```rust
fn into_iter(self) -> Self::IntoIter
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::TypedDictionaryArray", "path": "TypedDictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::ArrayAccessor", "path": "ArrayAccessor"}}}], "generic_params": [], "type": {"generic": "Self"}}}]}, "is_negative": false, "span": {"begin": [927, 1], "end": [938, 2], "filename": "src/array/dictionary_array.rs"}, "trait": {"args": null, "id": "core::iter::traits::collect::IntoIterator", "path": "IntoIterator"}, "trait_path": "core::iter::traits::collect::IntoIterator"}`

Source: `src/array/dictionary_array.rs:935`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d75e814a70da8b167968e57e"></a>
## is_empty

`function` · `arrow_array::array::dictionary_array::TypedDictionaryArray::is_empty` · arrow-array 59.3.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::TypedDictionaryArray", "path": "TypedDictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [864, 1], "end": [925, 2], "filename": "src/array/dictionary_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/dictionary_array.rs:889`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-67c9bdb7553c2bc952005364"></a>
## is_nullable

`function` · `arrow_array::array::dictionary_array::TypedDictionaryArray::is_nullable` · arrow-array 59.3.0

```rust
fn is_nullable(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::TypedDictionaryArray", "path": "TypedDictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [864, 1], "end": [925, 2], "filename": "src/array/dictionary_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/dictionary_array.rs:909`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7683c4ea49ecc6851b2a538d"></a>
## keys

`function` · `arrow_array::array::dictionary_array::TypedDictionaryArray::keys` · arrow-array 59.3.0

```rust
fn keys(&self) -> &'a PrimitiveArray<K>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::TypedDictionaryArray", "path": "TypedDictionaryArray"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [852, 1], "end": [862, 2], "filename": "src/array/dictionary_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/dictionary_array.rs:854`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the keys of this [`TypedDictionaryArray`](../operations/arrow_array.array.dictionary_array.TypedDictionaryArray.md#op-177ac4d9426c85e1ab27019d)

<a id="op-0b5190e30dfd681a2c50a7d5"></a>
## len

`function` · `arrow_array::array::dictionary_array::TypedDictionaryArray::len` · arrow-array 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::TypedDictionaryArray", "path": "TypedDictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [864, 1], "end": [925, 2], "filename": "src/array/dictionary_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/dictionary_array.rs:885`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e6f0d7d7b24be42462f48c4e"></a>
## logical_null_count

`function` · `arrow_array::array::dictionary_array::TypedDictionaryArray::logical_null_count` · arrow-array 59.3.0

```rust
fn logical_null_count(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::TypedDictionaryArray", "path": "TypedDictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [864, 1], "end": [925, 2], "filename": "src/array/dictionary_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/dictionary_array.rs:905`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eae5ab9018d208f9c067be7c"></a>
## logical_nulls

`function` · `arrow_array::array::dictionary_array::TypedDictionaryArray::logical_nulls` · arrow-array 59.3.0

```rust
fn logical_nulls(&self) -> Option<NullBuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::TypedDictionaryArray", "path": "TypedDictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [864, 1], "end": [925, 2], "filename": "src/array/dictionary_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/dictionary_array.rs:901`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cf24b499abc016dcd950d3fb"></a>
## nulls

`function` · `arrow_array::array::dictionary_array::TypedDictionaryArray::nulls` · arrow-array 59.3.0

```rust
fn nulls(&self) -> Option<&NullBuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::TypedDictionaryArray", "path": "TypedDictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [864, 1], "end": [925, 2], "filename": "src/array/dictionary_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/dictionary_array.rs:897`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-32a8a61e4a3a58c72f70ce78"></a>
## offset

`function` · `arrow_array::array::dictionary_array::TypedDictionaryArray::offset` · arrow-array 59.3.0

```rust
fn offset(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::TypedDictionaryArray", "path": "TypedDictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [864, 1], "end": [925, 2], "filename": "src/array/dictionary_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/dictionary_array.rs:893`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-206b262c0ea6b0f49e0965db"></a>
## slice

`function` · `arrow_array::array::dictionary_array::TypedDictionaryArray::slice` · arrow-array 59.3.0

```rust
fn slice(&self, offset: usize, length: usize) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::TypedDictionaryArray", "path": "TypedDictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [864, 1], "end": [925, 2], "filename": "src/array/dictionary_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/dictionary_array.rs:881`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-78aaeee658c30fcbb7d18a45"></a>
## to_data

`function` · `arrow_array::array::dictionary_array::TypedDictionaryArray::to_data` · arrow-array 59.3.0

```rust
fn to_data(&self) -> ArrayData
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::TypedDictionaryArray", "path": "TypedDictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [864, 1], "end": [925, 2], "filename": "src/array/dictionary_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/dictionary_array.rs:869`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-20a7c504164f10e15f6f2e25"></a>
## value

`function` · `arrow_array::array::dictionary_array::TypedDictionaryArray::value` · arrow-array 59.3.0

```rust
fn value(&self, index: usize) -> Self::Item
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::TypedDictionaryArray", "path": "TypedDictionaryArray"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "generic_params": [], "type": {"generic": "V"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::ArrayAccessor", "path": "ArrayAccessor"}}}], "generic_params": [], "type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"generic": "V"}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::default::Default", "path": "Default"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Item", "self_type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"generic": "V"}}}, "trait": {"args": null, "id": "arrow_array::array::ArrayAccessor", "path": "ArrayAccessor"}}}}}]}, "is_negative": false, "span": {"begin": [940, 1], "end": [970, 2], "filename": "src/array/dictionary_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::ArrayAccessor", "path": "ArrayAccessor"}, "trait_path": "arrow_array::array::ArrayAccessor"}`

Source: `src/array/dictionary_array.rs:949`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e140178cf6b9b7d7a6b723c9"></a>
## value_unchecked

`function` · `arrow_array::array::dictionary_array::TypedDictionaryArray::value_unchecked` · arrow-array 59.3.0

```rust
unsafe fn value_unchecked(&self, index: usize) -> Self::Item
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::TypedDictionaryArray", "path": "TypedDictionaryArray"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "generic_params": [], "type": {"generic": "V"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::ArrayAccessor", "path": "ArrayAccessor"}}}], "generic_params": [], "type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"generic": "V"}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::default::Default", "path": "Default"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Item", "self_type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"generic": "V"}}}, "trait": {"args": null, "id": "arrow_array::array::ArrayAccessor", "path": "ArrayAccessor"}}}}}]}, "is_negative": false, "span": {"begin": [940, 1], "end": [970, 2], "filename": "src/array/dictionary_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::ArrayAccessor", "path": "ArrayAccessor"}, "trait_path": "arrow_array::array::ArrayAccessor"}`

Source: `src/array/dictionary_array.rs:959`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ff685cc83a47388cdda5914b"></a>
## values

`function` · `arrow_array::array::dictionary_array::TypedDictionaryArray::values` · arrow-array 59.3.0

```rust
fn values(&self) -> &'a V
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::TypedDictionaryArray", "path": "TypedDictionaryArray"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [852, 1], "end": [862, 2], "filename": "src/array/dictionary_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/dictionary_array.rs:859`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the values of this [`TypedDictionaryArray`](../operations/arrow_array.array.dictionary_array.TypedDictionaryArray.md#op-177ac4d9426c85e1ab27019d)
