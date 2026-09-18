# `arrow_ipc::gen::Schema::BinaryView`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.Schema.BinaryView.json).

<a id="op-b8c0fce84d2a96e44e408060"></a>
## BinaryView

`struct` · `arrow_ipc::gen::Schema::BinaryView` · arrow-ipc 59.3.0

```rust
struct BinaryView<'a>
```

Source: `src/gen/Schema.rs:2755`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Logically the same as Binary, but the internal representation uses a view
struct that contains the string length and either the string's entire data
inline (for small strings) or an inlined prefix, an index of another buffer,
and an offset pointing to a slice in that buffer (for non-small strings).

Since it uses a variable number of data buffers, each Field with this type
must have a corresponding entry in `variadicBufferCounts`.

<a id="op-2abb10899cb5c736cb8ce10c"></a>
## Inner

`assoc_type` · `arrow_ipc::gen::Schema::BinaryView::Inner` · arrow-ipc 59.3.0

```rust
Inner
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::BinaryView", "path": "BinaryView"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2759, 1], "end": [2767, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "flatbuffers::follow::Follow", "path": "Follow"}, "trait_path": "flatbuffers::follow::Follow"}`

Source: `src/gen/Schema.rs:2760`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c35ba55d17308c2904f4448"></a>
## _tab

`struct_field` · `arrow_ipc::gen::Schema::BinaryView::_tab` · arrow-ipc 59.3.0

```rust
_tab: flatbuffers::Table<'a>
```

Source: `src/gen/Schema.rs:2756`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-77dfd760823612ee1a2a74e8"></a>
## clone

`function` · `arrow_ipc::gen::Schema::BinaryView::clone` · arrow-ipc 59.3.0

```rust
fn clone(&self) -> BinaryView<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::BinaryView", "path": "BinaryView"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2746, 16], "end": [2746, 21], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/gen/Schema.rs:2746`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0b8cf7ca65ccdfa79acd0849"></a>
## create

`function` · `arrow_ipc::gen::Schema::BinaryView::create` · arrow-ipc 59.3.0

```rust
fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(_fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>, _args: &'args BinaryViewArgs) -> flatbuffers::WIPOffset<BinaryView<'bldr>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::BinaryView", "path": "BinaryView"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2769, 1], "end": [2782, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:2775`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b790f74ab32efb03caae4b0"></a>
## eq

`function` · `arrow_ipc::gen::Schema::BinaryView::eq` · arrow-ipc 59.3.0

```rust
fn eq(&self, other: &BinaryView<'a>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::BinaryView", "path": "BinaryView"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2746, 23], "end": [2746, 32], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/gen/Schema.rs:2746`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8bf831379bf458b88bcee4fb"></a>
## fmt

`function` · `arrow_ipc::gen::Schema::BinaryView::fmt` · arrow-ipc 59.3.0

```rust
fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::BinaryView", "path": "BinaryView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2825, 1], "end": [2830, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/gen/Schema.rs:2826`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0b6e78fd290e6ec0b42355b0"></a>
## follow

`function` · `arrow_ipc::gen::Schema::BinaryView::follow` · arrow-ipc 59.3.0

```rust
unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::BinaryView", "path": "BinaryView"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2759, 1], "end": [2767, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "flatbuffers::follow::Follow", "path": "Follow"}, "trait_path": "flatbuffers::follow::Follow"}`

Source: `src/gen/Schema.rs:2762`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f3221110b944338f6ddf5c3c"></a>
## init_from_table

`function` · `arrow_ipc::gen::Schema::BinaryView::init_from_table` · arrow-ipc 59.3.0

```rust
unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::BinaryView", "path": "BinaryView"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2769, 1], "end": [2782, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:2771`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2afeb3fc05a0059245b5123c"></a>
## run_verifier

`function` · `arrow_ipc::gen::Schema::BinaryView::run_verifier` · arrow-ipc 59.3.0

```rust
fn run_verifier(v: &mut flatbuffers::Verifier<'_, '_>, pos: usize) -> Result<(), flatbuffers::InvalidFlatbuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::BinaryView", "path": "BinaryView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2784, 1], "end": [2794, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "flatbuffers::verifier::Verifiable", "path": "Verifiable"}, "trait_path": "flatbuffers::verifier::Verifiable"}`

Source: `src/gen/Schema.rs:2786`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
