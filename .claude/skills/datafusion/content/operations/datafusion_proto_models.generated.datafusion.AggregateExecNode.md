# `datafusion_proto_models::generated::datafusion::AggregateExecNode`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_models.generated.datafusion.AggregateExecNode.json).

<a id="op-8ab4ede5f0094b14a48abd4e"></a>
## AggregateExecNode

`struct` · `datafusion_proto_models::generated::datafusion::AggregateExecNode` · datafusion-proto-models 55.1.0

```rust
struct AggregateExecNode
```

Source: `src/generated/prost.rs:2215`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0b1d49c45c49ea4f7dc81dd9"></a>
## aggr_expr

`struct_field` · `datafusion_proto_models::generated::datafusion::AggregateExecNode::aggr_expr` · datafusion-proto-models 55.1.0

```rust
aggr_expr: ::prost::alloc::vec::Vec<PhysicalExprNode>
```

Source: `src/generated/prost.rs:2219`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ec8c076504c9741dc41bff54"></a>
## aggr_expr_name

`struct_field` · `datafusion_proto_models::generated::datafusion::AggregateExecNode::aggr_expr_name` · datafusion-proto-models 55.1.0

```rust
aggr_expr_name: ::prost::alloc::vec::Vec<::prost::alloc::string::String>
```

Source: `src/generated/prost.rs:2227`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-be8bd6d8973e0cb901501276"></a>
## clear

`function` · `datafusion_proto_models::generated::datafusion::AggregateExecNode::clear` · datafusion-proto-models 55.1.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::AggregateExecNode", "path": "AggregateExecNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2214, 28], "end": [2214, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:2214`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d26ae73ae7de4a20b06ed27c"></a>
## clone

`function` · `datafusion_proto_models::generated::datafusion::AggregateExecNode::clone` · datafusion-proto-models 55.1.0

```rust
fn clone(&self) -> AggregateExecNode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::AggregateExecNode", "path": "AggregateExecNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2214, 10], "end": [2214, 15], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generated/prost.rs:2214`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bf85e315dadbb2c046657d27"></a>
## default

`function` · `datafusion_proto_models::generated::datafusion::AggregateExecNode::default` · datafusion-proto-models 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::AggregateExecNode", "path": "AggregateExecNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2214, 28], "end": [2214, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/generated/prost.rs:2214`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8b778c07f64a4077a5c513cd"></a>
## deserialize

`function` · `datafusion_proto_models::generated::datafusion::AggregateExecNode::deserialize` · datafusion-proto-models 55.1.0

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::AggregateExecNode", "path": "AggregateExecNode"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [211, 1], "end": [435, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/generated/pbjson.rs:213`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e8e9cb80ada31f9823f8a147"></a>
## dynamic_filter

`struct_field` · `datafusion_proto_models::generated::datafusion::AggregateExecNode::dynamic_filter` · datafusion-proto-models 55.1.0

```rust
dynamic_filter: ::core::option::Option<PhysicalExprNode>
```

Source: `src/generated/prost.rs:2243`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Optional dynamic filter expression for pushing down to the child.

<a id="op-6de840fdb29d2c6c2a2b8f1a"></a>
## encoded_len

`function` · `datafusion_proto_models::generated::datafusion::AggregateExecNode::encoded_len` · datafusion-proto-models 55.1.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::AggregateExecNode", "path": "AggregateExecNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2214, 28], "end": [2214, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:2214`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-28a466c3c5c392b79dc3ee4c"></a>
## eq

`function` · `datafusion_proto_models::generated::datafusion::AggregateExecNode::eq` · datafusion-proto-models 55.1.0

```rust
fn eq(&self, other: &AggregateExecNode) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::AggregateExecNode", "path": "AggregateExecNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2214, 17], "end": [2214, 26], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/generated/prost.rs:2214`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a81d13d85843630719c1a076"></a>
## filter_expr

`struct_field` · `datafusion_proto_models::generated::datafusion::AggregateExecNode::filter_expr` · datafusion-proto-models 55.1.0

```rust
filter_expr: ::prost::alloc::vec::Vec<MaybeFilter>
```

Source: `src/generated/prost.rs:2236`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b873b9ac7cda12d6b066690a"></a>
## fmt

`function` · `datafusion_proto_models::generated::datafusion::AggregateExecNode::fmt` · datafusion-proto-models 55.1.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::AggregateExecNode", "path": "AggregateExecNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2214, 28], "end": [2214, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generated/prost.rs:2214`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a58c84c00483f9abbcc95b77"></a>
## group_expr

`struct_field` · `datafusion_proto_models::generated::datafusion::AggregateExecNode::group_expr` · datafusion-proto-models 55.1.0

```rust
group_expr: ::prost::alloc::vec::Vec<PhysicalExprNode>
```

Source: `src/generated/prost.rs:2217`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-77c66b45d5fc2313bbb2142f"></a>
## group_expr_name

`struct_field` · `datafusion_proto_models::generated::datafusion::AggregateExecNode::group_expr_name` · datafusion-proto-models 55.1.0

```rust
group_expr_name: ::prost::alloc::vec::Vec<::prost::alloc::string::String>
```

Source: `src/generated/prost.rs:2225`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cf18eca19d77bf4d66d731e9"></a>
## groups

`struct_field` · `datafusion_proto_models::generated::datafusion::AggregateExecNode::groups` · datafusion-proto-models 55.1.0

```rust
groups: ::prost::alloc::vec::Vec<bool>
```

Source: `src/generated/prost.rs:2234`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7ab616d2cdba85c9db2ac28d"></a>
## has_grouping_set

`struct_field` · `datafusion_proto_models::generated::datafusion::AggregateExecNode::has_grouping_set` · datafusion-proto-models 55.1.0

```rust
has_grouping_set: bool
```

Source: `src/generated/prost.rs:2240`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d82581b126824a1bdfad7fa0"></a>
## input

`struct_field` · `datafusion_proto_models::generated::datafusion::AggregateExecNode::input` · datafusion-proto-models 55.1.0

```rust
input: ::core::option::Option<::prost::alloc::boxed::Box<PhysicalPlanNode>>
```

Source: `src/generated/prost.rs:2223`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c4dcd8020c46c12b01bbadda"></a>
## input_schema

`struct_field` · `datafusion_proto_models::generated::datafusion::AggregateExecNode::input_schema` · datafusion-proto-models 55.1.0

```rust
input_schema: ::core::option::Option<super::datafusion_common::Schema>
```

Source: `src/generated/prost.rs:2230`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

we need the input schema to the partial aggregate to pass to the final aggregate

<a id="op-db720981aee65c595d2a81ef"></a>
## limit

`struct_field` · `datafusion_proto_models::generated::datafusion::AggregateExecNode::limit` · datafusion-proto-models 55.1.0

```rust
limit: ::core::option::Option<AggLimit>
```

Source: `src/generated/prost.rs:2238`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5dd87826329e526e1ae5fb5a"></a>
## mode

`function` · `datafusion_proto_models::generated::datafusion::AggregateExecNode::mode` · datafusion-proto-models 55.1.0

```rust
fn mode(&self) -> AggregateMode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::AggregateExecNode", "path": "AggregateExecNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2214, 28], "end": [2214, 44], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:2214`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Returns the enum value of `mode`, or the default if the field is set to an invalid enum value.

<a id="op-a70e7523099e1d0028e429b5"></a>
## mode

`struct_field` · `datafusion_proto_models::generated::datafusion::AggregateExecNode::mode` · datafusion-proto-models 55.1.0

```rust
mode: i32
```

Source: `src/generated/prost.rs:2221`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-324dc954b93bdf0b24407889"></a>
## null_expr

`struct_field` · `datafusion_proto_models::generated::datafusion::AggregateExecNode::null_expr` · datafusion-proto-models 55.1.0

```rust
null_expr: ::prost::alloc::vec::Vec<PhysicalExprNode>
```

Source: `src/generated/prost.rs:2232`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-88a893863fea791f9fe01527"></a>
## schema

`struct_field` · `datafusion_proto_models::generated::datafusion::AggregateExecNode::schema` · datafusion-proto-models 55.1.0

```rust
schema: ::core::option::Option<super::datafusion_common::Schema>
```

Source: `src/generated/prost.rs:2246`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Output schema preserved by physical optimizer rewrites.

<a id="op-70d056a0123348c322dbfdad"></a>
## serialize

`function` · `datafusion_proto_models::generated::datafusion::AggregateExecNode::serialize` · datafusion-proto-models 55.1.0

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::AggregateExecNode", "path": "AggregateExecNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [113, 1], "end": [210, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/generated/pbjson.rs:115`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-85d4118e3dd764341001af4a"></a>
## set_mode

`function` · `datafusion_proto_models::generated::datafusion::AggregateExecNode::set_mode` · datafusion-proto-models 55.1.0

```rust
fn set_mode(&mut self, value: AggregateMode)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::AggregateExecNode", "path": "AggregateExecNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2214, 28], "end": [2214, 44], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:2214`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Sets `mode` to the provided enum value.
