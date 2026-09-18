# `arrow_ipc::gen::Message::RecordBatchBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.Message.RecordBatchBuilder.json).

<a id="op-7371726261c705977685e448"></a>
## RecordBatchBuilder

`struct` · `arrow_ipc::gen::Message::RecordBatchBuilder` · arrow-ipc 59.3.0

```rust
struct RecordBatchBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a>
```

Source: `src/gen/Message.rs:827`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f89c8957026ac003a5152547"></a>
## add_buffers

`function` · `arrow_ipc::gen::Message::RecordBatchBuilder::add_buffers` · arrow-ipc 59.3.0

```rust
fn add_buffers(&mut self, buffers: flatbuffers::WIPOffset<flatbuffers::Vector<'b, Buffer>>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::Message::RecordBatchBuilder", "path": "RecordBatchBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [831, 1], "end": [883, 2], "filename": "src/gen/Message.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Message.rs:843`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b6640e6a814c9a2f2aea564"></a>
## add_compression

`function` · `arrow_ipc::gen::Message::RecordBatchBuilder::add_compression` · arrow-ipc 59.3.0

```rust
fn add_compression(&mut self, compression: flatbuffers::WIPOffset<BodyCompression<'b>>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::Message::RecordBatchBuilder", "path": "RecordBatchBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [831, 1], "end": [883, 2], "filename": "src/gen/Message.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Message.rs:851`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4726ffef49c4d42ec6c6a532"></a>
## add_length

`function` · `arrow_ipc::gen::Message::RecordBatchBuilder::add_length` · arrow-ipc 59.3.0

```rust
fn add_length(&mut self, length: i64)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::Message::RecordBatchBuilder", "path": "RecordBatchBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [831, 1], "end": [883, 2], "filename": "src/gen/Message.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Message.rs:833`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e4cf1d301e4b0b447947224e"></a>
## add_nodes

`function` · `arrow_ipc::gen::Message::RecordBatchBuilder::add_nodes` · arrow-ipc 59.3.0

```rust
fn add_nodes(&mut self, nodes: flatbuffers::WIPOffset<flatbuffers::Vector<'b, FieldNode>>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::Message::RecordBatchBuilder", "path": "RecordBatchBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [831, 1], "end": [883, 2], "filename": "src/gen/Message.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Message.rs:838`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0fa136f3818e91604cc1d35e"></a>
## add_variadicBufferCounts

`function` · `arrow_ipc::gen::Message::RecordBatchBuilder::add_variadicBufferCounts` · arrow-ipc 59.3.0

```rust
fn add_variadicBufferCounts(&mut self, variadicBufferCounts: flatbuffers::WIPOffset<flatbuffers::Vector<'b, i64>>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::Message::RecordBatchBuilder", "path": "RecordBatchBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [831, 1], "end": [883, 2], "filename": "src/gen/Message.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Message.rs:859`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0c1daee1a76b4299aa9eb58f"></a>
## finish

`function` · `arrow_ipc::gen::Message::RecordBatchBuilder::finish` · arrow-ipc 59.3.0

```rust
fn finish(self) -> flatbuffers::WIPOffset<RecordBatch<'a>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::Message::RecordBatchBuilder", "path": "RecordBatchBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [831, 1], "end": [883, 2], "filename": "src/gen/Message.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Message.rs:879`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-235eaf738c6311f63c99d3ac"></a>
## new

`function` · `arrow_ipc::gen::Message::RecordBatchBuilder::new` · arrow-ipc 59.3.0

```rust
fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> RecordBatchBuilder<'a, 'b, A>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::Message::RecordBatchBuilder", "path": "RecordBatchBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [831, 1], "end": [883, 2], "filename": "src/gen/Message.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Message.rs:869`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
