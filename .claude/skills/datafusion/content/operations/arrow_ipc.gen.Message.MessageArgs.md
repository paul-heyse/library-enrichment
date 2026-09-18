# `arrow_ipc::gen::Message::MessageArgs`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.Message.MessageArgs.json).

<a id="op-0a4bef950f8f431501083595"></a>
## MessageArgs

`struct` · `arrow_ipc::gen::Message::MessageArgs` · arrow-ipc 59.3.0

```rust
struct MessageArgs<'a>
```

Source: `src/gen/Message.rs:1291`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a143436d4d548fb59d8108f5"></a>
## bodyLength

`struct_field` · `arrow_ipc::gen::Message::MessageArgs::bodyLength` · arrow-ipc 59.3.0

```rust
bodyLength: i64
```

Source: `src/gen/Message.rs:1295`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-39f71ec08ffc63d38d8ee127"></a>
## custom_metadata

`struct_field` · `arrow_ipc::gen::Message::MessageArgs::custom_metadata` · arrow-ipc 59.3.0

```rust
custom_metadata: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<KeyValue<'a>>>>>
```

Source: `src/gen/Message.rs:1296`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-89a30743b1691368792f9304"></a>
## default

`function` · `arrow_ipc::gen::Message::MessageArgs::default` · arrow-ipc 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Message::MessageArgs", "path": "MessageArgs"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1300, 1], "end": [1311, 2], "filename": "src/gen/Message.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/gen/Message.rs:1302`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-616218e127c3877830889df4"></a>
## header

`struct_field` · `arrow_ipc::gen::Message::MessageArgs::header` · arrow-ipc 59.3.0

```rust
header: Option<flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>>
```

Source: `src/gen/Message.rs:1294`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-268b4879ab06b7fcabbe4de1"></a>
## header_type

`struct_field` · `arrow_ipc::gen::Message::MessageArgs::header_type` · arrow-ipc 59.3.0

```rust
header_type: MessageHeader
```

Source: `src/gen/Message.rs:1293`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f7a79f57c22e406b309bfa53"></a>
## version

`struct_field` · `arrow_ipc::gen::Message::MessageArgs::version` · arrow-ipc 59.3.0

```rust
version: MetadataVersion
```

Source: `src/gen/Message.rs:1292`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
