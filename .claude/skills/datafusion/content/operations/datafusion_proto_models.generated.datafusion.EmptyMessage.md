# `datafusion_proto_models::generated::datafusion::EmptyMessage`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_models.generated.datafusion.EmptyMessage.json).

<a id="op-3e4297acbb2034f021c75349"></a>
## EmptyMessage

`struct` · `datafusion_proto_models::generated::datafusion::EmptyMessage` · datafusion-proto-models 55.1.0

```rust
struct EmptyMessage
```

Source: `src/generated/datafusion_proto_common.rs:581`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Useful for representing an empty enum variant in rust
E.G. enum example{One, Two(i32)}
maps to
message example{
    oneof{
        EmptyMessage One = 1;
        i32 Two = 2;
   }
}

<a id="op-2b4c2efa53ed8e153108c394"></a>
## clear

`function` · `datafusion_proto_models::generated::datafusion::EmptyMessage::clear` · datafusion-proto-models 55.1.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::EmptyMessage", "path": "EmptyMessage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [580, 44], "end": [580, 60], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/datafusion_proto_common.rs:580`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dc60428596f11d642e4e4613"></a>
## clone

`function` · `datafusion_proto_models::generated::datafusion::EmptyMessage::clone` · datafusion-proto-models 55.1.0

```rust
fn clone(&self) -> EmptyMessage
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::EmptyMessage", "path": "EmptyMessage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [580, 10], "end": [580, 15], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generated/datafusion_proto_common.rs:580`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6d4966a22c810e83fe4b5096"></a>
## default

`function` · `datafusion_proto_models::generated::datafusion::EmptyMessage::default` · datafusion-proto-models 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::EmptyMessage", "path": "EmptyMessage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [580, 44], "end": [580, 60], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/generated/datafusion_proto_common.rs:580`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7e99d29f4a69de586634d4be"></a>
## encoded_len

`function` · `datafusion_proto_models::generated::datafusion::EmptyMessage::encoded_len` · datafusion-proto-models 55.1.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::EmptyMessage", "path": "EmptyMessage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [580, 44], "end": [580, 60], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/datafusion_proto_common.rs:580`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-19e2c64dcd7a2de582eb7d2e"></a>
## eq

`function` · `datafusion_proto_models::generated::datafusion::EmptyMessage::eq` · datafusion-proto-models 55.1.0

```rust
fn eq(&self, other: &EmptyMessage) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::EmptyMessage", "path": "EmptyMessage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [580, 23], "end": [580, 32], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/generated/datafusion_proto_common.rs:580`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-be409d1b5185ad6648e25534"></a>
## fmt

`function` · `datafusion_proto_models::generated::datafusion::EmptyMessage::fmt` · datafusion-proto-models 55.1.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::EmptyMessage", "path": "EmptyMessage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [580, 44], "end": [580, 60], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generated/datafusion_proto_common.rs:580`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-981e7f616cf3bd6101bf3131"></a>
## hash

`function` · `datafusion_proto_models::generated::datafusion::EmptyMessage::hash` · datafusion-proto-models 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::EmptyMessage", "path": "EmptyMessage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [580, 38], "end": [580, 42], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/generated/datafusion_proto_common.rs:580`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
