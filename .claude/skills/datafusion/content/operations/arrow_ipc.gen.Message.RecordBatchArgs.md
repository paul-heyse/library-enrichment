# `arrow_ipc::gen::Message::RecordBatchArgs`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.Message.RecordBatchArgs.json).

<a id="op-77b9d92f9354e087e6e84641"></a>
## RecordBatchArgs

`struct` · `arrow_ipc::gen::Message::RecordBatchArgs` · arrow-ipc 59.3.0

```rust
struct RecordBatchArgs<'a>
```

Source: `src/gen/Message.rs:807`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6f269797a87e8305ab4cbcbe"></a>
## buffers

`struct_field` · `arrow_ipc::gen::Message::RecordBatchArgs::buffers` · arrow-ipc 59.3.0

```rust
buffers: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, Buffer>>>
```

Source: `src/gen/Message.rs:810`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d24028520fe4917e5bf830b0"></a>
## compression

`struct_field` · `arrow_ipc::gen::Message::RecordBatchArgs::compression` · arrow-ipc 59.3.0

```rust
compression: Option<flatbuffers::WIPOffset<BodyCompression<'a>>>
```

Source: `src/gen/Message.rs:811`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d3b50847541ede111c7c0194"></a>
## default

`function` · `arrow_ipc::gen::Message::RecordBatchArgs::default` · arrow-ipc 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Message::RecordBatchArgs", "path": "RecordBatchArgs"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [814, 1], "end": [825, 2], "filename": "src/gen/Message.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/gen/Message.rs:816`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bd01f6c6b38220fb917ddc43"></a>
## length

`struct_field` · `arrow_ipc::gen::Message::RecordBatchArgs::length` · arrow-ipc 59.3.0

```rust
length: i64
```

Source: `src/gen/Message.rs:808`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8cb4c93620a54427c92cc247"></a>
## nodes

`struct_field` · `arrow_ipc::gen::Message::RecordBatchArgs::nodes` · arrow-ipc 59.3.0

```rust
nodes: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, FieldNode>>>
```

Source: `src/gen/Message.rs:809`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-83f35000a4198be53b4e6f6f"></a>
## variadicBufferCounts

`struct_field` · `arrow_ipc::gen::Message::RecordBatchArgs::variadicBufferCounts` · arrow-ipc 59.3.0

```rust
variadicBufferCounts: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, i64>>>
```

Source: `src/gen/Message.rs:812`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
