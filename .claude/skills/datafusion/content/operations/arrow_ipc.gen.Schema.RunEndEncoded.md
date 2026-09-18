# `arrow_ipc::gen::Schema::RunEndEncoded`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.Schema.RunEndEncoded.json).

<a id="op-779f74d11fca7f6227f9fe85"></a>
## RunEndEncoded

`struct` · `arrow_ipc::gen::Schema::RunEndEncoded` · arrow-ipc 59.3.0

```rust
struct RunEndEncoded<'a>
```

Source: `src/gen/Schema.rs:3021`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Contains two child arrays, run_ends and values.
The run_ends child array must be a 16/32/64-bit integer array
which encodes the indices at which the run with the value in
each corresponding index in the values child array ends.
Like list/struct types, the value array can be of any type.

<a id="op-237d573c6cd0c9f0a1e984c0"></a>
## Inner

`assoc_type` · `arrow_ipc::gen::Schema::RunEndEncoded::Inner` · arrow-ipc 59.3.0

```rust
Inner
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::RunEndEncoded", "path": "RunEndEncoded"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3025, 1], "end": [3033, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "flatbuffers::follow::Follow", "path": "Follow"}, "trait_path": "flatbuffers::follow::Follow"}`

Source: `src/gen/Schema.rs:3026`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7bd73589c895eeab08918c41"></a>
## _tab

`struct_field` · `arrow_ipc::gen::Schema::RunEndEncoded::_tab` · arrow-ipc 59.3.0

```rust
_tab: flatbuffers::Table<'a>
```

Source: `src/gen/Schema.rs:3022`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-80d3aab8ce32c7c9232dfe99"></a>
## clone

`function` · `arrow_ipc::gen::Schema::RunEndEncoded::clone` · arrow-ipc 59.3.0

```rust
fn clone(&self) -> RunEndEncoded<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::RunEndEncoded", "path": "RunEndEncoded"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3014, 16], "end": [3014, 21], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/gen/Schema.rs:3014`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b2816d93d8109a29287e364f"></a>
## create

`function` · `arrow_ipc::gen::Schema::RunEndEncoded::create` · arrow-ipc 59.3.0

```rust
fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(_fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>, _args: &'args RunEndEncodedArgs) -> flatbuffers::WIPOffset<RunEndEncoded<'bldr>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::RunEndEncoded", "path": "RunEndEncoded"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3035, 1], "end": [3048, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:3041`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-484b49f6d43d62cc64a341c0"></a>
## eq

`function` · `arrow_ipc::gen::Schema::RunEndEncoded::eq` · arrow-ipc 59.3.0

```rust
fn eq(&self, other: &RunEndEncoded<'a>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::RunEndEncoded", "path": "RunEndEncoded"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3014, 23], "end": [3014, 32], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/gen/Schema.rs:3014`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8eb63c4b5c6bb046aaf1e158"></a>
## fmt

`function` · `arrow_ipc::gen::Schema::RunEndEncoded::fmt` · arrow-ipc 59.3.0

```rust
fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::RunEndEncoded", "path": "RunEndEncoded"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3091, 1], "end": [3096, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/gen/Schema.rs:3092`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5b30ef7a4595fe1e33417c58"></a>
## follow

`function` · `arrow_ipc::gen::Schema::RunEndEncoded::follow` · arrow-ipc 59.3.0

```rust
unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::RunEndEncoded", "path": "RunEndEncoded"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3025, 1], "end": [3033, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "flatbuffers::follow::Follow", "path": "Follow"}, "trait_path": "flatbuffers::follow::Follow"}`

Source: `src/gen/Schema.rs:3028`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7d0802515de1965642234cbd"></a>
## init_from_table

`function` · `arrow_ipc::gen::Schema::RunEndEncoded::init_from_table` · arrow-ipc 59.3.0

```rust
unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::RunEndEncoded", "path": "RunEndEncoded"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3035, 1], "end": [3048, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:3037`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d8f46716f1b4abe3988681a0"></a>
## run_verifier

`function` · `arrow_ipc::gen::Schema::RunEndEncoded::run_verifier` · arrow-ipc 59.3.0

```rust
fn run_verifier(v: &mut flatbuffers::Verifier<'_, '_>, pos: usize) -> Result<(), flatbuffers::InvalidFlatbuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::RunEndEncoded", "path": "RunEndEncoded"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3050, 1], "end": [3060, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "flatbuffers::verifier::Verifiable", "path": "Verifiable"}, "trait_path": "flatbuffers::verifier::Verifiable"}`

Source: `src/gen/Schema.rs:3052`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
