# `arrow_array::builder::generic_bytes_builder::GenericByteBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.builder.generic_bytes_builder.GenericByteBuilder.json).

<a id="op-647a230051e61dc574fafd16"></a>
## GenericByteBuilder

`struct` · `arrow_array::builder::generic_bytes_builder::GenericByteBuilder` · arrow-array 59.3.0

```rust
struct GenericByteBuilder<T: ByteArrayType>
```

Source: `src/builder/generic_bytes_builder.rs:31`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builder for [`GenericByteArray`](../operations/arrow_array.array.byte_array.GenericByteArray.md#op-e39e3ecbe5a4126397f47443)

For building strings, see docs on [`GenericStringBuilder`](../operations/arrow_array.builder.generic_bytes_builder.GenericStringBuilder.md#op-261f9ccc4c15fcce9de29b74).
For building binary, see docs on [`GenericBinaryBuilder`](../operations/arrow_array.builder.generic_bytes_builder.GenericBinaryBuilder.md#op-8d05f60326b509bd9f5d4d69).

<a id="op-efbe045b4002174412a7914a"></a>
## append_array

`function` · `arrow_array::builder::generic_bytes_builder::GenericByteBuilder::append_array` · arrow-array 59.3.0

```rust
fn append_array(&mut self, array: &GenericByteArray<T>) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_builder::GenericByteBuilder", "path": "GenericByteBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [263, 2], "filename": "src/builder/generic_bytes_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_builder.rs:161`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends array values and null to this builder as is
(this means that underlying null values are copied as is).

<a id="op-b57495f4918c8483580b2120"></a>
## append_null

`function` · `arrow_array::builder::generic_bytes_builder::GenericByteBuilder::append_null` · arrow-array 59.3.0

```rust
fn append_null(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_builder::GenericByteBuilder", "path": "GenericByteBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [263, 2], "filename": "src/builder/generic_bytes_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_builder.rs:144`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Append a null value into the builder.

<a id="op-d98eccfa32c330a95c5b9a4f"></a>
## append_nulls

`function` · `arrow_array::builder::generic_bytes_builder::GenericByteBuilder::append_nulls` · arrow-array 59.3.0

```rust
fn append_nulls(&mut self, n: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_builder::GenericByteBuilder", "path": "GenericByteBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [263, 2], "filename": "src/builder/generic_bytes_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_builder.rs:151`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends `n` `null`s into the builder.

<a id="op-e828efd971535f52bf9f63bb"></a>
## append_option

`function` · `arrow_array::builder::generic_bytes_builder::GenericByteBuilder::append_option` · arrow-array 59.3.0

```rust
fn append_option(&mut self, value: Option<impl AsRef<T::Native>>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_builder::GenericByteBuilder", "path": "GenericByteBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [263, 2], "filename": "src/builder/generic_bytes_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_builder.rs:135`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Append an `Option` value into the builder.

- A `None` value will append a null value.
- A `Some` value will append the value.

See [`Self::append_value`](../operations/arrow_array.builder.generic_bytes_builder.GenericByteBuilder.md#op-3c729b3523eb69d04317a6c7) for more panic information.

<a id="op-3c729b3523eb69d04317a6c7"></a>
## append_value

`function` · `arrow_array::builder::generic_bytes_builder::GenericByteBuilder::append_value` · arrow-array 59.3.0

```rust
fn append_value(&mut self, value: impl AsRef<T::Native>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_builder::GenericByteBuilder", "path": "GenericByteBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [263, 2], "filename": "src/builder/generic_bytes_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_builder.rs:106`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends a value into the builder.

See the [GenericStringBuilder](../operations/arrow_array.builder.generic_bytes_builder.GenericStringBuilder.md#op-261f9ccc4c15fcce9de29b74) documentation for examples of
incrementally building string values with multiple `write!` calls.

# Panics

Panics if the resulting length of [`Self::values_slice`](../operations/arrow_array.builder.generic_bytes_builder.GenericByteBuilder.md#op-eff9089e8f0f46b073b45f41) would exceed
`T::Offset::MAX` bytes.

For example, this can happen with [`StringArray`] or [`BinaryArray`]
where the total length of all values exceeds 2GB

[`StringArray`]: crate::StringArray
[`BinaryArray`]: crate::BinaryArray

<a id="op-25213b88a129e17eeef068bb"></a>
## append_value_n

`function` · `arrow_array::builder::generic_bytes_builder::GenericByteBuilder::append_value_n` · arrow-array 59.3.0

```rust
fn append_value_n(&mut self, value: impl AsRef<T::Native>, n: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_builder::GenericByteBuilder", "path": "GenericByteBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [263, 2], "filename": "src/builder/generic_bytes_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_builder.rs:117`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends a value of type `T` into the builder `n` times.

See [`Self::append_value`](../operations/arrow_array.builder.generic_bytes_builder.GenericByteBuilder.md#op-3c729b3523eb69d04317a6c7) for more panic information.

<a id="op-6c6b2ee50258329c4fbc92bf"></a>
## as_any

`function` · `arrow_array::builder::generic_bytes_builder::GenericByteBuilder::as_any` · arrow-array 59.3.0

```rust
fn as_any(&self) -> &dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_builder::GenericByteBuilder", "path": "GenericByteBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [282, 1], "end": [312, 2], "filename": "src/builder/generic_bytes_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/generic_bytes_builder.rs:299`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the builder as a non-mutable `Any` reference.

<a id="op-d5a95c3b884fac55623285a7"></a>
## as_any_mut

`function` · `arrow_array::builder::generic_bytes_builder::GenericByteBuilder::as_any_mut` · arrow-array 59.3.0

```rust
fn as_any_mut(&mut self) -> &mut dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_builder::GenericByteBuilder", "path": "GenericByteBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [282, 1], "end": [312, 2], "filename": "src/builder/generic_bytes_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/generic_bytes_builder.rs:304`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the builder as a mutable `Any` reference.

<a id="op-7356576c06f7da6905151d17"></a>
## default

`function` · `arrow_array::builder::generic_bytes_builder::GenericByteBuilder::default` · arrow-array 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_builder::GenericByteBuilder", "path": "GenericByteBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [276, 1], "end": [280, 2], "filename": "src/builder/generic_bytes_builder.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/builder/generic_bytes_builder.rs:277`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5d39c3b7cbffac8dc8510433"></a>
## extend

`function` · `arrow_array::builder::generic_bytes_builder::GenericByteBuilder::extend` · arrow-array 59.3.0

```rust
fn extend<I: IntoIterator<Item = Option<V>>>(&mut self, iter: I)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_builder::GenericByteBuilder", "path": "GenericByteBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": ""}}}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [314, 1], "end": [321, 2], "filename": "src/builder/generic_bytes_builder.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "V"}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "core::iter::traits::collect::Extend", "path": "Extend"}, "trait_path": "core::iter::traits::collect::Extend"}`

Source: `src/builder/generic_bytes_builder.rs:316`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6b92d3b7fd1a8f4607053568"></a>
## finish

`function` · `arrow_array::builder::generic_bytes_builder::GenericByteBuilder::finish` · arrow-array 59.3.0

```rust
fn finish(&mut self) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_builder::GenericByteBuilder", "path": "GenericByteBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [282, 1], "end": [312, 2], "filename": "src/builder/generic_bytes_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/generic_bytes_builder.rs:289`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the array and reset this builder.

<a id="op-c82a5ad69a5d6800b7b8145c"></a>
## finish

`function` · `arrow_array::builder::generic_bytes_builder::GenericByteBuilder::finish` · arrow-array 59.3.0

```rust
fn finish(&mut self) -> GenericByteArray<T>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_builder::GenericByteBuilder", "path": "GenericByteBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [263, 2], "filename": "src/builder/generic_bytes_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_builder.rs:201`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the [`GenericByteArray`](../operations/arrow_array.array.byte_array.GenericByteArray.md#op-e39e3ecbe5a4126397f47443) and reset this builder.

<a id="op-60ecc44e78def39f294301ec"></a>
## finish_cloned

`function` · `arrow_array::builder::generic_bytes_builder::GenericByteBuilder::finish_cloned` · arrow-array 59.3.0

```rust
fn finish_cloned(&self) -> GenericByteArray<T>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_builder::GenericByteBuilder", "path": "GenericByteBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [263, 2], "filename": "src/builder/generic_bytes_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_builder.rs:215`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the [`GenericByteArray`](../operations/arrow_array.array.byte_array.GenericByteArray.md#op-e39e3ecbe5a4126397f47443) without resetting the builder.

<a id="op-de57ae28494d647213b80716"></a>
## finish_cloned

`function` · `arrow_array::builder::generic_bytes_builder::GenericByteBuilder::finish_cloned` · arrow-array 59.3.0

```rust
fn finish_cloned(&self) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_builder::GenericByteBuilder", "path": "GenericByteBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [282, 1], "end": [312, 2], "filename": "src/builder/generic_bytes_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/generic_bytes_builder.rs:294`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the array without resetting the builder.

<a id="op-41f2aae87243bf7e2a4a747c"></a>
## fmt

`function` · `arrow_array::builder::generic_bytes_builder::GenericByteBuilder::fmt` · arrow-array 59.3.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_builder::GenericByteBuilder", "path": "GenericByteBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [265, 1], "end": [274, 2], "filename": "src/builder/generic_bytes_builder.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/builder/generic_bytes_builder.rs:266`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a8c44e4878385860b23f88f7"></a>
## into_box_any

`function` · `arrow_array::builder::generic_bytes_builder::GenericByteBuilder::into_box_any` · arrow-array 59.3.0

```rust
fn into_box_any(Box<self>) -> Box<dyn Any>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_builder::GenericByteBuilder", "path": "GenericByteBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [282, 1], "end": [312, 2], "filename": "src/builder/generic_bytes_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/generic_bytes_builder.rs:309`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the boxed builder as a box of `Any`.

<a id="op-c0bbf80af389006cd6f80a3a"></a>
## len

`function` · `arrow_array::builder::generic_bytes_builder::GenericByteBuilder::len` · arrow-array 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_builder::GenericByteBuilder", "path": "GenericByteBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [282, 1], "end": [312, 2], "filename": "src/builder/generic_bytes_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/generic_bytes_builder.rs:284`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the number of binary slots in the builder

<a id="op-5906761d3ff86c2fbc3d091d"></a>
## new

`function` · `arrow_array::builder::generic_bytes_builder::GenericByteBuilder::new` · arrow-array 59.3.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_builder::GenericByteBuilder", "path": "GenericByteBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [263, 2], "filename": "src/builder/generic_bytes_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_builder.rs:39`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new [`GenericByteBuilder`](../operations/arrow_array.builder.generic_bytes_builder.GenericByteBuilder.md#op-647a230051e61dc574fafd16).

<a id="op-29ed8985d2e3d6d1257727d7"></a>
## new_from_buffer

`function` · `arrow_array::builder::generic_bytes_builder::GenericByteBuilder::new_from_buffer` · arrow-array 59.3.0

```rust
unsafe fn new_from_buffer(offsets_buffer: MutableBuffer, value_buffer: MutableBuffer, null_buffer: Option<MutableBuffer>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_builder::GenericByteBuilder", "path": "GenericByteBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [263, 2], "filename": "src/builder/generic_bytes_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_builder.rs:65`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new  [`GenericByteBuilder`](../operations/arrow_array.builder.generic_bytes_builder.GenericByteBuilder.md#op-647a230051e61dc574fafd16) from buffers.

# Safety

This doesn't verify buffer contents as it assumes the buffers are from
existing and valid [`GenericByteArray`](../operations/arrow_array.array.byte_array.GenericByteArray.md#op-e39e3ecbe5a4126397f47443).

<a id="op-a198c65908058e56f9299832"></a>
## offsets_capacity

`function` · `arrow_array::builder::generic_bytes_builder::GenericByteBuilder::offsets_capacity` · arrow-array 59.3.0

```rust
fn offsets_capacity(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_builder::GenericByteBuilder", "path": "GenericByteBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [263, 2], "filename": "src/builder/generic_bytes_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_builder.rs:245`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the current offsets buffer capacity, in offsets.

<a id="op-441f6f0e029cc91127755d9f"></a>
## offsets_slice

`function` · `arrow_array::builder::generic_bytes_builder::GenericByteBuilder::offsets_slice` · arrow-array 59.3.0

```rust
fn offsets_slice(&self) -> &[T::Offset]
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_builder::GenericByteBuilder", "path": "GenericByteBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [263, 2], "filename": "src/builder/generic_bytes_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_builder.rs:240`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the current offsets buffer as a slice

<a id="op-cb2a593ddb996caf25632a49"></a>
## validity_capacity

`function` · `arrow_array::builder::generic_bytes_builder::GenericByteBuilder::validity_capacity` · arrow-array 59.3.0

```rust
fn validity_capacity(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_builder::GenericByteBuilder", "path": "GenericByteBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [263, 2], "filename": "src/builder/generic_bytes_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_builder.rs:255`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the current null buffer allocated capacity, in bytes.

<a id="op-c9dab42d0cbd77c4c13894e2"></a>
## validity_slice

`function` · `arrow_array::builder::generic_bytes_builder::GenericByteBuilder::validity_slice` · arrow-array 59.3.0

```rust
fn validity_slice(&self) -> Option<&[u8]>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_builder::GenericByteBuilder", "path": "GenericByteBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [263, 2], "filename": "src/builder/generic_bytes_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_builder.rs:250`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the current null buffer as a slice

<a id="op-2d044d642b7f0d4bec4b382b"></a>
## validity_slice_mut

`function` · `arrow_array::builder::generic_bytes_builder::GenericByteBuilder::validity_slice_mut` · arrow-array 59.3.0

```rust
fn validity_slice_mut(&mut self) -> Option<&mut [u8]>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_builder::GenericByteBuilder", "path": "GenericByteBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [263, 2], "filename": "src/builder/generic_bytes_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_builder.rs:260`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the current null buffer as a mutable slice

<a id="op-154542e0be2a96822181bd96"></a>
## values_capacity

`function` · `arrow_array::builder::generic_bytes_builder::GenericByteBuilder::values_capacity` · arrow-array 59.3.0

```rust
fn values_capacity(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_builder::GenericByteBuilder", "path": "GenericByteBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [263, 2], "filename": "src/builder/generic_bytes_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_builder.rs:235`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the current values buffer capacity, in bytes.

<a id="op-eff9089e8f0f46b073b45f41"></a>
## values_slice

`function` · `arrow_array::builder::generic_bytes_builder::GenericByteBuilder::values_slice` · arrow-array 59.3.0

```rust
fn values_slice(&self) -> &[u8]
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_builder::GenericByteBuilder", "path": "GenericByteBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [263, 2], "filename": "src/builder/generic_bytes_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_builder.rs:230`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the current values buffer as a slice

<a id="op-6f35b1452c5cbe6bd220bd2c"></a>
## with_capacity

`function` · `arrow_array::builder::generic_bytes_builder::GenericByteBuilder::with_capacity` · arrow-array 59.3.0

```rust
fn with_capacity(item_capacity: usize, data_capacity: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_builder::GenericByteBuilder", "path": "GenericByteBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [263, 2], "filename": "src/builder/generic_bytes_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_builder.rs:49`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new [`GenericByteBuilder`](../operations/arrow_array.builder.generic_bytes_builder.GenericByteBuilder.md#op-647a230051e61dc574fafd16).

- `item_capacity` is the number of items to pre-allocate.
  The size of the preallocated buffer of offsets is the number of items plus one.
- `data_capacity` is the total number of bytes of data to pre-allocate
  (for all items, not per item).
