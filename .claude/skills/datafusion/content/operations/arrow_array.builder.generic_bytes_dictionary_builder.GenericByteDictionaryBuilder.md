# `arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.builder.generic_bytes_dictionary_builder.GenericByteDictionaryBuilder.json).

<a id="op-ca7687878a459205c56b6885"></a>
## GenericByteDictionaryBuilder

`struct` · `arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder` · arrow-array 59.3.0

```rust
struct GenericByteDictionaryBuilder<K, T> where K: ArrowDictionaryKeyType, T: ByteArrayType
```

Source: `src/builder/generic_bytes_dictionary_builder.rs:36`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builder for [`DictionaryArray`](../operations/arrow_array.array.dictionary_array.DictionaryArray.md#op-761ab0c11d727f026bdc2e47) of [`GenericByteArray`](../operations/arrow_array.array.byte_array.GenericByteArray.md#op-e39e3ecbe5a4126397f47443)

For example to map a set of byte indices to String values. Note that
the use of a `HashMap` here will not scale to very large arrays or
result in an ordered dictionary.

<a id="op-b88ad0bff5cb365205ff2c4f"></a>
## append

`function` · `arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder::append` · arrow-array 59.3.0

```rust
fn append(&mut self, value: impl AsRef<T::Native>) -> Result<K::Native, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder", "path": "GenericByteDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [265, 1], "end": [506, 2], "filename": "src/builder/generic_bytes_dictionary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_dictionary_builder.rs:302`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Append a value to the array. Return an existing index
if already present in the values array or a new index if the
value is appended to the values array.

Returns an error if the new index would overflow the key type.

<a id="op-efb5aa1678030068c24097fc"></a>
## append_n

`function` · `arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder::append_n` · arrow-array 59.3.0

```rust
fn append_n(&mut self, value: impl AsRef<T::Native>, count: usize) -> Result<K::Native, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder", "path": "GenericByteDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [265, 1], "end": [506, 2], "filename": "src/builder/generic_bytes_dictionary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_dictionary_builder.rs:312`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Append a value multiple times to the array.
This is the same as `append` but allows to append the same value multiple times without doing multiple lookups.

Returns an error if the new index would overflow the key type.

<a id="op-88b072e6bf8adb705017b382"></a>
## append_null

`function` · `arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder::append_null` · arrow-array 59.3.0

```rust
fn append_null(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder", "path": "GenericByteDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [265, 1], "end": [506, 2], "filename": "src/builder/generic_bytes_dictionary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_dictionary_builder.rs:344`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends a null slot into the builder

<a id="op-1fea70792fcf998daf01797e"></a>
## append_nulls

`function` · `arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder::append_nulls` · arrow-array 59.3.0

```rust
fn append_nulls(&mut self, n: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder", "path": "GenericByteDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [265, 1], "end": [506, 2], "filename": "src/builder/generic_bytes_dictionary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_dictionary_builder.rs:350`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Infallibly append `n` null slots into the builder

<a id="op-4229d9ebdb91dc294c375be4"></a>
## append_option

`function` · `arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder::append_option` · arrow-array 59.3.0

```rust
fn append_option(&mut self, value: Option<impl AsRef<T::Native>>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder", "path": "GenericByteDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [265, 1], "end": [506, 2], "filename": "src/builder/generic_bytes_dictionary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_dictionary_builder.rs:360`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Append an `Option` value into the builder

# Panics

Panics if the resulting length of the dictionary values array would exceed `T::Native::MAX`

<a id="op-f8b194b1a256056bcad8dbbe"></a>
## append_options

`function` · `arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder::append_options` · arrow-array 59.3.0

```rust
fn append_options(&mut self, value: Option<impl AsRef<T::Native>>, count: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder", "path": "GenericByteDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [265, 1], "end": [506, 2], "filename": "src/builder/generic_bytes_dictionary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_dictionary_builder.rs:373`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Append an `Option` value into the builder repeatedly `count` times.
This is the same as `append_option` but allows to append the same value multiple times without doing multiple lookups.

# Panics

Panics if the resulting length of the dictionary values array would exceed `T::Native::MAX`

<a id="op-026671f9b9b79fb85c3b1236"></a>
## append_value

`function` · `arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder::append_value` · arrow-array 59.3.0

```rust
fn append_value(&mut self, value: impl AsRef<T::Native>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder", "path": "GenericByteDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [265, 1], "end": [506, 2], "filename": "src/builder/generic_bytes_dictionary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_dictionary_builder.rs:327`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Infallibly append a value to this builder

# Panics

Panics if the resulting length of the dictionary values array would exceed `T::Native::MAX`

<a id="op-21ec209282d44460e669658a"></a>
## append_values

`function` · `arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder::append_values` · arrow-array 59.3.0

```rust
fn append_values(&mut self, value: impl AsRef<T::Native>, count: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder", "path": "GenericByteDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [265, 1], "end": [506, 2], "filename": "src/builder/generic_bytes_dictionary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_dictionary_builder.rs:337`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Infallibly append a value to this builder repeatedly `count` times.
This is the same as `append_value` but allows to append the same value multiple times without doing multiple lookups.

# Panics

Panics if the resulting length of the dictionary values array would exceed `T::Native::MAX`

<a id="op-16896344a88914c5ce77f221"></a>
## as_any

`function` · `arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder::as_any` · arrow-array 59.3.0

```rust
fn as_any(&self) -> &dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder", "path": "GenericByteDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [225, 1], "end": [263, 2], "filename": "src/builder/generic_bytes_dictionary_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/generic_bytes_dictionary_builder.rs:231`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the builder as an non-mutable `Any` reference.

<a id="op-68bf60e05634537a298937c2"></a>
## as_any_mut

`function` · `arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder::as_any_mut` · arrow-array 59.3.0

```rust
fn as_any_mut(&mut self) -> &mut dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder", "path": "GenericByteDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [225, 1], "end": [263, 2], "filename": "src/builder/generic_bytes_dictionary_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/generic_bytes_dictionary_builder.rs:236`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the builder as an mutable `Any` reference.

<a id="op-fc6ff4fbf47217630f3fcd93"></a>
## default

`function` · `arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder::default` · arrow-array 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder", "path": "GenericByteDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [48, 1], "end": [56, 2], "filename": "src/builder/generic_bytes_dictionary_builder.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/builder/generic_bytes_dictionary_builder.rs:53`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b8dd8a7c0837c33cfef525df"></a>
## extend

`function` · `arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder::extend` · arrow-array 59.3.0

```rust
fn extend<I: IntoIterator<Item = Option<V>>>(&mut self, iter: I)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder", "path": "GenericByteDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": ""}}}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [508, 1], "end": [517, 2], "filename": "src/builder/generic_bytes_dictionary_builder.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "V"}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "core::iter::traits::collect::Extend", "path": "Extend"}, "trait_path": "core::iter::traits::collect::Extend"}`

Source: `src/builder/generic_bytes_dictionary_builder.rs:512`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b603344edc9630ac76fe6cde"></a>
## extend_dictionary

`function` · `arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder::extend_dictionary` · arrow-array 59.3.0

```rust
fn extend_dictionary(&mut self, dictionary: &TypedDictionaryArray<'_, K, GenericByteArray<T>>) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder", "path": "GenericByteDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [265, 1], "end": [506, 2], "filename": "src/builder/generic_bytes_dictionary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_dictionary_builder.rs:387`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Extends builder with an existing dictionary array.

This is the same as [`Self::extend`](../operations/arrow_array.builder.generic_bytes_dictionary_builder.GenericByteDictionaryBuilder.md#op-b8dd8a7c0837c33cfef525df) but is faster as it translates
the dictionary values once rather than doing a lookup for each item in the iterator

when dictionary values are null (the actual mapped values) the keys are null


<a id="op-abe01caadd1fdd3b0abd79fe"></a>
## finish

`function` · `arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder::finish` · arrow-array 59.3.0

```rust
fn finish(&mut self) -> DictionaryArray<K>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder", "path": "GenericByteDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [265, 1], "end": [506, 2], "filename": "src/builder/generic_bytes_dictionary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_dictionary_builder.rs:438`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the `DictionaryArray` and reset this builder.

<a id="op-e2f0ed8f984db94a6390538e"></a>
## finish

`function` · `arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder::finish` · arrow-array 59.3.0

```rust
fn finish(&mut self) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder", "path": "GenericByteDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [225, 1], "end": [263, 2], "filename": "src/builder/generic_bytes_dictionary_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/generic_bytes_dictionary_builder.rs:251`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the array and reset this builder.

<a id="op-23ab0e902a677e38d777f7ed"></a>
## finish_cloned

`function` · `arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder::finish_cloned` · arrow-array 59.3.0

```rust
fn finish_cloned(&self) -> DictionaryArray<K>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder", "path": "GenericByteDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [265, 1], "end": [506, 2], "filename": "src/builder/generic_bytes_dictionary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_dictionary_builder.rs:455`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the `DictionaryArray` without resetting the builder.

<a id="op-71947cdbd000b9ecbf7265a6"></a>
## finish_cloned

`function` · `arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder::finish_cloned` · arrow-array 59.3.0

```rust
fn finish_cloned(&self) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder", "path": "GenericByteDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [225, 1], "end": [263, 2], "filename": "src/builder/generic_bytes_dictionary_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/generic_bytes_dictionary_builder.rs:256`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the array without resetting the builder.

<a id="op-1932dc6e22b29a54acd37ccf"></a>
## finish_preserve_values

`function` · `arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder::finish_preserve_values` · arrow-array 59.3.0

```rust
fn finish_preserve_values(&mut self) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder", "path": "GenericByteDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [225, 1], "end": [263, 2], "filename": "src/builder/generic_bytes_dictionary_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/generic_bytes_dictionary_builder.rs:260`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c36413e5bd8ca3cbea520193"></a>
## finish_preserve_values

`function` · `arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder::finish_preserve_values` · arrow-array 59.3.0

```rust
fn finish_preserve_values(&mut self) -> DictionaryArray<K>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder", "path": "GenericByteDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [265, 1], "end": [506, 2], "filename": "src/builder/generic_bytes_dictionary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_dictionary_builder.rs:487`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

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

<a id="op-a4df9ed34aedd310ecf111e1"></a>
## fmt

`function` · `arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder::fmt` · arrow-array 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder", "path": "GenericByteDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [35, 10], "end": [35, 15], "filename": "src/builder/generic_bytes_dictionary_builder.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/builder/generic_bytes_dictionary_builder.rs:35`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8df882b171f48ebaa5caa93b"></a>
## into_box_any

`function` · `arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder::into_box_any` · arrow-array 59.3.0

```rust
fn into_box_any(Box<self>) -> Box<dyn Any>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder", "path": "GenericByteDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [225, 1], "end": [263, 2], "filename": "src/builder/generic_bytes_dictionary_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/generic_bytes_dictionary_builder.rs:241`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the boxed builder as a box of `Any`.

<a id="op-96c0d37b4864b2bba57b1154"></a>
## len

`function` · `arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder::len` · arrow-array 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder", "path": "GenericByteDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [225, 1], "end": [263, 2], "filename": "src/builder/generic_bytes_dictionary_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/generic_bytes_dictionary_builder.rs:246`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the number of array slots in the builder

<a id="op-2e478e494b3cd5e353cf1969"></a>
## new

`function` · `arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder::new` · arrow-array 59.3.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder", "path": "GenericByteDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [58, 1], "end": [223, 2], "filename": "src/builder/generic_bytes_dictionary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_dictionary_builder.rs:64`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new `GenericByteDictionaryBuilder`

<a id="op-4375382b915a3e12d8e5fdca"></a>
## new_with_dictionary

`function` · `arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder::new_with_dictionary` · arrow-array 59.3.0

```rust
fn new_with_dictionary(keys_capacity: usize, dictionary_values: &GenericByteArray<T>) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder", "path": "GenericByteDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [58, 1], "end": [223, 2], "filename": "src/builder/generic_bytes_dictionary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_dictionary_builder.rs:116`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new `GenericByteDictionaryBuilder` from a keys capacity and a dictionary
which is initialized with the given values.
The indices of those dictionary values are used as keys.

# Example

```
# use arrow_array::builder::StringDictionaryBuilder;
# use arrow_array::{Int16Array, StringArray};

let dictionary_values = StringArray::from(vec![None, Some("abc"), Some("def")]);

let mut builder = StringDictionaryBuilder::new_with_dictionary(3, &dictionary_values).unwrap();
builder.append("def").unwrap();
builder.append_null();
builder.append("abc").unwrap();

let dictionary_array = builder.finish();

let keys = dictionary_array.keys();

assert_eq!(keys, &Int16Array::from(vec![Some(2), None, Some(1)]));
```

<a id="op-f57d16d4b413bdeff22873a1"></a>
## try_new_from_builder

`function` · `arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder::try_new_from_builder` · arrow-array 59.3.0

```rust
fn try_new_from_builder<K2>(source: GenericByteDictionaryBuilder<K2, T>) -> Result<Self, ArrowError> where K::Native: NumCast, K2: ArrowDictionaryKeyType, K2::Native: NumCast
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder", "path": "GenericByteDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [58, 1], "end": [223, 2], "filename": "src/builder/generic_bytes_dictionary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_dictionary_builder.rs:186`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new `GenericByteDictionaryBuilder` from the existing builder with the same
keys and values, but with a new data type for the keys.

# Example
```
#
# use arrow_array::builder::StringDictionaryBuilder;
# use arrow_array::types::{UInt8Type, UInt16Type};
# use arrow_array::UInt16Array;
# use arrow_schema::ArrowError;

let mut u8_keyed_builder = StringDictionaryBuilder::<UInt8Type>::new();

// appending too many values causes the dictionary to overflow
for i in 0..256 {
    u8_keyed_builder.append_value(format!("{}", i));
}
let result = u8_keyed_builder.append("256");
assert!(matches!(result, Err(ArrowError::DictionaryKeyOverflowError{})));

// we need to upgrade to a larger key type
let mut u16_keyed_builder = StringDictionaryBuilder::<UInt16Type>::try_new_from_builder(u8_keyed_builder).unwrap();
let dictionary_array = u16_keyed_builder.finish();
let keys = dictionary_array.keys();

assert_eq!(keys, &UInt16Array::from_iter(0..256));
```

<a id="op-094d3a086d7f07c9353f01f5"></a>
## validity_slice

`function` · `arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder::validity_slice` · arrow-array 59.3.0

```rust
fn validity_slice(&self) -> Option<&[u8]>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder", "path": "GenericByteDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [265, 1], "end": [506, 2], "filename": "src/builder/generic_bytes_dictionary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_dictionary_builder.rs:503`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the current null buffer as a slice

<a id="op-bf6a589b78b46de31bb2f5db"></a>
## with_capacity

`function` · `arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder::with_capacity` · arrow-array 59.3.0

```rust
fn with_capacity(keys_capacity: usize, value_capacity: usize, data_capacity: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder", "path": "GenericByteDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [58, 1], "end": [223, 2], "filename": "src/builder/generic_bytes_dictionary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_dictionary_builder.rs:80`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new `GenericByteDictionaryBuilder` with the provided capacities

`keys_capacity`: the number of keys, i.e. length of array to build
`value_capacity`: the number of distinct dictionary values, i.e. size of dictionary
`data_capacity`: the total number of bytes of all distinct bytes in the dictionary
