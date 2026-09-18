# `datafusion_proto_common::generated::datafusion_proto_common::CompressionTypeVariant`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_common.generated.datafusion_proto_common.CompressionTypeVariant.json).

<a id="op-37387fda95856fc03f20b54f"></a>
## CompressionTypeVariant

`enum` · `datafusion_proto_common::generated::datafusion_proto_common::CompressionTypeVariant` · datafusion-proto-common 55.1.0

```rust
enum CompressionTypeVariant
```

Source: `src/generated/prost.rs:1234`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eaa862a0592243a6aac4def1"></a>
## Bzip2

`variant` · `datafusion_proto_common::generated::datafusion_proto_common::CompressionTypeVariant::Bzip2` · datafusion-proto-common 55.1.0

```rust
Bzip2
```

Source: `src/generated/prost.rs:1236`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-39bc1cf434de1d8b72c7efb4"></a>
## Error

`assoc_type` · `datafusion_proto_common::generated::datafusion_proto_common::CompressionTypeVariant::Error` · datafusion-proto-common 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::CompressionTypeVariant", "path": "CompressionTypeVariant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1232, 68], "end": [1232, 88], "filename": "src/generated/prost.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/generated/prost.rs:1232`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-35d3dc9e74fdd0b48955048c"></a>
## Gzip

`variant` · `datafusion_proto_common::generated::datafusion_proto_common::CompressionTypeVariant::Gzip` · datafusion-proto-common 55.1.0

```rust
Gzip
```

Source: `src/generated/prost.rs:1235`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a51ac3394c6f58498498bcb7"></a>
## Uncompressed

`variant` · `datafusion_proto_common::generated::datafusion_proto_common::CompressionTypeVariant::Uncompressed` · datafusion-proto-common 55.1.0

```rust
Uncompressed
```

Source: `src/generated/prost.rs:1239`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-40000a0c65812875eddb7952"></a>
## Xz

`variant` · `datafusion_proto_common::generated::datafusion_proto_common::CompressionTypeVariant::Xz` · datafusion-proto-common 55.1.0

```rust
Xz
```

Source: `src/generated/prost.rs:1237`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9e06eb0a8b3413c94bca8a7f"></a>
## Zstd

`variant` · `datafusion_proto_common::generated::datafusion_proto_common::CompressionTypeVariant::Zstd` · datafusion-proto-common 55.1.0

```rust
Zstd
```

Source: `src/generated/prost.rs:1238`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-991509068266d5b696432b9e"></a>
## as_str_name

`function` · `datafusion_proto_common::generated::datafusion_proto_common::CompressionTypeVariant::as_str_name` · datafusion-proto-common 55.1.0

```rust
fn as_str_name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::CompressionTypeVariant", "path": "CompressionTypeVariant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1241, 1], "end": [1266, 2], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:1246`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

String value of the enum field names used in the ProtoBuf definition.

The values are not transformed in any way and thus are considered stable
(if the ProtoBuf definition does not change) and safe for programmatic use.

<a id="op-2166c793a5021fda4b2a1a12"></a>
## clone

`function` · `datafusion_proto_common::generated::datafusion_proto_common::CompressionTypeVariant::clone` · datafusion-proto-common 55.1.0

```rust
fn clone(&self) -> CompressionTypeVariant
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::CompressionTypeVariant", "path": "CompressionTypeVariant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1232, 10], "end": [1232, 15], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generated/prost.rs:1232`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b8ffe193d622e3a812f79895"></a>
## cmp

`function` · `datafusion_proto_common::generated::datafusion_proto_common::CompressionTypeVariant::cmp` · datafusion-proto-common 55.1.0

```rust
fn cmp(&self, other: &CompressionTypeVariant) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::CompressionTypeVariant", "path": "CompressionTypeVariant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1232, 63], "end": [1232, 66], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/generated/prost.rs:1232`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-542f4c22151ac7e4e81b1042"></a>
## default

`function` · `datafusion_proto_common::generated::datafusion_proto_common::CompressionTypeVariant::default` · datafusion-proto-common 55.1.0

```rust
fn default() -> CompressionTypeVariant
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::CompressionTypeVariant", "path": "CompressionTypeVariant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1232, 68], "end": [1232, 88], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/generated/prost.rs:1232`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4fe6c4ed3c14c0edc63f30ba"></a>
## deserialize

`function` · `datafusion_proto_common::generated::datafusion_proto_common::CompressionTypeVariant::deserialize` · datafusion-proto-common 55.1.0

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::CompressionTypeVariant", "path": "CompressionTypeVariant"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1311, 1], "end": [1374, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/generated/pbjson.rs:1313`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f4fc8bffa28123be3a8699ec"></a>
## eq

`function` · `datafusion_proto_common::generated::datafusion_proto_common::CompressionTypeVariant::eq` · datafusion-proto-common 55.1.0

```rust
fn eq(&self, other: &CompressionTypeVariant) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::CompressionTypeVariant", "path": "CompressionTypeVariant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1232, 30], "end": [1232, 39], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/generated/prost.rs:1232`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-944c31f102df663639987e09"></a>
## fmt

`function` · `datafusion_proto_common::generated::datafusion_proto_common::CompressionTypeVariant::fmt` · datafusion-proto-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::CompressionTypeVariant", "path": "CompressionTypeVariant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1232, 23], "end": [1232, 28], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generated/prost.rs:1232`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-692700f938f89d0fa0e1402d"></a>
## from

`function` · `datafusion_proto_common::generated::datafusion_proto_common::CompressionTypeVariant::from` · datafusion-proto-common 55.1.0

```rust
fn from(value: &CompressionTypeVariant) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::CompressionTypeVariant", "path": "protobuf::CompressionTypeVariant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [836, 1], "end": [846, 2], "filename": "src/to_proto/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_common::parsers::CompressionTypeVariant", "path": "CompressionTypeVariant"}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/to_proto/mod.rs:837`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d7a2c2a6e844b84ebc77882e"></a>
## from

`function` · `datafusion_proto_common::generated::datafusion_proto_common::CompressionTypeVariant::from` · datafusion-proto-common 55.1.0

```rust
fn from(value: CompressionTypeVariant) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::CompressionTypeVariant", "path": "protobuf::CompressionTypeVariant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [955, 1], "end": [965, 2], "filename": "src/from_proto/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_common::parsers::CompressionTypeVariant", "path": "CompressionTypeVariant"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/from_proto/mod.rs:956`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3fed8d80120fc9e0b80fd37c"></a>
## from_i32

`function` · `datafusion_proto_common::generated::datafusion_proto_common::CompressionTypeVariant::from_i32` · datafusion-proto-common 55.1.0

```rust
fn from_i32(value: i32) -> ::core::option::Option<CompressionTypeVariant>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::CompressionTypeVariant", "path": "CompressionTypeVariant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1232, 68], "end": [1232, 88], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:1232`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Converts an `i32` to a `CompressionTypeVariant`, or `None` if `value` is not a valid variant.

<a id="op-669512447ca53995f7254cb8"></a>
## from_str_name

`function` · `datafusion_proto_common::generated::datafusion_proto_common::CompressionTypeVariant::from_str_name` · datafusion-proto-common 55.1.0

```rust
fn from_str_name(value: &str) -> ::core::option::Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::CompressionTypeVariant", "path": "CompressionTypeVariant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1241, 1], "end": [1266, 2], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:1256`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Creates an enum from field names used in the ProtoBuf definition.

<a id="op-31bc04b46752974907907859"></a>
## hash

`function` · `datafusion_proto_common::generated::datafusion_proto_common::CompressionTypeVariant::hash` · datafusion-proto-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::CompressionTypeVariant", "path": "CompressionTypeVariant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1232, 45], "end": [1232, 49], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/generated/prost.rs:1232`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-26b9dd60510f8262780cdc10"></a>
## is_valid

`function` · `datafusion_proto_common::generated::datafusion_proto_common::CompressionTypeVariant::is_valid` · datafusion-proto-common 55.1.0

```rust
const fn is_valid(value: i32) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::CompressionTypeVariant", "path": "CompressionTypeVariant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1232, 68], "end": [1232, 88], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:1232`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Returns `true` if `value` is a variant of `CompressionTypeVariant`.

<a id="op-28aa430cc5887458d73b8a2e"></a>
## partial_cmp

`function` · `datafusion_proto_common::generated::datafusion_proto_common::CompressionTypeVariant::partial_cmp` · datafusion-proto-common 55.1.0

```rust
fn partial_cmp(&self, other: &CompressionTypeVariant) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::CompressionTypeVariant", "path": "CompressionTypeVariant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1232, 51], "end": [1232, 61], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/generated/prost.rs:1232`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-19e3b01238aff61931c915a0"></a>
## serialize

`function` · `datafusion_proto_common::generated::datafusion_proto_common::CompressionTypeVariant::serialize` · datafusion-proto-common 55.1.0

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::CompressionTypeVariant", "path": "CompressionTypeVariant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1295, 1], "end": [1310, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/generated/pbjson.rs:1297`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fefa23adb3685840dd9e1edd"></a>
## try_from

`function` · `datafusion_proto_common::generated::datafusion_proto_common::CompressionTypeVariant::try_from` · datafusion-proto-common 55.1.0

```rust
fn try_from(value: i32) -> ::core::result::Result<CompressionTypeVariant, ::prost::UnknownEnumValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::CompressionTypeVariant", "path": "CompressionTypeVariant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1232, 68], "end": [1232, 88], "filename": "src/generated/prost.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/generated/prost.rs:1232`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
