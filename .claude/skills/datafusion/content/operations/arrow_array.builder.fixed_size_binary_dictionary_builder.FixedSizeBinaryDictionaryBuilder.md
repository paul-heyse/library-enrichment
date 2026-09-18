# `arrow_array::builder::fixed_size_binary_dictionary_builder::FixedSizeBinaryDictionaryBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.builder.fixed_size_binary_dictionary_builder.FixedSizeBinaryDictionaryBuilder.json).

<a id="op-829376217ecac1017e254b5f"></a>
## FixedSizeBinaryDictionaryBuilder

`struct` · `arrow_array::builder::fixed_size_binary_dictionary_builder::FixedSizeBinaryDictionaryBuilder` · arrow-array 59.3.0

```rust
struct FixedSizeBinaryDictionaryBuilder<K> where K: ArrowDictionaryKeyType
```

Source: `src/builder/fixed_size_binary_dictionary_builder.rs:61`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builder for [`DictionaryArray`](../operations/arrow_array.array.dictionary_array.DictionaryArray.md#op-761ab0c11d727f026bdc2e47) of [`FixedSizeBinaryArray`]

The output array has a dictionary of unique, fixed-size binary values. The
builder handles deduplication.

# Example
```
# use arrow_array::builder::{FixedSizeBinaryDictionaryBuilder};
# use arrow_array::array::{Array, FixedSizeBinaryArray};
# use arrow_array::DictionaryArray;
# use arrow_array::types::Int8Type;
// Build 3 byte FixedBinaryArrays
let byte_width = 3;
let mut builder = FixedSizeBinaryDictionaryBuilder::<Int8Type>::new(3);
builder.append("abc").unwrap();
builder.append_null();
builder.append(b"def").unwrap();
builder.append(b"def").unwrap(); // duplicate value
// Result is a Dictionary Array
let array = builder.finish();
let dict_array = array.as_any().downcast_ref::<DictionaryArray<Int8Type>>().unwrap();
// The array represents "abc", null, "def", "def"
assert_eq!(array.keys().len(), 4);
// but there are only 2 unique values
assert_eq!(array.values().len(), 2);
let values = dict_array.values().as_any().downcast_ref::<FixedSizeBinaryArray>().unwrap();
assert_eq!(values.value(0), "abc".as_bytes());
assert_eq!(values.value(1), "def".as_bytes());
```

[`FixedSizeBinaryArray`]: crate::FixedSizeBinaryArray

<a id="op-2345395e310d72721c2b9ad9"></a>
## append

`function` · `arrow_array::builder::fixed_size_binary_dictionary_builder::FixedSizeBinaryDictionaryBuilder::append` · arrow-array 59.3.0

```rust
fn append(&mut self, value: impl AsRef<[u8]>) -> Result<K::Native, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}], "constraints": []}}, "id": "arrow_array::builder::fixed_size_binary_dictionary_builder::FixedSizeBinaryDictionaryBuilder", "path": "FixedSizeBinaryDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}]}, "is_negative": false, "span": {"begin": [210, 1], "end": [375, 2], "filename": "src/builder/fixed_size_binary_dictionary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/fixed_size_binary_dictionary_builder.rs:245`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Append a value to the array. Return an existing index
if already present in the values array or a new index if the
value is appended to the values array.

Returns an error if the new index would overflow the key type.

<a id="op-ce8c03b9fd52e5a4c9844edf"></a>
## append_n

`function` · `arrow_array::builder::fixed_size_binary_dictionary_builder::FixedSizeBinaryDictionaryBuilder::append_n` · arrow-array 59.3.0

```rust
fn append_n(&mut self, value: impl AsRef<[u8]>, count: usize) -> Result<K::Native, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}], "constraints": []}}, "id": "arrow_array::builder::fixed_size_binary_dictionary_builder::FixedSizeBinaryDictionaryBuilder", "path": "FixedSizeBinaryDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}]}, "is_negative": false, "span": {"begin": [210, 1], "end": [375, 2], "filename": "src/builder/fixed_size_binary_dictionary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/fixed_size_binary_dictionary_builder.rs:263`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Append a value multiple times to the array.
This is the same as [`Self::append`](../operations/arrow_array.builder.fixed_size_binary_dictionary_builder.FixedSizeBinaryDictionaryBuilder.md#op-2345395e310d72721c2b9ad9) but allows to append the same value multiple times without doing multiple lookups.

Returns an error if the new index would overflow the key type.

<a id="op-efe8f2559b1132e0a28ae412"></a>
## append_null

`function` · `arrow_array::builder::fixed_size_binary_dictionary_builder::FixedSizeBinaryDictionaryBuilder::append_null` · arrow-array 59.3.0

```rust
fn append_null(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}], "constraints": []}}, "id": "arrow_array::builder::fixed_size_binary_dictionary_builder::FixedSizeBinaryDictionaryBuilder", "path": "FixedSizeBinaryDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}]}, "is_negative": false, "span": {"begin": [210, 1], "end": [375, 2], "filename": "src/builder/fixed_size_binary_dictionary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/fixed_size_binary_dictionary_builder.rs:283`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends a null slot into the builder

<a id="op-aedbce550bddaeebaee5565e"></a>
## append_nulls

`function` · `arrow_array::builder::fixed_size_binary_dictionary_builder::FixedSizeBinaryDictionaryBuilder::append_nulls` · arrow-array 59.3.0

```rust
fn append_nulls(&mut self, n: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}], "constraints": []}}, "id": "arrow_array::builder::fixed_size_binary_dictionary_builder::FixedSizeBinaryDictionaryBuilder", "path": "FixedSizeBinaryDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}]}, "is_negative": false, "span": {"begin": [210, 1], "end": [375, 2], "filename": "src/builder/fixed_size_binary_dictionary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/fixed_size_binary_dictionary_builder.rs:289`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends `n` `null`s into the builder.

<a id="op-81b5b2588bd897e91ca496f3"></a>
## append_value

`function` · `arrow_array::builder::fixed_size_binary_dictionary_builder::FixedSizeBinaryDictionaryBuilder::append_value` · arrow-array 59.3.0

```rust
fn append_value(&mut self, value: impl AsRef<[u8]>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}], "constraints": []}}, "id": "arrow_array::builder::fixed_size_binary_dictionary_builder::FixedSizeBinaryDictionaryBuilder", "path": "FixedSizeBinaryDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}]}, "is_negative": false, "span": {"begin": [210, 1], "end": [375, 2], "filename": "src/builder/fixed_size_binary_dictionary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/fixed_size_binary_dictionary_builder.rs:298`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Infallibly append a value to this builder

# Panics

Panics if the resulting length of the dictionary values array would exceed `T::Native::MAX`

<a id="op-5c0bad6ad6fa3a534197df5d"></a>
## as_any

`function` · `arrow_array::builder::fixed_size_binary_dictionary_builder::FixedSizeBinaryDictionaryBuilder::as_any` · arrow-array 59.3.0

```rust
fn as_any(&self) -> &dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}], "constraints": []}}, "id": "arrow_array::builder::fixed_size_binary_dictionary_builder::FixedSizeBinaryDictionaryBuilder", "path": "FixedSizeBinaryDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}]}, "is_negative": false, "span": {"begin": [171, 1], "end": [208, 2], "filename": "src/builder/fixed_size_binary_dictionary_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/fixed_size_binary_dictionary_builder.rs:176`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the builder as an non-mutable `Any` reference.

<a id="op-a5c0e1ee3eb9e5a7e28188eb"></a>
## as_any_mut

`function` · `arrow_array::builder::fixed_size_binary_dictionary_builder::FixedSizeBinaryDictionaryBuilder::as_any_mut` · arrow-array 59.3.0

```rust
fn as_any_mut(&mut self) -> &mut dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}], "constraints": []}}, "id": "arrow_array::builder::fixed_size_binary_dictionary_builder::FixedSizeBinaryDictionaryBuilder", "path": "FixedSizeBinaryDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}]}, "is_negative": false, "span": {"begin": [171, 1], "end": [208, 2], "filename": "src/builder/fixed_size_binary_dictionary_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/fixed_size_binary_dictionary_builder.rs:181`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the builder as an mutable `Any` reference.

<a id="op-3c620e91649f9e3991ffd4f4"></a>
## finish

`function` · `arrow_array::builder::fixed_size_binary_dictionary_builder::FixedSizeBinaryDictionaryBuilder::finish` · arrow-array 59.3.0

```rust
fn finish(&mut self) -> DictionaryArray<K>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}], "constraints": []}}, "id": "arrow_array::builder::fixed_size_binary_dictionary_builder::FixedSizeBinaryDictionaryBuilder", "path": "FixedSizeBinaryDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}]}, "is_negative": false, "span": {"begin": [210, 1], "end": [375, 2], "filename": "src/builder/fixed_size_binary_dictionary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/fixed_size_binary_dictionary_builder.rs:303`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the `DictionaryArray` and reset this builder.

<a id="op-fc4adada199b4ff349c5c089"></a>
## finish

`function` · `arrow_array::builder::fixed_size_binary_dictionary_builder::FixedSizeBinaryDictionaryBuilder::finish` · arrow-array 59.3.0

```rust
fn finish(&mut self) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}], "constraints": []}}, "id": "arrow_array::builder::fixed_size_binary_dictionary_builder::FixedSizeBinaryDictionaryBuilder", "path": "FixedSizeBinaryDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}]}, "is_negative": false, "span": {"begin": [171, 1], "end": [208, 2], "filename": "src/builder/fixed_size_binary_dictionary_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/fixed_size_binary_dictionary_builder.rs:196`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the array and reset this builder.

<a id="op-a76849def90de7ec493d271b"></a>
## finish_cloned

`function` · `arrow_array::builder::fixed_size_binary_dictionary_builder::FixedSizeBinaryDictionaryBuilder::finish_cloned` · arrow-array 59.3.0

```rust
fn finish_cloned(&self) -> DictionaryArray<K>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}], "constraints": []}}, "id": "arrow_array::builder::fixed_size_binary_dictionary_builder::FixedSizeBinaryDictionaryBuilder", "path": "FixedSizeBinaryDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}]}, "is_negative": false, "span": {"begin": [210, 1], "end": [375, 2], "filename": "src/builder/fixed_size_binary_dictionary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/fixed_size_binary_dictionary_builder.rs:323`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the `DictionaryArray` without resetting the builder.

<a id="op-b7111c98a2db0432f0cfbbc6"></a>
## finish_cloned

`function` · `arrow_array::builder::fixed_size_binary_dictionary_builder::FixedSizeBinaryDictionaryBuilder::finish_cloned` · arrow-array 59.3.0

```rust
fn finish_cloned(&self) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}], "constraints": []}}, "id": "arrow_array::builder::fixed_size_binary_dictionary_builder::FixedSizeBinaryDictionaryBuilder", "path": "FixedSizeBinaryDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}]}, "is_negative": false, "span": {"begin": [171, 1], "end": [208, 2], "filename": "src/builder/fixed_size_binary_dictionary_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/fixed_size_binary_dictionary_builder.rs:201`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the array without resetting the builder.

<a id="op-ac401ab8f1dafd09371b22f5"></a>
## finish_preserve_values

`function` · `arrow_array::builder::fixed_size_binary_dictionary_builder::FixedSizeBinaryDictionaryBuilder::finish_preserve_values` · arrow-array 59.3.0

```rust
fn finish_preserve_values(&mut self) -> DictionaryArray<K>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}], "constraints": []}}, "id": "arrow_array::builder::fixed_size_binary_dictionary_builder::FixedSizeBinaryDictionaryBuilder", "path": "FixedSizeBinaryDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}]}, "is_negative": false, "span": {"begin": [210, 1], "end": [375, 2], "filename": "src/builder/fixed_size_binary_dictionary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/fixed_size_binary_dictionary_builder.rs:358`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the `DictionaryArray` without resetting the values builder or
the internal de-duplication map.

The advantage of doing this is that the values will represent the entire
set of what has been built so-far by this builder and ensures
consistency in the assignment of keys to values across multiple calls
to `finish_preserve_values`. This enables ipc writers to efficiently
emit delta dictionaries.

The downside to this is that building the record requires creating a
copy of the values, which can become slowly more expensive if the
dictionary grows.

Additionally, if record batches from multiple different dictionary
builders for the same column are fed into a single ipc writer, beware
that entire dictionaries are likely to be re-sent frequently even when
the majority of the values are not used by the current record batch.

<a id="op-c348a1183a2449cbe10dabc2"></a>
## finish_preserve_values

`function` · `arrow_array::builder::fixed_size_binary_dictionary_builder::FixedSizeBinaryDictionaryBuilder::finish_preserve_values` · arrow-array 59.3.0

```rust
fn finish_preserve_values(&mut self) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}], "constraints": []}}, "id": "arrow_array::builder::fixed_size_binary_dictionary_builder::FixedSizeBinaryDictionaryBuilder", "path": "FixedSizeBinaryDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}]}, "is_negative": false, "span": {"begin": [171, 1], "end": [208, 2], "filename": "src/builder/fixed_size_binary_dictionary_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/fixed_size_binary_dictionary_builder.rs:205`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6bc601382b013697c998a129"></a>
## fmt

`function` · `arrow_array::builder::fixed_size_binary_dictionary_builder::FixedSizeBinaryDictionaryBuilder::fmt` · arrow-array 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}], "constraints": []}}, "id": "arrow_array::builder::fixed_size_binary_dictionary_builder::FixedSizeBinaryDictionaryBuilder", "path": "FixedSizeBinaryDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "generic_params": [], "type": {"generic": "K"}}}]}, "is_negative": false, "span": {"begin": [60, 10], "end": [60, 15], "filename": "src/builder/fixed_size_binary_dictionary_builder.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/builder/fixed_size_binary_dictionary_builder.rs:60`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d9d7dfd605285679cf186e38"></a>
## into_box_any

`function` · `arrow_array::builder::fixed_size_binary_dictionary_builder::FixedSizeBinaryDictionaryBuilder::into_box_any` · arrow-array 59.3.0

```rust
fn into_box_any(Box<self>) -> Box<dyn Any>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}], "constraints": []}}, "id": "arrow_array::builder::fixed_size_binary_dictionary_builder::FixedSizeBinaryDictionaryBuilder", "path": "FixedSizeBinaryDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}]}, "is_negative": false, "span": {"begin": [171, 1], "end": [208, 2], "filename": "src/builder/fixed_size_binary_dictionary_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/fixed_size_binary_dictionary_builder.rs:186`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the boxed builder as a box of `Any`.

<a id="op-29cec1e5a9d7ec64e373f4eb"></a>
## len

`function` · `arrow_array::builder::fixed_size_binary_dictionary_builder::FixedSizeBinaryDictionaryBuilder::len` · arrow-array 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}], "constraints": []}}, "id": "arrow_array::builder::fixed_size_binary_dictionary_builder::FixedSizeBinaryDictionaryBuilder", "path": "FixedSizeBinaryDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}]}, "is_negative": false, "span": {"begin": [171, 1], "end": [208, 2], "filename": "src/builder/fixed_size_binary_dictionary_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/fixed_size_binary_dictionary_builder.rs:191`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the number of array slots in the builder

<a id="op-d7b1d12f1afc03a5844a2abb"></a>
## new

`function` · `arrow_array::builder::fixed_size_binary_dictionary_builder::FixedSizeBinaryDictionaryBuilder::new` · arrow-array 59.3.0

```rust
fn new(byte_width: i32) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}], "constraints": []}}, "id": "arrow_array::builder::fixed_size_binary_dictionary_builder::FixedSizeBinaryDictionaryBuilder", "path": "FixedSizeBinaryDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}]}, "is_negative": false, "span": {"begin": [73, 1], "end": [169, 2], "filename": "src/builder/fixed_size_binary_dictionary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/fixed_size_binary_dictionary_builder.rs:78`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new `FixedSizeBinaryDictionaryBuilder`

<a id="op-c97d3de0a9a865ae85a59e21"></a>
## try_new_from_builder

`function` · `arrow_array::builder::fixed_size_binary_dictionary_builder::FixedSizeBinaryDictionaryBuilder::try_new_from_builder` · arrow-array 59.3.0

```rust
fn try_new_from_builder<K2>(source: FixedSizeBinaryDictionaryBuilder<K2>) -> Result<Self, ArrowError> where K::Native: NumCast, K2: ArrowDictionaryKeyType, K2::Native: NumCast
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}], "constraints": []}}, "id": "arrow_array::builder::fixed_size_binary_dictionary_builder::FixedSizeBinaryDictionaryBuilder", "path": "FixedSizeBinaryDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}]}, "is_negative": false, "span": {"begin": [73, 1], "end": [169, 2], "filename": "src/builder/fixed_size_binary_dictionary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/fixed_size_binary_dictionary_builder.rs:130`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new `FixedSizeBinaryDictionaryBuilder` from the existing builder with the same
keys and values, but with a new data type for the keys.

# Example
```
# use arrow_array::builder::FixedSizeBinaryDictionaryBuilder;
# use arrow_array::types::{UInt8Type, UInt16Type, UInt64Type};
# use arrow_array::UInt16Array;
# use arrow_schema::ArrowError;

let mut u8_keyed_builder = FixedSizeBinaryDictionaryBuilder::<UInt8Type>::new(2);
// appending too many values causes the dictionary to overflow
for i in 0..=255 {
    u8_keyed_builder.append_value(vec![0, i]);
}
let result = u8_keyed_builder.append(vec![1, 0]);
assert!(matches!(result, Err(ArrowError::DictionaryKeyOverflowError{})));

// we need to upgrade to a larger key type
let mut u16_keyed_builder = FixedSizeBinaryDictionaryBuilder::<UInt16Type>::try_new_from_builder(u8_keyed_builder).unwrap();
let dictionary_array = u16_keyed_builder.finish();
let keys = dictionary_array.keys();

assert_eq!(keys, &UInt16Array::from_iter(0..256));
```

<a id="op-e451fad2e4c665adce47e701"></a>
## with_capacity

`function` · `arrow_array::builder::fixed_size_binary_dictionary_builder::FixedSizeBinaryDictionaryBuilder::with_capacity` · arrow-array 59.3.0

```rust
fn with_capacity(keys_capacity: usize, value_capacity: usize, byte_width: i32) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}], "constraints": []}}, "id": "arrow_array::builder::fixed_size_binary_dictionary_builder::FixedSizeBinaryDictionaryBuilder", "path": "FixedSizeBinaryDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}]}, "is_negative": false, "span": {"begin": [73, 1], "end": [169, 2], "filename": "src/builder/fixed_size_binary_dictionary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/fixed_size_binary_dictionary_builder.rs:95`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new `FixedSizeBinaryDictionaryBuilder` with the provided capacities

`keys_capacity`: the number of keys, i.e. length of array to build
`value_capacity`: the number of distinct dictionary values, i.e. size of dictionary
`byte_width`: the byte width for individual values in the values array
