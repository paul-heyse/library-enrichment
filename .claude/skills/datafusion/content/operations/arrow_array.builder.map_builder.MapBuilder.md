# `arrow_array::builder::map_builder::MapBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.builder.map_builder.MapBuilder.json).

<a id="op-29b3c25b26b6ef8ae009ae70"></a>
## MapBuilder

`struct` · `arrow_array::builder::map_builder::MapBuilder` · arrow-array 59.3.0

```rust
struct MapBuilder<K: ArrayBuilder, V: ArrayBuilder>
```

Source: `src/builder/map_builder.rs:58`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builder for [`MapArray`](../operations/arrow_array.array.map_array.MapArray.md#op-2c9f2f57a7578a4beb5adb8f)

```
# use arrow_array::builder::{Int32Builder, MapBuilder, StringBuilder};
# use arrow_array::{Int32Array, StringArray};

let string_builder = StringBuilder::new();
let int_builder = Int32Builder::with_capacity(4);

// Construct `[{"joe": 1}, {"blogs": 2, "foo": 4}, {}, null]`
let mut builder = MapBuilder::new(None, string_builder, int_builder);

builder.keys().append_value("joe");
builder.values().append_value(1);
builder.append(true).unwrap();

builder.keys().append_value("blogs");
builder.values().append_value(2);
builder.keys().append_value("foo");
builder.values().append_value(4);
builder.append(true).unwrap();
builder.append(true).unwrap();
builder.append(false).unwrap();

let array = builder.finish();
assert_eq!(array.value_offsets(), &[0, 1, 3, 3, 3]);
assert_eq!(array.values().as_ref(), &Int32Array::from(vec![1, 2, 4]));
assert_eq!(array.keys().as_ref(), &StringArray::from(vec!["joe", "blogs", "foo"]));

```

<a id="op-784e75e7851be4892a914020"></a>
## append

`function` · `arrow_array::builder::map_builder::MapBuilder::append` · arrow-array 59.3.0

```rust
fn append(&mut self, is_valid: bool) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::map_builder::MapBuilder", "path": "MapBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 1], "end": [286, 2], "filename": "src/builder/map_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/map_builder.rs:174`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Finish the current map array slot

Returns an error if the key and values builders are in an inconsistent state.

<a id="op-792d01b3ceb8de246e934ef0"></a>
## append_nulls

`function` · `arrow_array::builder::map_builder::MapBuilder::append_nulls` · arrow-array 59.3.0

```rust
fn append_nulls(&mut self, n: usize) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::map_builder::MapBuilder", "path": "MapBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 1], "end": [286, 2], "filename": "src/builder/map_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/map_builder.rs:185`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Append `n` nulls to this [`MapBuilder`](../operations/arrow_array.builder.map_builder.MapBuilder.md#op-29b3c25b26b6ef8ae009ae70)

Returns an error if the key and values builders are in an inconsistent state.

<a id="op-c8ae1d2a306d201bfac1154d"></a>
## as_any

`function` · `arrow_array::builder::map_builder::MapBuilder::as_any` · arrow-array 59.3.0

```rust
fn as_any(&self) -> &dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::map_builder::MapBuilder", "path": "MapBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [288, 1], "end": [317, 2], "filename": "src/builder/map_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/map_builder.rs:306`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9c8e8a04a5b84ebbf2b08662"></a>
## as_any_mut

`function` · `arrow_array::builder::map_builder::MapBuilder::as_any_mut` · arrow-array 59.3.0

```rust
fn as_any_mut(&mut self) -> &mut dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::map_builder::MapBuilder", "path": "MapBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [288, 1], "end": [317, 2], "filename": "src/builder/map_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/map_builder.rs:310`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-26ec9897a532213746001c63"></a>
## entries

`function` · `arrow_array::builder::map_builder::MapBuilder::entries` · arrow-array 59.3.0

```rust
fn entries(&mut self) -> (&mut K, &mut V)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::map_builder::MapBuilder", "path": "MapBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 1], "end": [286, 2], "filename": "src/builder/map_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/map_builder.rs:153`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns both the key and value array builders of the map

<a id="op-0abbce9c9a105561bf6c54a4"></a>
## finish

`function` · `arrow_array::builder::map_builder::MapBuilder::finish` · arrow-array 59.3.0

```rust
fn finish(&mut self) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::map_builder::MapBuilder", "path": "MapBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [288, 1], "end": [317, 2], "filename": "src/builder/map_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/map_builder.rs:293`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5804b9c5cc71c96d359e5e76"></a>
## finish

`function` · `arrow_array::builder::map_builder::MapBuilder::finish` · arrow-array 59.3.0

```rust
fn finish(&mut self) -> MapArray
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::map_builder::MapBuilder", "path": "MapBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 1], "end": [286, 2], "filename": "src/builder/map_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/map_builder.rs:194`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the [`MapArray`](../operations/arrow_array.array.map_array.MapArray.md#op-2c9f2f57a7578a4beb5adb8f)

<a id="op-2e1de6d63db26de373395c79"></a>
## finish_cloned

`function` · `arrow_array::builder::map_builder::MapBuilder::finish_cloned` · arrow-array 59.3.0

```rust
fn finish_cloned(&self) -> MapArray
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::map_builder::MapBuilder", "path": "MapBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 1], "end": [286, 2], "filename": "src/builder/map_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/map_builder.rs:207`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the [`MapArray`](../operations/arrow_array.array.map_array.MapArray.md#op-2c9f2f57a7578a4beb5adb8f) without resetting the builder.

<a id="op-8f7b80fc6b037a1b1c706a6b"></a>
## finish_cloned

`function` · `arrow_array::builder::map_builder::MapBuilder::finish_cloned` · arrow-array 59.3.0

```rust
fn finish_cloned(&self) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::map_builder::MapBuilder", "path": "MapBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [288, 1], "end": [317, 2], "filename": "src/builder/map_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/map_builder.rs:298`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the array without resetting the builder.

<a id="op-b2de7fb30e40b287462614e6"></a>
## finish_preserve_values

`function` · `arrow_array::builder::map_builder::MapBuilder::finish_preserve_values` · arrow-array 59.3.0

```rust
fn finish_preserve_values(&mut self) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::map_builder::MapBuilder", "path": "MapBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [288, 1], "end": [317, 2], "filename": "src/builder/map_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/map_builder.rs:302`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b14176e95edb706ddbb4bca2"></a>
## fmt

`function` · `arrow_array::builder::map_builder::MapBuilder::fmt` · arrow-array 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::map_builder::MapBuilder", "path": "MapBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 10], "end": [57, 15], "filename": "src/builder/map_builder.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/builder/map_builder.rs:57`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-35ebdef717087a289934f24f"></a>
## into_box_any

`function` · `arrow_array::builder::map_builder::MapBuilder::into_box_any` · arrow-array 59.3.0

```rust
fn into_box_any(Box<self>) -> Box<dyn Any>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::map_builder::MapBuilder", "path": "MapBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [288, 1], "end": [317, 2], "filename": "src/builder/map_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/map_builder.rs:314`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6fe951c30fe01b87d3d54579"></a>
## keys

`function` · `arrow_array::builder::map_builder::MapBuilder::keys` · arrow-array 59.3.0

```rust
fn keys(&mut self) -> &mut K
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::map_builder::MapBuilder", "path": "MapBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 1], "end": [286, 2], "filename": "src/builder/map_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/map_builder.rs:143`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the key array builder of the map

<a id="op-714d958a22581f458db11b32"></a>
## len

`function` · `arrow_array::builder::map_builder::MapBuilder::len` · arrow-array 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::map_builder::MapBuilder", "path": "MapBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [288, 1], "end": [317, 2], "filename": "src/builder/map_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/map_builder.rs:289`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-30efa23d9c442d4a2314472f"></a>
## new

`function` · `arrow_array::builder::map_builder::MapBuilder::new` · arrow-array 59.3.0

```rust
fn new(field_names: Option<MapFieldNames>, key_builder: K, value_builder: V) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::map_builder::MapBuilder", "path": "MapBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 1], "end": [286, 2], "filename": "src/builder/map_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/map_builder.rs:91`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new `MapBuilder`

<a id="op-77696c89d098743e5d30b3df"></a>
## validity_slice

`function` · `arrow_array::builder::map_builder::MapBuilder::validity_slice` · arrow-array 59.3.0

```rust
fn validity_slice(&self) -> Option<&[u8]>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::map_builder::MapBuilder", "path": "MapBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 1], "end": [286, 2], "filename": "src/builder/map_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/map_builder.rs:283`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the current null buffer as a slice

<a id="op-aa1a8aa37c3027dd1d595249"></a>
## values

`function` · `arrow_array::builder::map_builder::MapBuilder::values` · arrow-array 59.3.0

```rust
fn values(&mut self) -> &mut V
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::map_builder::MapBuilder", "path": "MapBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 1], "end": [286, 2], "filename": "src/builder/map_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/map_builder.rs:148`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the value array builder of the map

<a id="op-d34dc88817639e855e4388b4"></a>
## with_capacity

`function` · `arrow_array::builder::map_builder::MapBuilder::with_capacity` · arrow-array 59.3.0

```rust
fn with_capacity(field_names: Option<MapFieldNames>, key_builder: K, value_builder: V, capacity: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::map_builder::MapBuilder", "path": "MapBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 1], "end": [286, 2], "filename": "src/builder/map_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/map_builder.rs:97`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new `MapBuilder` with capacity

<a id="op-346d9295dc15cf192e6e7b84"></a>
## with_keys_field

`function` · `arrow_array::builder::map_builder::MapBuilder::with_keys_field` · arrow-array 59.3.0

```rust
fn with_keys_field(self, field: impl Into<FieldRef>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::map_builder::MapBuilder", "path": "MapBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 1], "end": [286, 2], "filename": "src/builder/map_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/map_builder.rs:122`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Override the field passed to [`MapBuilder::new`](../operations/arrow_array.builder.map_builder.MapBuilder.md#op-30efa23d9c442d4a2314472f)

By default, a non-nullable field is created with the name `keys`

Note: [`Self::finish`](../operations/arrow_array.builder.map_builder.MapBuilder.md#op-5804b9c5cc71c96d359e5e76) and [`Self::finish_cloned`](../operations/arrow_array.builder.map_builder.MapBuilder.md#op-2e1de6d63db26de373395c79) will panic if the
field's data type does not match that of `K` or the field is nullable

<a id="op-d99c08329598eb0254ce8c1d"></a>
## with_values_field

`function` · `arrow_array::builder::map_builder::MapBuilder::with_values_field` · arrow-array 59.3.0

```rust
fn with_values_field(self, field: impl Into<FieldRef>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::map_builder::MapBuilder", "path": "MapBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 1], "end": [286, 2], "filename": "src/builder/map_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/map_builder.rs:135`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Override the field passed to [`MapBuilder::new`](../operations/arrow_array.builder.map_builder.MapBuilder.md#op-30efa23d9c442d4a2314472f)

By default, a nullable field is created with the name `values`

Note: [`Self::finish`](../operations/arrow_array.builder.map_builder.MapBuilder.md#op-5804b9c5cc71c96d359e5e76) and [`Self::finish_cloned`](../operations/arrow_array.builder.map_builder.MapBuilder.md#op-2e1de6d63db26de373395c79) will panic if the
field's data type does not match that of `V`
