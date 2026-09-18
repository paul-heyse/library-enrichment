# `datafusion_proto_models::generated::datafusion::PrepareNode`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_models.generated.datafusion.PrepareNode.json).

<a id="op-a5ffe8951a6bb0b15e142898"></a>
## PrepareNode

`struct` · `datafusion_proto_models::generated::datafusion::PrepareNode` · datafusion-proto-models 55.1.0

```rust
struct PrepareNode
```

Source: `src/generated/prost.rs:293`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-24a75055018f7102e36dd6c2"></a>
## clear

`function` · `datafusion_proto_models::generated::datafusion::PrepareNode::clear` · datafusion-proto-models 55.1.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::PrepareNode", "path": "PrepareNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [292, 28], "end": [292, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:292`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-689302c13ebf2f3171dfd9be"></a>
## clone

`function` · `datafusion_proto_models::generated::datafusion::PrepareNode::clone` · datafusion-proto-models 55.1.0

```rust
fn clone(&self) -> PrepareNode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::PrepareNode", "path": "PrepareNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [292, 10], "end": [292, 15], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generated/prost.rs:292`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5c08a306e562294326d9bc34"></a>
## data_types

`struct_field` · `datafusion_proto_models::generated::datafusion::PrepareNode::data_types` · datafusion-proto-models 55.1.0

```rust
data_types: ::prost::alloc::vec::Vec<super::datafusion_common::ArrowType>
```

Source: `src/generated/prost.rs:299`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

We serialize both the data types and the fields for compatibility with
older versions (newer versions populate both).

<a id="op-a349e42820f1b1bc9cb623d7"></a>
## default

`function` · `datafusion_proto_models::generated::datafusion::PrepareNode::default` · datafusion-proto-models 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::PrepareNode", "path": "PrepareNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [292, 28], "end": [292, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/generated/prost.rs:292`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f82c28313cbe5d2be054a897"></a>
## deserialize

`function` · `datafusion_proto_models::generated::datafusion::PrepareNode::deserialize` · datafusion-proto-models 55.1.0

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::PrepareNode", "path": "PrepareNode"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [22749, 1], "end": [22855, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/generated/pbjson.rs:22751`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6fb53166834e3c25aaca25c7"></a>
## encoded_len

`function` · `datafusion_proto_models::generated::datafusion::PrepareNode::encoded_len` · datafusion-proto-models 55.1.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::PrepareNode", "path": "PrepareNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [292, 28], "end": [292, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:292`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f00cf42994e8a580ba055adb"></a>
## eq

`function` · `datafusion_proto_models::generated::datafusion::PrepareNode::eq` · datafusion-proto-models 55.1.0

```rust
fn eq(&self, other: &PrepareNode) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::PrepareNode", "path": "PrepareNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [292, 17], "end": [292, 26], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/generated/prost.rs:292`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c4fadaf78d57cf2c6357a7bd"></a>
## fields

`struct_field` · `datafusion_proto_models::generated::datafusion::PrepareNode::fields` · datafusion-proto-models 55.1.0

```rust
fields: ::prost::alloc::vec::Vec<super::datafusion_common::Field>
```

Source: `src/generated/prost.rs:303`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5e6cb887adffebef8f87fe96"></a>
## fmt

`function` · `datafusion_proto_models::generated::datafusion::PrepareNode::fmt` · datafusion-proto-models 55.1.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::PrepareNode", "path": "PrepareNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [292, 28], "end": [292, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generated/prost.rs:292`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-873a3b1d1d0f6c114982e8c8"></a>
## input

`struct_field` · `datafusion_proto_models::generated::datafusion::PrepareNode::input` · datafusion-proto-models 55.1.0

```rust
input: ::core::option::Option<::prost::alloc::boxed::Box<LogicalPlanNode>>
```

Source: `src/generated/prost.rs:301`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-28c427d4b1a730b07f38396f"></a>
## name

`struct_field` · `datafusion_proto_models::generated::datafusion::PrepareNode::name` · datafusion-proto-models 55.1.0

```rust
name: ::prost::alloc::string::String
```

Source: `src/generated/prost.rs:295`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-50f9e64e5577e7335b165d99"></a>
## serialize

`function` · `datafusion_proto_models::generated::datafusion::PrepareNode::serialize` · datafusion-proto-models 55.1.0

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::PrepareNode", "path": "PrepareNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22713, 1], "end": [22748, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/generated/pbjson.rs:22715`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
