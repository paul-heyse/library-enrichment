# `arrow_ipc::gen::Schema::IntBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.Schema.IntBuilder.json).

<a id="op-7c826735522514c6afa7f885"></a>
## IntBuilder

`struct` · `arrow_ipc::gen::Schema::IntBuilder` · arrow-ipc 59.3.0

```rust
struct IntBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a>
```

Source: `src/gen/Schema.rs:2202`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1692278c386a05f294926088"></a>
## add_bitWidth

`function` · `arrow_ipc::gen::Schema::IntBuilder::add_bitWidth` · arrow-ipc 59.3.0

```rust
fn add_bitWidth(&mut self, bitWidth: i32)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::IntBuilder", "path": "IntBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2206, 1], "end": [2229, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:2208`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2797c00455854490593b2b70"></a>
## add_is_signed

`function` · `arrow_ipc::gen::Schema::IntBuilder::add_is_signed` · arrow-ipc 59.3.0

```rust
fn add_is_signed(&mut self, is_signed: bool)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::IntBuilder", "path": "IntBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2206, 1], "end": [2229, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:2212`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-25876966a4fe1755d408e5c9"></a>
## finish

`function` · `arrow_ipc::gen::Schema::IntBuilder::finish` · arrow-ipc 59.3.0

```rust
fn finish(self) -> flatbuffers::WIPOffset<Int<'a>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::IntBuilder", "path": "IntBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2206, 1], "end": [2229, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:2225`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7e13ba8596abc61273080a58"></a>
## new

`function` · `arrow_ipc::gen::Schema::IntBuilder::new` · arrow-ipc 59.3.0

```rust
fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> IntBuilder<'a, 'b, A>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::IntBuilder", "path": "IntBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2206, 1], "end": [2229, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:2217`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
