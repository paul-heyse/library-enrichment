# `arrow_array::array::byte_array::GenericByteArray`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.byte_array.GenericByteArray.json).

<a id="op-e39e3ecbe5a4126397f47443"></a>
## GenericByteArray

`struct` · `arrow_array::array::byte_array::GenericByteArray` · arrow-array 59.3.0

```rust
struct GenericByteArray<T: ByteArrayType>
```

Source: `src/array/byte_array.rs:87`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

An array of [variable length byte arrays](https://arrow.apache.org/docs/format/Columnar.html#variable-size-binary-layout)

See [`StringArray`] and [`LargeStringArray`] for storing utf8 encoded string data

See [`BinaryArray`] and [`LargeBinaryArray`] for storing arbitrary bytes

# Example: From a Vec

```
# use arrow_array::{Array, GenericByteArray, types::Utf8Type};
let arr: GenericByteArray<Utf8Type> = vec!["hello", "world", ""].into();
assert_eq!(arr.value_data(), b"helloworld");
assert_eq!(arr.value_offsets(), &[0, 5, 10, 10]);
let values: Vec<_> = arr.iter().collect();
assert_eq!(values, &[Some("hello"), Some("world"), Some("")]);
```

# Example: From an optional Vec

```
# use arrow_array::{Array, GenericByteArray, types::Utf8Type};
let arr: GenericByteArray<Utf8Type> = vec![Some("hello"), Some("world"), Some(""), None].into();
assert_eq!(arr.value_data(), b"helloworld");
assert_eq!(arr.value_offsets(), &[0, 5, 10, 10, 10]);
let values: Vec<_> = arr.iter().collect();
assert_eq!(values, &[Some("hello"), Some("world"), Some(""), None]);
```

# Example: From an iterator of option

```
# use arrow_array::{Array, GenericByteArray, types::Utf8Type};
let arr: GenericByteArray<Utf8Type> = (0..5).map(|x| (x % 2 == 0).then(|| x.to_string())).collect();
let values: Vec<_> = arr.iter().collect();
assert_eq!(values, &[Some("0"), None, Some("2"), None, Some("4")]);
```

# Example: Using Builder

```
# use arrow_array::Array;
# use arrow_array::builder::GenericByteBuilder;
# use arrow_array::types::Utf8Type;
let mut builder = GenericByteBuilder::<Utf8Type>::new();
builder.append_value("hello");
builder.append_null();
builder.append_value("world");
let array = builder.finish();
let values: Vec<_> = array.iter().collect();
assert_eq!(values, &[Some("hello"), None, Some("world")]);
```

[`StringArray`]: crate::StringArray
[`LargeStringArray`]: crate::LargeStringArray
[`BinaryArray`]: crate::BinaryArray
[`LargeBinaryArray`]: crate::LargeBinaryArray

<a id="op-13f89f62bf779207f7e8e76d"></a>
## DATA_TYPE

`assoc_const` · `arrow_array::array::byte_array::GenericByteArray::DATA_TYPE` · arrow-array 59.3.0

```rust
DATA_TYPE
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_array::GenericByteArray", "path": "GenericByteArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [453, 2], "filename": "src/array/byte_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/byte_array.rs:107`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Data type of the array.

<a id="op-03adad1c8d553dc2880082d0"></a>
## as_any

`function` · `arrow_array::array::byte_array::GenericByteArray::as_any` · arrow-array 59.3.0

```rust
fn as_any(&self) -> &dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_array::GenericByteArray", "path": "GenericByteArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [466, 1], "end": [537, 2], "filename": "src/array/byte_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/byte_array.rs:467`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-57cc2285852ca7ce84470dde"></a>
## claim

`function` · `arrow_array::array::byte_array::GenericByteArray::claim` · arrow-array 59.3.0

```rust
fn claim(&self, pool: &dyn arrow_buffer::MemoryPool)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_array::GenericByteArray", "path": "GenericByteArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [466, 1], "end": [537, 2], "filename": "src/array/byte_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/byte_array.rs:530`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-900534f8087a93d770c64411"></a>
## clone

`function` · `arrow_array::array::byte_array::GenericByteArray::clone` · arrow-array 59.3.0

```rust
fn clone(&self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_array::GenericByteArray", "path": "GenericByteArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 1], "end": [103, 2], "filename": "src/array/byte_array.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/array/byte_array.rs:95`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2e972a2e5e174f617d28155f"></a>
## data_type

`function` · `arrow_array::array::byte_array::GenericByteArray::data_type` · arrow-array 59.3.0

```rust
fn data_type(&self) -> &DataType
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_array::GenericByteArray", "path": "GenericByteArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [466, 1], "end": [537, 2], "filename": "src/array/byte_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/byte_array.rs:479`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f9d67ff06848f6a639b3c81b"></a>
## fmt

`function` · `arrow_array::array::byte_array::GenericByteArray::fmt` · arrow-array 59.3.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_array::GenericByteArray", "path": "GenericByteArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [455, 1], "end": [463, 2], "filename": "src/array/byte_array.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/array/byte_array.rs:456`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4c6a6da2c6820993b498b9bd"></a>
## from

`function` · `arrow_array::array::byte_array::GenericByteArray::from` · arrow-array 59.3.0

```rust
fn from(data: ArrayData) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_array::GenericByteArray", "path": "GenericByteArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 1], "end": [583, 2], "filename": "src/array/byte_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayData", "path": "ArrayData"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/byte_array.rs:552`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0d65dd623600d080b0ea73b0"></a>
## from_iter

`function` · `arrow_array::array::byte_array::GenericByteArray::from_iter` · arrow-array 59.3.0

```rust
fn from_iter<I: IntoIterator<Item = Option<Ptr>>>(iter: I) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_array::GenericByteArray", "path": "GenericByteArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "Ptr"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": ""}}}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}}}], "generic_params": [], "type": {"generic": "Ptr"}}}]}, "is_negative": false, "span": {"begin": [619, 1], "end": [629, 2], "filename": "src/array/byte_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "Ptr"}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "core::iter::traits::collect::FromIterator", "path": "FromIterator"}, "trait_path": "core::iter::traits::collect::FromIterator"}`

Source: `src/array/byte_array.rs:623`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-75053d7434755fdb9d296015"></a>
## from_iter

`function` · `arrow_array::array::byte_array::GenericByteArray::from_iter` · arrow-array 59.3.0

```rust
fn from_iter<I: IntoIterator<Item = &'a Option<Ptr>>>(iter: I) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_array::GenericByteArray", "path": "GenericByteArray"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "Ptr"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": ""}}}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}}}, {"outlives": "'a"}], "generic_params": [], "type": {"generic": "Ptr"}}}]}, "is_negative": false, "span": {"begin": [608, 1], "end": [617, 2], "filename": "src/array/byte_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "Ptr"}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}}}], "constraints": []}}, "id": "core::iter::traits::collect::FromIterator", "path": "FromIterator"}, "trait_path": "core::iter::traits::collect::FromIterator"}`

Source: `src/array/byte_array.rs:612`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0eaf58973642664a09d52149"></a>
## from_iter_values

`function` · `arrow_array::array::byte_array::GenericByteArray::from_iter_values` · arrow-array 59.3.0

```rust
fn from_iter_values<Ptr, I>(iter: I) -> Self where Ptr: AsRef<T::Native>, I: IntoIterator<Item = Ptr>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_array::GenericByteArray", "path": "GenericByteArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [453, 2], "filename": "src/array/byte_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/byte_array.rs:217`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a [`GenericByteArray`](../operations/arrow_array.array.byte_array.GenericByteArray.md#op-e39e3ecbe5a4126397f47443) based on an iterator of values without nulls

<a id="op-dffe3a1b9f64211f991de566"></a>
## from_opt_vec

`function` · `arrow_array::array::byte_array::GenericByteArray::from_opt_vec` · arrow-array 59.3.0

```rust
fn from_opt_vec(v: Vec<Option<&[u8]>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::types::GenericBinaryType", "path": "GenericBinaryType"}}}], "constraints": []}}, "id": "arrow_array::array::byte_array::GenericByteArray", "path": "GenericByteArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [95, 2], "filename": "src/array/binary_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/binary_array.rs:35`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a [GenericBinaryArray](../operations/arrow_array.array.binary_array.GenericBinaryArray.md#op-faf606314f632d89aa2152d6) from a vector of Optional (null) byte slices

<a id="op-284346c5e948a7287c4b42b9"></a>
## from_vec

`function` · `arrow_array::array::byte_array::GenericByteArray::from_vec` · arrow-array 59.3.0

```rust
fn from_vec(v: Vec<&[u8]>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::types::GenericBinaryType", "path": "GenericBinaryType"}}}], "constraints": []}}, "id": "arrow_array::array::byte_array::GenericByteArray", "path": "GenericByteArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [95, 2], "filename": "src/array/binary_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/binary_array.rs:30`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a [GenericBinaryArray](../operations/arrow_array.array.binary_array.GenericBinaryArray.md#op-faf606314f632d89aa2152d6) from a vector of byte slices

See also [`Self::from_iter_values`](../operations/arrow_array.array.byte_array.GenericByteArray.md#op-0eaf58973642664a09d52149)

<a id="op-5aa79c5ca824bf3cff688e11"></a>
## get_array_memory_size

`function` · `arrow_array::array::byte_array::GenericByteArray::get_array_memory_size` · arrow-array 59.3.0

```rust
fn get_array_memory_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_array::GenericByteArray", "path": "GenericByteArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [466, 1], "end": [537, 2], "filename": "src/array/byte_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/byte_array.rs:525`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9e0f3b38a09ef5c1ed8f0a86"></a>
## get_buffer_memory_size

`function` · `arrow_array::array::byte_array::GenericByteArray::get_buffer_memory_size` · arrow-array 59.3.0

```rust
fn get_buffer_memory_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_array::GenericByteArray", "path": "GenericByteArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [466, 1], "end": [537, 2], "filename": "src/array/byte_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/byte_array.rs:516`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2b64313e4eaf7a2db35810ac"></a>
## into_builder

`function` · `arrow_array::array::byte_array::GenericByteArray::into_builder` · arrow-array 59.3.0

```rust
fn into_builder(self) -> Result<GenericByteBuilder<T>, Self>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_array::GenericByteArray", "path": "GenericByteArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [453, 2], "filename": "src/array/byte_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/byte_array.rs:373`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns `GenericByteBuilder` of this byte array for mutating its values if the underlying
offset and data buffers are not shared by others.

<a id="op-20f3c41ec4d7f2ff61514ef0"></a>
## into_data

`function` · `arrow_array::array::byte_array::GenericByteArray::into_data` · arrow-array 59.3.0

```rust
fn into_data(self) -> ArrayData
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_array::GenericByteArray", "path": "GenericByteArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [466, 1], "end": [537, 2], "filename": "src/array/byte_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/byte_array.rs:475`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-99099731e31b22e06710c878"></a>
## into_parts

`function` · `arrow_array::array::byte_array::GenericByteArray::into_parts` · arrow-array 59.3.0

```rust
fn into_parts(self) -> (OffsetBuffer<T::Offset>, Buffer, Option<NullBuffer>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_array::GenericByteArray", "path": "GenericByteArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [453, 2], "filename": "src/array/byte_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/byte_array.rs:251`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Deconstruct this array into its constituent parts

<a id="op-7cf5052d23bdd6e2e2154277"></a>
## is_ascii

`function` · `arrow_array::array::byte_array::GenericByteArray::is_ascii` · arrow-array 59.3.0

```rust
fn is_ascii(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_array::GenericByteArray", "path": "GenericByteArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [453, 2], "filename": "src/array/byte_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/byte_array.rs:288`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns true if all data within this array is ASCII

<a id="op-3fb17ff5e69cf01e93287d02"></a>
## is_empty

`function` · `arrow_array::array::byte_array::GenericByteArray::is_empty` · arrow-array 59.3.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_array::GenericByteArray", "path": "GenericByteArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [466, 1], "end": [537, 2], "filename": "src/array/byte_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/byte_array.rs:491`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b1262fada81cb4dbd3d9f405"></a>
## iter

`function` · `arrow_array::array::byte_array::GenericByteArray::iter` · arrow-array 59.3.0

```rust
fn iter(&self) -> ArrayIter<&Self>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_array::GenericByteArray", "path": "GenericByteArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [453, 2], "filename": "src/array/byte_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/byte_array.rs:357`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

constructs a new iterator

<a id="op-06ec2df565175c7580dce290"></a>
## len

`function` · `arrow_array::array::byte_array::GenericByteArray::len` · arrow-array 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_array::GenericByteArray", "path": "GenericByteArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [466, 1], "end": [537, 2], "filename": "src/array/byte_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/byte_array.rs:487`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ba38050845776ba29e8e8f90"></a>
## logical_null_count

`function` · `arrow_array::array::byte_array::GenericByteArray::logical_null_count` · arrow-array 59.3.0

```rust
fn logical_null_count(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_array::GenericByteArray", "path": "GenericByteArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [466, 1], "end": [537, 2], "filename": "src/array/byte_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/byte_array.rs:511`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e2b8a00411379785049e0a1b"></a>
## new

`function` · `arrow_array::array::byte_array::GenericByteArray::new` · arrow-array 59.3.0

```rust
fn new(offsets: OffsetBuffer<T::Offset>, values: Buffer, nulls: Option<NullBuffer>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_array::GenericByteArray", "path": "GenericByteArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [453, 2], "filename": "src/array/byte_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/byte_array.rs:114`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`GenericByteArray`](../operations/arrow_array.array.byte_array.GenericByteArray.md#op-e39e3ecbe5a4126397f47443) from the provided parts, panicking on failure

# Panics

Panics if [`GenericByteArray::try_new`](../operations/arrow_array.array.byte_array.GenericByteArray.md#op-3d99b9e46a035a180ae845b8) returns an error

<a id="op-f4219ac508e24b1f846f1984"></a>
## new_null

`function` · `arrow_array::array::byte_array::GenericByteArray::new_null` · arrow-array 59.3.0

```rust
fn new_null(len: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_array::GenericByteArray", "path": "GenericByteArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [453, 2], "filename": "src/array/byte_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/byte_array.rs:179`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`GenericByteArray`](../operations/arrow_array.array.byte_array.GenericByteArray.md#op-e39e3ecbe5a4126397f47443) of length `len` where all values are null

<a id="op-90d82dede89c9d0ded005e0e"></a>
## new_repeated

`function` · `arrow_array::array::byte_array::GenericByteArray::new_repeated` · arrow-array 59.3.0

```rust
fn new_repeated(value: impl AsRef<T::Native>, repeat_count: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_array::GenericByteArray", "path": "GenericByteArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [453, 2], "filename": "src/array/byte_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/byte_array.rs:198`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`GenericByteArray`](../operations/arrow_array.array.byte_array.GenericByteArray.md#op-e39e3ecbe5a4126397f47443) where `value` is repeated `repeat_count` times.

# Panics
This will panic if value's length multiplied by `repeat_count` overflows usize.


<a id="op-2507325e3bcbf1cb1010354d"></a>
## new_scalar

`function` · `arrow_array::array::byte_array::GenericByteArray::new_scalar` · arrow-array 59.3.0

```rust
fn new_scalar(value: impl AsRef<T::Native>) -> Scalar<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_array::GenericByteArray", "path": "GenericByteArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [453, 2], "filename": "src/array/byte_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/byte_array.rs:189`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`Scalar`](../operations/arrow_array.scalar.Scalar.md#op-0ca10f05b262b3afc7077257) from `v`

<a id="op-0527926d27504aa49ac674a3"></a>
## new_unchecked

`function` · `arrow_array::array::byte_array::GenericByteArray::new_unchecked` · arrow-array 59.3.0

```rust
unsafe fn new_unchecked(offsets: OffsetBuffer<T::Offset>, values: Buffer, nulls: Option<NullBuffer>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_array::GenericByteArray", "path": "GenericByteArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [453, 2], "filename": "src/array/byte_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/byte_array.rs:162`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`GenericByteArray`](../operations/arrow_array.array.byte_array.GenericByteArray.md#op-e39e3ecbe5a4126397f47443) from the provided parts, without validation

# Safety

Safe if [`Self::try_new`](../operations/arrow_array.array.byte_array.GenericByteArray.md#op-3d99b9e46a035a180ae845b8) would not error

<a id="op-eec567375ce762220c8bdffe"></a>
## nulls

`function` · `arrow_array::array::byte_array::GenericByteArray::nulls` · arrow-array 59.3.0

```rust
fn nulls(&self) -> Option<&NullBuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_array::GenericByteArray", "path": "GenericByteArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [466, 1], "end": [537, 2], "filename": "src/array/byte_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/byte_array.rs:507`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a1878eb132f0cbb40c89586f"></a>
## num_chars

`function` · `arrow_array::array::byte_array::GenericByteArray::num_chars` · arrow-array 59.3.0

```rust
fn num_chars(&self, i: usize) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::types::GenericStringType", "path": "GenericStringType"}}}], "constraints": []}}, "id": "arrow_array::array::byte_array::GenericByteArray", "path": "GenericByteArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [60, 2], "filename": "src/array/string_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/string_array.rs:31`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the number of `Unicode Scalar Value` in the string at index `i`.
# Performance
This function has `O(n)` time complexity where `n` is the string length.
If you can make sure that all chars in the string are in the range `U+0x0000` ~ `U+0x007F`,
please use the function [`value_length`](#method.value_length) which has O(1) time complexity.

<a id="op-0adaeef98b3d2297f1df48c6"></a>
## offset

`function` · `arrow_array::array::byte_array::GenericByteArray::offset` · arrow-array 59.3.0

```rust
fn offset(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_array::GenericByteArray", "path": "GenericByteArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [466, 1], "end": [537, 2], "filename": "src/array/byte_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/byte_array.rs:503`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-82b3049aee750bdf6cb17845"></a>
## offsets

`function` · `arrow_array::array::byte_array::GenericByteArray::offsets` · arrow-array 59.3.0

```rust
fn offsets(&self) -> &OffsetBuffer<T::Offset>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_array::GenericByteArray", "path": "GenericByteArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [453, 2], "filename": "src/array/byte_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/byte_array.rs:269`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a reference to the offsets of this array

Unlike [`Self::value_offsets`](../operations/arrow_array.array.byte_array.GenericByteArray.md#op-0acd8caae9d23cc93b70f50a) this returns the [`OffsetBuffer`](../operations/arrow_buffer.buffer.offset.OffsetBuffer.md#op-a91a6a87515fb5ae378e13fc)
allowing for zero-copy cloning

<a id="op-eb66735ad90e82fdce00a37b"></a>
## shrink_to_fit

`function` · `arrow_array::array::byte_array::GenericByteArray::shrink_to_fit` · arrow-array 59.3.0

```rust
fn shrink_to_fit(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_array::GenericByteArray", "path": "GenericByteArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [466, 1], "end": [537, 2], "filename": "src/array/byte_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/byte_array.rs:495`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-28d40e97d74faebd7a96179c"></a>
## slice

`function` · `arrow_array::array::byte_array::GenericByteArray::slice` · arrow-array 59.3.0

```rust
fn slice(&self, offset: usize, length: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_array::GenericByteArray", "path": "GenericByteArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [453, 2], "filename": "src/array/byte_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/byte_array.rs:362`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a zero-copy slice of this array with the indicated offset and length.

<a id="op-a093675f32237489d6f232d0"></a>
## slice

`function` · `arrow_array::array::byte_array::GenericByteArray::slice` · arrow-array 59.3.0

```rust
fn slice(&self, offset: usize, length: usize) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_array::GenericByteArray", "path": "GenericByteArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [466, 1], "end": [537, 2], "filename": "src/array/byte_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/byte_array.rs:483`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4dc6f2e7e400431dfbe613d6"></a>
## take_iter

`function` · `arrow_array::array::byte_array::GenericByteArray::take_iter` · arrow-array 59.3.0

```rust
fn take_iter<'a>(&'a self, indexes: impl Iterator<Item = Option<usize>> + 'a) -> impl Iterator<Item = Option<&'a [u8]>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::types::GenericBinaryType", "path": "GenericBinaryType"}}}], "constraints": []}}, "id": "arrow_array::array::byte_array::GenericByteArray", "path": "GenericByteArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [95, 2], "filename": "src/array/binary_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/binary_array.rs:78`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns an iterator that returns the values of `array.value(i)` for an iterator with each element `i`

<a id="op-a7e3d5a5b018f161ed7a749b"></a>
## take_iter

`function` · `arrow_array::array::byte_array::GenericByteArray::take_iter` · arrow-array 59.3.0

```rust
fn take_iter<'a>(&'a self, indexes: impl Iterator<Item = Option<usize>> + 'a) -> impl Iterator<Item = Option<&'a str>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::types::GenericStringType", "path": "GenericStringType"}}}], "constraints": []}}, "id": "arrow_array::array::byte_array::GenericByteArray", "path": "GenericByteArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [60, 2], "filename": "src/array/string_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/string_array.rs:36`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns an iterator that returns the values of `array.value(i)` for an iterator with each element `i`

<a id="op-673f3c32acdd68c56a238ddb"></a>
## take_iter_unchecked

`function` · `arrow_array::array::byte_array::GenericByteArray::take_iter_unchecked` · arrow-array 59.3.0

```rust
unsafe fn take_iter_unchecked<'a>(&'a self, indexes: impl Iterator<Item = Option<usize>> + 'a) -> impl Iterator<Item = Option<&'a str>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::types::GenericStringType", "path": "GenericStringType"}}}], "constraints": []}}, "id": "arrow_array::array::byte_array::GenericByteArray", "path": "GenericByteArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [60, 2], "filename": "src/array/string_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/string_array.rs:47`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns an iterator that returns the values of `array.value(i)` for an iterator with each element `i`
# Safety

caller must ensure that the indexes in the iterator are less than the `array.len()`

<a id="op-c3cfe1aeeea53b4fa1a3f8cf"></a>
## take_iter_unchecked

`function` · `arrow_array::array::byte_array::GenericByteArray::take_iter_unchecked` · arrow-array 59.3.0

```rust
unsafe fn take_iter_unchecked<'a>(&'a self, indexes: impl Iterator<Item = Option<usize>> + 'a) -> impl Iterator<Item = Option<&'a [u8]>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::types::GenericBinaryType", "path": "GenericBinaryType"}}}], "constraints": []}}, "id": "arrow_array::array::byte_array::GenericByteArray", "path": "GenericByteArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [95, 2], "filename": "src/array/binary_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/binary_array.rs:89`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns an iterator that returns the values of `array.value(i)` for an iterator with each element `i`
# Safety

caller must ensure that the indexes in the iterator are less than the `array.len()`

<a id="op-cbdc624ff36d3f1cfbeea1e7"></a>
## to_data

`function` · `arrow_array::array::byte_array::GenericByteArray::to_data` · arrow-array 59.3.0

```rust
fn to_data(&self) -> ArrayData
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_array::GenericByteArray", "path": "GenericByteArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [466, 1], "end": [537, 2], "filename": "src/array/byte_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/byte_array.rs:471`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-16e5ea14f9ed5496f0c28562"></a>
## try_from_binary

`function` · `arrow_array::array::byte_array::GenericByteArray::try_from_binary` · arrow-array 59.3.0

```rust
fn try_from_binary(v: GenericBinaryArray<OffsetSize>) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::types::GenericStringType", "path": "GenericStringType"}}}], "constraints": []}}, "id": "arrow_array::array::byte_array::GenericByteArray", "path": "GenericByteArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [60, 2], "filename": "src/array/string_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/string_array.rs:56`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Fallibly creates a [`GenericStringArray`](../operations/arrow_array.array.string_array.GenericStringArray.md#op-86b79a80b65c0571602ced76) from a [`GenericBinaryArray`](../operations/arrow_array.array.binary_array.GenericBinaryArray.md#op-faf606314f632d89aa2152d6) returning
an error if [`GenericBinaryArray`](../operations/arrow_array.array.binary_array.GenericBinaryArray.md#op-faf606314f632d89aa2152d6) contains invalid UTF-8 data

<a id="op-3d99b9e46a035a180ae845b8"></a>
## try_new

`function` · `arrow_array::array::byte_array::GenericByteArray::try_new` · arrow-array 59.3.0

```rust
fn try_new(offsets: OffsetBuffer<T::Offset>, values: Buffer, nulls: Option<NullBuffer>) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_array::GenericByteArray", "path": "GenericByteArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [453, 2], "filename": "src/array/byte_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/byte_array.rs:128`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`GenericByteArray`](../operations/arrow_array.array.byte_array.GenericByteArray.md#op-e39e3ecbe5a4126397f47443) from the provided parts, returning an error on failure

# Errors

* `offsets.len() - 1 != nulls.len()`
* Any consecutive pair of `offsets` does not denote a valid slice of `values`

<a id="op-db7567e79518ead400bf28c9"></a>
## value

`function` · `arrow_array::array::byte_array::GenericByteArray::value` · arrow-array 59.3.0

```rust
fn value(&self, i: usize) -> &T::Native
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_array::GenericByteArray", "path": "GenericByteArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [453, 2], "filename": "src/array/byte_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/byte_array.rs:342`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the element at index `i`

Note: This method does not check for nulls and the value is arbitrary
(but still well-defined) if [`is_null`](Self::is_null) returns true for the index.

# Panics
Panics if index `i` is out of bounds.

<a id="op-b3c122b7d501637aad464c8d"></a>
## value_data

`function` · `arrow_array::array::byte_array::GenericByteArray::value_data` · arrow-array 59.3.0

```rust
fn value_data(&self) -> &[u8]
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_array::GenericByteArray", "path": "GenericByteArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [453, 2], "filename": "src/array/byte_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/byte_array.rs:283`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the raw value data

<a id="op-dfeee78bfcbe66aa28c8c5cf"></a>
## value_length

`function` · `arrow_array::array::byte_array::GenericByteArray::value_length` · arrow-array 59.3.0

```rust
fn value_length(&self, i: usize) -> T::Offset
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_array::GenericByteArray", "path": "GenericByteArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [453, 2], "filename": "src/array/byte_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/byte_array.rs:259`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the length for value at index `i`.
# Panics
Panics if index `i` is out of bounds.

<a id="op-0acd8caae9d23cc93b70f50a"></a>
## value_offsets

`function` · `arrow_array::array::byte_array::GenericByteArray::value_offsets` · arrow-array 59.3.0

```rust
fn value_offsets(&self) -> &[T::Offset]
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_array::GenericByteArray", "path": "GenericByteArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [453, 2], "filename": "src/array/byte_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/byte_array.rs:297`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the offset values in the offsets buffer

<a id="op-a8feb2169b4f82178d3fe5ce"></a>
## value_unchecked

`function` · `arrow_array::array::byte_array::GenericByteArray::value_unchecked` · arrow-array 59.3.0

```rust
unsafe fn value_unchecked(&self, i: usize) -> &T::Native
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_array::GenericByteArray", "path": "GenericByteArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [453, 2], "filename": "src/array/byte_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/byte_array.rs:308`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the element at index `i`

Note: This method does not check for nulls and the value is arbitrary
if [`is_null`](Self::is_null) returns true for the index.

# Safety
Caller is responsible for ensuring that the index is within the bounds of the array

<a id="op-6a3201bc3baa1fc1a9a9a2f9"></a>
## values

`function` · `arrow_array::array::byte_array::GenericByteArray::values` · arrow-array 59.3.0

```rust
fn values(&self) -> &Buffer
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_array::GenericByteArray", "path": "GenericByteArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [453, 2], "filename": "src/array/byte_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/byte_array.rs:278`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the values of this array

Unlike [`Self::value_data`](../operations/arrow_array.array.byte_array.GenericByteArray.md#op-b3c122b7d501637aad464c8d) this returns the [`Buffer`](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-f54755e677e7b3529b3edb8b)
allowing for zero-copy cloning
