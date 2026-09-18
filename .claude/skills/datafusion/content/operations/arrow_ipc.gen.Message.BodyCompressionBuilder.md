# `arrow_ipc::gen::Message::BodyCompressionBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.Message.BodyCompressionBuilder.json).

<a id="op-25253adf8b4a3b86caaad778"></a>
## BodyCompressionBuilder

`struct` · `arrow_ipc::gen::Message::BodyCompressionBuilder` · arrow-ipc 59.3.0

```rust
struct BodyCompressionBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a>
```

Source: `src/gen/Message.rs:586`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9ca1e9ff7438266a6fc527f0"></a>
## add_codec

`function` · `arrow_ipc::gen::Message::BodyCompressionBuilder::add_codec` · arrow-ipc 59.3.0

```rust
fn add_codec(&mut self, codec: CompressionType)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::Message::BodyCompressionBuilder", "path": "BodyCompressionBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [590, 1], "end": [622, 2], "filename": "src/gen/Message.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Message.rs:592`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-369ca02b8023cebb5779f34c"></a>
## add_method

`function` · `arrow_ipc::gen::Message::BodyCompressionBuilder::add_method` · arrow-ipc 59.3.0

```rust
fn add_method(&mut self, method: BodyCompressionMethod)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::Message::BodyCompressionBuilder", "path": "BodyCompressionBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [590, 1], "end": [622, 2], "filename": "src/gen/Message.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Message.rs:600`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f14096ce1033d5ed75defe8d"></a>
## finish

`function` · `arrow_ipc::gen::Message::BodyCompressionBuilder::finish` · arrow-ipc 59.3.0

```rust
fn finish(self) -> flatbuffers::WIPOffset<BodyCompression<'a>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::Message::BodyCompressionBuilder", "path": "BodyCompressionBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [590, 1], "end": [622, 2], "filename": "src/gen/Message.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Message.rs:618`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e998dfa41b625db3d58a4372"></a>
## new

`function` · `arrow_ipc::gen::Message::BodyCompressionBuilder::new` · arrow-ipc 59.3.0

```rust
fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> BodyCompressionBuilder<'a, 'b, A>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::Message::BodyCompressionBuilder", "path": "BodyCompressionBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [590, 1], "end": [622, 2], "filename": "src/gen/Message.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Message.rs:608`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
