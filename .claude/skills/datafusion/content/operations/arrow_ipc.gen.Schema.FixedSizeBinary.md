# `arrow_ipc::gen::Schema::FixedSizeBinary`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.Schema.FixedSizeBinary.json).

<a id="op-5e8d3d2ca49c53030836a8ab"></a>
## FixedSizeBinary

`struct` · `arrow_ipc::gen::Schema::FixedSizeBinary` · arrow-ipc 59.3.0

```rust
struct FixedSizeBinary<'a>
```

Source: `src/gen/Schema.rs:2834`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f0d86820d005287cc5564e9e"></a>
## Inner

`assoc_type` · `arrow_ipc::gen::Schema::FixedSizeBinary::Inner` · arrow-ipc 59.3.0

```rust
Inner
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::FixedSizeBinary", "path": "FixedSizeBinary"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2838, 1], "end": [2846, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "flatbuffers::follow::Follow", "path": "Follow"}, "trait_path": "flatbuffers::follow::Follow"}`

Source: `src/gen/Schema.rs:2839`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-21c359d7b327e8a3c12e80f8"></a>
## VT_BYTEWIDTH

`assoc_const` · `arrow_ipc::gen::Schema::FixedSizeBinary::VT_BYTEWIDTH` · arrow-ipc 59.3.0

```rust
VT_BYTEWIDTH
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::FixedSizeBinary", "path": "FixedSizeBinary"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2848, 1], "end": [2877, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:2849`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-70e99db94ee1ccd628c15579"></a>
## _tab

`struct_field` · `arrow_ipc::gen::Schema::FixedSizeBinary::_tab` · arrow-ipc 59.3.0

```rust
_tab: flatbuffers::Table<'a>
```

Source: `src/gen/Schema.rs:2835`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e263ea1f483a1488c598359e"></a>
## byteWidth

`function` · `arrow_ipc::gen::Schema::FixedSizeBinary::byteWidth` · arrow-ipc 59.3.0

```rust
fn byteWidth(&self) -> i32
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::FixedSizeBinary", "path": "FixedSizeBinary"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2848, 1], "end": [2877, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:2867`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Number of bytes per value

<a id="op-38e3ea9fc1a51f4a71859bff"></a>
## clone

`function` · `arrow_ipc::gen::Schema::FixedSizeBinary::clone` · arrow-ipc 59.3.0

```rust
fn clone(&self) -> FixedSizeBinary<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::FixedSizeBinary", "path": "FixedSizeBinary"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2832, 16], "end": [2832, 21], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/gen/Schema.rs:2832`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5d10c4f36e0347b2989a0afb"></a>
## create

`function` · `arrow_ipc::gen::Schema::FixedSizeBinary::create` · arrow-ipc 59.3.0

```rust
fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(_fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>, args: &'args FixedSizeBinaryArgs) -> flatbuffers::WIPOffset<FixedSizeBinary<'bldr>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::FixedSizeBinary", "path": "FixedSizeBinary"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2848, 1], "end": [2877, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:2856`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b9d0ad9187e051a7c940b58c"></a>
## eq

`function` · `arrow_ipc::gen::Schema::FixedSizeBinary::eq` · arrow-ipc 59.3.0

```rust
fn eq(&self, other: &FixedSizeBinary<'a>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::FixedSizeBinary", "path": "FixedSizeBinary"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2832, 23], "end": [2832, 32], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/gen/Schema.rs:2832`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4ed63d7a0e6686e9c5db3343"></a>
## fmt

`function` · `arrow_ipc::gen::Schema::FixedSizeBinary::fmt` · arrow-ipc 59.3.0

```rust
fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::FixedSizeBinary", "path": "FixedSizeBinary"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2929, 1], "end": [2935, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/gen/Schema.rs:2930`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-34b075a33e180d2284a3838f"></a>
## follow

`function` · `arrow_ipc::gen::Schema::FixedSizeBinary::follow` · arrow-ipc 59.3.0

```rust
unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::FixedSizeBinary", "path": "FixedSizeBinary"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2838, 1], "end": [2846, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "flatbuffers::follow::Follow", "path": "Follow"}, "trait_path": "flatbuffers::follow::Follow"}`

Source: `src/gen/Schema.rs:2841`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aeb4052994142fc4b3a08a0b"></a>
## init_from_table

`function` · `arrow_ipc::gen::Schema::FixedSizeBinary::init_from_table` · arrow-ipc 59.3.0

```rust
unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::FixedSizeBinary", "path": "FixedSizeBinary"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2848, 1], "end": [2877, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:2852`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0c568b6898e0d25202090ef4"></a>
## run_verifier

`function` · `arrow_ipc::gen::Schema::FixedSizeBinary::run_verifier` · arrow-ipc 59.3.0

```rust
fn run_verifier(v: &mut flatbuffers::Verifier<'_, '_>, pos: usize) -> Result<(), flatbuffers::InvalidFlatbuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::FixedSizeBinary", "path": "FixedSizeBinary"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2879, 1], "end": [2891, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "flatbuffers::verifier::Verifiable", "path": "Verifiable"}, "trait_path": "flatbuffers::verifier::Verifiable"}`

Source: `src/gen/Schema.rs:2881`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
