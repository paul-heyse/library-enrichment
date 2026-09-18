# `datafusion_proto_models::generated::datafusion::repartition_node::PartitionMethod`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_models.generated.datafusion.repartition_node.PartitionMethod.json).

<a id="op-1744298d2294c9be8060654b"></a>
## PartitionMethod

`enum` · `datafusion_proto_models::generated::datafusion::repartition_node::PartitionMethod` · datafusion-proto-models 55.1.0

```rust
enum PartitionMethod
```

Source: `src/generated/prost.rs:219`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8f84da64502282e203f22fd0"></a>
## Hash

`variant` · `datafusion_proto_models::generated::datafusion::repartition_node::PartitionMethod::Hash` · datafusion-proto-models 55.1.0

```rust
Hash
```

Source: `src/generated/prost.rs:223`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f6c7f5d287737d8239c575e3"></a>
## Range

`variant` · `datafusion_proto_models::generated::datafusion::repartition_node::PartitionMethod::Range` · datafusion-proto-models 55.1.0

```rust
Range
```

Source: `src/generated/prost.rs:225`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c970924357604b61eb6b1ae4"></a>
## RoundRobin

`variant` · `datafusion_proto_models::generated::datafusion::repartition_node::PartitionMethod::RoundRobin` · datafusion-proto-models 55.1.0

```rust
RoundRobin
```

Source: `src/generated/prost.rs:221`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e30d75072d5fd67cc4a4796a"></a>
## clone

`function` · `datafusion_proto_models::generated::datafusion::repartition_node::PartitionMethod::clone` · datafusion-proto-models 55.1.0

```rust
fn clone(&self) -> PartitionMethod
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::repartition_node::PartitionMethod", "path": "PartitionMethod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [218, 14], "end": [218, 19], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generated/prost.rs:218`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c4ef9165fb7f64f1b2fb2384"></a>
## encode

`function` · `datafusion_proto_models::generated::datafusion::repartition_node::PartitionMethod::encode` · datafusion-proto-models 55.1.0

```rust
fn encode(&self, buf: &mut impl ::prost::bytes::BufMut)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::repartition_node::PartitionMethod", "path": "PartitionMethod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [218, 32], "end": [218, 46], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:218`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Encodes the message to a buffer.

<a id="op-957a7ed8e0038a18a50a3dd7"></a>
## encoded_len

`function` · `datafusion_proto_models::generated::datafusion::repartition_node::PartitionMethod::encoded_len` · datafusion-proto-models 55.1.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::repartition_node::PartitionMethod", "path": "PartitionMethod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [218, 32], "end": [218, 46], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:218`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Returns the encoded length of the message without a length delimiter.

<a id="op-5b8c255c882c3b3ff6e559b6"></a>
## eq

`function` · `datafusion_proto_models::generated::datafusion::repartition_node::PartitionMethod::eq` · datafusion-proto-models 55.1.0

```rust
fn eq(&self, other: &PartitionMethod) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::repartition_node::PartitionMethod", "path": "PartitionMethod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [218, 21], "end": [218, 30], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/generated/prost.rs:218`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c912495d04594e67334b63a1"></a>
## fmt

`function` · `datafusion_proto_models::generated::datafusion::repartition_node::PartitionMethod::fmt` · datafusion-proto-models 55.1.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::repartition_node::PartitionMethod", "path": "PartitionMethod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [218, 32], "end": [218, 46], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generated/prost.rs:218`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a89f250711cccadcf699f8c2"></a>
## merge

`function` · `datafusion_proto_models::generated::datafusion::repartition_node::PartitionMethod::merge` · datafusion-proto-models 55.1.0

```rust
fn merge(field: &mut ::core::option::Option<PartitionMethod>, tag: u32, wire_type: ::prost::encoding::wire_type::WireType, buf: &mut impl ::prost::bytes::Buf, ctx: ::prost::encoding::DecodeContext) -> ::core::result::Result<(), ::prost::DecodeError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::repartition_node::PartitionMethod", "path": "PartitionMethod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [218, 32], "end": [218, 46], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:218`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Decodes an instance of the message from a buffer, and merges it into self.
