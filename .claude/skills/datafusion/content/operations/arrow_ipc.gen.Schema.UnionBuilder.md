# `arrow_ipc::gen::Schema::UnionBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.Schema.UnionBuilder.json).

<a id="op-cba2916421b8d9d357700052"></a>
## UnionBuilder

`struct` · `arrow_ipc::gen::Schema::UnionBuilder` · arrow-ipc 59.3.0

```rust
struct UnionBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a>
```

Source: `src/gen/Schema.rs:2080`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e4615ad0e9748de4ff196bad"></a>
## add_mode

`function` · `arrow_ipc::gen::Schema::UnionBuilder::add_mode` · arrow-ipc 59.3.0

```rust
fn add_mode(&mut self, mode: UnionMode)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::UnionBuilder", "path": "UnionBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2084, 1], "end": [2108, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:2086`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b5273bb44b12a55b5aee5edf"></a>
## add_typeIds

`function` · `arrow_ipc::gen::Schema::UnionBuilder::add_typeIds` · arrow-ipc 59.3.0

```rust
fn add_typeIds(&mut self, typeIds: flatbuffers::WIPOffset<flatbuffers::Vector<'b, i32>>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::UnionBuilder", "path": "UnionBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2084, 1], "end": [2108, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:2091`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6ca1e424d43483131b32f5e0"></a>
## finish

`function` · `arrow_ipc::gen::Schema::UnionBuilder::finish` · arrow-ipc 59.3.0

```rust
fn finish(self) -> flatbuffers::WIPOffset<Union<'a>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::UnionBuilder", "path": "UnionBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2084, 1], "end": [2108, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:2104`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3541ddb7e4d10bca33b3ba1e"></a>
## new

`function` · `arrow_ipc::gen::Schema::UnionBuilder::new` · arrow-ipc 59.3.0

```rust
fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> UnionBuilder<'a, 'b, A>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::UnionBuilder", "path": "UnionBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2084, 1], "end": [2108, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:2096`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
