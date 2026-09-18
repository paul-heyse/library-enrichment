# `arrow_array::builder::fixed_size_binary_builder::FixedSizeBinaryBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.builder.fixed_size_binary_builder.FixedSizeBinaryBuilder.json).

<a id="op-682486c27bd2030ff9d6d0b9"></a>
## FixedSizeBinaryBuilder

`struct` · `arrow_array::builder::fixed_size_binary_builder::FixedSizeBinaryBuilder` · arrow-array 59.3.0

```rust
struct FixedSizeBinaryBuilder
```

Source: `src/builder/fixed_size_binary_builder.rs:45`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builder for [`FixedSizeBinaryArray`](../operations/arrow_array.array.fixed_size_binary_array.FixedSizeBinaryArray.md#op-9ede10c8076f692a67198c02)
```
# use arrow_array::builder::FixedSizeBinaryBuilder;
# use arrow_array::Array;
#
let mut builder = FixedSizeBinaryBuilder::with_capacity(3, 5);
// [b"hello", null, b"arrow"]
builder.append_value(b"hello").unwrap();
builder.append_null();
builder.append_value(b"arrow").unwrap();

let array = builder.finish();
assert_eq!(array.value(0), b"hello");
assert!(array.is_null(1));
assert_eq!(array.value(2), b"arrow");
```

<a id="op-df6a7795ea6536718bc8e7dc"></a>
## append_array

`function` · `arrow_array::builder::fixed_size_binary_builder::FixedSizeBinaryBuilder::append_array` · arrow-array 59.3.0

```rust
fn append_array(&mut self, array: &FixedSizeBinaryArray) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::fixed_size_binary_builder::FixedSizeBinaryBuilder", "path": "FixedSizeBinaryBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 1], "end": [154, 2], "filename": "src/builder/fixed_size_binary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/fixed_size_binary_builder.rs:106`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends all elements in array into the builder.

<a id="op-ba40b62ea6d50c82463e7873"></a>
## append_null

`function` · `arrow_array::builder::fixed_size_binary_builder::FixedSizeBinaryBuilder::append_null` · arrow-array 59.3.0

```rust
fn append_null(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::fixed_size_binary_builder::FixedSizeBinaryBuilder", "path": "FixedSizeBinaryBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 1], "end": [154, 2], "filename": "src/builder/fixed_size_binary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/fixed_size_binary_builder.rs:91`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Append a null value to the array.

<a id="op-9cef9fd4102eefa14ea2b05e"></a>
## append_nulls

`function` · `arrow_array::builder::fixed_size_binary_builder::FixedSizeBinaryBuilder::append_nulls` · arrow-array 59.3.0

```rust
fn append_nulls(&mut self, n: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::fixed_size_binary_builder::FixedSizeBinaryBuilder", "path": "FixedSizeBinaryBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 1], "end": [154, 2], "filename": "src/builder/fixed_size_binary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/fixed_size_binary_builder.rs:99`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends `n` `null`s into the builder.

<a id="op-885f4f9d12261fda60feb7d8"></a>
## append_value

`function` · `arrow_array::builder::fixed_size_binary_builder::FixedSizeBinaryBuilder::append_value` · arrow-array 59.3.0

```rust
fn append_value(&mut self, value: impl AsRef<[u8]>) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::fixed_size_binary_builder::FixedSizeBinaryBuilder", "path": "FixedSizeBinaryBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 1], "end": [154, 2], "filename": "src/builder/fixed_size_binary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/fixed_size_binary_builder.rs:76`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends a byte slice into the builder.

Automatically update the null buffer to delimit the slice appended in as a
distinct value element.

<a id="op-a3de6aad7174ee4e50cde9b1"></a>
## as_any

`function` · `arrow_array::builder::fixed_size_binary_builder::FixedSizeBinaryBuilder::as_any` · arrow-array 59.3.0

```rust
fn as_any(&self) -> &dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::fixed_size_binary_builder::FixedSizeBinaryBuilder", "path": "FixedSizeBinaryBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [156, 1], "end": [186, 2], "filename": "src/builder/fixed_size_binary_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/fixed_size_binary_builder.rs:158`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the builder as a non-mutable `Any` reference.

<a id="op-21ffb75299760ec6f6f9c987"></a>
## as_any_mut

`function` · `arrow_array::builder::fixed_size_binary_builder::FixedSizeBinaryBuilder::as_any_mut` · arrow-array 59.3.0

```rust
fn as_any_mut(&mut self) -> &mut dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::fixed_size_binary_builder::FixedSizeBinaryBuilder", "path": "FixedSizeBinaryBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [156, 1], "end": [186, 2], "filename": "src/builder/fixed_size_binary_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/fixed_size_binary_builder.rs:163`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the builder as a mutable `Any` reference.

<a id="op-7db1b24ac439e2470ebc52ad"></a>
## finish

`function` · `arrow_array::builder::fixed_size_binary_builder::FixedSizeBinaryBuilder::finish` · arrow-array 59.3.0

```rust
fn finish(&mut self) -> FixedSizeBinaryArray
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::fixed_size_binary_builder::FixedSizeBinaryBuilder", "path": "FixedSizeBinaryBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 1], "end": [154, 2], "filename": "src/builder/fixed_size_binary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/fixed_size_binary_builder.rs:128`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the [`FixedSizeBinaryArray`](../operations/arrow_array.array.fixed_size_binary_array.FixedSizeBinaryArray.md#op-9ede10c8076f692a67198c02) and reset this builder.

<a id="op-b8acf63681b165b3266f2eba"></a>
## finish

`function` · `arrow_array::builder::fixed_size_binary_builder::FixedSizeBinaryBuilder::finish` · arrow-array 59.3.0

```rust
fn finish(&mut self) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::fixed_size_binary_builder::FixedSizeBinaryBuilder", "path": "FixedSizeBinaryBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [156, 1], "end": [186, 2], "filename": "src/builder/fixed_size_binary_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/fixed_size_binary_builder.rs:178`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the array and reset this builder.

<a id="op-0f148ec7ecfee20d0177c4ef"></a>
## finish_cloned

`function` · `arrow_array::builder::fixed_size_binary_builder::FixedSizeBinaryBuilder::finish_cloned` · arrow-array 59.3.0

```rust
fn finish_cloned(&self) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::fixed_size_binary_builder::FixedSizeBinaryBuilder", "path": "FixedSizeBinaryBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [156, 1], "end": [186, 2], "filename": "src/builder/fixed_size_binary_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/fixed_size_binary_builder.rs:183`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the array without resetting the builder.

<a id="op-7a130ea97dd094872e85780c"></a>
## finish_cloned

`function` · `arrow_array::builder::fixed_size_binary_builder::FixedSizeBinaryBuilder::finish_cloned` · arrow-array 59.3.0

```rust
fn finish_cloned(&self) -> FixedSizeBinaryArray
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::fixed_size_binary_builder::FixedSizeBinaryBuilder", "path": "FixedSizeBinaryBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 1], "end": [154, 2], "filename": "src/builder/fixed_size_binary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/fixed_size_binary_builder.rs:139`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the [`FixedSizeBinaryArray`](../operations/arrow_array.array.fixed_size_binary_array.FixedSizeBinaryArray.md#op-9ede10c8076f692a67198c02) without resetting the builder.

<a id="op-edc998c3e4093cd67a0d803f"></a>
## fmt

`function` · `arrow_array::builder::fixed_size_binary_builder::FixedSizeBinaryBuilder::fmt` · arrow-array 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::fixed_size_binary_builder::FixedSizeBinaryBuilder", "path": "FixedSizeBinaryBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 10], "end": [44, 15], "filename": "src/builder/fixed_size_binary_builder.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/builder/fixed_size_binary_builder.rs:44`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bdd88a424295a6eba7dec43d"></a>
## into_box_any

`function` · `arrow_array::builder::fixed_size_binary_builder::FixedSizeBinaryBuilder::into_box_any` · arrow-array 59.3.0

```rust
fn into_box_any(Box<self>) -> Box<dyn Any>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::fixed_size_binary_builder::FixedSizeBinaryBuilder", "path": "FixedSizeBinaryBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [156, 1], "end": [186, 2], "filename": "src/builder/fixed_size_binary_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/fixed_size_binary_builder.rs:168`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the boxed builder as a box of `Any`.

<a id="op-32a1f8881640709a5731cc31"></a>
## len

`function` · `arrow_array::builder::fixed_size_binary_builder::FixedSizeBinaryBuilder::len` · arrow-array 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::fixed_size_binary_builder::FixedSizeBinaryBuilder", "path": "FixedSizeBinaryBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [156, 1], "end": [186, 2], "filename": "src/builder/fixed_size_binary_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/fixed_size_binary_builder.rs:173`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the number of array slots in the builder

<a id="op-3ab3e13586c8d1d5bafff3a3"></a>
## new

`function` · `arrow_array::builder::fixed_size_binary_builder::FixedSizeBinaryBuilder::new` · arrow-array 59.3.0

```rust
fn new(byte_width: i32) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::fixed_size_binary_builder::FixedSizeBinaryBuilder", "path": "FixedSizeBinaryBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 1], "end": [154, 2], "filename": "src/builder/fixed_size_binary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/fixed_size_binary_builder.rs:53`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new [`FixedSizeBinaryBuilder`](../operations/arrow_array.builder.fixed_size_binary_builder.FixedSizeBinaryBuilder.md#op-682486c27bd2030ff9d6d0b9)

<a id="op-516e9c1f9fe6ac93ed0ac6e1"></a>
## validity_slice

`function` · `arrow_array::builder::fixed_size_binary_builder::FixedSizeBinaryBuilder::validity_slice` · arrow-array 59.3.0

```rust
fn validity_slice(&self) -> Option<&[u8]>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::fixed_size_binary_builder::FixedSizeBinaryBuilder", "path": "FixedSizeBinaryBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 1], "end": [154, 2], "filename": "src/builder/fixed_size_binary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/fixed_size_binary_builder.rs:151`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the current null buffer as a slice

<a id="op-53304e49f329b888a8d33584"></a>
## values_slice

`function` · `arrow_array::builder::fixed_size_binary_builder::FixedSizeBinaryBuilder::values_slice` · arrow-array 59.3.0

```rust
fn values_slice(&self) -> &[u8]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::fixed_size_binary_builder::FixedSizeBinaryBuilder", "path": "FixedSizeBinaryBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 1], "end": [154, 2], "filename": "src/builder/fixed_size_binary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/fixed_size_binary_builder.rs:123`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the current values buffer as a slice

<a id="op-08facee7007f82cac04ca98d"></a>
## with_capacity

`function` · `arrow_array::builder::fixed_size_binary_builder::FixedSizeBinaryBuilder::with_capacity` · arrow-array 59.3.0

```rust
fn with_capacity(capacity: usize, byte_width: i32) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::fixed_size_binary_builder::FixedSizeBinaryBuilder", "path": "FixedSizeBinaryBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 1], "end": [154, 2], "filename": "src/builder/fixed_size_binary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/fixed_size_binary_builder.rs:59`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new [`FixedSizeBinaryBuilder`](../operations/arrow_array.builder.fixed_size_binary_builder.FixedSizeBinaryBuilder.md#op-682486c27bd2030ff9d6d0b9), `capacity` is the number of byte slices
that can be appended without reallocating
