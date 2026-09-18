# `datafusion_proto_models::generated::datafusion::AggregateNode`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_models.generated.datafusion.AggregateNode.json).

<a id="op-ee0749dbceb14b25b3bc2efa"></a>
## AggregateNode

`struct` · `datafusion_proto_models::generated::datafusion::AggregateNode` · datafusion-proto-models 55.1.0

```rust
struct AggregateNode
```

Source: `src/generated/prost.rs:387`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-01bed028c4dbf45c4eacf791"></a>
## aggr_expr

`struct_field` · `datafusion_proto_models::generated::datafusion::AggregateNode::aggr_expr` · datafusion-proto-models 55.1.0

```rust
aggr_expr: ::prost::alloc::vec::Vec<LogicalExprNode>
```

Source: `src/generated/prost.rs:393`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-efac133ee5efa605f1a57cda"></a>
## clear

`function` · `datafusion_proto_models::generated::datafusion::AggregateNode::clear` · datafusion-proto-models 55.1.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::AggregateNode", "path": "AggregateNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [386, 28], "end": [386, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:386`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a2267f2e154a7de192b78cfd"></a>
## clone

`function` · `datafusion_proto_models::generated::datafusion::AggregateNode::clone` · datafusion-proto-models 55.1.0

```rust
fn clone(&self) -> AggregateNode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::AggregateNode", "path": "AggregateNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [386, 10], "end": [386, 15], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generated/prost.rs:386`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b61609f3456f4baf3ead788a"></a>
## default

`function` · `datafusion_proto_models::generated::datafusion::AggregateNode::default` · datafusion-proto-models 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::AggregateNode", "path": "AggregateNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [386, 28], "end": [386, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/generated/prost.rs:386`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9f6469119098319173bd6011"></a>
## deserialize

`function` · `datafusion_proto_models::generated::datafusion::AggregateNode::deserialize` · datafusion-proto-models 55.1.0

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::AggregateNode", "path": "AggregateNode"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [549, 1], "end": [645, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/generated/pbjson.rs:551`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e39479a480cb4dd3c44b91ee"></a>
## encoded_len

`function` · `datafusion_proto_models::generated::datafusion::AggregateNode::encoded_len` · datafusion-proto-models 55.1.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::AggregateNode", "path": "AggregateNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [386, 28], "end": [386, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:386`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-efbaf7355bafd38d3602b54e"></a>
## eq

`function` · `datafusion_proto_models::generated::datafusion::AggregateNode::eq` · datafusion-proto-models 55.1.0

```rust
fn eq(&self, other: &AggregateNode) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::AggregateNode", "path": "AggregateNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [386, 17], "end": [386, 26], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/generated/prost.rs:386`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-12fda2adb795d4691c50ab82"></a>
## fmt

`function` · `datafusion_proto_models::generated::datafusion::AggregateNode::fmt` · datafusion-proto-models 55.1.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::AggregateNode", "path": "AggregateNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [386, 28], "end": [386, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generated/prost.rs:386`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-19166d2e1c8f0c899bd6b9f7"></a>
## group_expr

`struct_field` · `datafusion_proto_models::generated::datafusion::AggregateNode::group_expr` · datafusion-proto-models 55.1.0

```rust
group_expr: ::prost::alloc::vec::Vec<LogicalExprNode>
```

Source: `src/generated/prost.rs:391`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6424d91d75f51aa65d041d39"></a>
## input

`struct_field` · `datafusion_proto_models::generated::datafusion::AggregateNode::input` · datafusion-proto-models 55.1.0

```rust
input: ::core::option::Option<::prost::alloc::boxed::Box<LogicalPlanNode>>
```

Source: `src/generated/prost.rs:389`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a50bd4e8fa2dcbf3bc63e256"></a>
## serialize

`function` · `datafusion_proto_models::generated::datafusion::AggregateNode::serialize` · datafusion-proto-models 55.1.0

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::AggregateNode", "path": "AggregateNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [519, 1], "end": [548, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/generated/pbjson.rs:521`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
