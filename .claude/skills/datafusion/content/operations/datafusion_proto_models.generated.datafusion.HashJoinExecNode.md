# `datafusion_proto_models::generated::datafusion::HashJoinExecNode`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_models.generated.datafusion.HashJoinExecNode.json).

<a id="op-c41f598a208a07f49ee552f7"></a>
## HashJoinExecNode

`struct` · `datafusion_proto_models::generated::datafusion::HashJoinExecNode` · datafusion-proto-models 55.1.0

```rust
struct HashJoinExecNode
```

Source: `src/generated/prost.rs:2021`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-86391d52dceaf50350e55983"></a>
## clear

`function` · `datafusion_proto_models::generated::datafusion::HashJoinExecNode::clear` · datafusion-proto-models 55.1.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::HashJoinExecNode", "path": "HashJoinExecNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2020, 28], "end": [2020, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:2020`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-33411a271a80c4e51894faff"></a>
## clone

`function` · `datafusion_proto_models::generated::datafusion::HashJoinExecNode::clone` · datafusion-proto-models 55.1.0

```rust
fn clone(&self) -> HashJoinExecNode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::HashJoinExecNode", "path": "HashJoinExecNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2020, 10], "end": [2020, 15], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generated/prost.rs:2020`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-20508d008eac309369320e84"></a>
## default

`function` · `datafusion_proto_models::generated::datafusion::HashJoinExecNode::default` · datafusion-proto-models 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::HashJoinExecNode", "path": "HashJoinExecNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2020, 28], "end": [2020, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/generated/prost.rs:2020`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-14829cd0b1bc041912480d71"></a>
## deserialize

`function` · `datafusion_proto_models::generated::datafusion::HashJoinExecNode::deserialize` · datafusion-proto-models 55.1.0

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::HashJoinExecNode", "path": "HashJoinExecNode"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [9112, 1], "end": [9304, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/generated/pbjson.rs:9114`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6dfb0410ecb0b66513c79e52"></a>
## dynamic_filter

`struct_field` · `datafusion_proto_models::generated::datafusion::HashJoinExecNode::dynamic_filter` · datafusion-proto-models 55.1.0

```rust
dynamic_filter: ::core::option::Option<PhysicalExprNode>
```

Source: `src/generated/prost.rs:2042`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Optional dynamic filter expression for pushing down to the probe side.

<a id="op-e8c6c932c6d3cd06b4f0ee24"></a>
## encoded_len

`function` · `datafusion_proto_models::generated::datafusion::HashJoinExecNode::encoded_len` · datafusion-proto-models 55.1.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::HashJoinExecNode", "path": "HashJoinExecNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2020, 28], "end": [2020, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:2020`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c83e407cc8e71a53c71fa46b"></a>
## eq

`function` · `datafusion_proto_models::generated::datafusion::HashJoinExecNode::eq` · datafusion-proto-models 55.1.0

```rust
fn eq(&self, other: &HashJoinExecNode) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::HashJoinExecNode", "path": "HashJoinExecNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2020, 17], "end": [2020, 26], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/generated/prost.rs:2020`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-477d6bb0216fd6d518b350d3"></a>
## fetch

`struct_field` · `datafusion_proto_models::generated::datafusion::HashJoinExecNode::fetch` · datafusion-proto-models 55.1.0

```rust
fetch: ::core::option::Option<u64>
```

Source: `src/generated/prost.rs:2051`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Optional row limit pushed into the join by the `limit_pushdown` rule.

This is presence-tracked (`optional`) on purpose: messages produced by
versions predating this field carry no `fetch` at all, and a plain proto3
scalar would decode that absence as `0`, i.e. "fetch 0 rows", silently
turning old plans into empty results. With `optional`, absent decodes to
`None`, which is the correct reading of an older message.

<a id="op-4938267c71f38b9f3c490768"></a>
## fetch

`function` · `datafusion_proto_models::generated::datafusion::HashJoinExecNode::fetch` · datafusion-proto-models 55.1.0

```rust
fn fetch(&self) -> u64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::HashJoinExecNode", "path": "HashJoinExecNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2020, 28], "end": [2020, 44], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:2020`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Returns the value of `fetch`, or the default value if `fetch` is unset.

<a id="op-199b8b35158dfc62c137431d"></a>
## filter

`struct_field` · `datafusion_proto_models::generated::datafusion::HashJoinExecNode::filter` · datafusion-proto-models 55.1.0

```rust
filter: ::core::option::Option<JoinFilter>
```

Source: `src/generated/prost.rs:2035`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b9bea5f630d2327611d7e712"></a>
## fmt

`function` · `datafusion_proto_models::generated::datafusion::HashJoinExecNode::fmt` · datafusion-proto-models 55.1.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::HashJoinExecNode", "path": "HashJoinExecNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2020, 28], "end": [2020, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generated/prost.rs:2020`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9f85ea43a016a73e2f99d8db"></a>
## join_type

`function` · `datafusion_proto_models::generated::datafusion::HashJoinExecNode::join_type` · datafusion-proto-models 55.1.0

```rust
fn join_type(&self) -> super::datafusion_common::JoinType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::HashJoinExecNode", "path": "HashJoinExecNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2020, 28], "end": [2020, 44], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:2020`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Returns the enum value of `join_type`, or the default if the field is set to an invalid enum value.

<a id="op-e671b000f0c028153d20242f"></a>
## join_type

`struct_field` · `datafusion_proto_models::generated::datafusion::HashJoinExecNode::join_type` · datafusion-proto-models 55.1.0

```rust
join_type: i32
```

Source: `src/generated/prost.rs:2029`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3112a0225f6380438ff8bea7"></a>
## left

`struct_field` · `datafusion_proto_models::generated::datafusion::HashJoinExecNode::left` · datafusion-proto-models 55.1.0

```rust
left: ::core::option::Option<::prost::alloc::boxed::Box<PhysicalPlanNode>>
```

Source: `src/generated/prost.rs:2023`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3c0a9279a7be0f3a61dbb9d3"></a>
## null_aware

`struct_field` · `datafusion_proto_models::generated::datafusion::HashJoinExecNode::null_aware` · datafusion-proto-models 55.1.0

```rust
null_aware: bool
```

Source: `src/generated/prost.rs:2039`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c90133907558dd6c80dc487"></a>
## null_equality

`function` · `datafusion_proto_models::generated::datafusion::HashJoinExecNode::null_equality` · datafusion-proto-models 55.1.0

```rust
fn null_equality(&self) -> super::datafusion_common::NullEquality
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::HashJoinExecNode", "path": "HashJoinExecNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2020, 28], "end": [2020, 44], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:2020`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Returns the enum value of `null_equality`, or the default if the field is set to an invalid enum value.

<a id="op-9f088a0b0b7e07eb5f99b63f"></a>
## null_equality

`struct_field` · `datafusion_proto_models::generated::datafusion::HashJoinExecNode::null_equality` · datafusion-proto-models 55.1.0

```rust
null_equality: i32
```

Source: `src/generated/prost.rs:2033`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-391c753f44a0544d88960927"></a>
## on

`struct_field` · `datafusion_proto_models::generated::datafusion::HashJoinExecNode::on` · datafusion-proto-models 55.1.0

```rust
on: ::prost::alloc::vec::Vec<JoinOn>
```

Source: `src/generated/prost.rs:2027`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bde0bd8fbb33807424042837"></a>
## partition_mode

`function` · `datafusion_proto_models::generated::datafusion::HashJoinExecNode::partition_mode` · datafusion-proto-models 55.1.0

```rust
fn partition_mode(&self) -> PartitionMode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::HashJoinExecNode", "path": "HashJoinExecNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2020, 28], "end": [2020, 44], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:2020`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Returns the enum value of `partition_mode`, or the default if the field is set to an invalid enum value.

<a id="op-ffe4c253575b1c9e42b09fde"></a>
## partition_mode

`struct_field` · `datafusion_proto_models::generated::datafusion::HashJoinExecNode::partition_mode` · datafusion-proto-models 55.1.0

```rust
partition_mode: i32
```

Source: `src/generated/prost.rs:2031`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-307060189666ae3fc88ff7b4"></a>
## projection

`struct_field` · `datafusion_proto_models::generated::datafusion::HashJoinExecNode::projection` · datafusion-proto-models 55.1.0

```rust
projection: ::prost::alloc::vec::Vec<u32>
```

Source: `src/generated/prost.rs:2037`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7d9762752a320af4f8d2df83"></a>
## right

`struct_field` · `datafusion_proto_models::generated::datafusion::HashJoinExecNode::right` · datafusion-proto-models 55.1.0

```rust
right: ::core::option::Option<::prost::alloc::boxed::Box<PhysicalPlanNode>>
```

Source: `src/generated/prost.rs:2025`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e0e01167bbe53c765779f7c6"></a>
## serialize

`function` · `datafusion_proto_models::generated::datafusion::HashJoinExecNode::serialize` · datafusion-proto-models 55.1.0

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::HashJoinExecNode", "path": "HashJoinExecNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9026, 1], "end": [9111, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/generated/pbjson.rs:9028`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-acb4e002bce2f96b3aa4b418"></a>
## set_join_type

`function` · `datafusion_proto_models::generated::datafusion::HashJoinExecNode::set_join_type` · datafusion-proto-models 55.1.0

```rust
fn set_join_type(&mut self, value: super::datafusion_common::JoinType)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::HashJoinExecNode", "path": "HashJoinExecNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2020, 28], "end": [2020, 44], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:2020`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Sets `join_type` to the provided enum value.

<a id="op-5f39161df36409d151c1023b"></a>
## set_null_equality

`function` · `datafusion_proto_models::generated::datafusion::HashJoinExecNode::set_null_equality` · datafusion-proto-models 55.1.0

```rust
fn set_null_equality(&mut self, value: super::datafusion_common::NullEquality)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::HashJoinExecNode", "path": "HashJoinExecNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2020, 28], "end": [2020, 44], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:2020`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Sets `null_equality` to the provided enum value.

<a id="op-2f64055fb30ba8fe2bd08ad3"></a>
## set_partition_mode

`function` · `datafusion_proto_models::generated::datafusion::HashJoinExecNode::set_partition_mode` · datafusion-proto-models 55.1.0

```rust
fn set_partition_mode(&mut self, value: PartitionMode)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::HashJoinExecNode", "path": "HashJoinExecNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2020, 28], "end": [2020, 44], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:2020`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Sets `partition_mode` to the provided enum value.
