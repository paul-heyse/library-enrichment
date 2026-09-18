# `arrow_array::builder::primitive_builder::PrimitiveBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.builder.primitive_builder.PrimitiveBuilder.json).

<a id="op-ad7b023d1f59a6bba4232467"></a>
## PrimitiveBuilder

`struct` · `arrow_array::builder::primitive_builder::PrimitiveBuilder` · arrow-array 59.3.0

```rust
struct PrimitiveBuilder<T: ArrowPrimitiveType>
```

Source: `src/builder/primitive_builder.rs:100`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builder for [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814)

<a id="op-59a9cebbdac2f6f8c89e56f9"></a>
## append_array

`function` · `arrow_array::builder::primitive_builder::PrimitiveBuilder::append_array` · arrow-array 59.3.0

```rust
fn append_array(&mut self, array: &PrimitiveArray<T>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_builder::PrimitiveBuilder", "path": "PrimitiveBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [381, 2], "filename": "src/builder/primitive_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_builder.rs:290`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends array values and null to this builder as is
(this means that underlying null values are copied as is).

# Panics

Panics if `array` and `self` data types are different

<a id="op-a301d458a1365b7b3554371d"></a>
## append_null

`function` · `arrow_array::builder::primitive_builder::PrimitiveBuilder::append_null` · arrow-array 59.3.0

```rust
fn append_null(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_builder::PrimitiveBuilder", "path": "PrimitiveBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [381, 2], "filename": "src/builder/primitive_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_builder.rs:218`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends a null slot into the builder

<a id="op-1452db6e288c810115e2c5c5"></a>
## append_nulls

`function` · `arrow_array::builder::primitive_builder::PrimitiveBuilder::append_nulls` · arrow-array 59.3.0

```rust
fn append_nulls(&mut self, n: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_builder::PrimitiveBuilder", "path": "PrimitiveBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [381, 2], "filename": "src/builder/primitive_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_builder.rs:225`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends `n` no. of null's into the builder

<a id="op-006123ab8c3c6e514d7ed079"></a>
## append_option

`function` · `arrow_array::builder::primitive_builder::PrimitiveBuilder::append_option` · arrow-array 59.3.0

```rust
fn append_option(&mut self, v: Option<T::Native>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_builder::PrimitiveBuilder", "path": "PrimitiveBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [381, 2], "filename": "src/builder/primitive_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_builder.rs:233`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends an `Option<T>` into the builder

<a id="op-1b31aa6a96ae9d0979c5184a"></a>
## append_slice

`function` · `arrow_array::builder::primitive_builder::PrimitiveBuilder::append_slice` · arrow-array 59.3.0

```rust
fn append_slice(&mut self, v: &[T::Native])
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_builder::PrimitiveBuilder", "path": "PrimitiveBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [381, 2], "filename": "src/builder/primitive_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_builder.rs:242`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends a slice of type `T` into the builder

<a id="op-f5d4b65f26ae86669b110c37"></a>
## append_trusted_len_iter

`function` · `arrow_array::builder::primitive_builder::PrimitiveBuilder::append_trusted_len_iter` · arrow-array 59.3.0

```rust
unsafe fn append_trusted_len_iter(&mut self, iter: impl IntoIterator<Item = T::Native>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_builder::PrimitiveBuilder", "path": "PrimitiveBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [381, 2], "filename": "src/builder/primitive_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_builder.rs:311`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends values from a trusted length iterator.

# Safety
This requires the iterator be a trusted length. This could instead require
the iterator implement `TrustedLen` once that is stabilized.

<a id="op-1e4845c61a984b4af355ecd0"></a>
## append_value

`function` · `arrow_array::builder::primitive_builder::PrimitiveBuilder::append_value` · arrow-array 59.3.0

```rust
fn append_value(&mut self, v: T::Native)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_builder::PrimitiveBuilder", "path": "PrimitiveBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [381, 2], "filename": "src/builder/primitive_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_builder.rs:204`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends a value of type `T` into the builder

<a id="op-1d82d0a674568e53f55f6ec7"></a>
## append_value_n

`function` · `arrow_array::builder::primitive_builder::PrimitiveBuilder::append_value_n` · arrow-array 59.3.0

```rust
fn append_value_n(&mut self, v: T::Native, n: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_builder::PrimitiveBuilder", "path": "PrimitiveBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [381, 2], "filename": "src/builder/primitive_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_builder.rs:211`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends a value of type `T` into the builder `n` times

<a id="op-e302bec5e7ec863e666db607"></a>
## append_values

`function` · `arrow_array::builder::primitive_builder::PrimitiveBuilder::append_values` · arrow-array 59.3.0

```rust
fn append_values(&mut self, values: &[T::Native], is_valid: &[bool])
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_builder::PrimitiveBuilder", "path": "PrimitiveBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [381, 2], "filename": "src/builder/primitive_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_builder.rs:253`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends values from a slice of type `T` and a validity boolean slice

# Panics

Panics if `values` and `is_valid` have different lengths

<a id="op-9fea3dfe9a61d6a12a138801"></a>
## as_any

`function` · `arrow_array::builder::primitive_builder::PrimitiveBuilder::as_any` · arrow-array 59.3.0

```rust
fn as_any(&self) -> &dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_builder::PrimitiveBuilder", "path": "PrimitiveBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [106, 1], "end": [136, 2], "filename": "src/builder/primitive_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/primitive_builder.rs:108`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the builder as a non-mutable `Any` reference.

<a id="op-b5ed16b3377c72ff57c47613"></a>
## as_any_mut

`function` · `arrow_array::builder::primitive_builder::PrimitiveBuilder::as_any_mut` · arrow-array 59.3.0

```rust
fn as_any_mut(&mut self) -> &mut dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_builder::PrimitiveBuilder", "path": "PrimitiveBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [106, 1], "end": [136, 2], "filename": "src/builder/primitive_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/primitive_builder.rs:113`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the builder as a mutable `Any` reference.

<a id="op-f841f2f8e62513aa6403a5b5"></a>
## capacity

`function` · `arrow_array::builder::primitive_builder::PrimitiveBuilder::capacity` · arrow-array 59.3.0

```rust
fn capacity(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_builder::PrimitiveBuilder", "path": "PrimitiveBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [381, 2], "filename": "src/builder/primitive_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_builder.rs:198`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the capacity of this builder measured in slots of type `T`

<a id="op-5118932050f76ec8d2428af1"></a>
## default

`function` · `arrow_array::builder::primitive_builder::PrimitiveBuilder::default` · arrow-array 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_builder::PrimitiveBuilder", "path": "PrimitiveBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [138, 1], "end": [142, 2], "filename": "src/builder/primitive_builder.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/builder/primitive_builder.rs:139`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0e7b4146b2de4c755fc8797d"></a>
## extend

`function` · `arrow_array::builder::primitive_builder::PrimitiveBuilder::extend` · arrow-array 59.3.0

```rust
fn extend<T: IntoIterator<Item = Option<P::Native>>>(&mut self, iter: T)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "P"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_builder::PrimitiveBuilder", "path": "PrimitiveBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "P"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [409, 1], "end": [416, 2], "filename": "src/builder/primitive_builder.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "P"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "core::iter::traits::collect::Extend", "path": "Extend"}, "trait_path": "core::iter::traits::collect::Extend"}`

Source: `src/builder/primitive_builder.rs:411`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6f47b0ded790fca0148f0fc7"></a>
## extend_from_iter_option

`function` · `arrow_array::builder::primitive_builder::PrimitiveBuilder::extend_from_iter_option` · arrow-array 59.3.0

```rust
fn extend_from_iter_option<I: IntoIterator<Item = Option<T::Native>>>(&mut self, iter: I)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_builder::PrimitiveBuilder", "path": "PrimitiveBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [381, 2], "filename": "src/builder/primitive_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_builder.rs:269`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends values from a iter of type `Option<T>`

# Panics

Panics if `values` and `is_valid` have different lengths

<a id="op-4a8e3f91c595dae5382850af"></a>
## finish

`function` · `arrow_array::builder::primitive_builder::PrimitiveBuilder::finish` · arrow-array 59.3.0

```rust
fn finish(&mut self) -> PrimitiveArray<T>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_builder::PrimitiveBuilder", "path": "PrimitiveBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [381, 2], "filename": "src/builder/primitive_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_builder.rs:323`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) and reset this builder.

<a id="op-b8dd5092eade341827e00646"></a>
## finish

`function` · `arrow_array::builder::primitive_builder::PrimitiveBuilder::finish` · arrow-array 59.3.0

```rust
fn finish(&mut self) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_builder::PrimitiveBuilder", "path": "PrimitiveBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [106, 1], "end": [136, 2], "filename": "src/builder/primitive_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/primitive_builder.rs:128`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the array and reset this builder.

<a id="op-601d29b8c2e688738a27d85e"></a>
## finish_cloned

`function` · `arrow_array::builder::primitive_builder::PrimitiveBuilder::finish_cloned` · arrow-array 59.3.0

```rust
fn finish_cloned(&self) -> PrimitiveArray<T>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_builder::PrimitiveBuilder", "path": "PrimitiveBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [381, 2], "filename": "src/builder/primitive_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_builder.rs:336`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) without resetting the builder.

<a id="op-d3229a70f30307936739d615"></a>
## finish_cloned

`function` · `arrow_array::builder::primitive_builder::PrimitiveBuilder::finish_cloned` · arrow-array 59.3.0

```rust
fn finish_cloned(&self) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_builder::PrimitiveBuilder", "path": "PrimitiveBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [106, 1], "end": [136, 2], "filename": "src/builder/primitive_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/primitive_builder.rs:133`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the array without resetting the builder.

<a id="op-31dc34dcaecc5da08dcc5fc0"></a>
## fmt

`function` · `arrow_array::builder::primitive_builder::PrimitiveBuilder::fmt` · arrow-array 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_builder::PrimitiveBuilder", "path": "PrimitiveBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}}}]}, "is_negative": false, "span": {"begin": [99, 10], "end": [99, 15], "filename": "src/builder/primitive_builder.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/builder/primitive_builder.rs:99`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-645e0c2107262297668c4ff4"></a>
## into_box_any

`function` · `arrow_array::builder::primitive_builder::PrimitiveBuilder::into_box_any` · arrow-array 59.3.0

```rust
fn into_box_any(Box<self>) -> Box<dyn Any>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_builder::PrimitiveBuilder", "path": "PrimitiveBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [106, 1], "end": [136, 2], "filename": "src/builder/primitive_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/primitive_builder.rs:118`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the boxed builder as a box of `Any`.

<a id="op-d0ffdc6a1f7edfca0cac5b50"></a>
## len

`function` · `arrow_array::builder::primitive_builder::PrimitiveBuilder::len` · arrow-array 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_builder::PrimitiveBuilder", "path": "PrimitiveBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [106, 1], "end": [136, 2], "filename": "src/builder/primitive_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/primitive_builder.rs:123`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the number of array slots in the builder

<a id="op-27e52abca1c11ee22ba0292c"></a>
## new

`function` · `arrow_array::builder::primitive_builder::PrimitiveBuilder::new` · arrow-array 59.3.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_builder::PrimitiveBuilder", "path": "PrimitiveBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [381, 2], "filename": "src/builder/primitive_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_builder.rs:146`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new primitive array builder

<a id="op-ffb2118682a1c2137e9402a8"></a>
## new_from_buffer

`function` · `arrow_array::builder::primitive_builder::PrimitiveBuilder::new_from_buffer` · arrow-array 59.3.0

```rust
fn new_from_buffer(values_buffer: MutableBuffer, null_buffer: Option<MutableBuffer>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_builder::PrimitiveBuilder", "path": "PrimitiveBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [381, 2], "filename": "src/builder/primitive_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_builder.rs:160`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new primitive array builder from buffers

<a id="op-fbb31ef67bad1bd52e1cdefa"></a>
## slices_mut

`function` · `arrow_array::builder::primitive_builder::PrimitiveBuilder::slices_mut` · arrow-array 59.3.0

```rust
fn slices_mut(&mut self) -> (&mut [T::Native], Option<&mut [u8]>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_builder::PrimitiveBuilder", "path": "PrimitiveBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [381, 2], "filename": "src/builder/primitive_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_builder.rs:375`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the current values buffer and null buffer as a slice

<a id="op-9fea2c8e58ca44157a02a2ef"></a>
## validity_capacity

`function` · `arrow_array::builder::primitive_builder::PrimitiveBuilder::validity_capacity` · arrow-array 59.3.0

```rust
fn validity_capacity(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_builder::PrimitiveBuilder", "path": "PrimitiveBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [381, 2], "filename": "src/builder/primitive_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_builder.rs:365`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the current null buffer allocated capacity, in bytes.

<a id="op-c6684ec098954f423c8a571a"></a>
## validity_slice

`function` · `arrow_array::builder::primitive_builder::PrimitiveBuilder::validity_slice` · arrow-array 59.3.0

```rust
fn validity_slice(&self) -> Option<&[u8]>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_builder::PrimitiveBuilder", "path": "PrimitiveBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [381, 2], "filename": "src/builder/primitive_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_builder.rs:360`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the current null buffer as a slice

<a id="op-6c66750870ea8c1fb33e94c8"></a>
## validity_slice_mut

`function` · `arrow_array::builder::primitive_builder::PrimitiveBuilder::validity_slice_mut` · arrow-array 59.3.0

```rust
fn validity_slice_mut(&mut self) -> Option<&mut [u8]>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_builder::PrimitiveBuilder", "path": "PrimitiveBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [381, 2], "filename": "src/builder/primitive_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_builder.rs:370`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the current null buffer as a mutable slice

<a id="op-59cbcef2b5859534a5afe65c"></a>
## values_slice

`function` · `arrow_array::builder::primitive_builder::PrimitiveBuilder::values_slice` · arrow-array 59.3.0

```rust
fn values_slice(&self) -> &[T::Native]
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_builder::PrimitiveBuilder", "path": "PrimitiveBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [381, 2], "filename": "src/builder/primitive_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_builder.rs:350`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the current values buffer as a slice

<a id="op-a4865f82ed557128efc79c60"></a>
## values_slice_mut

`function` · `arrow_array::builder::primitive_builder::PrimitiveBuilder::values_slice_mut` · arrow-array 59.3.0

```rust
fn values_slice_mut(&mut self) -> &mut [T::Native]
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_builder::PrimitiveBuilder", "path": "PrimitiveBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [381, 2], "filename": "src/builder/primitive_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_builder.rs:355`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the current values buffer as a mutable slice

<a id="op-e48ba438bd55d35edfa48ec4"></a>
## with_capacity

`function` · `arrow_array::builder::primitive_builder::PrimitiveBuilder::with_capacity` · arrow-array 59.3.0

```rust
fn with_capacity(capacity: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_builder::PrimitiveBuilder", "path": "PrimitiveBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [381, 2], "filename": "src/builder/primitive_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_builder.rs:151`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new primitive array builder with capacity no of items

<a id="op-40b7ce7dfb700453ed66a1c4"></a>
## with_data_type

`function` · `arrow_array::builder::primitive_builder::PrimitiveBuilder::with_data_type` · arrow-array 59.3.0

```rust
fn with_data_type(self, data_type: DataType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_builder::PrimitiveBuilder", "path": "PrimitiveBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [381, 2], "filename": "src/builder/primitive_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_builder.rs:187`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

By default [`PrimitiveBuilder`](../operations/arrow_array.builder.primitive_builder.PrimitiveBuilder.md#op-ad7b023d1f59a6bba4232467) uses [`ArrowPrimitiveType::DATA_TYPE`](../operations/arrow_array.types.ArrowPrimitiveType.md#op-4e3fd55a44d0ecd0ff3d8d43) as the
data type of the generated array.

This method allows overriding the data type, to allow specifying timezones
for [`DataType::Timestamp`](../operations/arrow_schema.datatype.DataType.md#op-4311beb64d86f8afbc59cba2) or precision and scale for [`DataType::Decimal32`](../operations/arrow_schema.datatype.DataType.md#op-78b7678812b92ef892ce6b0f),
[`DataType::Decimal64`](../operations/arrow_schema.datatype.DataType.md#op-604a2f94cb7e1a898e3be66a), [`DataType::Decimal128`](../operations/arrow_schema.datatype.DataType.md#op-e54295a8e8bbe3cefae79bcd) and [`DataType::Decimal256`](../operations/arrow_schema.datatype.DataType.md#op-fe9990d44d3e962e1dadd391)

# Panics

This method panics if `data_type` is not [PrimitiveArray::is_compatible](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-041b60c471dc2d350eaf9f88)

<a id="op-d9dc0394d9173e79ca8500fd"></a>
## with_precision_and_scale

`function` · `arrow_array::builder::primitive_builder::PrimitiveBuilder::with_precision_and_scale` · arrow-array 59.3.0

```rust
fn with_precision_and_scale(self, precision: u8, scale: i8) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "P"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_builder::PrimitiveBuilder", "path": "PrimitiveBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::DecimalType", "path": "DecimalType"}}}], "default": null, "is_synthetic": false}}, "name": "P"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [383, 1], "end": [392, 2], "filename": "src/builder/primitive_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_builder.rs:385`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Sets the precision and scale

<a id="op-b031ac40a4410cc81cea1ca9"></a>
## with_timezone

`function` · `arrow_array::builder::primitive_builder::PrimitiveBuilder::with_timezone` · arrow-array 59.3.0

```rust
fn with_timezone(self, timezone: impl Into<Arc<str>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "P"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_builder::PrimitiveBuilder", "path": "PrimitiveBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowTimestampType", "path": "ArrowTimestampType"}}}], "default": null, "is_synthetic": false}}, "name": "P"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [394, 1], "end": [407, 2], "filename": "src/builder/primitive_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_builder.rs:396`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Sets the timezone

<a id="op-a1c4d3d68464d8d0f927b5e5"></a>
## with_timezone_opt

`function` · `arrow_array::builder::primitive_builder::PrimitiveBuilder::with_timezone_opt` · arrow-array 59.3.0

```rust
fn with_timezone_opt<S: Into<Arc<str>>>(self, timezone: Option<S>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "P"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_builder::PrimitiveBuilder", "path": "PrimitiveBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowTimestampType", "path": "ArrowTimestampType"}}}], "default": null, "is_synthetic": false}}, "name": "P"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [394, 1], "end": [407, 2], "filename": "src/builder/primitive_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_builder.rs:401`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Sets an optional timezone
