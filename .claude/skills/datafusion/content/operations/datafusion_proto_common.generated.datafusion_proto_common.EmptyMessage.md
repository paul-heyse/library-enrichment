# `datafusion_proto_common::generated::datafusion_proto_common::EmptyMessage`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_common.generated.datafusion_proto_common.EmptyMessage.json).

<a id="op-73672a3a406ee38f7f56bea2"></a>
## EmptyMessage

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::EmptyMessage` · datafusion-proto-common 55.1.0

```rust
struct EmptyMessage
```

Source: `src/generated/prost.rs:581`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Useful for representing an empty enum variant in rust
E.G. enum example{One, Two(i32)}
maps to
message example{
    oneof{
        EmptyMessage One = 1;
        i32 Two = 2;
   }
}

<a id="op-ac0ecfd05b7050e992fa307b"></a>
## clear

`function` · `datafusion_proto_common::generated::datafusion_proto_common::EmptyMessage::clear` · datafusion-proto-common 55.1.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::EmptyMessage", "path": "EmptyMessage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [580, 44], "end": [580, 60], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:580`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0cdab55ace67f9addd0351b0"></a>
## clone

`function` · `datafusion_proto_common::generated::datafusion_proto_common::EmptyMessage::clone` · datafusion-proto-common 55.1.0

```rust
fn clone(&self) -> EmptyMessage
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::EmptyMessage", "path": "EmptyMessage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [580, 10], "end": [580, 15], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generated/prost.rs:580`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-32c6837bb9c0caa5fa1fdde7"></a>
## default

`function` · `datafusion_proto_common::generated::datafusion_proto_common::EmptyMessage::default` · datafusion-proto-common 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::EmptyMessage", "path": "EmptyMessage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [580, 44], "end": [580, 60], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/generated/prost.rs:580`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a0ccfcc53ee59fe7e3d812fb"></a>
## deserialize

`function` · `datafusion_proto_common::generated::datafusion_proto_common::EmptyMessage::deserialize` · datafusion-proto-common 55.1.0

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::EmptyMessage", "path": "EmptyMessage"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3922, 1], "end": [3980, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/generated/pbjson.rs:3924`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-46eab5e8e16bfcd7971cccb7"></a>
## encoded_len

`function` · `datafusion_proto_common::generated::datafusion_proto_common::EmptyMessage::encoded_len` · datafusion-proto-common 55.1.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::EmptyMessage", "path": "EmptyMessage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [580, 44], "end": [580, 60], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:580`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-decea3d7184969972d16f69b"></a>
## eq

`function` · `datafusion_proto_common::generated::datafusion_proto_common::EmptyMessage::eq` · datafusion-proto-common 55.1.0

```rust
fn eq(&self, other: &EmptyMessage) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::EmptyMessage", "path": "EmptyMessage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [580, 23], "end": [580, 32], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/generated/prost.rs:580`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3124d239b5ee7fb4bab9e00e"></a>
## fmt

`function` · `datafusion_proto_common::generated::datafusion_proto_common::EmptyMessage::fmt` · datafusion-proto-common 55.1.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::EmptyMessage", "path": "EmptyMessage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [580, 44], "end": [580, 60], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generated/prost.rs:580`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-24904371412f333bbff5f4a1"></a>
## hash

`function` · `datafusion_proto_common::generated::datafusion_proto_common::EmptyMessage::hash` · datafusion-proto-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::EmptyMessage", "path": "EmptyMessage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [580, 38], "end": [580, 42], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/generated/prost.rs:580`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9908a3b9e5d689fdfddbe106"></a>
## serialize

`function` · `datafusion_proto_common::generated::datafusion_proto_common::EmptyMessage::serialize` · datafusion-proto-common 55.1.0

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::EmptyMessage", "path": "EmptyMessage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3910, 1], "end": [3921, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/generated/pbjson.rs:3912`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
