# `datafusion_proto_models::generated::datafusion::ValuesNode`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_models.generated.datafusion.ValuesNode.json).

<a id="op-f77799ddf07e7278c573b8fe"></a>
## ValuesNode

`struct` · `datafusion_proto_models::generated::datafusion::ValuesNode` · datafusion-proto-models 55.1.0

```rust
struct ValuesNode
```

Source: `src/generated/prost.rs:348`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

a node containing data for defining values list. unlike in SQL where it's two dimensional, here
the list is flattened, and with the field n_cols it can be parsed and partitioned into rows

<a id="op-ebb857a5d2f7100a9bf7cc64"></a>
## clear

`function` · `datafusion_proto_models::generated::datafusion::ValuesNode::clear` · datafusion-proto-models 55.1.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::ValuesNode", "path": "ValuesNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [347, 28], "end": [347, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:347`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b8a06bfb8a0a99c15762b3e8"></a>
## clone

`function` · `datafusion_proto_models::generated::datafusion::ValuesNode::clone` · datafusion-proto-models 55.1.0

```rust
fn clone(&self) -> ValuesNode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::ValuesNode", "path": "ValuesNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [347, 10], "end": [347, 15], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generated/prost.rs:347`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fdfd5d66435ec93cd5ccbdc9"></a>
## default

`function` · `datafusion_proto_models::generated::datafusion::ValuesNode::default` · datafusion-proto-models 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::ValuesNode", "path": "ValuesNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [347, 28], "end": [347, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/generated/prost.rs:347`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5131baaf3de488d61a4844d6"></a>
## deserialize

`function` · `datafusion_proto_models::generated::datafusion::ValuesNode::deserialize` · datafusion-proto-models 55.1.0

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::ValuesNode", "path": "ValuesNode"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [27701, 1], "end": [27788, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/generated/pbjson.rs:27703`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4004c2d0471be4bff9800e39"></a>
## encoded_len

`function` · `datafusion_proto_models::generated::datafusion::ValuesNode::encoded_len` · datafusion-proto-models 55.1.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::ValuesNode", "path": "ValuesNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [347, 28], "end": [347, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:347`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a67bbfb0df393d6af79cda28"></a>
## eq

`function` · `datafusion_proto_models::generated::datafusion::ValuesNode::eq` · datafusion-proto-models 55.1.0

```rust
fn eq(&self, other: &ValuesNode) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::ValuesNode", "path": "ValuesNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [347, 17], "end": [347, 26], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/generated/prost.rs:347`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f9444f1b2c5ecdfbafe36cb3"></a>
## fmt

`function` · `datafusion_proto_models::generated::datafusion::ValuesNode::fmt` · datafusion-proto-models 55.1.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::ValuesNode", "path": "ValuesNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [347, 28], "end": [347, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generated/prost.rs:347`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-369e94119ace082c8b176482"></a>
## n_cols

`struct_field` · `datafusion_proto_models::generated::datafusion::ValuesNode::n_cols` · datafusion-proto-models 55.1.0

```rust
n_cols: u64
```

Source: `src/generated/prost.rs:350`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-36ce594c55d224cf2e31cf28"></a>
## serialize

`function` · `datafusion_proto_models::generated::datafusion::ValuesNode::serialize` · datafusion-proto-models 55.1.0

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::ValuesNode", "path": "ValuesNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27675, 1], "end": [27700, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/generated/pbjson.rs:27677`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b7b189e6ffe87d9fe3fc2abe"></a>
## values_list

`struct_field` · `datafusion_proto_models::generated::datafusion::ValuesNode::values_list` · datafusion-proto-models 55.1.0

```rust
values_list: ::prost::alloc::vec::Vec<LogicalExprNode>
```

Source: `src/generated/prost.rs:352`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
