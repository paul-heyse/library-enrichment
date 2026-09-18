# `arrow_array::array::dictionary_array::DictionaryArray`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.dictionary_array.DictionaryArray.json).

<a id="op-761ab0c11d727f026bdc2e47"></a>
## DictionaryArray

`struct` · `arrow_array::array::dictionary_array::DictionaryArray` · arrow-array 59.3.0

```rust
struct DictionaryArray<K: ArrowDictionaryKeyType>
```

Source: `src/array/dictionary_array.rs:243`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

An array of [dictionary encoded values](https://arrow.apache.org/docs/format/Columnar.html#dictionary-encoded-layout)

This is mostly used to represent strings or a limited set of primitive types as integers,
for example when doing NLP analysis or representing chromosomes by name.

[`DictionaryArray`](../operations/arrow_array.array.dictionary_array.DictionaryArray.md#op-761ab0c11d727f026bdc2e47) are represented using a `keys` array and a
`values` array, which may be different lengths. The `keys` array
stores indexes in the `values` array which holds
the corresponding logical value, as shown here:

```text
┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
  ┌─────────────────┐  ┌─────────┐ │     ┌─────────────────┐
│ │        A        │  │    0    │       │        A        │     values[keys[0]]
  ├─────────────────┤  ├─────────┤ │     ├─────────────────┤
│ │        D        │  │    2    │       │        B        │     values[keys[1]]
  ├─────────────────┤  ├─────────┤ │     ├─────────────────┤
│ │        B        │  │    2    │       │        B        │     values[keys[2]]
  └─────────────────┘  ├─────────┤ │     ├─────────────────┤
│                      │    1    │       │        D        │     values[keys[3]]
                       ├─────────┤ │     ├─────────────────┤
│                      │    1    │       │        D        │     values[keys[4]]
                       ├─────────┤ │     ├─────────────────┤
│                      │    0    │       │        A        │     values[keys[5]]
                       └─────────┘ │     └─────────────────┘
│       values            keys
 ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┘
                                            Logical array
                                               Contents
          DictionaryArray
             length = 6
```

# Example: From Nullable Data

```
# use arrow_array::{DictionaryArray, Int8Array, types::Int8Type};
let test = vec!["a", "a", "b", "c"];
let array : DictionaryArray<Int8Type> = test.iter().map(|&x| if x == "b" {None} else {Some(x)}).collect();
assert_eq!(array.keys(), &Int8Array::from(vec![Some(0), Some(0), None, Some(1)]));
```

# Example: From Non-Nullable Data

```
# use arrow_array::{DictionaryArray, Int8Array, types::Int8Type};
let test = vec!["a", "a", "b", "c"];
let array : DictionaryArray<Int8Type> = test.into_iter().collect();
assert_eq!(array.keys(), &Int8Array::from(vec![0, 0, 1, 2]));
```

# Example: From Existing Arrays

```
# use std::sync::Arc;
# use arrow_array::{DictionaryArray, Int8Array, StringArray, types::Int8Type};
// You can form your own DictionaryArray by providing the
// values (dictionary) and keys (indexes into the dictionary):
let values = StringArray::from_iter_values(["a", "b", "c"]);
let keys = Int8Array::from_iter_values([0, 0, 1, 2]);
let array = DictionaryArray::<Int8Type>::try_new(keys, Arc::new(values)).unwrap();
let expected: DictionaryArray::<Int8Type> = vec!["a", "a", "b", "c"].into_iter().collect();
assert_eq!(&array, &expected);
```

# Example: Using Builder

```
# use arrow_array::{Array, StringArray};
# use arrow_array::builder::StringDictionaryBuilder;
# use arrow_array::types::Int32Type;
let mut builder = StringDictionaryBuilder::<Int32Type>::new();
builder.append_value("a");
builder.append_null();
builder.append_value("a");
builder.append_value("b");
let array = builder.finish();

let values: Vec<_> = array.downcast_dict::<StringArray>().unwrap().into_iter().collect();
assert_eq!(&values, &[Some("a"), None, Some("a"), Some("b")]);
```

<a id="op-f7351b53e6d222b1a12d4d5b"></a>
## as_any

`function` · `arrow_array::array::dictionary_array::DictionaryArray::as_any` · arrow-array 59.3.0

```rust
fn as_any(&self) -> &dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::DictionaryArray", "path": "DictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [699, 1], "end": [801, 2], "filename": "src/array/dictionary_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/dictionary_array.rs:700`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fcd627af411a796c3c7aacda"></a>
## claim

`function` · `arrow_array::array::dictionary_array::DictionaryArray::claim` · arrow-array 59.3.0

```rust
fn claim(&self, pool: &dyn arrow_buffer::MemoryPool)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::DictionaryArray", "path": "DictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [699, 1], "end": [801, 2], "filename": "src/array/dictionary_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/dictionary_array.rs:797`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d4d2290146e6ab97335afcb3"></a>
## clone

`function` · `arrow_array::array::dictionary_array::DictionaryArray::clone` · arrow-array 59.3.0

```rust
fn clone(&self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::DictionaryArray", "path": "DictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "K"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [259, 1], "end": [268, 2], "filename": "src/array/dictionary_array.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/array/dictionary_array.rs:260`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5492952e8fb779bf129e0385"></a>
## data_type

`function` · `arrow_array::array::dictionary_array::DictionaryArray::data_type` · arrow-array 59.3.0

```rust
fn data_type(&self) -> &DataType
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::DictionaryArray", "path": "DictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [699, 1], "end": [801, 2], "filename": "src/array/dictionary_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/dictionary_array.rs:712`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b866dbccb31de7d44f5aee4f"></a>
## downcast_dict

`function` · `arrow_array::array::dictionary_array::DictionaryArray::downcast_dict` · arrow-array 59.3.0

```rust
fn downcast_dict<V: 'static>(&self) -> Option<TypedDictionaryArray<'_, K, V>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::DictionaryArray", "path": "DictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "K"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [270, 1], "end": [584, 2], "filename": "src/array/dictionary_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/dictionary_array.rs:432`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Downcast this dictionary to a [`TypedDictionaryArray`](../operations/arrow_array.array.dictionary_array.TypedDictionaryArray.md#op-177ac4d9426c85e1ab27019d)

```
use arrow_array::{Array, ArrayAccessor, DictionaryArray, StringArray, types::Int32Type};

let orig = [Some("a"), Some("b"), None];
let dictionary = DictionaryArray::<Int32Type>::from_iter(orig);
let typed = dictionary.downcast_dict::<StringArray>().unwrap();
assert_eq!(typed.value(0), "a");
assert_eq!(typed.value(1), "b");
assert!(typed.is_null(2));
```


<a id="op-433170bd89cc37886f3d6bf9"></a>
## eq

`function` · `arrow_array::array::dictionary_array::DictionaryArray::eq` · arrow-array 59.3.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::DictionaryArray", "path": "DictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "K"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [788, 1], "end": [792, 2], "filename": "src/array/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/array/mod.rs:789`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4a6120cd3b3df1e642ec805a"></a>
## fmt

`function` · `arrow_array::array::dictionary_array::DictionaryArray::fmt` · arrow-array 59.3.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::DictionaryArray", "path": "DictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [803, 1], "end": [811, 2], "filename": "src/array/dictionary_array.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/array/dictionary_array.rs:804`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c197b823330bbd47ef29798b"></a>
## from

`function` · `arrow_array::array::dictionary_array::DictionaryArray::from` · arrow-array 59.3.0

```rust
fn from(data: ArrayData) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::DictionaryArray", "path": "DictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [587, 1], "end": [628, 2], "filename": "src/array/dictionary_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayData", "path": "ArrayData"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/dictionary_array.rs:588`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-35dc127484704e91bd222f18"></a>
## from_iter

`function` · `arrow_array::array::dictionary_array::DictionaryArray::from_iter` · arrow-array 59.3.0

```rust
fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::DictionaryArray", "path": "DictionaryArray"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [683, 1], "end": [696, 2], "filename": "src/array/dictionary_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"primitive": "str"}}}}], "constraints": []}}, "id": "core::iter::traits::collect::FromIterator", "path": "FromIterator"}, "trait_path": "core::iter::traits::collect::FromIterator"}`

Source: `src/array/dictionary_array.rs:684`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5f352aebc0cc26a2896e7c3d"></a>
## from_iter

`function` · `arrow_array::array::dictionary_array::DictionaryArray::from_iter` · arrow-array 59.3.0

```rust
fn from_iter<I: IntoIterator<Item = Option<&'a str>>>(iter: I) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::DictionaryArray", "path": "DictionaryArray"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [659, 1], "end": [667, 2], "filename": "src/array/dictionary_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"primitive": "str"}}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "core::iter::traits::collect::FromIterator", "path": "FromIterator"}, "trait_path": "core::iter::traits::collect::FromIterator"}`

Source: `src/array/dictionary_array.rs:660`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e937c497ba57d3337ac4952f"></a>
## get_array_memory_size

`function` · `arrow_array::array::dictionary_array::DictionaryArray::get_array_memory_size` · arrow-array 59.3.0

```rust
fn get_array_memory_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::DictionaryArray", "path": "DictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [699, 1], "end": [801, 2], "filename": "src/array/dictionary_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/dictionary_array.rs:790`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a001cd5db2539f164f66a0fa"></a>
## get_buffer_memory_size

`function` · `arrow_array::array::dictionary_array::DictionaryArray::get_buffer_memory_size` · arrow-array 59.3.0

```rust
fn get_buffer_memory_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::DictionaryArray", "path": "DictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [699, 1], "end": [801, 2], "filename": "src/array/dictionary_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/dictionary_array.rs:786`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-24f29e3c496053adf2a3c5d6"></a>
## into_data

`function` · `arrow_array::array::dictionary_array::DictionaryArray::into_data` · arrow-array 59.3.0

```rust
fn into_data(self) -> ArrayData
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::DictionaryArray", "path": "DictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [699, 1], "end": [801, 2], "filename": "src/array/dictionary_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/dictionary_array.rs:708`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-103d2be3b7e7429b85d599d6"></a>
## into_parts

`function` · `arrow_array::array::dictionary_array::DictionaryArray::into_parts` · arrow-array 59.3.0

```rust
fn into_parts(self) -> (PrimitiveArray<K>, ArrayRef)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::DictionaryArray", "path": "DictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "K"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [270, 1], "end": [584, 2], "filename": "src/array/dictionary_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/dictionary_array.rs:351`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Deconstruct this array into its constituent parts

<a id="op-b5833b95ee6bf2779f5eecd6"></a>
## into_primitive_dict_builder

`function` · `arrow_array::array::dictionary_array::DictionaryArray::into_primitive_dict_builder` · arrow-array 59.3.0

```rust
fn into_primitive_dict_builder<V>(self) -> Result<PrimitiveDictionaryBuilder<K, V>, Self> where V: ArrowPrimitiveType
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::DictionaryArray", "path": "DictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "K"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [270, 1], "end": [584, 2], "filename": "src/array/dictionary_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/dictionary_array.rs:492`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns `PrimitiveDictionaryBuilder` of this dictionary array for mutating
its keys and values if the underlying data buffer is not shared by others.

<a id="op-abda620ce84595038d6ebf90"></a>
## is_empty

`function` · `arrow_array::array::dictionary_array::DictionaryArray::is_empty` · arrow-array 59.3.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::DictionaryArray", "path": "DictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "K"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [270, 1], "end": [584, 2], "filename": "src/array/dictionary_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/dictionary_array.rs:389`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Whether this dictionary is empty

<a id="op-cf5653d1de7ae2a020027a52"></a>
## is_empty

`function` · `arrow_array::array::dictionary_array::DictionaryArray::is_empty` · arrow-array 59.3.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::DictionaryArray", "path": "DictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [699, 1], "end": [801, 2], "filename": "src/array/dictionary_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/dictionary_array.rs:724`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ffbcb1292491e074ba6ee6fe"></a>
## is_nullable

`function` · `arrow_array::array::dictionary_array::DictionaryArray::is_nullable` · arrow-array 59.3.0

```rust
fn is_nullable(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::DictionaryArray", "path": "DictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [699, 1], "end": [801, 2], "filename": "src/array/dictionary_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/dictionary_array.rs:782`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9226ac3b16a0ca795b9c6d43"></a>
## is_ordered

`function` · `arrow_array::array::dictionary_array::DictionaryArray::is_ordered` · arrow-array 59.3.0

```rust
fn is_ordered(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::DictionaryArray", "path": "DictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "K"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [270, 1], "end": [584, 2], "filename": "src/array/dictionary_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/dictionary_array.rs:394`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Currently exists for compatibility purposes with Arrow IPC.

<a id="op-16d9720c646f50e4a6e13503"></a>
## key

`function` · `arrow_array::array::dictionary_array::DictionaryArray::key` · arrow-array 59.3.0

```rust
fn key(&self, i: usize) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::DictionaryArray", "path": "DictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "K"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [270, 1], "end": [584, 2], "filename": "src/array/dictionary_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/dictionary_array.rs:405`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Return the value of `keys` (the dictionary key) at index `i`,
cast to `usize`, `None` if the value at `i` is `NULL`.

<a id="op-872c345ec5c68bc0d286e161"></a>
## keys

`function` · `arrow_array::array::dictionary_array::DictionaryArray::keys` · arrow-array 59.3.0

```rust
fn keys(&self) -> &PrimitiveArray<K>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::DictionaryArray", "path": "DictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "K"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [270, 1], "end": [584, 2], "filename": "src/array/dictionary_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/dictionary_array.rs:356`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Return an array view of the keys of this dictionary as a PrimitiveArray.

<a id="op-9387e7b217025ddee0d74fe0"></a>
## keys

`function` · `arrow_array::array::dictionary_array::DictionaryArray::keys` · arrow-array 59.3.0

```rust
fn keys(&self) -> &dyn Array
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::DictionaryArray", "path": "DictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "K"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1039, 1], "end": [1058, 2], "filename": "src/array/dictionary_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::dictionary_array::AnyDictionaryArray", "path": "AnyDictionaryArray"}, "trait_path": "arrow_array::array::dictionary_array::AnyDictionaryArray"}`

Source: `src/array/dictionary_array.rs:1040`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2fb10370615dc0b66690659f"></a>
## keys_iter

`function` · `arrow_array::array::dictionary_array::DictionaryArray::keys_iter` · arrow-array 59.3.0

```rust
fn keys_iter(&self) -> impl Iterator<Item = Option<usize>> + '_
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::DictionaryArray", "path": "DictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "K"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [270, 1], "end": [584, 2], "filename": "src/array/dictionary_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/dictionary_array.rs:399`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Return an iterator over the keys (indexes into the dictionary)

<a id="op-96ca19a8c2916fb253f0e183"></a>
## len

`function` · `arrow_array::array::dictionary_array::DictionaryArray::len` · arrow-array 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::DictionaryArray", "path": "DictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "K"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [270, 1], "end": [584, 2], "filename": "src/array/dictionary_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/dictionary_array.rs:384`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

The length of the dictionary is the length of the keys array.

<a id="op-bdd6887515367b4c2d42c6e7"></a>
## len

`function` · `arrow_array::array::dictionary_array::DictionaryArray::len` · arrow-array 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::DictionaryArray", "path": "DictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [699, 1], "end": [801, 2], "filename": "src/array/dictionary_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/dictionary_array.rs:720`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e2617b3e3932488c64381597"></a>
## logical_null_count

`function` · `arrow_array::array::dictionary_array::DictionaryArray::logical_null_count` · arrow-array 59.3.0

```rust
fn logical_null_count(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::DictionaryArray", "path": "DictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [699, 1], "end": [801, 2], "filename": "src/array/dictionary_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/dictionary_array.rs:762`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-543bb9886b5e3dadf2d3f73c"></a>
## logical_nulls

`function` · `arrow_array::array::dictionary_array::DictionaryArray::logical_nulls` · arrow-array 59.3.0

```rust
fn logical_nulls(&self) -> Option<NullBuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::DictionaryArray", "path": "DictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [699, 1], "end": [801, 2], "filename": "src/array/dictionary_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/dictionary_array.rs:741`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-195f2ede71dabb673dc6dad8"></a>
## lookup_key

`function` · `arrow_array::array::dictionary_array::DictionaryArray::lookup_key` · arrow-array 59.3.0

```rust
fn lookup_key(&self, value: &str) -> Option<K::Native>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::DictionaryArray", "path": "DictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "K"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [270, 1], "end": [584, 2], "filename": "src/array/dictionary_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/dictionary_array.rs:365`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

If `value` is present in `values` (aka the dictionary),
returns the corresponding key (index into the `values`
array). Otherwise returns `None`.

Panics if `values` is not a [`StringArray`](../operations/arrow_array.array.string_array.StringArray.md#op-5d32f770159d415652a55945).

<a id="op-20f77c559bd92d09a893cddd"></a>
## new

`function` · `arrow_array::array::dictionary_array::DictionaryArray::new` · arrow-array 59.3.0

```rust
fn new(keys: PrimitiveArray<K>, values: ArrayRef) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::DictionaryArray", "path": "DictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "K"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [270, 1], "end": [584, 2], "filename": "src/array/dictionary_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/dictionary_array.rs:278`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Attempt to create a new DictionaryArray with a specified keys
(indexes into the dictionary) and values (dictionary)
array.

# Panics

Panics if [`Self::try_new`](../operations/arrow_array.array.dictionary_array.DictionaryArray.md#op-adce3dab6a2865105cff013e) returns an error

<a id="op-b6871f91471ddad4f8e540b2"></a>
## new_scalar

`function` · `arrow_array::array::dictionary_array::DictionaryArray::new_scalar` · arrow-array 59.3.0

```rust
fn new_scalar<T: Array + 'static>(value: Scalar<T>) -> Scalar<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::DictionaryArray", "path": "DictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "K"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [270, 1], "end": [584, 2], "filename": "src/array/dictionary_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/dictionary_array.rs:320`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`Scalar`](../operations/arrow_array.scalar.Scalar.md#op-0ca10f05b262b3afc7077257) from `value`

<a id="op-582bf6be2500aa2dd0dd3cee"></a>
## new_unchecked

`function` · `arrow_array::array::dictionary_array::DictionaryArray::new_unchecked` · arrow-array 59.3.0

```rust
unsafe fn new_unchecked(keys: PrimitiveArray<K>, values: ArrayRef) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::DictionaryArray", "path": "DictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "K"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [270, 1], "end": [584, 2], "filename": "src/array/dictionary_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/dictionary_array.rs:332`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`DictionaryArray`](../operations/arrow_array.array.dictionary_array.DictionaryArray.md#op-761ab0c11d727f026bdc2e47) without performing validation

# Safety

Safe provided [`Self::try_new`](../operations/arrow_array.array.dictionary_array.DictionaryArray.md#op-adce3dab6a2865105cff013e) would not return an error

<a id="op-d3f0ab040cea62ca634b3f12"></a>
## normalized_keys

`function` · `arrow_array::array::dictionary_array::DictionaryArray::normalized_keys` · arrow-array 59.3.0

```rust
fn normalized_keys(&self) -> Vec<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::DictionaryArray", "path": "DictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "K"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1039, 1], "end": [1058, 2], "filename": "src/array/dictionary_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::dictionary_array::AnyDictionaryArray", "path": "AnyDictionaryArray"}, "trait_path": "arrow_array::array::dictionary_array::AnyDictionaryArray"}`

Source: `src/array/dictionary_array.rs:1048`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c559eee253a77e91afbf2b83"></a>
## nulls

`function` · `arrow_array::array::dictionary_array::DictionaryArray::nulls` · arrow-array 59.3.0

```rust
fn nulls(&self) -> Option<&NullBuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::DictionaryArray", "path": "DictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [699, 1], "end": [801, 2], "filename": "src/array/dictionary_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/dictionary_array.rs:737`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-357c724a031b57ddc75110b0"></a>
## occupancy

`function` · `arrow_array::array::dictionary_array::DictionaryArray::occupancy` · arrow-array 59.3.0

```rust
fn occupancy(&self) -> BooleanBuffer
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::DictionaryArray", "path": "DictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "K"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [270, 1], "end": [584, 2], "filename": "src/array/dictionary_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/dictionary_array.rs:566`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Computes an occupancy mask for this dictionary's values

For each value in [`Self::values`](../operations/arrow_array.array.dictionary_array.DictionaryArray.md#op-e04615315cfe741e7c23ed24) the corresponding bit will be set in the
returned mask if it is referenced by a key in this [`DictionaryArray`](../operations/arrow_array.array.dictionary_array.DictionaryArray.md#op-761ab0c11d727f026bdc2e47)

<a id="op-e4333024cb1b09af90586ae5"></a>
## offset

`function` · `arrow_array::array::dictionary_array::DictionaryArray::offset` · arrow-array 59.3.0

```rust
fn offset(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::DictionaryArray", "path": "DictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [699, 1], "end": [801, 2], "filename": "src/array/dictionary_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/dictionary_array.rs:733`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f5ca7fdb996531a5407f004e"></a>
## shrink_to_fit

`function` · `arrow_array::array::dictionary_array::DictionaryArray::shrink_to_fit` · arrow-array 59.3.0

```rust
fn shrink_to_fit(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::DictionaryArray", "path": "DictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [699, 1], "end": [801, 2], "filename": "src/array/dictionary_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/dictionary_array.rs:728`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7bff9616df78d0b4a0c82407"></a>
## slice

`function` · `arrow_array::array::dictionary_array::DictionaryArray::slice` · arrow-array 59.3.0

```rust
fn slice(&self, offset: usize, length: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::DictionaryArray", "path": "DictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "K"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [270, 1], "end": [584, 2], "filename": "src/array/dictionary_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/dictionary_array.rs:410`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a zero-copy slice of this array with the indicated offset and length.

<a id="op-eb7d2518dbb58ac7bbd22ba5"></a>
## slice

`function` · `arrow_array::array::dictionary_array::DictionaryArray::slice` · arrow-array 59.3.0

```rust
fn slice(&self, offset: usize, length: usize) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::DictionaryArray", "path": "DictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [699, 1], "end": [801, 2], "filename": "src/array/dictionary_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/dictionary_array.rs:716`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b40c80fb4f80c548158b412b"></a>
## to_data

`function` · `arrow_array::array::dictionary_array::DictionaryArray::to_data` · arrow-array 59.3.0

```rust
fn to_data(&self) -> ArrayData
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::DictionaryArray", "path": "DictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [699, 1], "end": [801, 2], "filename": "src/array/dictionary_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/dictionary_array.rs:704`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-adce3dab6a2865105cff013e"></a>
## try_new

`function` · `arrow_array::array::dictionary_array::DictionaryArray::try_new` · arrow-array 59.3.0

```rust
fn try_new(keys: PrimitiveArray<K>, values: ArrayRef) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::DictionaryArray", "path": "DictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "K"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [270, 1], "end": [584, 2], "filename": "src/array/dictionary_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/dictionary_array.rs:289`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Attempt to create a new DictionaryArray with a specified keys
(indexes into the dictionary) and values (dictionary)
array.

# Errors

Returns an error if any `keys[i] >= values.len() || keys[i] < 0`

<a id="op-d6e49e53881eb9b7bce5ebcd"></a>
## unary_mut

`function` · `arrow_array::array::dictionary_array::DictionaryArray::unary_mut` · arrow-array 59.3.0

```rust
fn unary_mut<F, V>(self, op: F) -> Result<DictionaryArray<K>, DictionaryArray<K>> where V: ArrowPrimitiveType, F: Fn(V::Native) -> V::Native
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::DictionaryArray", "path": "DictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "K"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [270, 1], "end": [584, 2], "filename": "src/array/dictionary_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/dictionary_array.rs:549`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Applies an unary and infallible function to a mutable dictionary array.
Mutable dictionary array means that the buffers are not shared with other arrays.
As a result, this mutates the buffers directly without allocating new buffers.

# Implementation

This will apply the function for all dictionary values, including those on null slots.
This implies that the operation must be infallible for any value of the corresponding type
or this function may panic.
# Example
```
# use std::sync::Arc;
# use arrow_array::{Array, ArrayAccessor, DictionaryArray, StringArray, types::{Int8Type, Int32Type}};
# use arrow_array::{Int8Array, Int32Array};
let values = Int32Array::from(vec![Some(10), Some(20), None]);
let keys = Int8Array::from_iter_values([0, 0, 1, 2]);
let dictionary = DictionaryArray::<Int8Type>::try_new(keys, Arc::new(values)).unwrap();
let c = dictionary.unary_mut::<_, Int32Type>(|x| x + 1).unwrap();
let typed = c.downcast_dict::<Int32Array>().unwrap();
assert_eq!(typed.value(0), 11);
assert_eq!(typed.value(1), 11);
assert_eq!(typed.value(2), 21);
```

<a id="op-45886009522894e333fcf62c"></a>
## value_type

`function` · `arrow_array::array::dictionary_array::DictionaryArray::value_type` · arrow-array 59.3.0

```rust
fn value_type(&self) -> DataType
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::DictionaryArray", "path": "DictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "K"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [270, 1], "end": [584, 2], "filename": "src/array/dictionary_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/dictionary_array.rs:379`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a clone of the value type of this list.

<a id="op-8b4cd37d526b3fd51339e328"></a>
## values

`function` · `arrow_array::array::dictionary_array::DictionaryArray::values` · arrow-array 59.3.0

```rust
fn values(&self) -> &ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::DictionaryArray", "path": "DictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "K"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1039, 1], "end": [1058, 2], "filename": "src/array/dictionary_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::dictionary_array::AnyDictionaryArray", "path": "AnyDictionaryArray"}, "trait_path": "arrow_array::array::dictionary_array::AnyDictionaryArray"}`

Source: `src/array/dictionary_array.rs:1044`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e04615315cfe741e7c23ed24"></a>
## values

`function` · `arrow_array::array::dictionary_array::DictionaryArray::values` · arrow-array 59.3.0

```rust
fn values(&self) -> &ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::DictionaryArray", "path": "DictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "K"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [270, 1], "end": [584, 2], "filename": "src/array/dictionary_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/dictionary_array.rs:374`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a reference to the dictionary values array

<a id="op-0282e1ff97cb915672985c5e"></a>
## with_values

`function` · `arrow_array::array::dictionary_array::DictionaryArray::with_values` · arrow-array 59.3.0

```rust
fn with_values(&self, values: ArrayRef) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::DictionaryArray", "path": "DictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "K"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [270, 1], "end": [584, 2], "filename": "src/array/dictionary_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/dictionary_array.rs:477`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a new dictionary with the same keys as the current instance
but with a different set of dictionary values

This can be used to perform an operation on the values of a dictionary

# Panics

Panics if `values` has a length less than the current values

```
# use std::sync::Arc;
# use arrow_array::builder::PrimitiveDictionaryBuilder;
# use arrow_array::{Int8Array, Int64Array, ArrayAccessor};
# use arrow_array::types::{Int32Type, Int8Type};

// Construct a Dict(Int32, Int8)
let mut builder = PrimitiveDictionaryBuilder::<Int32Type, Int8Type>::with_capacity(2, 200);
for i in 0..100 {
    builder.append(i % 2).unwrap();
}

let dictionary = builder.finish();

// Perform a widening cast of dictionary values
let typed_dictionary = dictionary.downcast_dict::<Int8Array>().unwrap();
let values: Int64Array = typed_dictionary.values().unary(|x| x as i64);

// Create a Dict(Int32,
let new = dictionary.with_values(Arc::new(values));

// Verify values are as expected
let new_typed = new.downcast_dict::<Int64Array>().unwrap();
for i in 0..100 {
    assert_eq!(new_typed.value(i), (i % 2) as i64)
}
```


<a id="op-9c7ef4c91a7f8487abe0812e"></a>
## with_values

`function` · `arrow_array::array::dictionary_array::DictionaryArray::with_values` · arrow-array 59.3.0

```rust
fn with_values(&self, values: ArrayRef) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}], "constraints": []}}, "id": "arrow_array::array::dictionary_array::DictionaryArray", "path": "DictionaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "K"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1039, 1], "end": [1058, 2], "filename": "src/array/dictionary_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::dictionary_array::AnyDictionaryArray", "path": "AnyDictionaryArray"}, "trait_path": "arrow_array::array::dictionary_array::AnyDictionaryArray"}`

Source: `src/array/dictionary_array.rs:1055`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
