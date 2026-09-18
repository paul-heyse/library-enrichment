# `datafusion_proto_models::generated::datafusion::parquet_options::CoerceInt96TzOpt`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_models.generated.datafusion.parquet_options.CoerceInt96TzOpt.json).

<a id="op-b23677db02310d547a492399"></a>
## CoerceInt96TzOpt

`enum` · `datafusion_proto_models::generated::datafusion::parquet_options::CoerceInt96TzOpt` · datafusion-proto-models 55.1.0

```rust
enum CoerceInt96TzOpt
```

Source: `src/generated/datafusion_proto_common.rs:983`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Optional timezone applied to INT96-coerced timestamps when `coerce_int96`
is set. When `Some`, INT96 columns coerce to
`Timestamp(<coerce_int96>, Some(<tz>))` instead of the default
`Timestamp(<coerce_int96>, None)`. No effect when `coerce_int96` is unset.

<a id="op-5b9749d9e4fae9672fd326d3"></a>
## CoerceInt96Tz

`variant` · `datafusion_proto_models::generated::datafusion::parquet_options::CoerceInt96TzOpt::CoerceInt96Tz` · datafusion-proto-models 55.1.0

```rust
CoerceInt96Tz
```

Source: `src/generated/datafusion_proto_common.rs:985`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f3f6d7cadaa78ee4b292112b"></a>
## clone

`function` · `datafusion_proto_models::generated::datafusion::parquet_options::CoerceInt96TzOpt::clone` · datafusion-proto-models 55.1.0

```rust
fn clone(&self) -> CoerceInt96TzOpt
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::parquet_options::CoerceInt96TzOpt", "path": "CoerceInt96TzOpt"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [982, 14], "end": [982, 19], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generated/datafusion_proto_common.rs:982`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1305f57aec83cef1bd07a4b2"></a>
## encode

`function` · `datafusion_proto_models::generated::datafusion::parquet_options::CoerceInt96TzOpt::encode` · datafusion-proto-models 55.1.0

```rust
fn encode(&self, buf: &mut impl ::prost::bytes::BufMut)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::parquet_options::CoerceInt96TzOpt", "path": "CoerceInt96TzOpt"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [982, 42], "end": [982, 56], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/datafusion_proto_common.rs:982`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Encodes the message to a buffer.

<a id="op-f8f52f8367820ba23c6e40c7"></a>
## encoded_len

`function` · `datafusion_proto_models::generated::datafusion::parquet_options::CoerceInt96TzOpt::encoded_len` · datafusion-proto-models 55.1.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::parquet_options::CoerceInt96TzOpt", "path": "CoerceInt96TzOpt"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [982, 42], "end": [982, 56], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/datafusion_proto_common.rs:982`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Returns the encoded length of the message without a length delimiter.

<a id="op-4518ac35ecfa6bf18ca8d360"></a>
## eq

`function` · `datafusion_proto_models::generated::datafusion::parquet_options::CoerceInt96TzOpt::eq` · datafusion-proto-models 55.1.0

```rust
fn eq(&self, other: &CoerceInt96TzOpt) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::parquet_options::CoerceInt96TzOpt", "path": "CoerceInt96TzOpt"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [982, 21], "end": [982, 30], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/generated/datafusion_proto_common.rs:982`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-672a34083768d54554faa718"></a>
## fmt

`function` · `datafusion_proto_models::generated::datafusion::parquet_options::CoerceInt96TzOpt::fmt` · datafusion-proto-models 55.1.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::parquet_options::CoerceInt96TzOpt", "path": "CoerceInt96TzOpt"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [982, 42], "end": [982, 56], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generated/datafusion_proto_common.rs:982`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d436d69870121b7027a440e5"></a>
## hash

`function` · `datafusion_proto_models::generated::datafusion::parquet_options::CoerceInt96TzOpt::hash` · datafusion-proto-models 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::parquet_options::CoerceInt96TzOpt", "path": "CoerceInt96TzOpt"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [982, 36], "end": [982, 40], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/generated/datafusion_proto_common.rs:982`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8d0d9f8a7c8de58af7edc348"></a>
## merge

`function` · `datafusion_proto_models::generated::datafusion::parquet_options::CoerceInt96TzOpt::merge` · datafusion-proto-models 55.1.0

```rust
fn merge(field: &mut ::core::option::Option<CoerceInt96TzOpt>, tag: u32, wire_type: ::prost::encoding::wire_type::WireType, buf: &mut impl ::prost::bytes::Buf, ctx: ::prost::encoding::DecodeContext) -> ::core::result::Result<(), ::prost::DecodeError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::parquet_options::CoerceInt96TzOpt", "path": "CoerceInt96TzOpt"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [982, 42], "end": [982, 56], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/datafusion_proto_common.rs:982`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Decodes an instance of the message from a buffer, and merges it into self.
