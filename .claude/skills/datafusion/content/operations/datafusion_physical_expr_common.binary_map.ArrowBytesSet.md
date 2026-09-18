# `datafusion_physical_expr_common::binary_map::ArrowBytesSet`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.binary_map.ArrowBytesSet.json).

<a id="op-7a4c9a98e6de3999c7bf14fd"></a>
## ArrowBytesSet

`struct` · `datafusion_physical_expr_common::binary_map::ArrowBytesSet` · datafusion-physical-expr-common 55.1.0

```rust
struct ArrowBytesSet<O: OffsetSizeTrait>
```

Source: `src/binary_map.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

HashSet optimized for storing string or binary values that can produce that
the final set as a GenericStringArray with minimal copies.

<a id="op-54622b10e583fcc4c538cbda"></a>
## fmt

`function` · `datafusion_physical_expr_common::binary_map::ArrowBytesSet::fmt` · datafusion-physical-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "datafusion_physical_expr_common::binary_map::ArrowBytesSet", "path": "ArrowBytesSet"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [53, 10], "end": [53, 15], "filename": "src/binary_map.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/binary_map.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7350cd376d18cf455fb1edcb"></a>
## insert

`function` · `datafusion_physical_expr_common::binary_map::ArrowBytesSet::insert` · datafusion-physical-expr-common 55.1.0

```rust
fn insert(&mut self, values: &ArrayRef)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "datafusion_physical_expr_common::binary_map::ArrowBytesSet", "path": "ArrowBytesSet"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [101, 2], "filename": "src/binary_map.rs"}, "trait": null, "trait_path": null}`

Source: `src/binary_map.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Inserts each value from `values` into the set

<a id="op-46b045979709d1b19d309d88"></a>
## into_state

`function` · `datafusion_physical_expr_common::binary_map::ArrowBytesSet::into_state` · datafusion-physical-expr-common 55.1.0

```rust
fn into_state(self) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "datafusion_physical_expr_common::binary_map::ArrowBytesSet", "path": "ArrowBytesSet"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [101, 2], "filename": "src/binary_map.rs"}, "trait": null, "trait_path": null}`

Source: `src/binary_map.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Converts this set into a `StringArray`/`LargeStringArray` or
`BinaryArray`/`LargeBinaryArray` containing each distinct value that
was interned. This is done without copying the values.

<a id="op-d0d322cdb68fec690f5d6729"></a>
## is_empty

`function` · `datafusion_physical_expr_common::binary_map::ArrowBytesSet::is_empty` · datafusion-physical-expr-common 55.1.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "datafusion_physical_expr_common::binary_map::ArrowBytesSet", "path": "ArrowBytesSet"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [101, 2], "filename": "src/binary_map.rs"}, "trait": null, "trait_path": null}`

Source: `src/binary_map.rs:87`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d65ff2e1f87b7d2d730230d8"></a>
## len

`function` · `datafusion_physical_expr_common::binary_map::ArrowBytesSet::len` · datafusion-physical-expr-common 55.1.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "datafusion_physical_expr_common::binary_map::ArrowBytesSet", "path": "ArrowBytesSet"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [101, 2], "filename": "src/binary_map.rs"}, "trait": null, "trait_path": null}`

Source: `src/binary_map.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Returns the total number of distinct values (including nulls) seen so far

<a id="op-177fe472d80f8c08f8ce48a3"></a>
## new

`function` · `datafusion_physical_expr_common::binary_map::ArrowBytesSet::new` · datafusion-physical-expr-common 55.1.0

```rust
fn new(output_type: OutputType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "datafusion_physical_expr_common::binary_map::ArrowBytesSet", "path": "ArrowBytesSet"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [101, 2], "filename": "src/binary_map.rs"}, "trait": null, "trait_path": null}`

Source: `src/binary_map.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5b9acd182d0d080f8f8731c7"></a>
## non_null_len

`function` · `datafusion_physical_expr_common::binary_map::ArrowBytesSet::non_null_len` · datafusion-physical-expr-common 55.1.0

```rust
fn non_null_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "datafusion_physical_expr_common::binary_map::ArrowBytesSet", "path": "ArrowBytesSet"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [101, 2], "filename": "src/binary_map.rs"}, "trait": null, "trait_path": null}`

Source: `src/binary_map.rs:92`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

returns the total number of distinct values (not including nulls) seen so far

<a id="op-8b52626f6bd59c10d2226d6b"></a>
## size

`function` · `datafusion_physical_expr_common::binary_map::ArrowBytesSet::size` · datafusion-physical-expr-common 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "datafusion_physical_expr_common::binary_map::ArrowBytesSet", "path": "ArrowBytesSet"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [101, 2], "filename": "src/binary_map.rs"}, "trait": null, "trait_path": null}`

Source: `src/binary_map.rs:98`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Return the total size, in bytes, of memory used to store the data in
this set, not including `self`

<a id="op-e26383fe231ff5247a614e59"></a>
## take

`function` · `datafusion_physical_expr_common::binary_map::ArrowBytesSet::take` · datafusion-physical-expr-common 55.1.0

```rust
fn take(&mut self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "datafusion_physical_expr_common::binary_map::ArrowBytesSet", "path": "ArrowBytesSet"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [101, 2], "filename": "src/binary_map.rs"}, "trait": null, "trait_path": null}`

Source: `src/binary_map.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Return the contents of this set and replace it with a new empty
set with the same output type
