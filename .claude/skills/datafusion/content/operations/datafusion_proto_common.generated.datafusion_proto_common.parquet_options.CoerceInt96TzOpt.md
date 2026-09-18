# `datafusion_proto_common::generated::datafusion_proto_common::parquet_options::CoerceInt96TzOpt`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_common.generated.datafusion_proto_common.parquet_options.CoerceInt96TzOpt.json).

<a id="op-25b3aaf825e2d54a61a33a9a"></a>
## CoerceInt96TzOpt

`enum` · `datafusion_proto_common::generated::datafusion_proto_common::parquet_options::CoerceInt96TzOpt` · datafusion-proto-common 55.1.0

```rust
enum CoerceInt96TzOpt
```

Source: `src/generated/prost.rs:983`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Optional timezone applied to INT96-coerced timestamps when `coerce_int96`
is set. When `Some`, INT96 columns coerce to
`Timestamp(<coerce_int96>, Some(<tz>))` instead of the default
`Timestamp(<coerce_int96>, None)`. No effect when `coerce_int96` is unset.

<a id="op-5f60353620e660afe875e6e8"></a>
## CoerceInt96Tz

`variant` · `datafusion_proto_common::generated::datafusion_proto_common::parquet_options::CoerceInt96TzOpt::CoerceInt96Tz` · datafusion-proto-common 55.1.0

```rust
CoerceInt96Tz
```

Source: `src/generated/prost.rs:985`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2450a02ac6c95a8460580a2b"></a>
## clone

`function` · `datafusion_proto_common::generated::datafusion_proto_common::parquet_options::CoerceInt96TzOpt::clone` · datafusion-proto-common 55.1.0

```rust
fn clone(&self) -> CoerceInt96TzOpt
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::parquet_options::CoerceInt96TzOpt", "path": "CoerceInt96TzOpt"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [982, 14], "end": [982, 19], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generated/prost.rs:982`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-803b8c1ef3df8a58b5f21715"></a>
## encode

`function` · `datafusion_proto_common::generated::datafusion_proto_common::parquet_options::CoerceInt96TzOpt::encode` · datafusion-proto-common 55.1.0

```rust
fn encode(&self, buf: &mut impl ::prost::bytes::BufMut)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::parquet_options::CoerceInt96TzOpt", "path": "CoerceInt96TzOpt"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [982, 42], "end": [982, 56], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:982`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Encodes the message to a buffer.

<a id="op-5b3a29f917280d9d782e8c4c"></a>
## encoded_len

`function` · `datafusion_proto_common::generated::datafusion_proto_common::parquet_options::CoerceInt96TzOpt::encoded_len` · datafusion-proto-common 55.1.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::parquet_options::CoerceInt96TzOpt", "path": "CoerceInt96TzOpt"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [982, 42], "end": [982, 56], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:982`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Returns the encoded length of the message without a length delimiter.

<a id="op-55c38e8cdc250689cfca4a50"></a>
## eq

`function` · `datafusion_proto_common::generated::datafusion_proto_common::parquet_options::CoerceInt96TzOpt::eq` · datafusion-proto-common 55.1.0

```rust
fn eq(&self, other: &CoerceInt96TzOpt) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::parquet_options::CoerceInt96TzOpt", "path": "CoerceInt96TzOpt"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [982, 21], "end": [982, 30], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/generated/prost.rs:982`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-99c49a852cb2214280c67fa7"></a>
## fmt

`function` · `datafusion_proto_common::generated::datafusion_proto_common::parquet_options::CoerceInt96TzOpt::fmt` · datafusion-proto-common 55.1.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::parquet_options::CoerceInt96TzOpt", "path": "CoerceInt96TzOpt"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [982, 42], "end": [982, 56], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generated/prost.rs:982`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f05302fa7c2d249662167c3b"></a>
## hash

`function` · `datafusion_proto_common::generated::datafusion_proto_common::parquet_options::CoerceInt96TzOpt::hash` · datafusion-proto-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::parquet_options::CoerceInt96TzOpt", "path": "CoerceInt96TzOpt"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [982, 36], "end": [982, 40], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/generated/prost.rs:982`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cc7178d0ec311eb9a4e46d9a"></a>
## merge

`function` · `datafusion_proto_common::generated::datafusion_proto_common::parquet_options::CoerceInt96TzOpt::merge` · datafusion-proto-common 55.1.0

```rust
fn merge(field: &mut ::core::option::Option<CoerceInt96TzOpt>, tag: u32, wire_type: ::prost::encoding::wire_type::WireType, buf: &mut impl ::prost::bytes::Buf, ctx: ::prost::encoding::DecodeContext) -> ::core::result::Result<(), ::prost::DecodeError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::parquet_options::CoerceInt96TzOpt", "path": "CoerceInt96TzOpt"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [982, 42], "end": [982, 56], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:982`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Decodes an instance of the message from a buffer, and merges it into self.
