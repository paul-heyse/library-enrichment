# `datafusion_physical_expr_common::binary_view_map::ArrowBytesViewSet`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.binary_view_map.ArrowBytesViewSet.json).

<a id="op-d7b17f5b34ca9a2c511e979b"></a>
## ArrowBytesViewSet

`struct` · `datafusion_physical_expr_common::binary_view_map::ArrowBytesViewSet` · datafusion-physical-expr-common 55.1.0

```rust
struct ArrowBytesViewSet
```

Source: `src/binary_view_map.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

HashSet optimized for storing string or binary values that can produce that
the final set as a `GenericBinaryViewArray` with minimal copies.

<a id="op-6263c26bf867557735b4b299"></a>
## fmt

`function` · `datafusion_physical_expr_common::binary_view_map::ArrowBytesViewSet::fmt` · datafusion-physical-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::binary_view_map::ArrowBytesViewSet", "path": "ArrowBytesViewSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 10], "end": [35, 15], "filename": "src/binary_view_map.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/binary_view_map.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-889eb9891207f5cca08518e9"></a>
## insert

`function` · `datafusion_physical_expr_common::binary_view_map::ArrowBytesViewSet::insert` · datafusion-physical-expr-common 55.1.0

```rust
fn insert(&mut self, values: &ArrayRef)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::binary_view_map::ArrowBytesViewSet", "path": "ArrowBytesViewSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 1], "end": [85, 2], "filename": "src/binary_view_map.rs"}, "trait": null, "trait_path": null}`

Source: `src/binary_view_map.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Inserts each value from `values` into the set

<a id="op-e2c7bc84bd2fa7540300f9de"></a>
## into_state

`function` · `datafusion_physical_expr_common::binary_view_map::ArrowBytesViewSet::into_state` · datafusion-physical-expr-common 55.1.0

```rust
fn into_state(self) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::binary_view_map::ArrowBytesViewSet", "path": "ArrowBytesViewSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 1], "end": [85, 2], "filename": "src/binary_view_map.rs"}, "trait": null, "trait_path": null}`

Source: `src/binary_view_map.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Converts this set into a `StringViewArray` or `BinaryViewArray`
containing each distinct value that was interned.
This is done without copying the values.

<a id="op-4bfb7343776682e653a0730a"></a>
## is_empty

`function` · `datafusion_physical_expr_common::binary_view_map::ArrowBytesViewSet::is_empty` · datafusion-physical-expr-common 55.1.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::binary_view_map::ArrowBytesViewSet", "path": "ArrowBytesViewSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 1], "end": [85, 2], "filename": "src/binary_view_map.rs"}, "trait": null, "trait_path": null}`

Source: `src/binary_view_map.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d23bbdc2e9c484d83f300e7d"></a>
## len

`function` · `datafusion_physical_expr_common::binary_view_map::ArrowBytesViewSet::len` · datafusion-physical-expr-common 55.1.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::binary_view_map::ArrowBytesViewSet", "path": "ArrowBytesViewSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 1], "end": [85, 2], "filename": "src/binary_view_map.rs"}, "trait": null, "trait_path": null}`

Source: `src/binary_view_map.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Returns the total number of distinct values (including nulls) seen so far

<a id="op-12e845cab466a07d9db8ecbc"></a>
## new

`function` · `datafusion_physical_expr_common::binary_view_map::ArrowBytesViewSet::new` · datafusion-physical-expr-common 55.1.0

```rust
fn new(output_type: OutputType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::binary_view_map::ArrowBytesViewSet", "path": "ArrowBytesViewSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 1], "end": [85, 2], "filename": "src/binary_view_map.rs"}, "trait": null, "trait_path": null}`

Source: `src/binary_view_map.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0af32def69e4ea1cfcefa965"></a>
## non_null_len

`function` · `datafusion_physical_expr_common::binary_view_map::ArrowBytesViewSet::non_null_len` · datafusion-physical-expr-common 55.1.0

```rust
fn non_null_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::binary_view_map::ArrowBytesViewSet", "path": "ArrowBytesViewSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 1], "end": [85, 2], "filename": "src/binary_view_map.rs"}, "trait": null, "trait_path": null}`

Source: `src/binary_view_map.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

returns the total number of distinct values (not including nulls) seen so far

<a id="op-113043325d684385e868ac77"></a>
## size

`function` · `datafusion_physical_expr_common::binary_view_map::ArrowBytesViewSet::size` · datafusion-physical-expr-common 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::binary_view_map::ArrowBytesViewSet", "path": "ArrowBytesViewSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 1], "end": [85, 2], "filename": "src/binary_view_map.rs"}, "trait": null, "trait_path": null}`

Source: `src/binary_view_map.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Return the total size, in bytes, of memory used to store the data in
this set, not including `self`

<a id="op-f8fe6c8a7d9b0849b8f66dab"></a>
## take

`function` · `datafusion_physical_expr_common::binary_view_map::ArrowBytesViewSet::take` · datafusion-physical-expr-common 55.1.0

```rust
fn take(&mut self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::binary_view_map::ArrowBytesViewSet", "path": "ArrowBytesViewSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 1], "end": [85, 2], "filename": "src/binary_view_map.rs"}, "trait": null, "trait_path": null}`

Source: `src/binary_view_map.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Return the contents of this map and replace it with a new empty map with
the same output type
