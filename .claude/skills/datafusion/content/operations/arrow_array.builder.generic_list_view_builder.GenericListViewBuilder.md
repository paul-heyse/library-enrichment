# `arrow_array::builder::generic_list_view_builder::GenericListViewBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.builder.generic_list_view_builder.GenericListViewBuilder.json).

<a id="op-5ca7d134853cecdadbb30ee4"></a>
## GenericListViewBuilder

`struct` · `arrow_array::builder::generic_list_view_builder::GenericListViewBuilder` · arrow-array 59.3.0

```rust
struct GenericListViewBuilder<OffsetSize: OffsetSizeTrait, T: ArrayBuilder>
```

Source: `src/builder/generic_list_view_builder.rs:27`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builder for [`GenericListViewArray`](../operations/arrow_array.array.list_view_array.GenericListViewArray.md#op-e77399dcef65864e81be3a9a)

<a id="op-e9345c56952ce8ef93cb7d5a"></a>
## append

`function` · `arrow_array::builder::generic_list_view_builder::GenericListViewBuilder::append` · arrow-array 59.3.0

```rust
fn append(&mut self, is_valid: bool)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_list_view_builder::GenericListViewBuilder", "path": "GenericListViewBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [115, 1], "end": [244, 2], "filename": "src/builder/generic_list_view_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_list_view_builder.rs:138`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Finish the current variable-length list array slot

# Panics

Panics if the length of [`Self::values`](../operations/arrow_array.builder.generic_list_view_builder.GenericListViewBuilder.md#op-71bdd41a67a34ca784ea8005) exceeds `OffsetSize::MAX`

<a id="op-f9e6fb6ca9eb55fe3b220c12"></a>
## append_null

`function` · `arrow_array::builder::generic_list_view_builder::GenericListViewBuilder::append_null` · arrow-array 59.3.0

```rust
fn append_null(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_list_view_builder::GenericListViewBuilder", "path": "GenericListViewBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [115, 1], "end": [244, 2], "filename": "src/builder/generic_list_view_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_list_view_builder.rs:164`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Append a null to this [`GenericListViewBuilder`](../operations/arrow_array.builder.generic_list_view_builder.GenericListViewBuilder.md#op-5ca7d134853cecdadbb30ee4)

See [`Self::append_value`](../operations/arrow_array.builder.generic_list_view_builder.GenericListViewBuilder.md#op-43814d227c60c792bc428bda) for an example use.

<a id="op-fef736417611d1ca56703ef2"></a>
## append_option

`function` · `arrow_array::builder::generic_list_view_builder::GenericListViewBuilder::append_option` · arrow-array 59.3.0

```rust
fn append_option<I, V>(&mut self, i: Option<I>) where T: Extend<Option<V>>, I: IntoIterator<Item = Option<V>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_list_view_builder::GenericListViewBuilder", "path": "GenericListViewBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [115, 1], "end": [244, 2], "filename": "src/builder/generic_list_view_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_list_view_builder.rs:174`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends an optional value into this [`GenericListViewBuilder`](../operations/arrow_array.builder.generic_list_view_builder.GenericListViewBuilder.md#op-5ca7d134853cecdadbb30ee4)

If `Some` calls [`Self::append_value`](../operations/arrow_array.builder.generic_list_view_builder.GenericListViewBuilder.md#op-43814d227c60c792bc428bda) otherwise calls [`Self::append_null`](../operations/arrow_array.builder.generic_list_view_builder.GenericListViewBuilder.md#op-f9e6fb6ca9eb55fe3b220c12)

<a id="op-43814d227c60c792bc428bda"></a>
## append_value

`function` · `arrow_array::builder::generic_list_view_builder::GenericListViewBuilder::append_value` · arrow-array 59.3.0

```rust
fn append_value<I, V>(&mut self, i: I) where T: Extend<Option<V>>, I: IntoIterator<Item = Option<V>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_list_view_builder::GenericListViewBuilder", "path": "GenericListViewBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [115, 1], "end": [244, 2], "filename": "src/builder/generic_list_view_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_list_view_builder.rs:152`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Append value into this [`GenericListViewBuilder`](../operations/arrow_array.builder.generic_list_view_builder.GenericListViewBuilder.md#op-5ca7d134853cecdadbb30ee4)

<a id="op-cde30be8218df4d6e986cfe2"></a>
## as_any

`function` · `arrow_array::builder::generic_list_view_builder::GenericListViewBuilder::as_any` · arrow-array 59.3.0

```rust
fn as_any(&self) -> &dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_list_view_builder::GenericListViewBuilder", "path": "GenericListViewBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [78, 2], "filename": "src/builder/generic_list_view_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/generic_list_view_builder.rs:46`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the builder as a non-mutable `Any` reference.

<a id="op-45879417a0177213c684717d"></a>
## as_any_mut

`function` · `arrow_array::builder::generic_list_view_builder::GenericListViewBuilder::as_any_mut` · arrow-array 59.3.0

```rust
fn as_any_mut(&mut self) -> &mut dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_list_view_builder::GenericListViewBuilder", "path": "GenericListViewBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [78, 2], "filename": "src/builder/generic_list_view_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/generic_list_view_builder.rs:51`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the builder as a mutable `Any` reference.

<a id="op-df2d039cba2e80bdc43c454e"></a>
## default

`function` · `arrow_array::builder::generic_list_view_builder::GenericListViewBuilder::default` · arrow-array 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_list_view_builder::GenericListViewBuilder", "path": "GenericListViewBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "O"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::default::Default", "path": "Default"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 1], "end": [40, 2], "filename": "src/builder/generic_list_view_builder.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/builder/generic_list_view_builder.rs:37`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0a23122713d2f75ae19fdd51"></a>
## extend

`function` · `arrow_array::builder::generic_list_view_builder::GenericListViewBuilder::extend` · arrow-array 59.3.0

```rust
fn extend<T: IntoIterator<Item = Option<V>>>(&mut self, iter: T)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}, {"type": {"generic": "B"}}], "constraints": []}}, "id": "arrow_array::builder::generic_list_view_builder::GenericListViewBuilder", "path": "GenericListViewBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "O"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "B"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "generic_params": [], "type": {"generic": "O"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "core::iter::traits::collect::Extend", "path": "Extend"}}}], "generic_params": [], "type": {"generic": "B"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"generic": "E"}}}, "name": "Item"}]}}, "id": "core::iter::traits::collect::IntoIterator", "path": "IntoIterator"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [246, 1], "end": [264, 2], "filename": "src/builder/generic_list_view_builder.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "V"}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "core::iter::traits::collect::Extend", "path": "Extend"}, "trait_path": "core::iter::traits::collect::Extend"}`

Source: `src/builder/generic_list_view_builder.rs:253`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b9299ddf7ec3108f3bab382"></a>
## finish

`function` · `arrow_array::builder::generic_list_view_builder::GenericListViewBuilder::finish` · arrow-array 59.3.0

```rust
fn finish(&mut self) -> GenericListViewArray<OffsetSize>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_list_view_builder::GenericListViewBuilder", "path": "GenericListViewBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [115, 1], "end": [244, 2], "filename": "src/builder/generic_list_view_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_list_view_builder.rs:186`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the [`GenericListViewArray`](../operations/arrow_array.array.list_view_array.GenericListViewArray.md#op-e77399dcef65864e81be3a9a) and reset this builder.

<a id="op-c0f874a3baaa41099b1339d5"></a>
## finish

`function` · `arrow_array::builder::generic_list_view_builder::GenericListViewBuilder::finish` · arrow-array 59.3.0

```rust
fn finish(&mut self) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_list_view_builder::GenericListViewBuilder", "path": "GenericListViewBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [78, 2], "filename": "src/builder/generic_list_view_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/generic_list_view_builder.rs:66`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the array and reset this builder.

<a id="op-3f03905ed88edc8a24223d82"></a>
## finish_cloned

`function` · `arrow_array::builder::generic_list_view_builder::GenericListViewBuilder::finish_cloned` · arrow-array 59.3.0

```rust
fn finish_cloned(&self) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_list_view_builder::GenericListViewBuilder", "path": "GenericListViewBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [78, 2], "filename": "src/builder/generic_list_view_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/generic_list_view_builder.rs:71`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the array without resetting the builder.

<a id="op-ae2a44846d33785c27d46bf5"></a>
## finish_cloned

`function` · `arrow_array::builder::generic_list_view_builder::GenericListViewBuilder::finish_cloned` · arrow-array 59.3.0

```rust
fn finish_cloned(&self) -> GenericListViewArray<OffsetSize>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_list_view_builder::GenericListViewBuilder", "path": "GenericListViewBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [115, 1], "end": [244, 2], "filename": "src/builder/generic_list_view_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_list_view_builder.rs:204`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the [`GenericListViewArray`](../operations/arrow_array.array.list_view_array.GenericListViewArray.md#op-e77399dcef65864e81be3a9a) without resetting the builder.

<a id="op-e3415602eb7015aaa2ae9e2d"></a>
## finish_preserve_values

`function` · `arrow_array::builder::generic_list_view_builder::GenericListViewBuilder::finish_preserve_values` · arrow-array 59.3.0

```rust
fn finish_preserve_values(&mut self) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_list_view_builder::GenericListViewBuilder", "path": "GenericListViewBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [78, 2], "filename": "src/builder/generic_list_view_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/generic_list_view_builder.rs:75`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-57294c64b41c3c2c869b0aee"></a>
## fmt

`function` · `arrow_array::builder::generic_list_view_builder::GenericListViewBuilder::fmt` · arrow-array 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_list_view_builder::GenericListViewBuilder", "path": "GenericListViewBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 10], "end": [26, 15], "filename": "src/builder/generic_list_view_builder.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/builder/generic_list_view_builder.rs:26`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-65bb10359ad0b3a559b7d699"></a>
## into_box_any

`function` · `arrow_array::builder::generic_list_view_builder::GenericListViewBuilder::into_box_any` · arrow-array 59.3.0

```rust
fn into_box_any(Box<self>) -> Box<dyn Any>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_list_view_builder::GenericListViewBuilder", "path": "GenericListViewBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [78, 2], "filename": "src/builder/generic_list_view_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/generic_list_view_builder.rs:56`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the boxed builder as a box of `Any`.

<a id="op-f0ec445f62bf6e00aec4af98"></a>
## len

`function` · `arrow_array::builder::generic_list_view_builder::GenericListViewBuilder::len` · arrow-array 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_list_view_builder::GenericListViewBuilder", "path": "GenericListViewBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [78, 2], "filename": "src/builder/generic_list_view_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/generic_list_view_builder.rs:61`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the number of array slots in the builder

<a id="op-59b88cad75c48a6a0ac910f6"></a>
## new

`function` · `arrow_array::builder::generic_list_view_builder::GenericListViewBuilder::new` · arrow-array 59.3.0

```rust
fn new(values_builder: T) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_list_view_builder::GenericListViewBuilder", "path": "GenericListViewBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [80, 1], "end": [113, 2], "filename": "src/builder/generic_list_view_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_list_view_builder.rs:82`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new [`GenericListViewBuilder`](../operations/arrow_array.builder.generic_list_view_builder.GenericListViewBuilder.md#op-5ca7d134853cecdadbb30ee4) from a given values array builder

<a id="op-934ad10b46f31d0f195e2996"></a>
## offsets_slice

`function` · `arrow_array::builder::generic_list_view_builder::GenericListViewBuilder::offsets_slice` · arrow-array 59.3.0

```rust
fn offsets_slice(&self) -> &[OffsetSize]
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_list_view_builder::GenericListViewBuilder", "path": "GenericListViewBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [115, 1], "end": [244, 2], "filename": "src/builder/generic_list_view_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_list_view_builder.rs:241`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the current offsets buffer as a slice

<a id="op-71bdd41a67a34ca784ea8005"></a>
## values

`function` · `arrow_array::builder::generic_list_view_builder::GenericListViewBuilder::values` · arrow-array 59.3.0

```rust
fn values(&mut self) -> &mut T
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_list_view_builder::GenericListViewBuilder", "path": "GenericListViewBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [115, 1], "end": [244, 2], "filename": "src/builder/generic_list_view_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_list_view_builder.rs:123`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the child array builder as a mutable reference.

This mutable reference can be used to append values into the child array builder,
but you must call [`append`](#method.append) to delimit each distinct list value.

<a id="op-5cf8cb1b838323e4d57bfd23"></a>
## values_ref

`function` · `arrow_array::builder::generic_list_view_builder::GenericListViewBuilder::values_ref` · arrow-array 59.3.0

```rust
fn values_ref(&self) -> &T
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_list_view_builder::GenericListViewBuilder", "path": "GenericListViewBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [115, 1], "end": [244, 2], "filename": "src/builder/generic_list_view_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_list_view_builder.rs:128`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the child array builder as an immutable reference

<a id="op-4ee620896c76ebc9405d4d3a"></a>
## with_capacity

`function` · `arrow_array::builder::generic_list_view_builder::GenericListViewBuilder::with_capacity` · arrow-array 59.3.0

```rust
fn with_capacity(values_builder: T, capacity: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_list_view_builder::GenericListViewBuilder", "path": "GenericListViewBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [80, 1], "end": [113, 2], "filename": "src/builder/generic_list_view_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_list_view_builder.rs:89`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new [`GenericListViewBuilder`](../operations/arrow_array.builder.generic_list_view_builder.GenericListViewBuilder.md#op-5ca7d134853cecdadbb30ee4) from a given values array builder
`capacity` is the number of items to pre-allocate space for in this builder

<a id="op-76be9e709108293a2e114575"></a>
## with_field

`function` · `arrow_array::builder::generic_list_view_builder::GenericListViewBuilder::with_field` · arrow-array 59.3.0

```rust
fn with_field(self, field: impl Into<FieldRef>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::generic_list_view_builder::GenericListViewBuilder", "path": "GenericListViewBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [80, 1], "end": [113, 2], "filename": "src/builder/generic_list_view_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/generic_list_view_builder.rs:107`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).


By default a nullable field is created with the name `item`

Note: [`Self::finish`](../operations/arrow_array.builder.generic_list_view_builder.GenericListViewBuilder.md#op-1b9299ddf7ec3108f3bab382) and [`Self::finish_cloned`](../operations/arrow_array.builder.generic_list_view_builder.GenericListViewBuilder.md#op-ae2a44846d33785c27d46bf5) will panic if the
field's data type does not match that of `T`
