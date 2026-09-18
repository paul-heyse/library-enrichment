# `datafusion_proto_models::generated::datafusion::MergeIntoActionNode`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_models.generated.datafusion.MergeIntoActionNode.json).

<a id="op-04deeed58591525cf11be9c6"></a>
## MergeIntoActionNode

`struct` · `datafusion_proto_models::generated::datafusion::MergeIntoActionNode` · datafusion-proto-models 55.1.0

```rust
struct MergeIntoActionNode
```

Source: `src/generated/prost.rs:596`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

The action for a single WHEN clause.

<a id="op-85605aa65ef7041ad33abbab"></a>
## action

`struct_field` · `datafusion_proto_models::generated::datafusion::MergeIntoActionNode::action` · datafusion-proto-models 55.1.0

```rust
action: ::core::option::Option<merge_into_action_node::Action>
```

Source: `src/generated/prost.rs:598`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-66578cabb2355c35ee08ae19"></a>
## clear

`function` · `datafusion_proto_models::generated::datafusion::MergeIntoActionNode::clear` · datafusion-proto-models 55.1.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::MergeIntoActionNode", "path": "MergeIntoActionNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [595, 28], "end": [595, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:595`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b7f0092b612eb8ea200a6767"></a>
## clone

`function` · `datafusion_proto_models::generated::datafusion::MergeIntoActionNode::clone` · datafusion-proto-models 55.1.0

```rust
fn clone(&self) -> MergeIntoActionNode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::MergeIntoActionNode", "path": "MergeIntoActionNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [595, 10], "end": [595, 15], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generated/prost.rs:595`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-000a4255983d58aabe275e0a"></a>
## default

`function` · `datafusion_proto_models::generated::datafusion::MergeIntoActionNode::default` · datafusion-proto-models 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::MergeIntoActionNode", "path": "MergeIntoActionNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [595, 28], "end": [595, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/generated/prost.rs:595`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4d5c8c5771a8bf5eec4068de"></a>
## deserialize

`function` · `datafusion_proto_models::generated::datafusion::MergeIntoActionNode::deserialize` · datafusion-proto-models 55.1.0

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::MergeIntoActionNode", "path": "MergeIntoActionNode"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [14832, 1], "end": [14925, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/generated/pbjson.rs:14834`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-75bab38cc350949accf2c6e0"></a>
## encoded_len

`function` · `datafusion_proto_models::generated::datafusion::MergeIntoActionNode::encoded_len` · datafusion-proto-models 55.1.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::MergeIntoActionNode", "path": "MergeIntoActionNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [595, 28], "end": [595, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:595`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8d1061db507a47af0c9c8d19"></a>
## eq

`function` · `datafusion_proto_models::generated::datafusion::MergeIntoActionNode::eq` · datafusion-proto-models 55.1.0

```rust
fn eq(&self, other: &MergeIntoActionNode) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::MergeIntoActionNode", "path": "MergeIntoActionNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [595, 17], "end": [595, 26], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/generated/prost.rs:595`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7a67ba921e766420b0a7521e"></a>
## fmt

`function` · `datafusion_proto_models::generated::datafusion::MergeIntoActionNode::fmt` · datafusion-proto-models 55.1.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::MergeIntoActionNode", "path": "MergeIntoActionNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [595, 28], "end": [595, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generated/prost.rs:595`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b578027d1e52ea4ac703df86"></a>
## serialize

`function` · `datafusion_proto_models::generated::datafusion::MergeIntoActionNode::serialize` · datafusion-proto-models 55.1.0

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::MergeIntoActionNode", "path": "MergeIntoActionNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [14804, 1], "end": [14831, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/generated/pbjson.rs:14806`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
