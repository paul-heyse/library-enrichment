# `arrow_array::builder::primitive_run_builder::PrimitiveRunBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.builder.primitive_run_builder.PrimitiveRunBuilder.json).

<a id="op-c92c3d84b1d23dd06da0f81a"></a>
## PrimitiveRunBuilder

`struct` · `arrow_array::builder::primitive_run_builder::PrimitiveRunBuilder` · arrow-array 59.3.0

```rust
struct PrimitiveRunBuilder<R, V> where R: RunEndIndexType, V: ArrowPrimitiveType
```

Source: `src/builder/primitive_run_builder.rs:61`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builder for [`RunArray`](../operations/arrow_array.array.run_array.RunArray.md#op-f0742b2d9e045922a076478f) of [`PrimitiveArray`](crate::array::PrimitiveArray)

# Example:

```

# use arrow_array::builder::PrimitiveRunBuilder;
# use arrow_array::cast::AsArray;
# use arrow_array::types::{UInt32Type, Int16Type};
# use arrow_array::{Array, UInt32Array, Int16Array};

let mut builder =
PrimitiveRunBuilder::<Int16Type, UInt32Type>::new();
builder.append_value(1234);
builder.append_value(1234);
builder.append_value(1234);
builder.append_null();
builder.append_value(5678);
builder.append_value(5678);
let array = builder.finish();

assert_eq!(array.run_ends().values(), &[3, 4, 6]);

let av = array.values();

assert!(!av.is_null(0));
assert!(av.is_null(1));
assert!(!av.is_null(2));

// Values are polymorphic and so require a downcast.
let ava: &UInt32Array = av.as_primitive::<UInt32Type>();

assert_eq!(ava, &UInt32Array::from(vec![Some(1234), None, Some(5678)]));
```

<a id="op-3e83dbd4b972425c6d212e47"></a>
## append_null

`function` · `arrow_array::builder::primitive_run_builder::PrimitiveRunBuilder::append_null` · arrow-array 59.3.0

```rust
fn append_null(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_run_builder::PrimitiveRunBuilder", "path": "PrimitiveRunBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "generic_params": [], "type": {"generic": "R"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [164, 1], "end": [260, 2], "filename": "src/builder/primitive_run_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_run_builder.rs:190`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends null to the logical array encoded by the run-ends array.

<a id="op-877f019880848dd9d09f8df8"></a>
## append_option

`function` · `arrow_array::builder::primitive_run_builder::PrimitiveRunBuilder::append_option` · arrow-array 59.3.0

```rust
fn append_option(&mut self, value: Option<V::Native>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_run_builder::PrimitiveRunBuilder", "path": "PrimitiveRunBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "generic_params": [], "type": {"generic": "R"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [164, 1], "end": [260, 2], "filename": "src/builder/primitive_run_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_run_builder.rs:170`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends optional value to the logical array encoded by the RunArray.

<a id="op-563dfaeb84cb8b72ddcb2cab"></a>
## append_value

`function` · `arrow_array::builder::primitive_run_builder::PrimitiveRunBuilder::append_value` · arrow-array 59.3.0

```rust
fn append_value(&mut self, value: V::Native)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_run_builder::PrimitiveRunBuilder", "path": "PrimitiveRunBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "generic_params": [], "type": {"generic": "R"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [164, 1], "end": [260, 2], "filename": "src/builder/primitive_run_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_run_builder.rs:185`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends value to the logical array encoded by the run-ends array.

<a id="op-d0b14daf5604f9bc0120c859"></a>
## as_any

`function` · `arrow_array::builder::primitive_run_builder::PrimitiveRunBuilder::as_any` · arrow-array 59.3.0

```rust
fn as_any(&self) -> &dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_run_builder::PrimitiveRunBuilder", "path": "PrimitiveRunBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "generic_params": [], "type": {"generic": "R"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [127, 1], "end": [162, 2], "filename": "src/builder/primitive_run_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/primitive_run_builder.rs:133`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the builder as a non-mutable `Any` reference.

<a id="op-af2a925f3db3e2fcbefe647f"></a>
## as_any_mut

`function` · `arrow_array::builder::primitive_run_builder::PrimitiveRunBuilder::as_any_mut` · arrow-array 59.3.0

```rust
fn as_any_mut(&mut self) -> &mut dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_run_builder::PrimitiveRunBuilder", "path": "PrimitiveRunBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "generic_params": [], "type": {"generic": "R"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [127, 1], "end": [162, 2], "filename": "src/builder/primitive_run_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/primitive_run_builder.rs:138`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the builder as a mutable `Any` reference.

<a id="op-f3cc5d5c186208850aedbd68"></a>
## default

`function` · `arrow_array::builder::primitive_run_builder::PrimitiveRunBuilder::default` · arrow-array 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_run_builder::PrimitiveRunBuilder", "path": "PrimitiveRunBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "generic_params": [], "type": {"generic": "R"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [73, 1], "end": [81, 2], "filename": "src/builder/primitive_run_builder.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/builder/primitive_run_builder.rs:78`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e0b92c02bce18c32815dc375"></a>
## extend

`function` · `arrow_array::builder::primitive_run_builder::PrimitiveRunBuilder::extend` · arrow-array 59.3.0

```rust
fn extend<T: IntoIterator<Item = Option<V::Native>>>(&mut self, iter: T)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_run_builder::PrimitiveRunBuilder", "path": "PrimitiveRunBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "generic_params": [], "type": {"generic": "R"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [262, 1], "end": [272, 2], "filename": "src/builder/primitive_run_builder.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "V"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "core::iter::traits::collect::Extend", "path": "Extend"}, "trait_path": "core::iter::traits::collect::Extend"}`

Source: `src/builder/primitive_run_builder.rs:267`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-060a0a59d09ccab0568fd10a"></a>
## finish

`function` · `arrow_array::builder::primitive_run_builder::PrimitiveRunBuilder::finish` · arrow-array 59.3.0

```rust
fn finish(&mut self) -> RunArray<R>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_run_builder::PrimitiveRunBuilder", "path": "PrimitiveRunBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "generic_params": [], "type": {"generic": "R"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [164, 1], "end": [260, 2], "filename": "src/builder/primitive_run_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_run_builder.rs:196`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates the RunArray and resets the builder.
Panics if RunArray cannot be built.

<a id="op-ad51e05ed2d7c473fdaa9297"></a>
## finish

`function` · `arrow_array::builder::primitive_run_builder::PrimitiveRunBuilder::finish` · arrow-array 59.3.0

```rust
fn finish(&mut self) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_run_builder::PrimitiveRunBuilder", "path": "PrimitiveRunBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "generic_params": [], "type": {"generic": "R"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [127, 1], "end": [162, 2], "filename": "src/builder/primitive_run_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/primitive_run_builder.rs:154`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the array and reset this builder.

<a id="op-3917e55f7877c3fc3d635f0d"></a>
## finish_cloned

`function` · `arrow_array::builder::primitive_run_builder::PrimitiveRunBuilder::finish_cloned` · arrow-array 59.3.0

```rust
fn finish_cloned(&self) -> RunArray<R>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_run_builder::PrimitiveRunBuilder", "path": "PrimitiveRunBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "generic_params": [], "type": {"generic": "R"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [164, 1], "end": [260, 2], "filename": "src/builder/primitive_run_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_run_builder.rs:212`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates the RunArray and without resetting the builder.
Panics if RunArray cannot be built.

<a id="op-a0ca519f4ca68b4eb99c307a"></a>
## finish_cloned

`function` · `arrow_array::builder::primitive_run_builder::PrimitiveRunBuilder::finish_cloned` · arrow-array 59.3.0

```rust
fn finish_cloned(&self) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_run_builder::PrimitiveRunBuilder", "path": "PrimitiveRunBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "generic_params": [], "type": {"generic": "R"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [127, 1], "end": [162, 2], "filename": "src/builder/primitive_run_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/primitive_run_builder.rs:159`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the array without resetting the builder.

<a id="op-c21d64bbf31a0faf3fd7b7e6"></a>
## fmt

`function` · `arrow_array::builder::primitive_run_builder::PrimitiveRunBuilder::fmt` · arrow-array 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_run_builder::PrimitiveRunBuilder", "path": "PrimitiveRunBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "generic_params": [], "type": {"generic": "R"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "generic_params": [], "type": {"generic": "V"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "V"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}}}]}, "is_negative": false, "span": {"begin": [60, 10], "end": [60, 15], "filename": "src/builder/primitive_run_builder.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/builder/primitive_run_builder.rs:60`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-221197f2312068f9dc475c2f"></a>
## into_box_any

`function` · `arrow_array::builder::primitive_run_builder::PrimitiveRunBuilder::into_box_any` · arrow-array 59.3.0

```rust
fn into_box_any(Box<self>) -> Box<dyn Any>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_run_builder::PrimitiveRunBuilder", "path": "PrimitiveRunBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "generic_params": [], "type": {"generic": "R"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [127, 1], "end": [162, 2], "filename": "src/builder/primitive_run_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/primitive_run_builder.rs:143`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the boxed builder as a box of `Any`.

<a id="op-f291aa68af202a9ba2254661"></a>
## len

`function` · `arrow_array::builder::primitive_run_builder::PrimitiveRunBuilder::len` · arrow-array 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_run_builder::PrimitiveRunBuilder", "path": "PrimitiveRunBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "generic_params": [], "type": {"generic": "R"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [127, 1], "end": [162, 2], "filename": "src/builder/primitive_run_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/primitive_run_builder.rs:149`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the length of logical array encoded by
the eventual runs array.

<a id="op-f138d528b22931f9f9a21b27"></a>
## new

`function` · `arrow_array::builder::primitive_run_builder::PrimitiveRunBuilder::new` · arrow-array 59.3.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_run_builder::PrimitiveRunBuilder", "path": "PrimitiveRunBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "generic_params": [], "type": {"generic": "R"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [83, 1], "end": [125, 2], "filename": "src/builder/primitive_run_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_run_builder.rs:89`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new `PrimitiveRunBuilder`

<a id="op-5d80e0b9a3c1ab0ebd94884b"></a>
## with_capacity

`function` · `arrow_array::builder::primitive_run_builder::PrimitiveRunBuilder::with_capacity` · arrow-array 59.3.0

```rust
fn with_capacity(capacity: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_run_builder::PrimitiveRunBuilder", "path": "PrimitiveRunBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "generic_params": [], "type": {"generic": "R"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [83, 1], "end": [125, 2], "filename": "src/builder/primitive_run_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_run_builder.rs:102`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new `PrimitiveRunBuilder` with the provided capacity

`capacity`: the expected number of run-end encoded values.

<a id="op-8635393acea28ceabd0f000e"></a>
## with_data_type

`function` · `arrow_array::builder::primitive_run_builder::PrimitiveRunBuilder::with_data_type` · arrow-array 59.3.0

```rust
fn with_data_type(self, data_type: arrow_schema::DataType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_run_builder::PrimitiveRunBuilder", "path": "PrimitiveRunBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "generic_params": [], "type": {"generic": "R"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [83, 1], "end": [125, 2], "filename": "src/builder/primitive_run_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_run_builder.rs:121`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Overrides the data type of the values child array.

By default, `V::DATA_TYPE` is used (via [`PrimitiveBuilder`](../operations/arrow_array.builder.primitive_builder.PrimitiveBuilder.md#op-ad7b023d1f59a6bba4232467)). This
allows setting the timezone of a Timestamp, the precision & scale of a
Decimal, etc.

# Panics

This method panics if `values_builder` rejects `data_type`.
