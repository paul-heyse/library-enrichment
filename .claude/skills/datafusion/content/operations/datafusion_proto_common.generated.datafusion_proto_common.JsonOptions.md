# `datafusion_proto_common::generated::datafusion_proto_common::JsonOptions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_common.generated.datafusion_proto_common.JsonOptions.json).

<a id="op-e70fb03380bc56f2d3d89bad"></a>
## JsonOptions

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::JsonOptions` · datafusion-proto-common 55.1.0

```rust
struct JsonOptions
```

Source: `src/generated/prost.rs:704`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Options controlling CSV format

<a id="op-ca93d883aa32c66405e43546"></a>
## Error

`assoc_type` · `datafusion_proto_common::generated::datafusion_proto_common::JsonOptions::Error` · datafusion-proto-common 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::JsonOptions", "path": "protobuf::JsonOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1056, 1], "end": [1068, 2], "filename": "src/to_proto/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_common::config::JsonOptions", "path": "JsonOptions"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/to_proto/mod.rs:1057`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b4a4b915ee48d6eade89b30b"></a>
## clear

`function` · `datafusion_proto_common::generated::datafusion_proto_common::JsonOptions::clear` · datafusion-proto-common 55.1.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::JsonOptions", "path": "JsonOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [703, 44], "end": [703, 60], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:703`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-340950094c8f130a00289f29"></a>
## clone

`function` · `datafusion_proto_common::generated::datafusion_proto_common::JsonOptions::clone` · datafusion-proto-common 55.1.0

```rust
fn clone(&self) -> JsonOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::JsonOptions", "path": "JsonOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [703, 10], "end": [703, 15], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generated/prost.rs:703`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3003df4f6a1c5bd687e865fc"></a>
## compression

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::JsonOptions::compression` · datafusion-proto-common 55.1.0

```rust
compression: i32
```

Source: `src/generated/prost.rs:707`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Compression type

<a id="op-6a84029b31cc655b629cc513"></a>
## compression

`function` · `datafusion_proto_common::generated::datafusion_proto_common::JsonOptions::compression` · datafusion-proto-common 55.1.0

```rust
fn compression(&self) -> CompressionTypeVariant
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::JsonOptions", "path": "JsonOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [703, 44], "end": [703, 60], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:703`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Returns the enum value of `compression`, or the default if the field is set to an invalid enum value.

<a id="op-3e6cdda4e211749d02b32adc"></a>
## compression_level

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::JsonOptions::compression_level` · datafusion-proto-common 55.1.0

```rust
compression_level: ::core::option::Option<u32>
```

Source: `src/generated/prost.rs:713`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Optional compression level

<a id="op-c9c1d1511bcdf072ca88a8b3"></a>
## compression_level

`function` · `datafusion_proto_common::generated::datafusion_proto_common::JsonOptions::compression_level` · datafusion-proto-common 55.1.0

```rust
fn compression_level(&self) -> u32
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::JsonOptions", "path": "JsonOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [703, 44], "end": [703, 60], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:703`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Returns the value of `compression_level`, or the default value if `compression_level` is unset.

<a id="op-a6b54984691538aec1a65fe3"></a>
## default

`function` · `datafusion_proto_common::generated::datafusion_proto_common::JsonOptions::default` · datafusion-proto-common 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::JsonOptions", "path": "JsonOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [703, 44], "end": [703, 60], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/generated/prost.rs:703`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-20681c170a15801d888b37e2"></a>
## deserialize

`function` · `datafusion_proto_common::generated::datafusion_proto_common::JsonOptions::deserialize` · datafusion-proto-common 55.1.0

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::JsonOptions", "path": "JsonOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [5043, 1], "end": [5155, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/generated/pbjson.rs:5045`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6e6b129efc7c914049c3edbe"></a>
## encoded_len

`function` · `datafusion_proto_common::generated::datafusion_proto_common::JsonOptions::encoded_len` · datafusion-proto-common 55.1.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::JsonOptions", "path": "JsonOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [703, 44], "end": [703, 60], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:703`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b75f2efb613c02c1ba8ee199"></a>
## eq

`function` · `datafusion_proto_common::generated::datafusion_proto_common::JsonOptions::eq` · datafusion-proto-common 55.1.0

```rust
fn eq(&self, other: &JsonOptions) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::JsonOptions", "path": "JsonOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [703, 23], "end": [703, 32], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/generated/prost.rs:703`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-02f9871e24b2022ae0ad3f52"></a>
## fmt

`function` · `datafusion_proto_common::generated::datafusion_proto_common::JsonOptions::fmt` · datafusion-proto-common 55.1.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::JsonOptions", "path": "JsonOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [703, 44], "end": [703, 60], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generated/prost.rs:703`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-74bfe2ab5afb71bb20b1125e"></a>
## hash

`function` · `datafusion_proto_common::generated::datafusion_proto_common::JsonOptions::hash` · datafusion-proto-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::JsonOptions", "path": "JsonOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [703, 38], "end": [703, 42], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/generated/prost.rs:703`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3854a21a6011cc4f090d7d75"></a>
## newline_delimited

`function` · `datafusion_proto_common::generated::datafusion_proto_common::JsonOptions::newline_delimited` · datafusion-proto-common 55.1.0

```rust
fn newline_delimited(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::JsonOptions", "path": "JsonOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [703, 44], "end": [703, 60], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:703`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Returns the value of `newline_delimited`, or the default value if `newline_delimited` is unset.

<a id="op-f85c4e9710a622f096644239"></a>
## newline_delimited

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::JsonOptions::newline_delimited` · datafusion-proto-common 55.1.0

```rust
newline_delimited: ::core::option::Option<bool>
```

Source: `src/generated/prost.rs:716`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Whether to read as newline-delimited JSON (default true). When false, expects JSON array format \[{},...\]

<a id="op-56952e672e09ca3af5767c82"></a>
## schema_infer_max_rec

`function` · `datafusion_proto_common::generated::datafusion_proto_common::JsonOptions::schema_infer_max_rec` · datafusion-proto-common 55.1.0

```rust
fn schema_infer_max_rec(&self) -> u64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::JsonOptions", "path": "JsonOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [703, 44], "end": [703, 60], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:703`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Returns the value of `schema_infer_max_rec`, or the default value if `schema_infer_max_rec` is unset.

<a id="op-6f377e45de167db012c40096"></a>
## schema_infer_max_rec

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::JsonOptions::schema_infer_max_rec` · datafusion-proto-common 55.1.0

```rust
schema_infer_max_rec: ::core::option::Option<u64>
```

Source: `src/generated/prost.rs:710`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Optional max records for schema inference

<a id="op-9933c8cb85ffe5de6c4ef8f7"></a>
## serialize

`function` · `datafusion_proto_common::generated::datafusion_proto_common::JsonOptions::serialize` · datafusion-proto-common 55.1.0

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::JsonOptions", "path": "JsonOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5003, 1], "end": [5042, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/generated/pbjson.rs:5005`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a90537a7ccebdbfd80cf0a90"></a>
## set_compression

`function` · `datafusion_proto_common::generated::datafusion_proto_common::JsonOptions::set_compression` · datafusion-proto-common 55.1.0

```rust
fn set_compression(&mut self, value: CompressionTypeVariant)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::JsonOptions", "path": "JsonOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [703, 44], "end": [703, 60], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:703`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Sets `compression` to the provided enum value.

<a id="op-9e2cbbdd71a0dfeca28733e0"></a>
## try_from

`function` · `datafusion_proto_common::generated::datafusion_proto_common::JsonOptions::try_from` · datafusion-proto-common 55.1.0

```rust
fn try_from(opts: &JsonOptions) -> datafusion_common::Result<Self, Self::Error>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::JsonOptions", "path": "protobuf::JsonOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1056, 1], "end": [1068, 2], "filename": "src/to_proto/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_common::config::JsonOptions", "path": "JsonOptions"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/to_proto/mod.rs:1059`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
