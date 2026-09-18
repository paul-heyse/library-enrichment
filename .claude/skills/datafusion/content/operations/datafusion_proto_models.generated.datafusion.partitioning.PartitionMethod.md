# `datafusion_proto_models::generated::datafusion::partitioning::PartitionMethod`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_models.generated.datafusion.partitioning.PartitionMethod.json).

<a id="op-69ec232ed40ef2b5bfce7aaa"></a>
## PartitionMethod

`enum` · `datafusion_proto_models::generated::datafusion::partitioning::PartitionMethod` · datafusion-proto-models 55.1.0

```rust
enum PartitionMethod
```

Source: `src/generated/prost.rs:2367`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-da4d5a5c68856b1669daed84"></a>
## Hash

`variant` · `datafusion_proto_models::generated::datafusion::partitioning::PartitionMethod::Hash` · datafusion-proto-models 55.1.0

```rust
Hash
```

Source: `src/generated/prost.rs:2371`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b76d87b960c4621f97114d59"></a>
## Range

`variant` · `datafusion_proto_models::generated::datafusion::partitioning::PartitionMethod::Range` · datafusion-proto-models 55.1.0

```rust
Range
```

Source: `src/generated/prost.rs:2375`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-45d0fea262521611996d3b0f"></a>
## RoundRobin

`variant` · `datafusion_proto_models::generated::datafusion::partitioning::PartitionMethod::RoundRobin` · datafusion-proto-models 55.1.0

```rust
RoundRobin
```

Source: `src/generated/prost.rs:2369`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b6e6dd3fa7da56e41c347a69"></a>
## Unknown

`variant` · `datafusion_proto_models::generated::datafusion::partitioning::PartitionMethod::Unknown` · datafusion-proto-models 55.1.0

```rust
Unknown
```

Source: `src/generated/prost.rs:2373`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d1773fd6acc568dcc05add23"></a>
## clone

`function` · `datafusion_proto_models::generated::datafusion::partitioning::PartitionMethod::clone` · datafusion-proto-models 55.1.0

```rust
fn clone(&self) -> PartitionMethod
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::partitioning::PartitionMethod", "path": "PartitionMethod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2366, 14], "end": [2366, 19], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generated/prost.rs:2366`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-436a4a92defaafb6d620f586"></a>
## encode

`function` · `datafusion_proto_models::generated::datafusion::partitioning::PartitionMethod::encode` · datafusion-proto-models 55.1.0

```rust
fn encode(&self, buf: &mut impl ::prost::bytes::BufMut)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::partitioning::PartitionMethod", "path": "PartitionMethod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2366, 32], "end": [2366, 46], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:2366`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Encodes the message to a buffer.

<a id="op-9dc40dd53f58e7d894eef715"></a>
## encoded_len

`function` · `datafusion_proto_models::generated::datafusion::partitioning::PartitionMethod::encoded_len` · datafusion-proto-models 55.1.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::partitioning::PartitionMethod", "path": "PartitionMethod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2366, 32], "end": [2366, 46], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:2366`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Returns the encoded length of the message without a length delimiter.

<a id="op-043a87f9322acbbc7b79e685"></a>
## eq

`function` · `datafusion_proto_models::generated::datafusion::partitioning::PartitionMethod::eq` · datafusion-proto-models 55.1.0

```rust
fn eq(&self, other: &PartitionMethod) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::partitioning::PartitionMethod", "path": "PartitionMethod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2366, 21], "end": [2366, 30], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/generated/prost.rs:2366`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0d616d43e44a4513d8fcefac"></a>
## fmt

`function` · `datafusion_proto_models::generated::datafusion::partitioning::PartitionMethod::fmt` · datafusion-proto-models 55.1.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::partitioning::PartitionMethod", "path": "PartitionMethod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2366, 32], "end": [2366, 46], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generated/prost.rs:2366`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0d8cf1c96a78b82d930e5db0"></a>
## merge

`function` · `datafusion_proto_models::generated::datafusion::partitioning::PartitionMethod::merge` · datafusion-proto-models 55.1.0

```rust
fn merge(field: &mut ::core::option::Option<PartitionMethod>, tag: u32, wire_type: ::prost::encoding::wire_type::WireType, buf: &mut impl ::prost::bytes::Buf, ctx: ::prost::encoding::DecodeContext) -> ::core::result::Result<(), ::prost::DecodeError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::partitioning::PartitionMethod", "path": "PartitionMethod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2366, 32], "end": [2366, 46], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:2366`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Decodes an instance of the message from a buffer, and merges it into self.
