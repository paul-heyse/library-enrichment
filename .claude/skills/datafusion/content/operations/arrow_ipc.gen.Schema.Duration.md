# `arrow_ipc::gen::Schema::Duration`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.Schema.Duration.json).

<a id="op-5e42f8dca21bd0f69edfe337"></a>
## Duration

`struct` · `arrow_ipc::gen::Schema::Duration` · arrow-ipc 59.3.0

```rust
struct Duration<'a>
```

Source: `src/gen/Schema.rs:3846`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-12eb7fde4504ac13bf148f9d"></a>
## Inner

`assoc_type` · `arrow_ipc::gen::Schema::Duration::Inner` · arrow-ipc 59.3.0

```rust
Inner
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::Duration", "path": "Duration"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3850, 1], "end": [3858, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "flatbuffers::follow::Follow", "path": "Follow"}, "trait_path": "flatbuffers::follow::Follow"}`

Source: `src/gen/Schema.rs:3851`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2219f2cff501a958f2430582"></a>
## VT_UNIT

`assoc_const` · `arrow_ipc::gen::Schema::Duration::VT_UNIT` · arrow-ipc 59.3.0

```rust
VT_UNIT
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::Duration", "path": "Duration"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3860, 1], "end": [3888, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:3861`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-740ff05665b15651774d8062"></a>
## _tab

`struct_field` · `arrow_ipc::gen::Schema::Duration::_tab` · arrow-ipc 59.3.0

```rust
_tab: flatbuffers::Table<'a>
```

Source: `src/gen/Schema.rs:3847`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-34ac505575ee14885c65f82e"></a>
## clone

`function` · `arrow_ipc::gen::Schema::Duration::clone` · arrow-ipc 59.3.0

```rust
fn clone(&self) -> Duration<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::Duration", "path": "Duration"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3844, 16], "end": [3844, 21], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/gen/Schema.rs:3844`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-816f8568403c394451c5454a"></a>
## create

`function` · `arrow_ipc::gen::Schema::Duration::create` · arrow-ipc 59.3.0

```rust
fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(_fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>, args: &'args DurationArgs) -> flatbuffers::WIPOffset<Duration<'bldr>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::Duration", "path": "Duration"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3860, 1], "end": [3888, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:3868`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cf93f1abc62ad174e9c1a784"></a>
## eq

`function` · `arrow_ipc::gen::Schema::Duration::eq` · arrow-ipc 59.3.0

```rust
fn eq(&self, other: &Duration<'a>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::Duration", "path": "Duration"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3844, 23], "end": [3844, 32], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/gen/Schema.rs:3844`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-31c04ee799f558c64bfeb8e3"></a>
## fmt

`function` · `arrow_ipc::gen::Schema::Duration::fmt` · arrow-ipc 59.3.0

```rust
fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::Duration", "path": "Duration"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3940, 1], "end": [3946, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/gen/Schema.rs:3941`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-93651fa9f40e39be184bb485"></a>
## follow

`function` · `arrow_ipc::gen::Schema::Duration::follow` · arrow-ipc 59.3.0

```rust
unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::Duration", "path": "Duration"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3850, 1], "end": [3858, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "flatbuffers::follow::Follow", "path": "Follow"}, "trait_path": "flatbuffers::follow::Follow"}`

Source: `src/gen/Schema.rs:3853`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f98e8689772a94d2284ccdcb"></a>
## init_from_table

`function` · `arrow_ipc::gen::Schema::Duration::init_from_table` · arrow-ipc 59.3.0

```rust
unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::Duration", "path": "Duration"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3860, 1], "end": [3888, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:3864`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-725ac49f6176d19b899d26af"></a>
## run_verifier

`function` · `arrow_ipc::gen::Schema::Duration::run_verifier` · arrow-ipc 59.3.0

```rust
fn run_verifier(v: &mut flatbuffers::Verifier<'_, '_>, pos: usize) -> Result<(), flatbuffers::InvalidFlatbuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::Duration", "path": "Duration"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3890, 1], "end": [3902, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "flatbuffers::verifier::Verifiable", "path": "Verifiable"}, "trait_path": "flatbuffers::verifier::Verifiable"}`

Source: `src/gen/Schema.rs:3892`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-31582f91926b7a1c050724f8"></a>
## unit

`function` · `arrow_ipc::gen::Schema::Duration::unit` · arrow-ipc 59.3.0

```rust
fn unit(&self) -> TimeUnit
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::Duration", "path": "Duration"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3860, 1], "end": [3888, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:3878`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
