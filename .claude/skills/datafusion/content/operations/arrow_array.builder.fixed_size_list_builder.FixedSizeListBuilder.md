# `arrow_array::builder::fixed_size_list_builder::FixedSizeListBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.builder.fixed_size_list_builder.FixedSizeListBuilder.json).

<a id="op-f08edffd6af5d83e255a4d2a"></a>
## FixedSizeListBuilder

`struct` · `arrow_array::builder::fixed_size_list_builder::FixedSizeListBuilder` · arrow-array 59.3.0

```rust
struct FixedSizeListBuilder<T: ArrayBuilder>
```

Source: `src/builder/fixed_size_list_builder.rs:65`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

 Builder for [`FixedSizeListArray`](../operations/arrow_array.array.fixed_size_list_array.FixedSizeListArray.md#op-4772cb4965341d875c1269ee)
```
use arrow_array::{builder::{Int32Builder, FixedSizeListBuilder}, Array, Int32Array};
let values_builder = Int32Builder::new();
let mut builder = FixedSizeListBuilder::new(values_builder, 3);

//  [[0, 1, 2], null, [3, null, 5], [6, 7, null]]
builder.values().append_value(0);
builder.values().append_value(1);
builder.values().append_value(2);
builder.append(true);
builder.values().append_null();
builder.values().append_null();
builder.values().append_null();
builder.append(false);
builder.values().append_value(3);
builder.values().append_null();
builder.values().append_value(5);
builder.append(true);
builder.values().append_value(6);
builder.values().append_value(7);
builder.values().append_null();
builder.append(true);
let list_array = builder.finish();
assert_eq!(
    *list_array.value(0),
    Int32Array::from(vec![Some(0), Some(1), Some(2)])
);
assert!(list_array.is_null(1));
assert_eq!(
    *list_array.value(2),
    Int32Array::from(vec![Some(3), None, Some(5)])
);
assert_eq!(
    *list_array.value(3),
    Int32Array::from(vec![Some(6), Some(7), None])
)
```


<a id="op-feeb8c35f96b10607d0a3b40"></a>
## append

`function` · `arrow_array::builder::fixed_size_list_builder::FixedSizeListBuilder::append` · arrow-array 59.3.0

```rust
fn append(&mut self, is_valid: bool)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::fixed_size_list_builder::FixedSizeListBuilder", "path": "FixedSizeListBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [149, 1], "end": [244, 2], "filename": "src/builder/fixed_size_list_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/fixed_size_list_builder.rs:168`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Finish the current fixed-length list array slot

<a id="op-b467db17496a0deb45ce2a0e"></a>
## as_any

`function` · `arrow_array::builder::fixed_size_list_builder::FixedSizeListBuilder::as_any` · arrow-array 59.3.0

```rust
fn as_any(&self) -> &dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::fixed_size_list_builder::FixedSizeListBuilder", "path": "FixedSizeListBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [110, 1], "end": [147, 2], "filename": "src/builder/fixed_size_list_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/fixed_size_list_builder.rs:115`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the builder as a non-mutable `Any` reference.

<a id="op-426e41ea608c13c9932506f3"></a>
## as_any_mut

`function` · `arrow_array::builder::fixed_size_list_builder::FixedSizeListBuilder::as_any_mut` · arrow-array 59.3.0

```rust
fn as_any_mut(&mut self) -> &mut dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::fixed_size_list_builder::FixedSizeListBuilder", "path": "FixedSizeListBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [110, 1], "end": [147, 2], "filename": "src/builder/fixed_size_list_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/fixed_size_list_builder.rs:120`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the builder as a mutable `Any` reference.

<a id="op-3dcba07e0b2fad7529947d6c"></a>
## finish

`function` · `arrow_array::builder::fixed_size_list_builder::FixedSizeListBuilder::finish` · arrow-array 59.3.0

```rust
fn finish(&mut self) -> FixedSizeListArray
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::fixed_size_list_builder::FixedSizeListBuilder", "path": "FixedSizeListBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [149, 1], "end": [244, 2], "filename": "src/builder/fixed_size_list_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/fixed_size_list_builder.rs:173`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the [`FixedSizeListBuilder`](../operations/arrow_array.builder.fixed_size_list_builder.FixedSizeListBuilder.md#op-f08edffd6af5d83e255a4d2a) and reset this builder.

<a id="op-62f80ac3cd9c881bd1a0449b"></a>
## finish

`function` · `arrow_array::builder::fixed_size_list_builder::FixedSizeListBuilder::finish` · arrow-array 59.3.0

```rust
fn finish(&mut self) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::fixed_size_list_builder::FixedSizeListBuilder", "path": "FixedSizeListBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [110, 1], "end": [147, 2], "filename": "src/builder/fixed_size_list_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/fixed_size_list_builder.rs:135`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the array and reset this builder.

<a id="op-32fde638e28124663121cf05"></a>
## finish_cloned

`function` · `arrow_array::builder::fixed_size_list_builder::FixedSizeListBuilder::finish_cloned` · arrow-array 59.3.0

```rust
fn finish_cloned(&self) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::fixed_size_list_builder::FixedSizeListBuilder", "path": "FixedSizeListBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [110, 1], "end": [147, 2], "filename": "src/builder/fixed_size_list_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/fixed_size_list_builder.rs:140`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the array without resetting the builder.

<a id="op-e3682c1238c33c8343995ac7"></a>
## finish_cloned

`function` · `arrow_array::builder::fixed_size_list_builder::FixedSizeListBuilder::finish_cloned` · arrow-array 59.3.0

```rust
fn finish_cloned(&self) -> FixedSizeListArray
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::fixed_size_list_builder::FixedSizeListBuilder", "path": "FixedSizeListBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [149, 1], "end": [244, 2], "filename": "src/builder/fixed_size_list_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/fixed_size_list_builder.rs:196`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the [`FixedSizeListBuilder`](../operations/arrow_array.builder.fixed_size_list_builder.FixedSizeListBuilder.md#op-f08edffd6af5d83e255a4d2a) without resetting the builder.

<a id="op-68c9c9543f03298438a21a71"></a>
## finish_preserve_values

`function` · `arrow_array::builder::fixed_size_list_builder::FixedSizeListBuilder::finish_preserve_values` · arrow-array 59.3.0

```rust
fn finish_preserve_values(&mut self) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::fixed_size_list_builder::FixedSizeListBuilder", "path": "FixedSizeListBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [110, 1], "end": [147, 2], "filename": "src/builder/fixed_size_list_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/fixed_size_list_builder.rs:144`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-af09486e3cf0a8b5c27c575d"></a>
## fmt

`function` · `arrow_array::builder::fixed_size_list_builder::FixedSizeListBuilder::fmt` · arrow-array 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::fixed_size_list_builder::FixedSizeListBuilder", "path": "FixedSizeListBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 10], "end": [64, 15], "filename": "src/builder/fixed_size_list_builder.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/builder/fixed_size_list_builder.rs:64`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5013500e27afad92da80e62f"></a>
## into_box_any

`function` · `arrow_array::builder::fixed_size_list_builder::FixedSizeListBuilder::into_box_any` · arrow-array 59.3.0

```rust
fn into_box_any(Box<self>) -> Box<dyn Any>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::fixed_size_list_builder::FixedSizeListBuilder", "path": "FixedSizeListBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [110, 1], "end": [147, 2], "filename": "src/builder/fixed_size_list_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/fixed_size_list_builder.rs:125`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the boxed builder as a box of `Any`.

<a id="op-121139467a99f0d60852381d"></a>
## len

`function` · `arrow_array::builder::fixed_size_list_builder::FixedSizeListBuilder::len` · arrow-array 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::fixed_size_list_builder::FixedSizeListBuilder", "path": "FixedSizeListBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [110, 1], "end": [147, 2], "filename": "src/builder/fixed_size_list_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/fixed_size_list_builder.rs:130`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the number of array slots in the builder

<a id="op-40ba78b7ffeb9b40a18a1211"></a>
## new

`function` · `arrow_array::builder::fixed_size_list_builder::FixedSizeListBuilder::new` · arrow-array 59.3.0

```rust
fn new(values_builder: T, value_length: i32) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::fixed_size_list_builder::FixedSizeListBuilder", "path": "FixedSizeListBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [108, 2], "filename": "src/builder/fixed_size_list_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/fixed_size_list_builder.rs:75`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new [`FixedSizeListBuilder`](../operations/arrow_array.builder.fixed_size_list_builder.FixedSizeListBuilder.md#op-f08edffd6af5d83e255a4d2a) from a given values array builder
`value_length` is the number of values within each array

<a id="op-817623f6b929c78caec25e9a"></a>
## validity_slice

`function` · `arrow_array::builder::fixed_size_list_builder::FixedSizeListBuilder::validity_slice` · arrow-array 59.3.0

```rust
fn validity_slice(&self) -> Option<&[u8]>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::fixed_size_list_builder::FixedSizeListBuilder", "path": "FixedSizeListBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [149, 1], "end": [244, 2], "filename": "src/builder/fixed_size_list_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/fixed_size_list_builder.rs:241`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the current null buffer as a slice

<a id="op-b2639d64d88b5f3888b14327"></a>
## value_length

`function` · `arrow_array::builder::fixed_size_list_builder::FixedSizeListBuilder::value_length` · arrow-array 59.3.0

```rust
fn value_length(&self) -> i32
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::fixed_size_list_builder::FixedSizeListBuilder", "path": "FixedSizeListBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [149, 1], "end": [244, 2], "filename": "src/builder/fixed_size_list_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/fixed_size_list_builder.rs:162`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the length of the list

<a id="op-9aba6e5fabebff927690aea1"></a>
## values

`function` · `arrow_array::builder::fixed_size_list_builder::FixedSizeListBuilder::values` · arrow-array 59.3.0

```rust
fn values(&mut self) -> &mut T
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::fixed_size_list_builder::FixedSizeListBuilder", "path": "FixedSizeListBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [149, 1], "end": [244, 2], "filename": "src/builder/fixed_size_list_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/fixed_size_list_builder.rs:157`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the child array builder as a mutable reference.

This mutable reference can be used to append values into the child array builder,
but you must call [`append`](#method.append) to delimit each distinct list value.

<a id="op-139dcbef3c36b806ec32f37f"></a>
## with_capacity

`function` · `arrow_array::builder::fixed_size_list_builder::FixedSizeListBuilder::with_capacity` · arrow-array 59.3.0

```rust
fn with_capacity(values_builder: T, value_length: i32, capacity: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::fixed_size_list_builder::FixedSizeListBuilder", "path": "FixedSizeListBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [108, 2], "filename": "src/builder/fixed_size_list_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/fixed_size_list_builder.rs:87`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new [`FixedSizeListBuilder`](../operations/arrow_array.builder.fixed_size_list_builder.FixedSizeListBuilder.md#op-f08edffd6af5d83e255a4d2a) from a given values array builder
`value_length` is the number of values within each array
`capacity` is the number of items to pre-allocate space for in this builder

<a id="op-ce795f9cca1d1e6d884b6085"></a>
## with_field

`function` · `arrow_array::builder::fixed_size_list_builder::FixedSizeListBuilder::with_field` · arrow-array 59.3.0

```rust
fn with_field(self, field: impl Into<FieldRef>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::builder::fixed_size_list_builder::FixedSizeListBuilder", "path": "FixedSizeListBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [108, 2], "filename": "src/builder/fixed_size_list_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/fixed_size_list_builder.rs:102`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Override the field passed to [`FixedSizeListArray::new`](../operations/arrow_array.array.fixed_size_list_array.FixedSizeListArray.md#op-16fc84a521d8d5b7badac796)

By default, a nullable field is created with the name `item`

Note: [`Self::finish`](../operations/arrow_array.builder.fixed_size_list_builder.FixedSizeListBuilder.md#op-3dcba07e0b2fad7529947d6c) and [`Self::finish_cloned`](../operations/arrow_array.builder.fixed_size_list_builder.FixedSizeListBuilder.md#op-e3682c1238c33c8343995ac7) will panic if the
field's data type does not match that of `T`
