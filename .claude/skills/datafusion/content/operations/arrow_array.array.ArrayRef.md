# `arrow_array::array::ArrayRef`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.ArrayRef.json).

<a id="op-657cc3ff3d24afcacda590e1"></a>
## ArrayRef

`type_alias` · `arrow_array::array::ArrayRef` · arrow-array 59.3.0

```rust
type ArrayRef = std::sync::Arc<dyn Array>
```

Source: `src/array/mod.rs:429`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A reference-counted reference to a generic `Array`

<a id="op-e00eb37ba8df960ad39765c2"></a>
## as_any

`function` · `arrow_array::array::ArrayRef::as_any` · arrow-array 59.3.0

```rust
fn as_any(&self) -> &dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::ArrayRef", "path": "ArrayRef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [432, 1], "end": [514, 2], "filename": "src/array/mod.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/mod.rs:433`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7e38d55232bbb0d8f2c29c0f"></a>
## as_any_dictionary_opt

`function` · `arrow_array::array::ArrayRef::as_any_dictionary_opt` · arrow-array 59.3.0

```rust
fn as_any_dictionary_opt(&self) -> Option<&dyn AnyDictionaryArray>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::ArrayRef", "path": "ArrayRef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1071, 1], "end": [1135, 2], "filename": "src/cast.rs"}, "trait": {"args": null, "id": "arrow_array::cast::AsArray", "path": "AsArray"}, "trait_path": "arrow_array::cast::AsArray"}`

Source: `src/cast.rs:1120`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-789f0fca6139afb62167cc45"></a>
## as_any_ree_opt

`function` · `arrow_array::array::ArrayRef::as_any_ree_opt` · arrow-array 59.3.0

```rust
fn as_any_ree_opt(&self) -> Option<&dyn AnyRunEndArray>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::ArrayRef", "path": "ArrayRef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1071, 1], "end": [1135, 2], "filename": "src/cast.rs"}, "trait": {"args": null, "id": "arrow_array::cast::AsArray", "path": "AsArray"}, "trait_path": "arrow_array::cast::AsArray"}`

Source: `src/cast.rs:1124`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-53b90d123d2bf6b653e2a10d"></a>
## as_boolean_opt

`function` · `arrow_array::array::ArrayRef::as_boolean_opt` · arrow-array 59.3.0

```rust
fn as_boolean_opt(&self) -> Option<&BooleanArray>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::ArrayRef", "path": "ArrayRef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1071, 1], "end": [1135, 2], "filename": "src/cast.rs"}, "trait": {"args": null, "id": "arrow_array::cast::AsArray", "path": "AsArray"}, "trait_path": "arrow_array::cast::AsArray"}`

Source: `src/cast.rs:1072`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a006e40aa950629f1d50bc19"></a>
## as_byte_view_opt

`function` · `arrow_array::array::ArrayRef::as_byte_view_opt` · arrow-array 59.3.0

```rust
fn as_byte_view_opt<T: ByteViewType>(&self) -> Option<&GenericByteViewArray<T>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::ArrayRef", "path": "ArrayRef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1071, 1], "end": [1135, 2], "filename": "src/cast.rs"}, "trait": {"args": null, "id": "arrow_array::cast::AsArray", "path": "AsArray"}, "trait_path": "arrow_array::cast::AsArray"}`

Source: `src/cast.rs:1084`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fcb20e319642d4e9ce2f438f"></a>
## as_bytes_opt

`function` · `arrow_array::array::ArrayRef::as_bytes_opt` · arrow-array 59.3.0

```rust
fn as_bytes_opt<T: ByteArrayType>(&self) -> Option<&GenericByteArray<T>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::ArrayRef", "path": "ArrayRef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1071, 1], "end": [1135, 2], "filename": "src/cast.rs"}, "trait": {"args": null, "id": "arrow_array::cast::AsArray", "path": "AsArray"}, "trait_path": "arrow_array::cast::AsArray"}`

Source: `src/cast.rs:1080`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-773878f59f0e7cc1416ac4cf"></a>
## as_dictionary_opt

`function` · `arrow_array::array::ArrayRef::as_dictionary_opt` · arrow-array 59.3.0

```rust
fn as_dictionary_opt<K: ArrowDictionaryKeyType>(&self) -> Option<&DictionaryArray<K>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::ArrayRef", "path": "ArrayRef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1071, 1], "end": [1135, 2], "filename": "src/cast.rs"}, "trait": {"args": null, "id": "arrow_array::cast::AsArray", "path": "AsArray"}, "trait_path": "arrow_array::cast::AsArray"}`

Source: `src/cast.rs:1116`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f0198c1c7bac3b32bebec4bc"></a>
## as_fixed_size_binary_opt

`function` · `arrow_array::array::ArrayRef::as_fixed_size_binary_opt` · arrow-array 59.3.0

```rust
fn as_fixed_size_binary_opt(&self) -> Option<&FixedSizeBinaryArray>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::ArrayRef", "path": "ArrayRef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1071, 1], "end": [1135, 2], "filename": "src/cast.rs"}, "trait": {"args": null, "id": "arrow_array::cast::AsArray", "path": "AsArray"}, "trait_path": "arrow_array::cast::AsArray"}`

Source: `src/cast.rs:1104`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b1093a20ad4ce1fe1fa03b50"></a>
## as_fixed_size_list_opt

`function` · `arrow_array::array::ArrayRef::as_fixed_size_list_opt` · arrow-array 59.3.0

```rust
fn as_fixed_size_list_opt(&self) -> Option<&FixedSizeListArray>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::ArrayRef", "path": "ArrayRef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1071, 1], "end": [1135, 2], "filename": "src/cast.rs"}, "trait": {"args": null, "id": "arrow_array::cast::AsArray", "path": "AsArray"}, "trait_path": "arrow_array::cast::AsArray"}`

Source: `src/cast.rs:1108`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d36337d2c3693189060897f3"></a>
## as_list_opt

`function` · `arrow_array::array::ArrayRef::as_list_opt` · arrow-array 59.3.0

```rust
fn as_list_opt<O: OffsetSizeTrait>(&self) -> Option<&GenericListArray<O>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::ArrayRef", "path": "ArrayRef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1071, 1], "end": [1135, 2], "filename": "src/cast.rs"}, "trait": {"args": null, "id": "arrow_array::cast::AsArray", "path": "AsArray"}, "trait_path": "arrow_array::cast::AsArray"}`

Source: `src/cast.rs:1096`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-69764c55e937f7536cc53305"></a>
## as_list_view_opt

`function` · `arrow_array::array::ArrayRef::as_list_view_opt` · arrow-array 59.3.0

```rust
fn as_list_view_opt<O: OffsetSizeTrait>(&self) -> Option<&GenericListViewArray<O>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::ArrayRef", "path": "ArrayRef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1071, 1], "end": [1135, 2], "filename": "src/cast.rs"}, "trait": {"args": null, "id": "arrow_array::cast::AsArray", "path": "AsArray"}, "trait_path": "arrow_array::cast::AsArray"}`

Source: `src/cast.rs:1100`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-15459097682be45475abc844"></a>
## as_map_opt

`function` · `arrow_array::array::ArrayRef::as_map_opt` · arrow-array 59.3.0

```rust
fn as_map_opt(&self) -> Option<&MapArray>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::ArrayRef", "path": "ArrayRef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1071, 1], "end": [1135, 2], "filename": "src/cast.rs"}, "trait": {"args": null, "id": "arrow_array::cast::AsArray", "path": "AsArray"}, "trait_path": "arrow_array::cast::AsArray"}`

Source: `src/cast.rs:1112`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5f8aa8e0c94acb15956bb1ae"></a>
## as_primitive_opt

`function` · `arrow_array::array::ArrayRef::as_primitive_opt` · arrow-array 59.3.0

```rust
fn as_primitive_opt<T: ArrowPrimitiveType>(&self) -> Option<&PrimitiveArray<T>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::ArrayRef", "path": "ArrayRef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1071, 1], "end": [1135, 2], "filename": "src/cast.rs"}, "trait": {"args": null, "id": "arrow_array::cast::AsArray", "path": "AsArray"}, "trait_path": "arrow_array::cast::AsArray"}`

Source: `src/cast.rs:1076`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bc44232c0baa4ce8064a5172"></a>
## as_run_opt

`function` · `arrow_array::array::ArrayRef::as_run_opt` · arrow-array 59.3.0

```rust
fn as_run_opt<K: RunEndIndexType>(&self) -> Option<&RunArray<K>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::ArrayRef", "path": "ArrayRef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1071, 1], "end": [1135, 2], "filename": "src/cast.rs"}, "trait": {"args": null, "id": "arrow_array::cast::AsArray", "path": "AsArray"}, "trait_path": "arrow_array::cast::AsArray"}`

Source: `src/cast.rs:1128`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8fa2f5e0b34099878765ea67"></a>
## as_string_opt

`function` · `arrow_array::array::ArrayRef::as_string_opt` · arrow-array 59.3.0

```rust
fn as_string_opt<O: OffsetSizeTrait>(&self) -> Option<&GenericStringArray<O>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::ArrayRef", "path": "ArrayRef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1071, 1], "end": [1135, 2], "filename": "src/cast.rs"}, "trait": {"args": null, "id": "arrow_array::cast::AsArray", "path": "AsArray"}, "trait_path": "arrow_array::cast::AsArray"}`

Source: `src/cast.rs:1132`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0a080d7d6fef6127cd72b251"></a>
## as_struct_opt

`function` · `arrow_array::array::ArrayRef::as_struct_opt` · arrow-array 59.3.0

```rust
fn as_struct_opt(&self) -> Option<&StructArray>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::ArrayRef", "path": "ArrayRef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1071, 1], "end": [1135, 2], "filename": "src/cast.rs"}, "trait": {"args": null, "id": "arrow_array::cast::AsArray", "path": "AsArray"}, "trait_path": "arrow_array::cast::AsArray"}`

Source: `src/cast.rs:1088`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3459a598ecc9edcce5119d44"></a>
## as_union_opt

`function` · `arrow_array::array::ArrayRef::as_union_opt` · arrow-array 59.3.0

```rust
fn as_union_opt(&self) -> Option<&UnionArray>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::ArrayRef", "path": "ArrayRef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1071, 1], "end": [1135, 2], "filename": "src/cast.rs"}, "trait": {"args": null, "id": "arrow_array::cast::AsArray", "path": "AsArray"}, "trait_path": "arrow_array::cast::AsArray"}`

Source: `src/cast.rs:1092`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-21b27bc3a0e7b7a869d78138"></a>
## claim

`function` · `arrow_array::array::ArrayRef::claim` · arrow-array 59.3.0

```rust
fn claim(&self, pool: &dyn arrow_buffer::MemoryPool)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::ArrayRef", "path": "ArrayRef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [432, 1], "end": [514, 2], "filename": "src/array/mod.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/mod.rs:511`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-603a7b05505c26935592adb0"></a>
## data_type

`function` · `arrow_array::array::ArrayRef::data_type` · arrow-array 59.3.0

```rust
fn data_type(&self) -> &DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::ArrayRef", "path": "ArrayRef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [432, 1], "end": [514, 2], "filename": "src/array/mod.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/mod.rs:445`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea3588efb78a906db5256b1c"></a>
## get_array_memory_size

`function` · `arrow_array::array::ArrayRef::get_array_memory_size` · arrow-array 59.3.0

```rust
fn get_array_memory_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::ArrayRef", "path": "ArrayRef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [432, 1], "end": [514, 2], "filename": "src/array/mod.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/mod.rs:506`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-17bd074f13074414da9d9e88"></a>
## get_buffer_memory_size

`function` · `arrow_array::array::ArrayRef::get_buffer_memory_size` · arrow-array 59.3.0

```rust
fn get_buffer_memory_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::ArrayRef", "path": "ArrayRef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [432, 1], "end": [514, 2], "filename": "src/array/mod.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/mod.rs:502`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-51bfc5b812fe2eabc1e39282"></a>
## into_data

`function` · `arrow_array::array::ArrayRef::into_data` · arrow-array 59.3.0

```rust
fn into_data(self) -> ArrayData
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::ArrayRef", "path": "ArrayRef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [432, 1], "end": [514, 2], "filename": "src/array/mod.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/mod.rs:441`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-646278e29a8325120d0cb3d3"></a>
## is_empty

`function` · `arrow_array::array::ArrayRef::is_empty` · arrow-array 59.3.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::ArrayRef", "path": "ArrayRef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [432, 1], "end": [514, 2], "filename": "src/array/mod.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/mod.rs:457`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b12c3afcc57601f5d5a431c5"></a>
## is_null

`function` · `arrow_array::array::ArrayRef::is_null` · arrow-array 59.3.0

```rust
fn is_null(&self, index: usize) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::ArrayRef", "path": "ArrayRef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [432, 1], "end": [514, 2], "filename": "src/array/mod.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/mod.rs:482`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-45ff2dc8c41cc23b8231a84b"></a>
## is_nullable

`function` · `arrow_array::array::ArrayRef::is_nullable` · arrow-array 59.3.0

```rust
fn is_nullable(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::ArrayRef", "path": "ArrayRef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [432, 1], "end": [514, 2], "filename": "src/array/mod.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/mod.rs:498`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-489623ac57d6a8720837aee5"></a>
## is_valid

`function` · `arrow_array::array::ArrayRef::is_valid` · arrow-array 59.3.0

```rust
fn is_valid(&self, index: usize) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::ArrayRef", "path": "ArrayRef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [432, 1], "end": [514, 2], "filename": "src/array/mod.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/mod.rs:486`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a774f4c2a1f1904978ab5556"></a>
## len

`function` · `arrow_array::array::ArrayRef::len` · arrow-array 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::ArrayRef", "path": "ArrayRef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [432, 1], "end": [514, 2], "filename": "src/array/mod.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/mod.rs:453`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f731758b15ff551ebbf71f50"></a>
## logical_null_count

`function` · `arrow_array::array::ArrayRef::logical_null_count` · arrow-array 59.3.0

```rust
fn logical_null_count(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::ArrayRef", "path": "ArrayRef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [432, 1], "end": [514, 2], "filename": "src/array/mod.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/mod.rs:494`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-58323e66d0e74575fc2f9faf"></a>
## logical_nulls

`function` · `arrow_array::array::ArrayRef::logical_nulls` · arrow-array 59.3.0

```rust
fn logical_nulls(&self) -> Option<NullBuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::ArrayRef", "path": "ArrayRef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [432, 1], "end": [514, 2], "filename": "src/array/mod.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/mod.rs:478`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-60f82ba7d2649c59a89948f3"></a>
## null_count

`function` · `arrow_array::array::ArrayRef::null_count` · arrow-array 59.3.0

```rust
fn null_count(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::ArrayRef", "path": "ArrayRef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [432, 1], "end": [514, 2], "filename": "src/array/mod.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/mod.rs:490`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-27cc5a3049f7f549d91b3327"></a>
## nulls

`function` · `arrow_array::array::ArrayRef::nulls` · arrow-array 59.3.0

```rust
fn nulls(&self) -> Option<&NullBuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::ArrayRef", "path": "ArrayRef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [432, 1], "end": [514, 2], "filename": "src/array/mod.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/mod.rs:474`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dd3de6562eb6e60e610866af"></a>
## offset

`function` · `arrow_array::array::ArrayRef::offset` · arrow-array 59.3.0

```rust
fn offset(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::ArrayRef", "path": "ArrayRef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [432, 1], "end": [514, 2], "filename": "src/array/mod.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/mod.rs:470`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb37adb4cff36bccfdd10418"></a>
## shrink_to_fit

`function` · `arrow_array::array::ArrayRef::shrink_to_fit` · arrow-array 59.3.0

```rust
fn shrink_to_fit(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::ArrayRef", "path": "ArrayRef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [432, 1], "end": [514, 2], "filename": "src/array/mod.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/mod.rs:462`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

For shared buffers, this is a no-op.

<a id="op-9a7bc5fd649d41d03e9feb9b"></a>
## slice

`function` · `arrow_array::array::ArrayRef::slice` · arrow-array 59.3.0

```rust
fn slice(&self, offset: usize, length: usize) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::ArrayRef", "path": "ArrayRef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [432, 1], "end": [514, 2], "filename": "src/array/mod.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/mod.rs:449`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ab097bb532a3bfce9fd2984f"></a>
## to_data

`function` · `arrow_array::array::ArrayRef::to_data` · arrow-array 59.3.0

```rust
fn to_data(&self) -> ArrayData
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::ArrayRef", "path": "ArrayRef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [432, 1], "end": [514, 2], "filename": "src/array/mod.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/mod.rs:437`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
