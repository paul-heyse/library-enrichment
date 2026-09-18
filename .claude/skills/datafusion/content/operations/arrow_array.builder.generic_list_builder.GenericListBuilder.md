# `arrow_array::builder::generic_list_builder::GenericListBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.builder.generic_list_builder.GenericListBuilder.json).

<a id="op-97b03adbae152fb71b763a5d"></a>
## GenericListBuilder

`struct` · `arrow_array::builder::generic_list_builder::GenericListBuilder` · arrow-array 59.3.0

```rust
struct GenericListBuilder<OffsetSize: OffsetSizeTrait, T: ArrayBuilder>
```

Source: `src/builder/generic_list_builder.rs:88`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builder for [`GenericListArray`](../operations/arrow_array.array.list_array.GenericListArray.md#op-f5cb16d85a7337f2dafa403b)

Use [`ListBuilder`] to build [`ListArray`]s and [`LargeListBuilder`] to build [`LargeListArray`]s.

# Example

Here is code that constructs a ListArray with the contents:
`[[A,B,C], [], NULL, [D], [NULL, F]]`

```
# use std::sync::Arc;
# use arrow_array::{builder::ListBuilder, builder::StringBuilder, ArrayRef, StringArray, Array};
#
let values_builder = StringBuilder::new();
let mut builder = ListBuilder::new(values_builder);

// [A, B, C]
builder.values().append_value("A");
builder.values().append_value("B");
builder.values().append_value("C");
builder.append(true);

// [ ] (empty list)
builder.append(true);

// Null
builder.append(false);

// [D]
builder.values().append_value("D");
builder.append(true);

// [NULL, F]
builder.values().append_null();
builder.values().append_value("F");
builder.append(true);

// Build the array
let array = builder.finish();

// Values is a string array
// "A", "B" "C", "?", "D", NULL, "F"
assert_eq!(
  array.values().as_ref(),
  &StringArray::from(vec![
    Some("A"), Some("B"), Some("C"),
    Some("D"), None, Some("F")
  ])
);

// Offsets are indexes into the values array
assert_eq!(
  array.value_offsets(),
  &[0, 3, 3, 3, 4, 6]
);
```

[`ListBuilder`]: crate::builder::ListBuilder
[`ListArray`]: crate::array::ListArray
[`LargeListBuilder`]: crate::builder::LargeListBuilder
[`LargeListArray`]: crate::array::LargeListArray

<a id="op-3f07bc2b3e088a50b27a58e0"></a>
## append

`function` · `arrow_array::builder::generic_list_builder::GenericListBuilder::append` · arrow-array 59.3.0

```rust
fn append(&mut self, is_valid: bool)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_list_builder::GenericListBuilder", "path": "GenericListBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [175, 1], "end": [372, 2], "filename": "src/builder/generic_list_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_list_builder.rs:198`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Finish the current variable-length list array slot

# Panics

Panics if the length of [`Self::values`](../operations/arrow_array.builder.generic_list_builder.GenericListBuilder.md#op-04132d2c706be111ef80cd0a) exceeds `OffsetSize::MAX`

<a id="op-27f41266b5a0b086ad09abc2"></a>
## append_null

`function` · `arrow_array::builder::generic_list_builder::GenericListBuilder::append_null` · arrow-array 59.3.0

```rust
fn append_null(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_list_builder::GenericListBuilder", "path": "GenericListBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [175, 1], "end": [372, 2], "filename": "src/builder/generic_list_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_list_builder.rs:272`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Append a null to this [`GenericListBuilder`](../operations/arrow_array.builder.generic_list_builder.GenericListBuilder.md#op-97b03adbae152fb71b763a5d)

See [`Self::append_value`](../operations/arrow_array.builder.generic_list_builder.GenericListBuilder.md#op-50f9c4572d2b52ec947f8a76) for an example use.

<a id="op-cadbb9905e32804954cc3e3c"></a>
## append_nulls

`function` · `arrow_array::builder::generic_list_builder::GenericListBuilder::append_nulls` · arrow-array 59.3.0

```rust
fn append_nulls(&mut self, n: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_list_builder::GenericListBuilder", "path": "GenericListBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [175, 1], "end": [372, 2], "filename": "src/builder/generic_list_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_list_builder.rs:279`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends `n` `null`s into the builder.

<a id="op-0b72df140c7343d7f56f1903"></a>
## append_option

`function` · `arrow_array::builder::generic_list_builder::GenericListBuilder::append_option` · arrow-array 59.3.0

```rust
fn append_option<I, V>(&mut self, i: Option<I>) where T: Extend<Option<V>>, I: IntoIterator<Item = Option<V>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_list_builder::GenericListBuilder", "path": "GenericListBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [175, 1], "end": [372, 2], "filename": "src/builder/generic_list_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_list_builder.rs:290`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends an optional value into this [`GenericListBuilder`](../operations/arrow_array.builder.generic_list_builder.GenericListBuilder.md#op-97b03adbae152fb71b763a5d)

If `Some` calls [`Self::append_value`](../operations/arrow_array.builder.generic_list_builder.GenericListBuilder.md#op-50f9c4572d2b52ec947f8a76) otherwise calls [`Self::append_null`](../operations/arrow_array.builder.generic_list_builder.GenericListBuilder.md#op-27f41266b5a0b086ad09abc2)

<a id="op-50f9c4572d2b52ec947f8a76"></a>
## append_value

`function` · `arrow_array::builder::generic_list_builder::GenericListBuilder::append_value` · arrow-array 59.3.0

```rust
fn append_value<I, V>(&mut self, i: I) where T: Extend<Option<V>>, I: IntoIterator<Item = Option<V>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_list_builder::GenericListBuilder", "path": "GenericListBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [175, 1], "end": [372, 2], "filename": "src/builder/generic_list_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_list_builder.rs:260`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Append a value to this [`GenericListBuilder`](../operations/arrow_array.builder.generic_list_builder.GenericListBuilder.md#op-97b03adbae152fb71b763a5d)

```
# use arrow_array::builder::{Int32Builder, ListBuilder};
# use arrow_array::cast::AsArray;
# use arrow_array::{Array, Int32Array};
# use arrow_array::types::Int32Type;
let mut builder = ListBuilder::new(Int32Builder::new());

builder.append_value([Some(1), Some(2), Some(3)]);
builder.append_value([]);
builder.append_value([None]);

let array = builder.finish();
assert_eq!(array.len(), 3);

assert_eq!(array.value_offsets(), &[0, 3, 3, 4]);
let values = array.values().as_primitive::<Int32Type>();
assert_eq!(values, &Int32Array::from(vec![Some(1), Some(2), Some(3), None]));
```

This is an alternative API to appending directly to [`Self::values`](../operations/arrow_array.builder.generic_list_builder.GenericListBuilder.md#op-04132d2c706be111ef80cd0a) and
delimiting the result with [`Self::append`](../operations/arrow_array.builder.generic_list_builder.GenericListBuilder.md#op-3f07bc2b3e088a50b27a58e0)

```
# use arrow_array::builder::{Int32Builder, ListBuilder};
# use arrow_array::cast::AsArray;
# use arrow_array::{Array, Int32Array};
# use arrow_array::types::Int32Type;
let mut builder = ListBuilder::new(Int32Builder::new());

builder.values().append_value(1);
builder.values().append_value(2);
builder.values().append_value(3);
builder.append(true);
builder.append(true);
builder.values().append_null();
builder.append(true);

let array = builder.finish();
assert_eq!(array.len(), 3);

assert_eq!(array.value_offsets(), &[0, 3, 3, 4]);
let values = array.values().as_primitive::<Int32Type>();
assert_eq!(values, &Int32Array::from(vec![Some(1), Some(2), Some(3), None]));
```

<a id="op-7e00a5400d8285ae0de1badb"></a>
## as_any

`function` · `arrow_array::builder::generic_list_builder::GenericListBuilder::as_any` · arrow-array 59.3.0

```rust
fn as_any(&self) -> &dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_list_builder::GenericListBuilder", "path": "GenericListBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [135, 1], "end": [173, 2], "filename": "src/builder/generic_list_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/generic_list_builder.rs:141`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the builder as a non-mutable `Any` reference.

<a id="op-4d7f0fc2b2995a08a9780cd8"></a>
## as_any_mut

`function` · `arrow_array::builder::generic_list_builder::GenericListBuilder::as_any_mut` · arrow-array 59.3.0

```rust
fn as_any_mut(&mut self) -> &mut dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_list_builder::GenericListBuilder", "path": "GenericListBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [135, 1], "end": [173, 2], "filename": "src/builder/generic_list_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/generic_list_builder.rs:146`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the builder as a mutable `Any` reference.

<a id="op-e6b909a7d62f1d19d09912b0"></a>
## default

`function` · `arrow_array::builder::generic_list_builder::GenericListBuilder::default` · arrow-array 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_list_builder::GenericListBuilder", "path": "GenericListBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "O"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::default::Default", "path": "Default"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 1], "end": [99, 2], "filename": "src/builder/generic_list_builder.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/builder/generic_list_builder.rs:96`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a10a9ef18b310cd346d52a84"></a>
## extend

`function` · `arrow_array::builder::generic_list_builder::GenericListBuilder::extend` · arrow-array 59.3.0

```rust
fn extend<T: IntoIterator<Item = Option<V>>>(&mut self, iter: T)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}, {"type": {"generic": "B"}}], "constraints": []}}, "id": "arrow_array::builder::generic_list_builder::GenericListBuilder", "path": "GenericListBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "O"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "B"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "generic_params": [], "type": {"generic": "O"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "core::iter::traits::collect::Extend", "path": "Extend"}}}], "generic_params": [], "type": {"generic": "B"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"generic": "E"}}}, "name": "Item"}]}}, "id": "core::iter::traits::collect::IntoIterator", "path": "IntoIterator"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [374, 1], "end": [392, 2], "filename": "src/builder/generic_list_builder.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "V"}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "core::iter::traits::collect::Extend", "path": "Extend"}, "trait_path": "core::iter::traits::collect::Extend"}`

Source: `src/builder/generic_list_builder.rs:381`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-27103857ca2db3b709e7a9f6"></a>
## finish

`function` · `arrow_array::builder::generic_list_builder::GenericListBuilder::finish` · arrow-array 59.3.0

```rust
fn finish(&mut self) -> GenericListArray<OffsetSize>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_list_builder::GenericListBuilder", "path": "GenericListBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [175, 1], "end": [372, 2], "filename": "src/builder/generic_list_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_list_builder.rs:302`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the [`GenericListArray`](../operations/arrow_array.array.list_array.GenericListArray.md#op-f5cb16d85a7337f2dafa403b) and reset this builder.

<a id="op-7b8cde64806b993c40d96beb"></a>
## finish

`function` · `arrow_array::builder::generic_list_builder::GenericListBuilder::finish` · arrow-array 59.3.0

```rust
fn finish(&mut self) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_list_builder::GenericListBuilder", "path": "GenericListBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [135, 1], "end": [173, 2], "filename": "src/builder/generic_list_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/generic_list_builder.rs:161`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the array and reset this builder.

<a id="op-5c26ed72086ec331df829c45"></a>
## finish_cloned

`function` · `arrow_array::builder::generic_list_builder::GenericListBuilder::finish_cloned` · arrow-array 59.3.0

```rust
fn finish_cloned(&self) -> GenericListArray<OffsetSize>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_list_builder::GenericListBuilder", "path": "GenericListBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [175, 1], "end": [372, 2], "filename": "src/builder/generic_list_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_list_builder.rs:320`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the [`GenericListArray`](../operations/arrow_array.array.list_array.GenericListArray.md#op-f5cb16d85a7337f2dafa403b) without resetting the builder.

<a id="op-85e4e7e7c6602105aa1f0793"></a>
## finish_cloned

`function` · `arrow_array::builder::generic_list_builder::GenericListBuilder::finish_cloned` · arrow-array 59.3.0

```rust
fn finish_cloned(&self) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_list_builder::GenericListBuilder", "path": "GenericListBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [135, 1], "end": [173, 2], "filename": "src/builder/generic_list_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/generic_list_builder.rs:166`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the array without resetting the builder.

<a id="op-be67eac7bfbeb795b0e794a0"></a>
## finish_preserve_values

`function` · `arrow_array::builder::generic_list_builder::GenericListBuilder::finish_preserve_values` · arrow-array 59.3.0

```rust
fn finish_preserve_values(&mut self) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_list_builder::GenericListBuilder", "path": "GenericListBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [135, 1], "end": [173, 2], "filename": "src/builder/generic_list_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/generic_list_builder.rs:170`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4d6812a9ffe56ddb361f5ac5"></a>
## fmt

`function` · `arrow_array::builder::generic_list_builder::GenericListBuilder::fmt` · arrow-array 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_list_builder::GenericListBuilder", "path": "GenericListBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 10], "end": [87, 15], "filename": "src/builder/generic_list_builder.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/builder/generic_list_builder.rs:87`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4afc949dafc5fa9048ae8051"></a>
## into_box_any

`function` · `arrow_array::builder::generic_list_builder::GenericListBuilder::into_box_any` · arrow-array 59.3.0

```rust
fn into_box_any(Box<self>) -> Box<dyn Any>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_list_builder::GenericListBuilder", "path": "GenericListBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [135, 1], "end": [173, 2], "filename": "src/builder/generic_list_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/generic_list_builder.rs:151`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the boxed builder as a box of `Any`.

<a id="op-395d2ca4eab9150b0eebfb55"></a>
## len

`function` · `arrow_array::builder::generic_list_builder::GenericListBuilder::len` · arrow-array 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_list_builder::GenericListBuilder", "path": "GenericListBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [135, 1], "end": [173, 2], "filename": "src/builder/generic_list_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/generic_list_builder.rs:156`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the number of array slots in the builder

<a id="op-6c334d01a991baf1bfb9e54f"></a>
## new

`function` · `arrow_array::builder::generic_list_builder::GenericListBuilder::new` · arrow-array 59.3.0

```rust
fn new(values_builder: T) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_list_builder::GenericListBuilder", "path": "GenericListBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [101, 1], "end": [133, 2], "filename": "src/builder/generic_list_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_list_builder.rs:103`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new [`GenericListBuilder`](../operations/arrow_array.builder.generic_list_builder.GenericListBuilder.md#op-97b03adbae152fb71b763a5d) from a given values array builder

<a id="op-677f3673728227b097a2015d"></a>
## offsets_capacity

`function` · `arrow_array::builder::generic_list_builder::GenericListBuilder::offsets_capacity` · arrow-array 59.3.0

```rust
fn offsets_capacity(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_list_builder::GenericListBuilder", "path": "GenericListBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [175, 1], "end": [372, 2], "filename": "src/builder/generic_list_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_list_builder.rs:359`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the current offsets buffer capacity, in offsets.

<a id="op-4beaae6cf66e960255174a44"></a>
## offsets_slice

`function` · `arrow_array::builder::generic_list_builder::GenericListBuilder::offsets_slice` · arrow-array 59.3.0

```rust
fn offsets_slice(&self) -> &[OffsetSize]
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_list_builder::GenericListBuilder", "path": "GenericListBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [175, 1], "end": [372, 2], "filename": "src/builder/generic_list_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_list_builder.rs:354`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the current offsets buffer as a slice

<a id="op-8fbe9b54fdf7802e06a370ed"></a>
## validity_capacity

`function` · `arrow_array::builder::generic_list_builder::GenericListBuilder::validity_capacity` · arrow-array 59.3.0

```rust
fn validity_capacity(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_list_builder::GenericListBuilder", "path": "GenericListBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [175, 1], "end": [372, 2], "filename": "src/builder/generic_list_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_list_builder.rs:369`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the current null buffer allocated capacity, in bytes.

<a id="op-0b7560b969682c658177a6f6"></a>
## validity_slice

`function` · `arrow_array::builder::generic_list_builder::GenericListBuilder::validity_slice` · arrow-array 59.3.0

```rust
fn validity_slice(&self) -> Option<&[u8]>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_list_builder::GenericListBuilder", "path": "GenericListBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [175, 1], "end": [372, 2], "filename": "src/builder/generic_list_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_list_builder.rs:364`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the current null buffer as a slice

<a id="op-04132d2c706be111ef80cd0a"></a>
## values

`function` · `arrow_array::builder::generic_list_builder::GenericListBuilder::values` · arrow-array 59.3.0

```rust
fn values(&mut self) -> &mut T
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_list_builder::GenericListBuilder", "path": "GenericListBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [175, 1], "end": [372, 2], "filename": "src/builder/generic_list_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_list_builder.rs:183`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the child array builder as a mutable reference.

This mutable reference can be used to append values into the child array builder,
but you must call [`append`](#method.append) to delimit each distinct list value.

<a id="op-6db0ca2a0494576445440847"></a>
## values_ref

`function` · `arrow_array::builder::generic_list_builder::GenericListBuilder::values_ref` · arrow-array 59.3.0

```rust
fn values_ref(&self) -> &T
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_list_builder::GenericListBuilder", "path": "GenericListBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [175, 1], "end": [372, 2], "filename": "src/builder/generic_list_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_list_builder.rs:188`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the child array builder as an immutable reference

<a id="op-db4a2d4b4330f281d2aeb1ec"></a>
## with_capacity

`function` · `arrow_array::builder::generic_list_builder::GenericListBuilder::with_capacity` · arrow-array 59.3.0

```rust
fn with_capacity(values_builder: T, capacity: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_list_builder::GenericListBuilder", "path": "GenericListBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [101, 1], "end": [133, 2], "filename": "src/builder/generic_list_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_list_builder.rs:110`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new [`GenericListBuilder`](../operations/arrow_array.builder.generic_list_builder.GenericListBuilder.md#op-97b03adbae152fb71b763a5d) from a given values array builder
`capacity` is the number of items to pre-allocate space for in this builder

<a id="op-93efbbfc29e1f9d6d4ea6118"></a>
## with_field

`function` · `arrow_array::builder::generic_list_builder::GenericListBuilder::with_field` · arrow-array 59.3.0

```rust
fn with_field(self, field: impl Into<FieldRef>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_list_builder::GenericListBuilder", "path": "GenericListBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [101, 1], "end": [133, 2], "filename": "src/builder/generic_list_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_list_builder.rs:127`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Override the field passed to [`GenericListArray::new`](../operations/arrow_array.array.list_array.GenericListArray.md#op-b9bece8511b73316b0612d22)

By default a nullable field is created with the name `item`

Note: [`Self::finish`](../operations/arrow_array.builder.generic_list_builder.GenericListBuilder.md#op-27103857ca2db3b709e7a9f6) and [`Self::finish_cloned`](../operations/arrow_array.builder.generic_list_builder.GenericListBuilder.md#op-5c26ed72086ec331df829c45) will panic if the
field's data type does not match that of `T`
