# `arrow_ipc::gen::Schema::ListView`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.Schema.ListView.json).

<a id="op-fda9f0c205ad6eb214969530"></a>
## ListView

`struct` · `arrow_ipc::gen::Schema::ListView` · arrow-ipc 59.3.0

```rust
struct ListView<'a>
```

Source: `src/gen/Schema.rs:1592`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Represents the same logical types that List can, but contains offsets and
sizes allowing for writes in any order and sharing of child values among
list values.

<a id="op-5cd62ccc673008300c71dda1"></a>
## Inner

`assoc_type` · `arrow_ipc::gen::Schema::ListView::Inner` · arrow-ipc 59.3.0

```rust
Inner
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::ListView", "path": "ListView"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1596, 1], "end": [1604, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "flatbuffers::follow::Follow", "path": "Follow"}, "trait_path": "flatbuffers::follow::Follow"}`

Source: `src/gen/Schema.rs:1597`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8f8851640cd9d74ef33423c3"></a>
## _tab

`struct_field` · `arrow_ipc::gen::Schema::ListView::_tab` · arrow-ipc 59.3.0

```rust
_tab: flatbuffers::Table<'a>
```

Source: `src/gen/Schema.rs:1593`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-79f982d0a7a61d4150f9a43e"></a>
## clone

`function` · `arrow_ipc::gen::Schema::ListView::clone` · arrow-ipc 59.3.0

```rust
fn clone(&self) -> ListView<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::ListView", "path": "ListView"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1587, 16], "end": [1587, 21], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/gen/Schema.rs:1587`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-376b5d909b1e77a69414cfba"></a>
## create

`function` · `arrow_ipc::gen::Schema::ListView::create` · arrow-ipc 59.3.0

```rust
fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(_fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>, _args: &'args ListViewArgs) -> flatbuffers::WIPOffset<ListView<'bldr>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::ListView", "path": "ListView"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1606, 1], "end": [1619, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:1612`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6ebd145df33a6c2ba6cef8e5"></a>
## eq

`function` · `arrow_ipc::gen::Schema::ListView::eq` · arrow-ipc 59.3.0

```rust
fn eq(&self, other: &ListView<'a>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::ListView", "path": "ListView"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1587, 23], "end": [1587, 32], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/gen/Schema.rs:1587`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3367bde8ef4d37da17e217b8"></a>
## fmt

`function` · `arrow_ipc::gen::Schema::ListView::fmt` · arrow-ipc 59.3.0

```rust
fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::ListView", "path": "ListView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1660, 1], "end": [1665, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/gen/Schema.rs:1661`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4aeeb4a17256bdee72174323"></a>
## follow

`function` · `arrow_ipc::gen::Schema::ListView::follow` · arrow-ipc 59.3.0

```rust
unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::ListView", "path": "ListView"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1596, 1], "end": [1604, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "flatbuffers::follow::Follow", "path": "Follow"}, "trait_path": "flatbuffers::follow::Follow"}`

Source: `src/gen/Schema.rs:1599`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d569a019b61adc53e859ba56"></a>
## init_from_table

`function` · `arrow_ipc::gen::Schema::ListView::init_from_table` · arrow-ipc 59.3.0

```rust
unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::ListView", "path": "ListView"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1606, 1], "end": [1619, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:1608`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-038f724bd2c4b75363ee32ac"></a>
## run_verifier

`function` · `arrow_ipc::gen::Schema::ListView::run_verifier` · arrow-ipc 59.3.0

```rust
fn run_verifier(v: &mut flatbuffers::Verifier<'_, '_>, pos: usize) -> Result<(), flatbuffers::InvalidFlatbuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::ListView", "path": "ListView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1621, 1], "end": [1631, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "flatbuffers::verifier::Verifiable", "path": "Verifiable"}, "trait_path": "flatbuffers::verifier::Verifiable"}`

Source: `src/gen/Schema.rs:1623`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
