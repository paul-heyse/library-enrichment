# `datafusion_proto_models::generated::datafusion::AnalyzeNode`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_models.generated.datafusion.AnalyzeNode.json).

<a id="op-5283c9364241774ff48dea7e"></a>
## AnalyzeNode

`struct` · `datafusion_proto_models::generated::datafusion::AnalyzeNode` · datafusion-proto-models 55.1.0

```rust
struct AnalyzeNode
```

Source: `src/generated/prost.rs:355`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a3dab98cb524989581f93b20"></a>
## analyze_categories

`struct_field` · `datafusion_proto_models::generated::datafusion::AnalyzeNode::analyze_categories` · datafusion-proto-models 55.1.0

```rust
analyze_categories: ::core::option::Option<super::datafusion_common::ExplainAnalyzeCategoriesNode>
```

Source: `src/generated/prost.rs:367`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Statement-level override for `datafusion.explain.analyze_categories`.
Absent means "fall back to session config".

<a id="op-60d897ec76b0fcbdf9d5284b"></a>
## analyze_level

`struct_field` · `datafusion_proto_models::generated::datafusion::AnalyzeNode::analyze_level` · datafusion-proto-models 55.1.0

```rust
analyze_level: ::core::option::Option<i32>
```

Source: `src/generated/prost.rs:363`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Statement-level override for `datafusion.explain.analyze_level`.
Absent means "fall back to session config".

<a id="op-be625a8951aa5943589d13e2"></a>
## analyze_level

`function` · `datafusion_proto_models::generated::datafusion::AnalyzeNode::analyze_level` · datafusion-proto-models 55.1.0

```rust
fn analyze_level(&self) -> super::datafusion_common::MetricType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::AnalyzeNode", "path": "AnalyzeNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 28], "end": [354, 44], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:354`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Returns the enum value of `analyze_level`, or the default if the field is unset or set to an invalid enum value.

<a id="op-7413a344465d477925db8beb"></a>
## clear

`function` · `datafusion_proto_models::generated::datafusion::AnalyzeNode::clear` · datafusion-proto-models 55.1.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::AnalyzeNode", "path": "AnalyzeNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 28], "end": [354, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:354`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0eb8ba95b2b291ea3a49c0ac"></a>
## clone

`function` · `datafusion_proto_models::generated::datafusion::AnalyzeNode::clone` · datafusion-proto-models 55.1.0

```rust
fn clone(&self) -> AnalyzeNode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::AnalyzeNode", "path": "AnalyzeNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 10], "end": [354, 15], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generated/prost.rs:354`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7696fc9d0eab549d958dcb8c"></a>
## default

`function` · `datafusion_proto_models::generated::datafusion::AnalyzeNode::default` · datafusion-proto-models 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::AnalyzeNode", "path": "AnalyzeNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 28], "end": [354, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/generated/prost.rs:354`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9547a1f63e39c917602493e6"></a>
## deserialize

`function` · `datafusion_proto_models::generated::datafusion::AnalyzeNode::deserialize` · datafusion-proto-models 55.1.0

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::AnalyzeNode", "path": "AnalyzeNode"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1237, 1], "end": [1355, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/generated/pbjson.rs:1239`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-89fb69f17a3e56022214143f"></a>
## encoded_len

`function` · `datafusion_proto_models::generated::datafusion::AnalyzeNode::encoded_len` · datafusion-proto-models 55.1.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::AnalyzeNode", "path": "AnalyzeNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 28], "end": [354, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:354`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1e0fa46ec1f13ba28a67d8c4"></a>
## eq

`function` · `datafusion_proto_models::generated::datafusion::AnalyzeNode::eq` · datafusion-proto-models 55.1.0

```rust
fn eq(&self, other: &AnalyzeNode) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::AnalyzeNode", "path": "AnalyzeNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 17], "end": [354, 26], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/generated/prost.rs:354`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-13710f33e887d1a28157efcd"></a>
## fmt

`function` · `datafusion_proto_models::generated::datafusion::AnalyzeNode::fmt` · datafusion-proto-models 55.1.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::AnalyzeNode", "path": "AnalyzeNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 28], "end": [354, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generated/prost.rs:354`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6e4e9765b550125a8f14a0fa"></a>
## format

`function` · `datafusion_proto_models::generated::datafusion::AnalyzeNode::format` · datafusion-proto-models 55.1.0

```rust
fn format(&self) -> super::datafusion_common::ExplainFormat
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::AnalyzeNode", "path": "AnalyzeNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 28], "end": [354, 44], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:354`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Returns the enum value of `format`, or the default if the field is set to an invalid enum value.

<a id="op-bec362318ad84eef04cdaa6e"></a>
## format

`struct_field` · `datafusion_proto_models::generated::datafusion::AnalyzeNode::format` · datafusion-proto-models 55.1.0

```rust
format: i32
```

Source: `src/generated/prost.rs:371`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c69d50b831746bf98d9c9d19"></a>
## input

`struct_field` · `datafusion_proto_models::generated::datafusion::AnalyzeNode::input` · datafusion-proto-models 55.1.0

```rust
input: ::core::option::Option<::prost::alloc::boxed::Box<LogicalPlanNode>>
```

Source: `src/generated/prost.rs:357`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0a7d49a6b3e2e77da0d12ddf"></a>
## serialize

`function` · `datafusion_proto_models::generated::datafusion::AnalyzeNode::serialize` · datafusion-proto-models 55.1.0

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::AnalyzeNode", "path": "AnalyzeNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1191, 1], "end": [1236, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/generated/pbjson.rs:1193`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-df7312b2acd8f82817ce66d9"></a>
## set_analyze_level

`function` · `datafusion_proto_models::generated::datafusion::AnalyzeNode::set_analyze_level` · datafusion-proto-models 55.1.0

```rust
fn set_analyze_level(&mut self, value: super::datafusion_common::MetricType)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::AnalyzeNode", "path": "AnalyzeNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 28], "end": [354, 44], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:354`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Sets `analyze_level` to the provided enum value.

<a id="op-d91656fc105ab3cd71a07d23"></a>
## set_format

`function` · `datafusion_proto_models::generated::datafusion::AnalyzeNode::set_format` · datafusion-proto-models 55.1.0

```rust
fn set_format(&mut self, value: super::datafusion_common::ExplainFormat)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::AnalyzeNode", "path": "AnalyzeNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 28], "end": [354, 44], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:354`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Sets `format` to the provided enum value.

<a id="op-0134a1292240ac92d9c510f1"></a>
## verbose

`struct_field` · `datafusion_proto_models::generated::datafusion::AnalyzeNode::verbose` · datafusion-proto-models 55.1.0

```rust
verbose: bool
```

Source: `src/generated/prost.rs:359`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
