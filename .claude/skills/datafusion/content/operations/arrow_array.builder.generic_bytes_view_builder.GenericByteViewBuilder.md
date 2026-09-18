# `arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.builder.generic_bytes_view_builder.GenericByteViewBuilder.json).

<a id="op-99a90723608a79969b813022"></a>
## GenericByteViewBuilder

`struct` · `arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder` · arrow-array 59.3.0

```rust
struct GenericByteViewBuilder<T: ByteViewType + ?Sized>
```

Source: `src/builder/generic_bytes_view_builder.rs:81`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A builder for [`GenericByteViewArray`](../operations/arrow_array.array.byte_view_array.GenericByteViewArray.md#op-af313e332b96011fb72a0226)

A [`GenericByteViewArray`](../operations/arrow_array.array.byte_view_array.GenericByteViewArray.md#op-af313e332b96011fb72a0226) consists of a list of data blocks containing string data,
and a list of views into those buffers.

See examples on [`StringViewBuilder`](../operations/arrow_array.builder.generic_bytes_view_builder.StringViewBuilder.md#op-dd25f73e404dcf18f7926e36) and [`BinaryViewBuilder`](../operations/arrow_array.builder.generic_bytes_view_builder.BinaryViewBuilder.md#op-e2f59013d1dccace932df585)

This builder can be used in two ways

# Append Values

To avoid bump allocating, this builder allocates data in fixed size blocks, configurable
using [`GenericByteViewBuilder::with_fixed_block_size`](../operations/arrow_array.builder.generic_bytes_view_builder.GenericByteViewBuilder.md#op-077084c5969596193df3a93c). [`GenericByteViewBuilder::append_value`](../operations/arrow_array.builder.generic_bytes_view_builder.GenericByteViewBuilder.md#op-912730ad0d01774730091567)
writes values larger than [`MAX_INLINE_VIEW_LEN`](../operations/arrow_data.byte_view.MAX_INLINE_VIEW_LEN.md#op-cc5752925eeccf8fdeefe7ed) bytes to the current in-progress block, with values smaller
than [`MAX_INLINE_VIEW_LEN`](../operations/arrow_data.byte_view.MAX_INLINE_VIEW_LEN.md#op-cc5752925eeccf8fdeefe7ed) bytes inlined into the views. If a value is appended that will not fit in the
in-progress block, it will be closed, and a new block of sufficient size allocated

# Append Views

Some use-cases may wish to reuse an existing allocation containing string data, for example,
when parsing data from a parquet data page. In such a case entire blocks can be appended
using [`GenericByteViewBuilder::append_block`](../operations/arrow_array.builder.generic_bytes_view_builder.GenericByteViewBuilder.md#op-f7319e533e0197315872c988) and then views into this block appended
using [`GenericByteViewBuilder::try_append_view`](../operations/arrow_array.builder.generic_bytes_view_builder.GenericByteViewBuilder.md#op-4de6e7823589907000f7301e)

<a id="op-872032454c77e439535c7bfc"></a>
## allocated_size

`function` · `arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder::allocated_size` · arrow-array 59.3.0

```rust
fn allocated_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder", "path": "GenericByteViewBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 1], "end": [531, 2], "filename": "src/builder/generic_bytes_view_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_view_builder.rs:520`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Return the allocated size of this builder in bytes, useful for memory accounting.

<a id="op-300c97f5382b2c4c59096f05"></a>
## append_array

`function` · `arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder::append_array` · arrow-array 59.3.0

```rust
fn append_array(&mut self, array: &GenericByteViewArray<T>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder", "path": "GenericByteViewBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 1], "end": [531, 2], "filename": "src/builder/generic_bytes_view_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_view_builder.rs:223`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends an array to the builder.
This will flush any in-progress block and append the data buffers
and add the (adapted) views.

<a id="op-f7319e533e0197315872c988"></a>
## append_block

`function` · `arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder::append_block` · arrow-array 59.3.0

```rust
fn append_block(&mut self, buffer: Buffer) -> u32
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder", "path": "GenericByteViewBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 1], "end": [531, 2], "filename": "src/builder/generic_bytes_view_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_view_builder.rs:194`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Append a new data block returning the new block offset

Note: this will first flush any in-progress block

This allows appending views from blocks added using [`Self::append_block`](../operations/arrow_array.builder.generic_bytes_view_builder.GenericByteViewBuilder.md#op-f7319e533e0197315872c988). See
[`Self::append_value`](../operations/arrow_array.builder.generic_bytes_view_builder.GenericByteViewBuilder.md#op-912730ad0d01774730091567) for appending individual values

```
# use arrow_array::builder::StringViewBuilder;
let mut builder = StringViewBuilder::new();

let block = builder.append_block(b"helloworldbingobongo".into());

builder.try_append_view(block, 0, 5).unwrap();
builder.try_append_view(block, 5, 5).unwrap();
builder.try_append_view(block, 10, 5).unwrap();
builder.try_append_view(block, 15, 5).unwrap();
builder.try_append_view(block, 0, 15).unwrap();
let array = builder.finish();

let actual: Vec<_> = array.iter().flatten().collect();
let expected = &["hello", "world", "bingo", "bongo", "helloworldbingo"];
assert_eq!(actual, expected);
```

<a id="op-4095e95bee313f6075773ea1"></a>
## append_null

`function` · `arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder::append_null` · arrow-array 59.3.0

```rust
fn append_null(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder", "path": "GenericByteViewBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 1], "end": [531, 2], "filename": "src/builder/generic_bytes_view_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_view_builder.rs:482`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Append a null value into the builder

<a id="op-1723f4b2745ee29426468281"></a>
## append_option

`function` · `arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder::append_option` · arrow-array 59.3.0

```rust
fn append_option(&mut self, value: Option<impl AsRef<T::Native>>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder", "path": "GenericByteViewBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 1], "end": [531, 2], "filename": "src/builder/generic_bytes_view_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_view_builder.rs:426`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Append an `Option` value into the builder

<a id="op-912730ad0d01774730091567"></a>
## append_value

`function` · `arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder::append_value` · arrow-array 59.3.0

```rust
fn append_value(&mut self, value: impl AsRef<T::Native>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder", "path": "GenericByteViewBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 1], "end": [531, 2], "filename": "src/builder/generic_bytes_view_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_view_builder.rs:327`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends a value into the builder

# Panics

Panics if
- String buffer count exceeds `u32::MAX`
- String length exceeds `u32::MAX`

<a id="op-da6f834a782708776df20c14"></a>
## append_view_unchecked

`function` · `arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder::append_view_unchecked` · arrow-array 59.3.0

```rust
unsafe fn append_view_unchecked(&mut self, block: u32, offset: u32, len: u32)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder", "path": "GenericByteViewBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 1], "end": [531, 2], "filename": "src/builder/generic_bytes_view_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_view_builder.rs:209`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Append a view of the given `block`, `offset` and `length`

# Safety
(1) The block must have been added using [`Self::append_block`](../operations/arrow_array.builder.generic_bytes_view_builder.GenericByteViewBuilder.md#op-f7319e533e0197315872c988)
(2) The range `offset..offset+length` must be within the bounds of the block
(3) The data in the block must be valid of type `T`

<a id="op-468daadf30c4a7b10f18a988"></a>
## as_any

`function` · `arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder::as_any` · arrow-array 59.3.0

```rust
fn as_any(&self) -> &dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder", "path": "GenericByteViewBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 1], "end": [575, 2], "filename": "src/builder/generic_bytes_view_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/generic_bytes_view_builder.rs:564`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a13a1e0f79e6e525c198fa0d"></a>
## as_any_mut

`function` · `arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder::as_any_mut` · arrow-array 59.3.0

```rust
fn as_any_mut(&mut self) -> &mut dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder", "path": "GenericByteViewBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 1], "end": [575, 2], "filename": "src/builder/generic_bytes_view_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/generic_bytes_view_builder.rs:568`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-85fcdd2ba3435bb7a85085f7"></a>
## default

`function` · `arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder::default` · arrow-array 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder", "path": "GenericByteViewBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [533, 1], "end": [537, 2], "filename": "src/builder/generic_bytes_view_builder.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/builder/generic_bytes_view_builder.rs:534`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-392dad2447364c2deeab65bf"></a>
## extend

`function` · `arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder::extend` · arrow-array 59.3.0

```rust
fn extend<I: IntoIterator<Item = Option<V>>>(&mut self, iter: I)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder", "path": "GenericByteViewBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": ""}}}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [577, 1], "end": [586, 2], "filename": "src/builder/generic_bytes_view_builder.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "V"}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "core::iter::traits::collect::Extend", "path": "Extend"}, "trait_path": "core::iter::traits::collect::Extend"}`

Source: `src/builder/generic_bytes_view_builder.rs:581`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-40c7c4d2a67cb20f095c5144"></a>
## finish

`function` · `arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder::finish` · arrow-array 59.3.0

```rust
fn finish(&mut self) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder", "path": "GenericByteViewBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 1], "end": [575, 2], "filename": "src/builder/generic_bytes_view_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/generic_bytes_view_builder.rs:556`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4a4c75b172ae199c22ffd01a"></a>
## finish

`function` · `arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder::finish` · arrow-array 59.3.0

```rust
fn finish(&mut self) -> GenericByteViewArray<T>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder", "path": "GenericByteViewBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 1], "end": [531, 2], "filename": "src/builder/generic_bytes_view_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_view_builder.rs:488`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the [`GenericByteViewArray`](../operations/arrow_array.array.byte_view_array.GenericByteViewArray.md#op-af313e332b96011fb72a0226) and reset this builder

<a id="op-99aee95b148f7e61b3fb6c74"></a>
## finish_cloned

`function` · `arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder::finish_cloned` · arrow-array 59.3.0

```rust
fn finish_cloned(&self) -> GenericByteViewArray<T>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder", "path": "GenericByteViewBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 1], "end": [531, 2], "filename": "src/builder/generic_bytes_view_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_view_builder.rs:501`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the [`GenericByteViewArray`](../operations/arrow_array.array.byte_view_array.GenericByteViewArray.md#op-af313e332b96011fb72a0226) without resetting the builder

<a id="op-9bcc08ef577b8a6ea93cfa20"></a>
## finish_cloned

`function` · `arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder::finish_cloned` · arrow-array 59.3.0

```rust
fn finish_cloned(&self) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder", "path": "GenericByteViewBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 1], "end": [575, 2], "filename": "src/builder/generic_bytes_view_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/generic_bytes_view_builder.rs:560`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-48869486f4bcec629947a0ea"></a>
## fmt

`function` · `arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder::fmt` · arrow-array 59.3.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder", "path": "GenericByteViewBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [539, 1], "end": [549, 2], "filename": "src/builder/generic_bytes_view_builder.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/builder/generic_bytes_view_builder.rs:540`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a643e0988fe6c3950231f416"></a>
## get_value

`function` · `arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder::get_value` · arrow-array 59.3.0

```rust
fn get_value(&self, index: usize) -> &[u8]
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder", "path": "GenericByteViewBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 1], "end": [531, 2], "filename": "src/builder/generic_bytes_view_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_view_builder.rs:301`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the value at the given index
Useful if we want to know what value has been inserted to the builder
The index has to be smaller than `self.len()`, otherwise it will panic

<a id="op-6e5b3ccfaf155ca7429a6155"></a>
## into_box_any

`function` · `arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder::into_box_any` · arrow-array 59.3.0

```rust
fn into_box_any(Box<self>) -> Box<dyn Any>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder", "path": "GenericByteViewBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 1], "end": [575, 2], "filename": "src/builder/generic_bytes_view_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/generic_bytes_view_builder.rs:572`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9f2edbaa4896f545a42975a0"></a>
## len

`function` · `arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder::len` · arrow-array 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder", "path": "GenericByteViewBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 1], "end": [575, 2], "filename": "src/builder/generic_bytes_view_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/generic_bytes_view_builder.rs:552`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-26279836c719fd02fb3d21c8"></a>
## new

`function` · `arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder::new` · arrow-array 59.3.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder", "path": "GenericByteViewBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 1], "end": [531, 2], "filename": "src/builder/generic_bytes_view_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_view_builder.rs:96`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new [`GenericByteViewBuilder`](../operations/arrow_array.builder.generic_bytes_view_builder.GenericByteViewBuilder.md#op-99a90723608a79969b813022).

<a id="op-812fccaeaf73e3760fba476b"></a>
## try_append_value

`function` · `arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder::try_append_value` · arrow-array 59.3.0

```rust
fn try_append_value(&mut self, value: impl AsRef<T::Native>) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder", "path": "GenericByteViewBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 1], "end": [531, 2], "filename": "src/builder/generic_bytes_view_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_view_builder.rs:339`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends a value into the builder

# Errors

Returns an error if:
- String buffer count exceeds `u32::MAX`
- String length exceeds `u32::MAX`

<a id="op-054cfbc04f081da4e024249d"></a>
## try_append_value_n

`function` · `arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder::try_append_value_n` · arrow-array 59.3.0

```rust
fn try_append_value_n(&mut self, value: impl AsRef<T::Native>, n: usize) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder", "path": "GenericByteViewBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 1], "end": [531, 2], "filename": "src/builder/generic_bytes_view_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_view_builder.rs:463`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Append the same value `n` times into the builder

This is more efficient than calling [`Self::try_append_value`](../operations/arrow_array.builder.generic_bytes_view_builder.GenericByteViewBuilder.md#op-812fccaeaf73e3760fba476b) `n` times,
especially when deduplication is enabled, as it only hashes the value once.

# Errors

Returns an error if
- String buffer count exceeds `u32::MAX`
- String length exceeds `u32::MAX`

# Example
```
# use arrow_array::builder::StringViewBuilder;
# use arrow_array::Array;
let mut builder = StringViewBuilder::new().with_deduplicate_strings();

// Append "hello" 1000 times efficiently
builder.try_append_value_n("hello", 1000)?;

let array = builder.finish();
assert_eq!(array.len(), 1000);

// All values are "hello"
for value in array.iter() {
    assert_eq!(value, Some("hello"));
}
# Ok::<(), arrow_schema::ArrowError>(())
```

<a id="op-4de6e7823589907000f7301e"></a>
## try_append_view

`function` · `arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder::try_append_view` · arrow-array 59.3.0

```rust
fn try_append_view(&mut self, block: u32, offset: u32, len: u32) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder", "path": "GenericByteViewBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 1], "end": [531, 2], "filename": "src/builder/generic_bytes_view_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_view_builder.rs:255`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Try to append a view of the given `block`, `offset` and `length`

See [`Self::append_block`](../operations/arrow_array.builder.generic_bytes_view_builder.GenericByteViewBuilder.md#op-f7319e533e0197315872c988)

<a id="op-1c7d9964a1671432adfa16b2"></a>
## validity_slice

`function` · `arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder::validity_slice` · arrow-array 59.3.0

```rust
fn validity_slice(&self) -> Option<&[u8]>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder", "path": "GenericByteViewBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 1], "end": [531, 2], "filename": "src/builder/generic_bytes_view_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_view_builder.rs:515`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the current null buffer as a slice

<a id="op-a3a6f26257e126b9e3b5fca3"></a>
## with_capacity

`function` · `arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder::with_capacity` · arrow-array 59.3.0

```rust
fn with_capacity(capacity: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder", "path": "GenericByteViewBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 1], "end": [531, 2], "filename": "src/builder/generic_bytes_view_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_view_builder.rs:101`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new [`GenericByteViewBuilder`](../operations/arrow_array.builder.generic_bytes_view_builder.GenericByteViewBuilder.md#op-99a90723608a79969b813022) with space for `capacity` string values.

<a id="op-1ffb842587c4b7fab00361d4"></a>
## with_deduplicate_strings

`function` · `arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder::with_deduplicate_strings` · arrow-array 59.3.0

```rust
fn with_deduplicate_strings(self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder", "path": "GenericByteViewBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 1], "end": [531, 2], "filename": "src/builder/generic_bytes_view_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_view_builder.rs:160`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Deduplicate strings while building the array

This will potentially decrease the memory usage if the array have repeated strings
It will also increase the time to build the array as it needs to hash the strings

<a id="op-077084c5969596193df3a93c"></a>
## with_fixed_block_size

`function` · `arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder::with_fixed_block_size` · arrow-array 59.3.0

```rust
fn with_fixed_block_size(self, block_size: u32) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder", "path": "GenericByteViewBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 1], "end": [531, 2], "filename": "src/builder/generic_bytes_view_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_view_builder.rs:148`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Set a fixed buffer size for variable length strings

The block size is the size of the buffer used to store values greater
than [`MAX_INLINE_VIEW_LEN`](../operations/arrow_data.byte_view.MAX_INLINE_VIEW_LEN.md#op-cc5752925eeccf8fdeefe7ed) bytes. The builder allocates new buffers when the current
buffer is full.

By default the builder balances buffer size and buffer count by
growing buffer size exponentially from 8KB up to 2MB. The
first buffer allocated is 8KB, then 16KB, then 32KB, etc up to 2MB.

If this method is used, any new buffers allocated are
exactly this size. This can be useful for advanced users
that want to control the memory usage and buffer count.

See <https://github.com/apache/arrow-rs/issues/6094> for more details on the implications.

<a id="op-f82f88df922db84e3ec899eb"></a>
## with_max_deduplication_len

`function` · `arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder::with_max_deduplication_len` · arrow-array 59.3.0

```rust
fn with_max_deduplication_len(self, max_deduplication_len: u32) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder", "path": "GenericByteViewBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 1], "end": [531, 2], "filename": "src/builder/generic_bytes_view_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_bytes_view_builder.rs:122`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Configure max deduplication length when deduplicating strings while building the array.
Default is None.

When [`Self::with_deduplicate_strings`](../operations/arrow_array.builder.generic_bytes_view_builder.GenericByteViewBuilder.md#op-1ffb842587c4b7fab00361d4) is enabled, the builder attempts to deduplicate
any strings longer than 12 bytes. However, since it takes time proportional to the length
of the string to deduplicate, setting this option limits the CPU overhead for this option.  
