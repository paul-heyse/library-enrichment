# `datafusion_proto_models::generated::datafusion::ExplainAnalyzeCategoriesNode`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_models.generated.datafusion.ExplainAnalyzeCategoriesNode.json).

<a id="op-449b7d7169b8326c99ed787e"></a>
## ExplainAnalyzeCategoriesNode

`struct` · `datafusion_proto_models::generated::datafusion::ExplainAnalyzeCategoriesNode` · datafusion-proto-models 55.1.0

```rust
struct ExplainAnalyzeCategoriesNode
```

Source: `src/generated/datafusion_proto_common.rs:1037`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Wire encoding for `datafusion_common::format::ExplainAnalyzeCategories`.

If `all` is true, every category is shown (the `only` list is ignored).
If `all` is false, only the categories listed in `only` are shown — an
empty `only` means "plan only", i.e. suppress all metrics.

<a id="op-06b166ac8d92d2480824c6cc"></a>
## all

`struct_field` · `datafusion_proto_models::generated::datafusion::ExplainAnalyzeCategoriesNode::all` · datafusion-proto-models 55.1.0

```rust
all: bool
```

Source: `src/generated/datafusion_proto_common.rs:1039`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-720df1bef08382026ec51ec7"></a>
## clear

`function` · `datafusion_proto_models::generated::datafusion::ExplainAnalyzeCategoriesNode::clear` · datafusion-proto-models 55.1.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::ExplainAnalyzeCategoriesNode", "path": "ExplainAnalyzeCategoriesNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1036, 38], "end": [1036, 54], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/datafusion_proto_common.rs:1036`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fbbc1f3aaf23db9e48ff74c7"></a>
## clone

`function` · `datafusion_proto_models::generated::datafusion::ExplainAnalyzeCategoriesNode::clone` · datafusion-proto-models 55.1.0

```rust
fn clone(&self) -> ExplainAnalyzeCategoriesNode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::ExplainAnalyzeCategoriesNode", "path": "ExplainAnalyzeCategoriesNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1036, 10], "end": [1036, 15], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generated/datafusion_proto_common.rs:1036`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cfa07303420bb4c34e12198b"></a>
## default

`function` · `datafusion_proto_models::generated::datafusion::ExplainAnalyzeCategoriesNode::default` · datafusion-proto-models 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::ExplainAnalyzeCategoriesNode", "path": "ExplainAnalyzeCategoriesNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1036, 38], "end": [1036, 54], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/generated/datafusion_proto_common.rs:1036`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8ed91b7e395509b5af045583"></a>
## encoded_len

`function` · `datafusion_proto_models::generated::datafusion::ExplainAnalyzeCategoriesNode::encoded_len` · datafusion-proto-models 55.1.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::ExplainAnalyzeCategoriesNode", "path": "ExplainAnalyzeCategoriesNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1036, 38], "end": [1036, 54], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/datafusion_proto_common.rs:1036`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b733c72548a2eb1c0435ddfb"></a>
## eq

`function` · `datafusion_proto_models::generated::datafusion::ExplainAnalyzeCategoriesNode::eq` · datafusion-proto-models 55.1.0

```rust
fn eq(&self, other: &ExplainAnalyzeCategoriesNode) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::ExplainAnalyzeCategoriesNode", "path": "ExplainAnalyzeCategoriesNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1036, 17], "end": [1036, 26], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/generated/datafusion_proto_common.rs:1036`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fc52cfaeea523816296723f1"></a>
## fmt

`function` · `datafusion_proto_models::generated::datafusion::ExplainAnalyzeCategoriesNode::fmt` · datafusion-proto-models 55.1.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::ExplainAnalyzeCategoriesNode", "path": "ExplainAnalyzeCategoriesNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1036, 38], "end": [1036, 54], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generated/datafusion_proto_common.rs:1036`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e60d7f7ad41f5f5513680424"></a>
## hash

`function` · `datafusion_proto_models::generated::datafusion::ExplainAnalyzeCategoriesNode::hash` · datafusion-proto-models 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::ExplainAnalyzeCategoriesNode", "path": "ExplainAnalyzeCategoriesNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1036, 32], "end": [1036, 36], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/generated/datafusion_proto_common.rs:1036`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-39c7b369573cbe2eb9792ba8"></a>
## only

`function` · `datafusion_proto_models::generated::datafusion::ExplainAnalyzeCategoriesNode::only` · datafusion-proto-models 55.1.0

```rust
fn only(&self) -> ::core::iter::FilterMap<::core::iter::Cloned<::core::slice::Iter<'_, i32>>, fn(i32) -> ::core::option::Option<MetricCategory>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::ExplainAnalyzeCategoriesNode", "path": "ExplainAnalyzeCategoriesNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1036, 38], "end": [1036, 54], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/datafusion_proto_common.rs:1036`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Returns an iterator which yields the valid enum values contained in `only`.

<a id="op-7afbb8b6bf473f6a490084f8"></a>
## only

`struct_field` · `datafusion_proto_models::generated::datafusion::ExplainAnalyzeCategoriesNode::only` · datafusion-proto-models 55.1.0

```rust
only: ::prost::alloc::vec::Vec<i32>
```

Source: `src/generated/datafusion_proto_common.rs:1041`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a789dafa00e984de30078aa2"></a>
## push_only

`function` · `datafusion_proto_models::generated::datafusion::ExplainAnalyzeCategoriesNode::push_only` · datafusion-proto-models 55.1.0

```rust
fn push_only(&mut self, value: MetricCategory)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::ExplainAnalyzeCategoriesNode", "path": "ExplainAnalyzeCategoriesNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1036, 38], "end": [1036, 54], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/datafusion_proto_common.rs:1036`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Appends the provided enum value to `only`.
