# `datafusion_proto_models::generated::datafusion::window_expr_node::WindowFunction`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_models.generated.datafusion.window_expr_node.WindowFunction.json).

<a id="op-dfc16556216fd5d67d84ad7d"></a>
## WindowFunction

`enum` · `datafusion_proto_models::generated::datafusion::window_expr_node::WindowFunction` · datafusion-proto-models 55.1.0

```rust
enum WindowFunction
```

Source: `src/generated/prost.rs:1100`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a099ffcc4c6012bd46e4adcf"></a>
## Udaf

`variant` · `datafusion_proto_models::generated::datafusion::window_expr_node::WindowFunction::Udaf` · datafusion-proto-models 55.1.0

```rust
Udaf
```

Source: `src/generated/prost.rs:1103`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

BuiltInWindowFunction built_in_function = 2;

<a id="op-837b1224eeacf10695a805df"></a>
## Udwf

`variant` · `datafusion_proto_models::generated::datafusion::window_expr_node::WindowFunction::Udwf` · datafusion-proto-models 55.1.0

```rust
Udwf
```

Source: `src/generated/prost.rs:1105`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ee8409c7158d9a0937600bcb"></a>
## clone

`function` · `datafusion_proto_models::generated::datafusion::window_expr_node::WindowFunction::clone` · datafusion-proto-models 55.1.0

```rust
fn clone(&self) -> WindowFunction
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::window_expr_node::WindowFunction", "path": "WindowFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1099, 14], "end": [1099, 19], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generated/prost.rs:1099`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6433c4260d8c2673e448443e"></a>
## encode

`function` · `datafusion_proto_models::generated::datafusion::window_expr_node::WindowFunction::encode` · datafusion-proto-models 55.1.0

```rust
fn encode(&self, buf: &mut impl ::prost::bytes::BufMut)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::window_expr_node::WindowFunction", "path": "WindowFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1099, 42], "end": [1099, 56], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:1099`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Encodes the message to a buffer.

<a id="op-2111a559ba34ca5471b48f68"></a>
## encoded_len

`function` · `datafusion_proto_models::generated::datafusion::window_expr_node::WindowFunction::encoded_len` · datafusion-proto-models 55.1.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::window_expr_node::WindowFunction", "path": "WindowFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1099, 42], "end": [1099, 56], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:1099`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Returns the encoded length of the message without a length delimiter.

<a id="op-0787db87a6e9f90fa93f1f95"></a>
## eq

`function` · `datafusion_proto_models::generated::datafusion::window_expr_node::WindowFunction::eq` · datafusion-proto-models 55.1.0

```rust
fn eq(&self, other: &WindowFunction) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::window_expr_node::WindowFunction", "path": "WindowFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1099, 21], "end": [1099, 30], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/generated/prost.rs:1099`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-de15d3d9378ef63f9b7a6c6e"></a>
## fmt

`function` · `datafusion_proto_models::generated::datafusion::window_expr_node::WindowFunction::fmt` · datafusion-proto-models 55.1.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::window_expr_node::WindowFunction", "path": "WindowFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1099, 42], "end": [1099, 56], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generated/prost.rs:1099`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb050888b35fb7298bf819e4"></a>
## hash

`function` · `datafusion_proto_models::generated::datafusion::window_expr_node::WindowFunction::hash` · datafusion-proto-models 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::window_expr_node::WindowFunction", "path": "WindowFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1099, 36], "end": [1099, 40], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/generated/prost.rs:1099`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-92d1fc24c99bda21fecf3eb7"></a>
## merge

`function` · `datafusion_proto_models::generated::datafusion::window_expr_node::WindowFunction::merge` · datafusion-proto-models 55.1.0

```rust
fn merge(field: &mut ::core::option::Option<WindowFunction>, tag: u32, wire_type: ::prost::encoding::wire_type::WireType, buf: &mut impl ::prost::bytes::Buf, ctx: ::prost::encoding::DecodeContext) -> ::core::result::Result<(), ::prost::DecodeError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::window_expr_node::WindowFunction", "path": "WindowFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1099, 42], "end": [1099, 56], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:1099`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Decodes an instance of the message from a buffer, and merges it into self.
