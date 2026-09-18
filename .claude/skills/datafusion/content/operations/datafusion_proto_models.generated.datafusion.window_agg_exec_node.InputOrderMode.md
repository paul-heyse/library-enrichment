# `datafusion_proto_models::generated::datafusion::window_agg_exec_node::InputOrderMode`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_models.generated.datafusion.window_agg_exec_node.InputOrderMode.json).

<a id="op-ae883a35f734e633b7cea129"></a>
## InputOrderMode

`enum` · `datafusion_proto_models::generated::datafusion::window_agg_exec_node::InputOrderMode` · datafusion-proto-models 55.1.0

```rust
enum InputOrderMode
```

Source: `src/generated/prost.rs:2186`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Set optional to `None` for `BoundedWindowAggExec`.

<a id="op-5da3f3ecd49f7df053d83797"></a>
## Linear

`variant` · `datafusion_proto_models::generated::datafusion::window_agg_exec_node::InputOrderMode::Linear` · datafusion-proto-models 55.1.0

```rust
Linear
```

Source: `src/generated/prost.rs:2188`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8934d7693de448fec0ab7ca4"></a>
## PartiallySorted

`variant` · `datafusion_proto_models::generated::datafusion::window_agg_exec_node::InputOrderMode::PartiallySorted` · datafusion-proto-models 55.1.0

```rust
PartiallySorted
```

Source: `src/generated/prost.rs:2190`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d6f4e3b15e3a35a361031ba1"></a>
## Sorted

`variant` · `datafusion_proto_models::generated::datafusion::window_agg_exec_node::InputOrderMode::Sorted` · datafusion-proto-models 55.1.0

```rust
Sorted
```

Source: `src/generated/prost.rs:2192`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-102227b5921afadc6b584272"></a>
## clone

`function` · `datafusion_proto_models::generated::datafusion::window_agg_exec_node::InputOrderMode::clone` · datafusion-proto-models 55.1.0

```rust
fn clone(&self) -> InputOrderMode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::window_agg_exec_node::InputOrderMode", "path": "InputOrderMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2185, 14], "end": [2185, 19], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generated/prost.rs:2185`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-372aaa83f48d257414e8b94e"></a>
## encode

`function` · `datafusion_proto_models::generated::datafusion::window_agg_exec_node::InputOrderMode::encode` · datafusion-proto-models 55.1.0

```rust
fn encode(&self, buf: &mut impl ::prost::bytes::BufMut)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::window_agg_exec_node::InputOrderMode", "path": "InputOrderMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2185, 42], "end": [2185, 56], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:2185`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Encodes the message to a buffer.

<a id="op-c38a272b4d6ddba642c2150a"></a>
## encoded_len

`function` · `datafusion_proto_models::generated::datafusion::window_agg_exec_node::InputOrderMode::encoded_len` · datafusion-proto-models 55.1.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::window_agg_exec_node::InputOrderMode", "path": "InputOrderMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2185, 42], "end": [2185, 56], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:2185`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Returns the encoded length of the message without a length delimiter.

<a id="op-83a8b0cb04917bc84cd16ddc"></a>
## eq

`function` · `datafusion_proto_models::generated::datafusion::window_agg_exec_node::InputOrderMode::eq` · datafusion-proto-models 55.1.0

```rust
fn eq(&self, other: &InputOrderMode) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::window_agg_exec_node::InputOrderMode", "path": "InputOrderMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2185, 21], "end": [2185, 30], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/generated/prost.rs:2185`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-12709280f95fd0ecde3c6bc4"></a>
## fmt

`function` · `datafusion_proto_models::generated::datafusion::window_agg_exec_node::InputOrderMode::fmt` · datafusion-proto-models 55.1.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::window_agg_exec_node::InputOrderMode", "path": "InputOrderMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2185, 42], "end": [2185, 56], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generated/prost.rs:2185`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-599a24208302ca79c27baadc"></a>
## hash

`function` · `datafusion_proto_models::generated::datafusion::window_agg_exec_node::InputOrderMode::hash` · datafusion-proto-models 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::window_agg_exec_node::InputOrderMode", "path": "InputOrderMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2185, 36], "end": [2185, 40], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/generated/prost.rs:2185`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6782d76be22557e06b1a4eef"></a>
## merge

`function` · `datafusion_proto_models::generated::datafusion::window_agg_exec_node::InputOrderMode::merge` · datafusion-proto-models 55.1.0

```rust
fn merge(field: &mut ::core::option::Option<InputOrderMode>, tag: u32, wire_type: ::prost::encoding::wire_type::WireType, buf: &mut impl ::prost::bytes::Buf, ctx: ::prost::encoding::DecodeContext) -> ::core::result::Result<(), ::prost::DecodeError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::window_agg_exec_node::InputOrderMode", "path": "InputOrderMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2185, 42], "end": [2185, 56], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:2185`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Decodes an instance of the message from a buffer, and merges it into self.
