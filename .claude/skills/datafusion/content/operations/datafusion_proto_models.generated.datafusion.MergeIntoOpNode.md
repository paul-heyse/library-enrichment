# `datafusion_proto_models::generated::datafusion::MergeIntoOpNode`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_models.generated.datafusion.MergeIntoOpNode.json).

<a id="op-4843a9beb4b343bc2b43e2a1"></a>
## MergeIntoOpNode

`struct` · `datafusion_proto_models::generated::datafusion::MergeIntoOpNode` · datafusion-proto-models 55.1.0

```rust
struct MergeIntoOpNode
```

Source: `src/generated/prost.rs:532`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Carries the ON condition and WHEN clauses of a MERGE INTO operation.

<a id="op-57bd858686e93f7c67b4ae83"></a>
## clauses

`struct_field` · `datafusion_proto_models::generated::datafusion::MergeIntoOpNode::clauses` · datafusion-proto-models 55.1.0

```rust
clauses: ::prost::alloc::vec::Vec<MergeIntoClauseNode>
```

Source: `src/generated/prost.rs:536`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9ae2165bb01a276fa00d8b4a"></a>
## clear

`function` · `datafusion_proto_models::generated::datafusion::MergeIntoOpNode::clear` · datafusion-proto-models 55.1.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::MergeIntoOpNode", "path": "MergeIntoOpNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [531, 28], "end": [531, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:531`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8428100d847a192942f2d436"></a>
## clone

`function` · `datafusion_proto_models::generated::datafusion::MergeIntoOpNode::clone` · datafusion-proto-models 55.1.0

```rust
fn clone(&self) -> MergeIntoOpNode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::MergeIntoOpNode", "path": "MergeIntoOpNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [531, 10], "end": [531, 15], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generated/prost.rs:531`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ec16b0d72161d0b308b069f2"></a>
## default

`function` · `datafusion_proto_models::generated::datafusion::MergeIntoOpNode::default` · datafusion-proto-models 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::MergeIntoOpNode", "path": "MergeIntoOpNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [531, 28], "end": [531, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/generated/prost.rs:531`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e19fbb32005fe1573c170758"></a>
## deserialize

`function` · `datafusion_proto_models::generated::datafusion::MergeIntoOpNode::deserialize` · datafusion-proto-models 55.1.0

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::MergeIntoOpNode", "path": "MergeIntoOpNode"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [15154, 1], "end": [15237, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/generated/pbjson.rs:15156`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7a10d6cc7c22e7e9fd0aae7a"></a>
## encoded_len

`function` · `datafusion_proto_models::generated::datafusion::MergeIntoOpNode::encoded_len` · datafusion-proto-models 55.1.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::MergeIntoOpNode", "path": "MergeIntoOpNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [531, 28], "end": [531, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:531`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-171c0ed8cb442b5b2cd8263f"></a>
## eq

`function` · `datafusion_proto_models::generated::datafusion::MergeIntoOpNode::eq` · datafusion-proto-models 55.1.0

```rust
fn eq(&self, other: &MergeIntoOpNode) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::MergeIntoOpNode", "path": "MergeIntoOpNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [531, 17], "end": [531, 26], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/generated/prost.rs:531`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e445314c663225c5b9636b0f"></a>
## fmt

`function` · `datafusion_proto_models::generated::datafusion::MergeIntoOpNode::fmt` · datafusion-proto-models 55.1.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::MergeIntoOpNode", "path": "MergeIntoOpNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [531, 28], "end": [531, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generated/prost.rs:531`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3e47eab030fee2b4b6365d6f"></a>
## on

`struct_field` · `datafusion_proto_models::generated::datafusion::MergeIntoOpNode::on` · datafusion-proto-models 55.1.0

```rust
on: ::core::option::Option<::prost::alloc::boxed::Box<LogicalExprNode>>
```

Source: `src/generated/prost.rs:534`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3d2b0c70bce61d74d54711fc"></a>
## serialize

`function` · `datafusion_proto_models::generated::datafusion::MergeIntoOpNode::serialize` · datafusion-proto-models 55.1.0

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::MergeIntoOpNode", "path": "MergeIntoOpNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [15130, 1], "end": [15153, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/generated/pbjson.rs:15132`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
