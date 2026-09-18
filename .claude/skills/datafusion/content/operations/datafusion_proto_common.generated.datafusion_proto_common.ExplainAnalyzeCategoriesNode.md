# `datafusion_proto_common::generated::datafusion_proto_common::ExplainAnalyzeCategoriesNode`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_common.generated.datafusion_proto_common.ExplainAnalyzeCategoriesNode.json).

<a id="op-03a8c119325850c5a249ade9"></a>
## ExplainAnalyzeCategoriesNode

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::ExplainAnalyzeCategoriesNode` · datafusion-proto-common 55.1.0

```rust
struct ExplainAnalyzeCategoriesNode
```

Source: `src/generated/prost.rs:1037`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Wire encoding for `datafusion_common::format::ExplainAnalyzeCategories`.

If `all` is true, every category is shown (the `only` list is ignored).
If `all` is false, only the categories listed in `only` are shown — an
empty `only` means "plan only", i.e. suppress all metrics.

<a id="op-f44e0fb0d097f94c9ddd6b86"></a>
## all

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::ExplainAnalyzeCategoriesNode::all` · datafusion-proto-common 55.1.0

```rust
all: bool
```

Source: `src/generated/prost.rs:1039`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-957ff26a18ac6a29c2888e80"></a>
## clear

`function` · `datafusion_proto_common::generated::datafusion_proto_common::ExplainAnalyzeCategoriesNode::clear` · datafusion-proto-common 55.1.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::ExplainAnalyzeCategoriesNode", "path": "ExplainAnalyzeCategoriesNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1036, 38], "end": [1036, 54], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:1036`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-68ecdf0a88aeb1f952d613d9"></a>
## clone

`function` · `datafusion_proto_common::generated::datafusion_proto_common::ExplainAnalyzeCategoriesNode::clone` · datafusion-proto-common 55.1.0

```rust
fn clone(&self) -> ExplainAnalyzeCategoriesNode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::ExplainAnalyzeCategoriesNode", "path": "ExplainAnalyzeCategoriesNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1036, 10], "end": [1036, 15], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generated/prost.rs:1036`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d24ad531dc2f5ed3948d9e25"></a>
## default

`function` · `datafusion_proto_common::generated::datafusion_proto_common::ExplainAnalyzeCategoriesNode::default` · datafusion-proto-common 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::ExplainAnalyzeCategoriesNode", "path": "ExplainAnalyzeCategoriesNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1036, 38], "end": [1036, 54], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/generated/prost.rs:1036`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-48c58d4337c4a6704fe7188f"></a>
## deserialize

`function` · `datafusion_proto_common::generated::datafusion_proto_common::ExplainAnalyzeCategoriesNode::deserialize` · datafusion-proto-common 55.1.0

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::ExplainAnalyzeCategoriesNode", "path": "ExplainAnalyzeCategoriesNode"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [4009, 1], "end": [4092, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/generated/pbjson.rs:4011`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-28d455f9b90e1f84c5fc27d5"></a>
## encoded_len

`function` · `datafusion_proto_common::generated::datafusion_proto_common::ExplainAnalyzeCategoriesNode::encoded_len` · datafusion-proto-common 55.1.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::ExplainAnalyzeCategoriesNode", "path": "ExplainAnalyzeCategoriesNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1036, 38], "end": [1036, 54], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:1036`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d6e500daf1da8e9746c959e9"></a>
## eq

`function` · `datafusion_proto_common::generated::datafusion_proto_common::ExplainAnalyzeCategoriesNode::eq` · datafusion-proto-common 55.1.0

```rust
fn eq(&self, other: &ExplainAnalyzeCategoriesNode) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::ExplainAnalyzeCategoriesNode", "path": "ExplainAnalyzeCategoriesNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1036, 17], "end": [1036, 26], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/generated/prost.rs:1036`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-072463c5bb2264e12190baed"></a>
## fmt

`function` · `datafusion_proto_common::generated::datafusion_proto_common::ExplainAnalyzeCategoriesNode::fmt` · datafusion-proto-common 55.1.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::ExplainAnalyzeCategoriesNode", "path": "ExplainAnalyzeCategoriesNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1036, 38], "end": [1036, 54], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generated/prost.rs:1036`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-995e265fee28697af1186358"></a>
## hash

`function` · `datafusion_proto_common::generated::datafusion_proto_common::ExplainAnalyzeCategoriesNode::hash` · datafusion-proto-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::ExplainAnalyzeCategoriesNode", "path": "ExplainAnalyzeCategoriesNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1036, 32], "end": [1036, 36], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/generated/prost.rs:1036`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3bfb207fc09139eb749965cc"></a>
## only

`function` · `datafusion_proto_common::generated::datafusion_proto_common::ExplainAnalyzeCategoriesNode::only` · datafusion-proto-common 55.1.0

```rust
fn only(&self) -> ::core::iter::FilterMap<::core::iter::Cloned<::core::slice::Iter<'_, i32>>, fn(i32) -> ::core::option::Option<MetricCategory>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::ExplainAnalyzeCategoriesNode", "path": "ExplainAnalyzeCategoriesNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1036, 38], "end": [1036, 54], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:1036`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Returns an iterator which yields the valid enum values contained in `only`.

<a id="op-606af56c5459927bc9b96206"></a>
## only

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::ExplainAnalyzeCategoriesNode::only` · datafusion-proto-common 55.1.0

```rust
only: ::prost::alloc::vec::Vec<i32>
```

Source: `src/generated/prost.rs:1041`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a67811b236b5eaa2b96865e5"></a>
## push_only

`function` · `datafusion_proto_common::generated::datafusion_proto_common::ExplainAnalyzeCategoriesNode::push_only` · datafusion-proto-common 55.1.0

```rust
fn push_only(&mut self, value: MetricCategory)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::ExplainAnalyzeCategoriesNode", "path": "ExplainAnalyzeCategoriesNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1036, 38], "end": [1036, 54], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:1036`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Appends the provided enum value to `only`.

<a id="op-8ed931b9de37c840520199bd"></a>
## serialize

`function` · `datafusion_proto_common::generated::datafusion_proto_common::ExplainAnalyzeCategoriesNode::serialize` · datafusion-proto-common 55.1.0

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::ExplainAnalyzeCategoriesNode", "path": "ExplainAnalyzeCategoriesNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3981, 1], "end": [4008, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/generated/pbjson.rs:3983`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
