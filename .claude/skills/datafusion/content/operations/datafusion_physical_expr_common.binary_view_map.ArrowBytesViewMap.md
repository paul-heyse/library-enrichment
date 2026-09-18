# `datafusion_physical_expr_common::binary_view_map::ArrowBytesViewMap`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.binary_view_map.ArrowBytesViewMap.json).

<a id="op-243032a0359f8c33282db59a"></a>
## ArrowBytesViewMap

`struct` · `datafusion_physical_expr_common::binary_view_map::ArrowBytesViewMap` · datafusion-physical-expr-common 55.1.0

```rust
struct ArrowBytesViewMap<V> where V: Debug + PartialEq + Eq + Clone + Copy + Default
```

Source: `src/binary_view_map.rs:120`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-521461c6238e9367f783b8e1"></a>
## fmt

`function` · `datafusion_physical_expr_common::binary_view_map::ArrowBytesViewMap::fmt` · datafusion-physical-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_physical_expr_common::binary_view_map::ArrowBytesViewMap", "path": "ArrowBytesViewMap"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Copy", "path": "Copy"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::default::Default", "path": "Default"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [488, 1], "end": [502, 2], "filename": "src/binary_view_map.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/binary_view_map.rs:492`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4f1b9b6f854defc0527398aa"></a>
## insert_if_new

`function` · `datafusion_physical_expr_common::binary_view_map::ArrowBytesViewMap::insert_if_new` · datafusion-physical-expr-common 55.1.0

```rust
fn insert_if_new<MP, OP>(&mut self, values: &ArrayRef, make_payload_fn: MP, observe_payload_fn: OP) where MP: FnMut(Option<&[u8]>) -> V, OP: FnMut(V)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_physical_expr_common::binary_view_map::ArrowBytesViewMap", "path": "ArrowBytesViewMap"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Copy", "path": "Copy"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::default::Default", "path": "Default"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [153, 1], "end": [486, 2], "filename": "src/binary_view_map.rs"}, "trait": null, "trait_path": null}`

Source: `src/binary_view_map.rs:206`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Inserts each value from `values` into the map, invoking `payload_fn` for
each value if *not* already present, deferring the allocation of the
payload until it is needed.

Note that this is different than a normal map that would replace the
existing entry

# Arguments:

`values`: array whose values are inserted

`make_payload_fn`:  invoked for each value that is not already present
to create the payload, in order of the values in `values`

`observe_payload_fn`: invoked once, for each value in `values`, that was
already present in the map, with corresponding payload value.

# Returns

The payload value for the entry, either the existing value or
the newly inserted value

# Safety:

Note that `make_payload_fn` and `observe_payload_fn` are only invoked
with valid values from `values`, not for the `NULL` value.

<a id="op-6a6b963f820820735366915e"></a>
## into_state

`function` · `datafusion_physical_expr_common::binary_view_map::ArrowBytesViewMap::into_state` · datafusion-physical-expr-common 55.1.0

```rust
fn into_state(self) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_physical_expr_common::binary_view_map::ArrowBytesViewMap", "path": "ArrowBytesViewMap"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Copy", "path": "Copy"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::default::Default", "path": "Default"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [153, 1], "end": [486, 2], "filename": "src/binary_view_map.rs"}, "trait": null, "trait_path": null}`

Source: `src/binary_view_map.rs:384`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Converts this set into a `StringViewArray`, or `BinaryViewArray`,
containing each distinct value
that was inserted. This is done without copying the values.

The values are guaranteed to be returned in the same order in which
they were first seen.

<a id="op-b0d3ab0a56baa7feeb58b61d"></a>
## is_empty

`function` · `datafusion_physical_expr_common::binary_view_map::ArrowBytesViewMap::is_empty` · datafusion-physical-expr-common 55.1.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_physical_expr_common::binary_view_map::ArrowBytesViewMap", "path": "ArrowBytesViewMap"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Copy", "path": "Copy"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::default::Default", "path": "Default"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [153, 1], "end": [486, 2], "filename": "src/binary_view_map.rs"}, "trait": null, "trait_path": null}`

Source: `src/binary_view_map.rs:462`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Is the set empty?

<a id="op-6ef769bf5ae5b0af5c2a5603"></a>
## len

`function` · `datafusion_physical_expr_common::binary_view_map::ArrowBytesViewMap::len` · datafusion-physical-expr-common 55.1.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_physical_expr_common::binary_view_map::ArrowBytesViewMap", "path": "ArrowBytesViewMap"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Copy", "path": "Copy"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::default::Default", "path": "Default"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [153, 1], "end": [486, 2], "filename": "src/binary_view_map.rs"}, "trait": null, "trait_path": null}`

Source: `src/binary_view_map.rs:457`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Total number of entries (including null, if present)

<a id="op-89c464aaa877b8ea3c671b55"></a>
## new

`function` · `datafusion_physical_expr_common::binary_view_map::ArrowBytesViewMap::new` · datafusion-physical-expr-common 55.1.0

```rust
fn new(output_type: OutputType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_physical_expr_common::binary_view_map::ArrowBytesViewMap", "path": "ArrowBytesViewMap"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Copy", "path": "Copy"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::default::Default", "path": "Default"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [153, 1], "end": [486, 2], "filename": "src/binary_view_map.rs"}, "trait": null, "trait_path": null}`

Source: `src/binary_view_map.rs:157`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-00ca0da637be99435abc1940"></a>
## non_null_len

`function` · `datafusion_physical_expr_common::binary_view_map::ArrowBytesViewMap::non_null_len` · datafusion-physical-expr-common 55.1.0

```rust
fn non_null_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_physical_expr_common::binary_view_map::ArrowBytesViewMap", "path": "ArrowBytesViewMap"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Copy", "path": "Copy"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::default::Default", "path": "Default"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [153, 1], "end": [486, 2], "filename": "src/binary_view_map.rs"}, "trait": null, "trait_path": null}`

Source: `src/binary_view_map.rs:467`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Number of non null entries

<a id="op-87aa2235ec1a23bdfddeff3f"></a>
## size

`function` · `datafusion_physical_expr_common::binary_view_map::ArrowBytesViewMap::size` · datafusion-physical-expr-common 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_physical_expr_common::binary_view_map::ArrowBytesViewMap", "path": "ArrowBytesViewMap"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Copy", "path": "Copy"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::default::Default", "path": "Default"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [153, 1], "end": [486, 2], "filename": "src/binary_view_map.rs"}, "trait": null, "trait_path": null}`

Source: `src/binary_view_map.rs:473`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Return the total size, in bytes, of memory used to store the data in
this set, not including `self`

<a id="op-e76ebf8b5fec89835a46218e"></a>
## take

`function` · `datafusion_physical_expr_common::binary_view_map::ArrowBytesViewMap::take` · datafusion-physical-expr-common 55.1.0

```rust
fn take(&mut self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_physical_expr_common::binary_view_map::ArrowBytesViewMap", "path": "ArrowBytesViewMap"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Copy", "path": "Copy"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::default::Default", "path": "Default"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [153, 1], "end": [486, 2], "filename": "src/binary_view_map.rs"}, "trait": null, "trait_path": null}`

Source: `src/binary_view_map.rs:174`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Return the contents of this map and replace it with a new empty map with
the same output type
