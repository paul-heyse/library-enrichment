# `arrow_ipc::gen::Schema::Utf8ViewBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.Schema.Utf8ViewBuilder.json).

<a id="op-e168ed2d292311b6d2f43db7"></a>
## Utf8ViewBuilder

`struct` · `arrow_ipc::gen::Schema::Utf8ViewBuilder` · arrow-ipc 59.3.0

```rust
struct Utf8ViewBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a>
```

Source: `src/gen/Schema.rs:2719`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6254ce31462f28536634b2b8"></a>
## finish

`function` · `arrow_ipc::gen::Schema::Utf8ViewBuilder::finish` · arrow-ipc 59.3.0

```rust
fn finish(self) -> flatbuffers::WIPOffset<Utf8View<'a>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::Utf8ViewBuilder", "path": "Utf8ViewBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2723, 1], "end": [2737, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:2733`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f7ed28f1e41766a39afb8705"></a>
## new

`function` · `arrow_ipc::gen::Schema::Utf8ViewBuilder::new` · arrow-ipc 59.3.0

```rust
fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> Utf8ViewBuilder<'a, 'b, A>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::Utf8ViewBuilder", "path": "Utf8ViewBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2723, 1], "end": [2737, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:2725`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
