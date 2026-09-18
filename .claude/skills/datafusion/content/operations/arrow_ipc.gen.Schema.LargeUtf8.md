# `arrow_ipc::gen::Schema::LargeUtf8`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.Schema.LargeUtf8.json).

<a id="op-c32d8669c58c00cb5fbe41d7"></a>
## LargeUtf8

`struct` · `arrow_ipc::gen::Schema::LargeUtf8` · arrow-ipc 59.3.0

```rust
struct LargeUtf8<'a>
```

Source: `src/gen/Schema.rs:2506`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Same as Utf8, but with 64-bit offsets, allowing to represent
extremely large data values.

<a id="op-578680423a228035dd2cfe97"></a>
## Inner

`assoc_type` · `arrow_ipc::gen::Schema::LargeUtf8::Inner` · arrow-ipc 59.3.0

```rust
Inner
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::LargeUtf8", "path": "LargeUtf8"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2510, 1], "end": [2518, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "flatbuffers::follow::Follow", "path": "Follow"}, "trait_path": "flatbuffers::follow::Follow"}`

Source: `src/gen/Schema.rs:2511`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-867c435a436df7447b56041f"></a>
## _tab

`struct_field` · `arrow_ipc::gen::Schema::LargeUtf8::_tab` · arrow-ipc 59.3.0

```rust
_tab: flatbuffers::Table<'a>
```

Source: `src/gen/Schema.rs:2507`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ca063476f02069ded9f00092"></a>
## clone

`function` · `arrow_ipc::gen::Schema::LargeUtf8::clone` · arrow-ipc 59.3.0

```rust
fn clone(&self) -> LargeUtf8<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::LargeUtf8", "path": "LargeUtf8"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2502, 16], "end": [2502, 21], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/gen/Schema.rs:2502`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b3756530e8374b8297877ad3"></a>
## create

`function` · `arrow_ipc::gen::Schema::LargeUtf8::create` · arrow-ipc 59.3.0

```rust
fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(_fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>, _args: &'args LargeUtf8Args) -> flatbuffers::WIPOffset<LargeUtf8<'bldr>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::LargeUtf8", "path": "LargeUtf8"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2520, 1], "end": [2533, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:2526`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e649bc3b13942bbf8c15f1ef"></a>
## eq

`function` · `arrow_ipc::gen::Schema::LargeUtf8::eq` · arrow-ipc 59.3.0

```rust
fn eq(&self, other: &LargeUtf8<'a>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::LargeUtf8", "path": "LargeUtf8"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2502, 23], "end": [2502, 32], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/gen/Schema.rs:2502`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fc59bdcbd597f85474e89a9f"></a>
## fmt

`function` · `arrow_ipc::gen::Schema::LargeUtf8::fmt` · arrow-ipc 59.3.0

```rust
fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::LargeUtf8", "path": "LargeUtf8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2574, 1], "end": [2579, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/gen/Schema.rs:2575`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8627bec4e39190e06450f469"></a>
## follow

`function` · `arrow_ipc::gen::Schema::LargeUtf8::follow` · arrow-ipc 59.3.0

```rust
unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::LargeUtf8", "path": "LargeUtf8"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2510, 1], "end": [2518, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "flatbuffers::follow::Follow", "path": "Follow"}, "trait_path": "flatbuffers::follow::Follow"}`

Source: `src/gen/Schema.rs:2513`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1737392b4b9651c73cef5dcb"></a>
## init_from_table

`function` · `arrow_ipc::gen::Schema::LargeUtf8::init_from_table` · arrow-ipc 59.3.0

```rust
unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::LargeUtf8", "path": "LargeUtf8"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2520, 1], "end": [2533, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:2522`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-25579c5ce2c831124652f54d"></a>
## run_verifier

`function` · `arrow_ipc::gen::Schema::LargeUtf8::run_verifier` · arrow-ipc 59.3.0

```rust
fn run_verifier(v: &mut flatbuffers::Verifier<'_, '_>, pos: usize) -> Result<(), flatbuffers::InvalidFlatbuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::LargeUtf8", "path": "LargeUtf8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2535, 1], "end": [2545, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "flatbuffers::verifier::Verifiable", "path": "Verifiable"}, "trait_path": "flatbuffers::verifier::Verifiable"}`

Source: `src/gen/Schema.rs:2537`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
