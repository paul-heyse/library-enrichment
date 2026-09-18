# `datafusion_physical_expr_common::binary_map::ArrowBytesMap`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.binary_map.ArrowBytesMap.json).

<a id="op-447f14acc2b0df76e4cd3de1"></a>
## ArrowBytesMap

`struct` · `datafusion_physical_expr_common::binary_map::ArrowBytesMap` · datafusion-physical-expr-common 55.1.0

```rust
struct ArrowBytesMap<O, V> where O: OffsetSizeTrait, V: Debug + PartialEq + Eq + Clone + Copy + Default
```

Source: `src/binary_map.rs:210`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Optimized map for storing Arrow "bytes" types (`String`, `LargeString`,
`Binary`, and `LargeBinary`) values that can produce the set of keys on
output as `GenericBinaryArray` without copies.

Equivalent to `HashSet<String, V>` but with better performance if you need
to emit the keys as an Arrow `StringArray` / `BinaryArray`. For other
purposes it is the same as a `HashMap<String, V>`

# Generic Arguments

* `O`: OffsetSize (String/LargeString)
* `V`: payload type

# Description

This is a specialized HashMap with the following properties:

1. Optimized for storing and emitting Arrow byte types  (e.g.
   `StringArray` / `BinaryArray`) very efficiently by minimizing copying of
   the string values themselves, both when inserting and when emitting the
   final array.


2. Retains the insertion order of entries in the final array. The values are
   in the same order as they were inserted.

Note this structure can be used as a `HashSet` by specifying the value type
as `()`, as is done by [`ArrowBytesSet`](../operations/datafusion_physical_expr_common.binary_map.ArrowBytesSet.md#op-7a4c9a98e6de3999c7bf14fd).

This map is used by the special `COUNT DISTINCT` aggregate function to
store the distinct values, and by the `GROUP BY` operator to store
group values when they are a single string array.

# Example

The following diagram shows how the map would store the four strings
"Foo", NULL, "Bar", "TheQuickBrownFox":

* `hashtable` stores entries for each distinct string that has been
  inserted. The entries contain the payload as well as information about the
  value (either an offset or the actual bytes, see `Entry` docs for more
  details)

* `offsets` stores offsets into `buffer` for each distinct string value,
  following the same convention as the offsets in a `StringArray` or
  `LargeStringArray`.

* `buffer` stores the actual byte data

* `null`: stores the index and payload of the null value, in this case the
  second value (index 1)

```text
┌───────────────────────────────────┐    ┌─────┐    ┌────┐
│                ...                │    │  0  │    │FooB│
│ ┌──────────────────────────────┐  │    │  0  │    │arTh│
│ │      <Entry for "Bar">       │  │    │  3  │    │eQui│
│ │            len: 3            │  │    │  3  │    │ckBr│
│ │   offset_or_inline: "Bar"    │  │    │  6  │    │ownF│
│ │         payload:...          │  │    │     │    │ox  │
│ └──────────────────────────────┘  │    │     │    │    │
│                ...                │    └─────┘    └────┘
│ ┌──────────────────────────────┐  │
│ │<Entry for "TheQuickBrownFox">│  │    offsets    buffer
│ │           len: 16            │  │
│ │     offset_or_inline: 6      │  │    ┌───────────────┐
│ │         payload: ...         │  │    │    Some(1)    │
│ └──────────────────────────────┘  │    │ payload: ...  │
│                ...                │    └───────────────┘
└───────────────────────────────────┘
                                             null
              HashTable
```

# Entry Format

Entries stored in a [`ArrowBytesMap`](../operations/datafusion_physical_expr_common.binary_map.ArrowBytesMap.md#op-447f14acc2b0df76e4cd3de1) represents a value that is either
stored inline or in the buffer

This helps the case where there are many short (less than 8 bytes) strings
that are the same (e.g. "MA", "CA", "NY", "TX", etc)

```text
                                                               ┌──────────────────┐
                                                 ─ ─ ─ ─ ─ ─ ─▶│...               │
                                                │              │TheQuickBrownFox  │
                                                               │...               │
                                                │              │                  │
                                                               └──────────────────┘
                                                │               buffer of u8

                                                │
                       ┌────────────────┬───────────────┬───────────────┐
 Storing               │                │ starting byte │  length, in   │
 "TheQuickBrownFox"    │   hash value   │   offset in   │  bytes (not   │
 (long string)         │                │    buffer     │  characters)  │
                       └────────────────┴───────────────┴───────────────┘
                             8 bytes          8 bytes       4 or 8


                        ┌───────────────┬─┬─┬─┬─┬─┬─┬─┬─┬───────────────┐
Storing "foobar"        │               │ │ │ │ │ │ │ │ │  length, in   │
(short string)          │  hash value   │?│?│f│o│o│b│a│r│  bytes (not   │
                        │               │ │ │ │ │ │ │ │ │  characters)  │
                        └───────────────┴─┴─┴─┴─┴─┴─┴─┴─┴───────────────┘
                             8 bytes         8 bytes        4 or 8
```

<a id="op-740f673d2017f98ac83a6854"></a>
## fmt

`function` · `datafusion_physical_expr_common::binary_map::ArrowBytesMap::fmt` · datafusion-physical-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_physical_expr_common::binary_map::ArrowBytesMap", "path": "ArrowBytesMap"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "O"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Copy", "path": "Copy"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::default::Default", "path": "Default"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [564, 1], "end": [577, 2], "filename": "src/binary_map.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/binary_map.rs:568`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ef4cc07697c1e9df2a2fa7c7"></a>
## insert_if_new

`function` · `datafusion_physical_expr_common::binary_map::ArrowBytesMap::insert_if_new` · datafusion-physical-expr-common 55.1.0

```rust
fn insert_if_new<MP, OP>(&mut self, values: &ArrayRef, make_payload_fn: MP, observe_payload_fn: OP) where MP: FnMut(Option<&[u8]>) -> V, OP: FnMut(V)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_physical_expr_common::binary_map::ArrowBytesMap", "path": "ArrowBytesMap"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "O"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Copy", "path": "Copy"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::default::Default", "path": "Default"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [242, 1], "end": [552, 2], "filename": "src/binary_map.rs"}, "trait": null, "trait_path": null}`

Source: `src/binary_map.rs:293`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

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

<a id="op-896db70f7067c570cb9e93ad"></a>
## into_state

`function` · `datafusion_physical_expr_common::binary_map::ArrowBytesMap::into_state` · datafusion-physical-expr-common 55.1.0

```rust
fn into_state(self) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_physical_expr_common::binary_map::ArrowBytesMap", "path": "ArrowBytesMap"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "O"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Copy", "path": "Copy"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::default::Default", "path": "Default"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [242, 1], "end": [552, 2], "filename": "src/binary_map.rs"}, "trait": null, "trait_path": null}`

Source: `src/binary_map.rs:485`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Converts this set into a `StringArray`, `LargeStringArray`,
`BinaryArray`, or `LargeBinaryArray` containing each distinct value
that was inserted. This is done without copying the values.

The values are guaranteed to be returned in the same order in which
they were first seen.

<a id="op-35cd45100f6342556c13a2bc"></a>
## is_empty

`function` · `datafusion_physical_expr_common::binary_map::ArrowBytesMap::is_empty` · datafusion-physical-expr-common 55.1.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_physical_expr_common::binary_map::ArrowBytesMap", "path": "ArrowBytesMap"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "O"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Copy", "path": "Copy"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::default::Default", "path": "Default"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [242, 1], "end": [552, 2], "filename": "src/binary_map.rs"}, "trait": null, "trait_path": null}`

Source: `src/binary_map.rs:535`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Is the set empty?

<a id="op-bf2eab556fc44a4e711b4d34"></a>
## len

`function` · `datafusion_physical_expr_common::binary_map::ArrowBytesMap::len` · datafusion-physical-expr-common 55.1.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_physical_expr_common::binary_map::ArrowBytesMap", "path": "ArrowBytesMap"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "O"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Copy", "path": "Copy"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::default::Default", "path": "Default"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [242, 1], "end": [552, 2], "filename": "src/binary_map.rs"}, "trait": null, "trait_path": null}`

Source: `src/binary_map.rs:530`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Total number of entries (including null, if present)

<a id="op-dfb9093e7d8be3273bc7fd16"></a>
## new

`function` · `datafusion_physical_expr_common::binary_map::ArrowBytesMap::new` · datafusion-physical-expr-common 55.1.0

```rust
fn new(output_type: OutputType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_physical_expr_common::binary_map::ArrowBytesMap", "path": "ArrowBytesMap"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "O"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Copy", "path": "Copy"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::default::Default", "path": "Default"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [242, 1], "end": [552, 2], "filename": "src/binary_map.rs"}, "trait": null, "trait_path": null}`

Source: `src/binary_map.rs:246`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea8dafc91aeccf15e2867537"></a>
## non_null_len

`function` · `datafusion_physical_expr_common::binary_map::ArrowBytesMap::non_null_len` · datafusion-physical-expr-common 55.1.0

```rust
fn non_null_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_physical_expr_common::binary_map::ArrowBytesMap", "path": "ArrowBytesMap"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "O"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Copy", "path": "Copy"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::default::Default", "path": "Default"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [242, 1], "end": [552, 2], "filename": "src/binary_map.rs"}, "trait": null, "trait_path": null}`

Source: `src/binary_map.rs:540`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Number of non null entries

<a id="op-cbad5423d6e8e46a9b159eb4"></a>
## size

`function` · `datafusion_physical_expr_common::binary_map::ArrowBytesMap::size` · datafusion-physical-expr-common 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_physical_expr_common::binary_map::ArrowBytesMap", "path": "ArrowBytesMap"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "O"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Copy", "path": "Copy"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::default::Default", "path": "Default"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [242, 1], "end": [552, 2], "filename": "src/binary_map.rs"}, "trait": null, "trait_path": null}`

Source: `src/binary_map.rs:546`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Return the total size, in bytes, of memory used to store the data in
this set, not including `self`

<a id="op-4697b6ebafd2bdbb881fa1a7"></a>
## take

`function` · `datafusion_physical_expr_common::binary_map::ArrowBytesMap::take` · datafusion-physical-expr-common 55.1.0

```rust
fn take(&mut self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_physical_expr_common::binary_map::ArrowBytesMap", "path": "ArrowBytesMap"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "O"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Copy", "path": "Copy"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::default::Default", "path": "Default"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [242, 1], "end": [552, 2], "filename": "src/binary_map.rs"}, "trait": null, "trait_path": null}`

Source: `src/binary_map.rs:261`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Return the contents of this map and replace it with a new empty map with
the same output type
