# `arrow_array::builder::generic_byte_run_builder::GenericByteRunBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.builder.generic_byte_run_builder.GenericByteRunBuilder.json).

<a id="op-24a011c2948c766aabe58bb5"></a>
## GenericByteRunBuilder

`struct` · `arrow_array::builder::generic_byte_run_builder::GenericByteRunBuilder` · arrow-array 59.3.0

```rust
struct GenericByteRunBuilder<R, V> where R: ArrowPrimitiveType, V: ByteArrayType
```

Source: `src/builder/generic_byte_run_builder.rs:65`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builder for [`RunArray`](../operations/arrow_array.array.run_array.RunArray.md#op-f0742b2d9e045922a076478f) of [`GenericByteArray`](crate::array::GenericByteArray)

# Example:

```

# use arrow_array::builder::GenericByteRunBuilder;
# use arrow_array::{GenericByteArray, BinaryArray};
# use arrow_array::types::{BinaryType, Int16Type};
# use arrow_array::{Array, Int16Array};
# use arrow_array::cast::AsArray;

let mut builder =
GenericByteRunBuilder::<Int16Type, BinaryType>::new();
builder.extend([Some(b"abc"), Some(b"abc"), None, Some(b"def")].into_iter());
builder.append_value(b"def");
builder.append_null();
let array = builder.finish();

assert_eq!(array.run_ends().values(), &[2, 3, 5, 6]);

let av = array.values();

assert!(!av.is_null(0));
assert!(av.is_null(1));
assert!(!av.is_null(2));
assert!(av.is_null(3));

// Values are polymorphic and so require a downcast.
let ava: &BinaryArray = av.as_binary();

assert_eq!(ava.value(0), b"abc");
assert_eq!(ava.value(2), b"def");
```

<a id="op-0a0e1418b97f13973b48a8be"></a>
## append_null

`function` · `arrow_array::builder::generic_byte_run_builder::GenericByteRunBuilder::append_null` · arrow-array 59.3.0

```rust
fn append_null(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::generic_byte_run_builder::GenericByteRunBuilder", "path": "GenericByteRunBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "generic_params": [], "type": {"generic": "R"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [158, 1], "end": [287, 2], "filename": "src/builder/generic_byte_run_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_byte_run_builder.rs:187`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends null to the logical array encoded by the RunArray.

<a id="op-4675b5404cbb7f50916e83ca"></a>
## append_option

`function` · `arrow_array::builder::generic_byte_run_builder::GenericByteRunBuilder::append_option` · arrow-array 59.3.0

```rust
fn append_option(&mut self, input_value: Option<impl AsRef<V::Native>>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::generic_byte_run_builder::GenericByteRunBuilder", "path": "GenericByteRunBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "generic_params": [], "type": {"generic": "R"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [158, 1], "end": [287, 2], "filename": "src/builder/generic_byte_run_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_byte_run_builder.rs:164`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends optional value to the logical array encoded by the RunArray.

<a id="op-7f54c6510a7de2e2ffc83c8e"></a>
## append_value

`function` · `arrow_array::builder::generic_byte_run_builder::GenericByteRunBuilder::append_value` · arrow-array 59.3.0

```rust
fn append_value(&mut self, input_value: impl AsRef<V::Native>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::generic_byte_run_builder::GenericByteRunBuilder", "path": "GenericByteRunBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "generic_params": [], "type": {"generic": "R"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [158, 1], "end": [287, 2], "filename": "src/builder/generic_byte_run_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_byte_run_builder.rs:172`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends value to the logical array encoded by the RunArray.

<a id="op-3773cf9a93122a3f23b96e4f"></a>
## as_any

`function` · `arrow_array::builder::generic_byte_run_builder::GenericByteRunBuilder::as_any` · arrow-array 59.3.0

```rust
fn as_any(&self) -> &dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::generic_byte_run_builder::GenericByteRunBuilder", "path": "GenericByteRunBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "generic_params": [], "type": {"generic": "R"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [121, 1], "end": [156, 2], "filename": "src/builder/generic_byte_run_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/generic_byte_run_builder.rs:127`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the builder as a non-mutable `Any` reference.

<a id="op-c910e5d045370726c1c04cf3"></a>
## as_any_mut

`function` · `arrow_array::builder::generic_byte_run_builder::GenericByteRunBuilder::as_any_mut` · arrow-array 59.3.0

```rust
fn as_any_mut(&mut self) -> &mut dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::generic_byte_run_builder::GenericByteRunBuilder", "path": "GenericByteRunBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "generic_params": [], "type": {"generic": "R"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [121, 1], "end": [156, 2], "filename": "src/builder/generic_byte_run_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/generic_byte_run_builder.rs:132`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the builder as a mutable `Any` reference.

<a id="op-b0b9dff4b7eabdfef8967a31"></a>
## default

`function` · `arrow_array::builder::generic_byte_run_builder::GenericByteRunBuilder::default` · arrow-array 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::generic_byte_run_builder::GenericByteRunBuilder", "path": "GenericByteRunBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "R"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [78, 1], "end": [86, 2], "filename": "src/builder/generic_byte_run_builder.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/builder/generic_byte_run_builder.rs:83`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3397881d45a1a92ad6bc0994"></a>
## extend

`function` · `arrow_array::builder::generic_byte_run_builder::GenericByteRunBuilder::extend` · arrow-array 59.3.0

```rust
fn extend<T: IntoIterator<Item = Option<S>>>(&mut self, iter: T)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::generic_byte_run_builder::GenericByteRunBuilder", "path": "GenericByteRunBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "generic_params": [], "type": {"generic": "R"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "generic_params": [], "type": {"generic": "V"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "V"}, "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": ""}}}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}}}], "generic_params": [], "type": {"generic": "S"}}}]}, "is_negative": false, "span": {"begin": [289, 1], "end": [300, 2], "filename": "src/builder/generic_byte_run_builder.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "S"}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "core::iter::traits::collect::Extend", "path": "Extend"}, "trait_path": "core::iter::traits::collect::Extend"}`

Source: `src/builder/generic_byte_run_builder.rs:295`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-633ee28695b3b5250292feec"></a>
## finish

`function` · `arrow_array::builder::generic_byte_run_builder::GenericByteRunBuilder::finish` · arrow-array 59.3.0

```rust
fn finish(&mut self) -> RunArray<R>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::generic_byte_run_builder::GenericByteRunBuilder", "path": "GenericByteRunBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "generic_params": [], "type": {"generic": "R"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [158, 1], "end": [287, 2], "filename": "src/builder/generic_byte_run_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_byte_run_builder.rs:198`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates the RunArray and resets the builder.
Panics if RunArray cannot be built.

<a id="op-b0d72ac4e6f3f93dc5dd35eb"></a>
## finish

`function` · `arrow_array::builder::generic_byte_run_builder::GenericByteRunBuilder::finish` · arrow-array 59.3.0

```rust
fn finish(&mut self) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::generic_byte_run_builder::GenericByteRunBuilder", "path": "GenericByteRunBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "generic_params": [], "type": {"generic": "R"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [121, 1], "end": [156, 2], "filename": "src/builder/generic_byte_run_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/generic_byte_run_builder.rs:148`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the array and reset this builder.

<a id="op-64d000c8f57a14a99433aa05"></a>
## finish_cloned

`function` · `arrow_array::builder::generic_byte_run_builder::GenericByteRunBuilder::finish_cloned` · arrow-array 59.3.0

```rust
fn finish_cloned(&self) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::generic_byte_run_builder::GenericByteRunBuilder", "path": "GenericByteRunBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "generic_params": [], "type": {"generic": "R"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [121, 1], "end": [156, 2], "filename": "src/builder/generic_byte_run_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/generic_byte_run_builder.rs:153`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the array without resetting the builder.

<a id="op-e083060b6fa4a70fe13d482d"></a>
## finish_cloned

`function` · `arrow_array::builder::generic_byte_run_builder::GenericByteRunBuilder::finish_cloned` · arrow-array 59.3.0

```rust
fn finish_cloned(&self) -> RunArray<R>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::generic_byte_run_builder::GenericByteRunBuilder", "path": "GenericByteRunBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "generic_params": [], "type": {"generic": "R"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [158, 1], "end": [287, 2], "filename": "src/builder/generic_byte_run_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_byte_run_builder.rs:216`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates the RunArray and without resetting the builder.
Panics if RunArray cannot be built.

<a id="op-6a8879864dbb6e68a58c2c2a"></a>
## fmt

`function` · `arrow_array::builder::generic_byte_run_builder::GenericByteRunBuilder::fmt` · arrow-array 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::generic_byte_run_builder::GenericByteRunBuilder", "path": "GenericByteRunBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "generic_params": [], "type": {"generic": "R"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [64, 10], "end": [64, 15], "filename": "src/builder/generic_byte_run_builder.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/builder/generic_byte_run_builder.rs:64`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-259042d18703da4b8ec9d450"></a>
## into_box_any

`function` · `arrow_array::builder::generic_byte_run_builder::GenericByteRunBuilder::into_box_any` · arrow-array 59.3.0

```rust
fn into_box_any(Box<self>) -> Box<dyn Any>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::generic_byte_run_builder::GenericByteRunBuilder", "path": "GenericByteRunBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "generic_params": [], "type": {"generic": "R"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [121, 1], "end": [156, 2], "filename": "src/builder/generic_byte_run_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/generic_byte_run_builder.rs:137`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the boxed builder as a box of `Any`.

<a id="op-6c9d6d86a2299f25e2f6f69b"></a>
## len

`function` · `arrow_array::builder::generic_byte_run_builder::GenericByteRunBuilder::len` · arrow-array 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::generic_byte_run_builder::GenericByteRunBuilder", "path": "GenericByteRunBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "generic_params": [], "type": {"generic": "R"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [121, 1], "end": [156, 2], "filename": "src/builder/generic_byte_run_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/generic_byte_run_builder.rs:143`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the length of logical array encoded by
the eventual runs array.

<a id="op-e541b14fa3f261618ab43551"></a>
## new

`function` · `arrow_array::builder::generic_byte_run_builder::GenericByteRunBuilder::new` · arrow-array 59.3.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::generic_byte_run_builder::GenericByteRunBuilder", "path": "GenericByteRunBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "R"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [88, 1], "end": [119, 2], "filename": "src/builder/generic_byte_run_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_byte_run_builder.rs:94`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new `GenericByteRunBuilder`

<a id="op-323892f4bad9029a12d1b56c"></a>
## with_capacity

`function` · `arrow_array::builder::generic_byte_run_builder::GenericByteRunBuilder::with_capacity` · arrow-array 59.3.0

```rust
fn with_capacity(capacity: usize, data_capacity: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::generic_byte_run_builder::GenericByteRunBuilder", "path": "GenericByteRunBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "R"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [88, 1], "end": [119, 2], "filename": "src/builder/generic_byte_run_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_byte_run_builder.rs:109`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new `GenericByteRunBuilder` with the provided capacity

`capacity`: the expected number of run-end encoded values.
`data_capacity`: the expected number of bytes of run end encoded values
