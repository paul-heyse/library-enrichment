# `arrow_ipc::gen::Schema::Map`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.Schema.Map.json).

<a id="op-4ff703876bc1c3ff4ae1d45d"></a>
## Map

`struct` · `arrow_ipc::gen::Schema::Map` · arrow-ipc 59.3.0

```rust
struct Map<'a>
```

Source: `src/gen/Schema.rs:1880`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

A Map is a logical nested type that is represented as

List<entries: Struct<key: K, value: V>>

In this layout, the keys and values are each respectively contiguous. We do
not constrain the key and value types, so the application is responsible
for ensuring that the keys are hashable and unique. Whether the keys are sorted
may be set in the metadata for this field.

In a field with Map type, the field has a child Struct field, which then
has two children: key type and the second the value type. The names of the
child fields may be respectively "entries", "key", and "value", but this is
not enforced.

Map
```text
  - child[0] entries: Struct
    - child[0] key: K
    - child[1] value: V
```
Neither the "entries" field nor the "key" field may be nullable.

The metadata is structured so that Arrow systems without special handling
for Map can make Map an alias for List. The "layout" attribute for the Map
field must have the same contents as a List.

<a id="op-9f137024cea0a2ec4a45aa4a"></a>
## Inner

`assoc_type` · `arrow_ipc::gen::Schema::Map::Inner` · arrow-ipc 59.3.0

```rust
Inner
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::Map", "path": "Map"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1884, 1], "end": [1892, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "flatbuffers::follow::Follow", "path": "Follow"}, "trait_path": "flatbuffers::follow::Follow"}`

Source: `src/gen/Schema.rs:1885`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c42dd383c87289ae2900983"></a>
## VT_KEYSSORTED

`assoc_const` · `arrow_ipc::gen::Schema::Map::VT_KEYSSORTED` · arrow-ipc 59.3.0

```rust
VT_KEYSSORTED
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::Map", "path": "Map"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1894, 1], "end": [1923, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:1895`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1bb5de27948deacd9d621c4f"></a>
## _tab

`struct_field` · `arrow_ipc::gen::Schema::Map::_tab` · arrow-ipc 59.3.0

```rust
_tab: flatbuffers::Table<'a>
```

Source: `src/gen/Schema.rs:1881`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-309f6a5d6172ad7ce6811ebb"></a>
## clone

`function` · `arrow_ipc::gen::Schema::Map::clone` · arrow-ipc 59.3.0

```rust
fn clone(&self) -> Map<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::Map", "path": "Map"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1853, 16], "end": [1853, 21], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/gen/Schema.rs:1853`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-801890689b725928fa7dbc16"></a>
## create

`function` · `arrow_ipc::gen::Schema::Map::create` · arrow-ipc 59.3.0

```rust
fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(_fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>, args: &'args MapArgs) -> flatbuffers::WIPOffset<Map<'bldr>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::Map", "path": "Map"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1894, 1], "end": [1923, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:1902`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6d5ae75b22b76c2313f2a2f1"></a>
## eq

`function` · `arrow_ipc::gen::Schema::Map::eq` · arrow-ipc 59.3.0

```rust
fn eq(&self, other: &Map<'a>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::Map", "path": "Map"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1853, 23], "end": [1853, 32], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/gen/Schema.rs:1853`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a4bbe2253613b9a3ea8fd1fe"></a>
## fmt

`function` · `arrow_ipc::gen::Schema::Map::fmt` · arrow-ipc 59.3.0

```rust
fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::Map", "path": "Map"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1973, 1], "end": [1979, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/gen/Schema.rs:1974`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5de637628dbadf090d0869c6"></a>
## follow

`function` · `arrow_ipc::gen::Schema::Map::follow` · arrow-ipc 59.3.0

```rust
unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::Map", "path": "Map"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1884, 1], "end": [1892, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "flatbuffers::follow::Follow", "path": "Follow"}, "trait_path": "flatbuffers::follow::Follow"}`

Source: `src/gen/Schema.rs:1887`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9cfe532d8ade0b78f7710914"></a>
## init_from_table

`function` · `arrow_ipc::gen::Schema::Map::init_from_table` · arrow-ipc 59.3.0

```rust
unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::Map", "path": "Map"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1894, 1], "end": [1923, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:1898`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-68dabecf3ddb1259a19df154"></a>
## keysSorted

`function` · `arrow_ipc::gen::Schema::Map::keysSorted` · arrow-ipc 59.3.0

```rust
fn keysSorted(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::Map", "path": "Map"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1894, 1], "end": [1923, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:1913`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Set to true if the keys within each value are sorted

<a id="op-72ceb7c9961779778e9a84e4"></a>
## run_verifier

`function` · `arrow_ipc::gen::Schema::Map::run_verifier` · arrow-ipc 59.3.0

```rust
fn run_verifier(v: &mut flatbuffers::Verifier<'_, '_>, pos: usize) -> Result<(), flatbuffers::InvalidFlatbuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::Map", "path": "Map"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1925, 1], "end": [1937, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "flatbuffers::verifier::Verifiable", "path": "Verifiable"}, "trait_path": "flatbuffers::verifier::Verifiable"}`

Source: `src/gen/Schema.rs:1927`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
