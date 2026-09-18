# `datafusion_proto_models::generated::datafusion::CompressionTypeVariant`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_models.generated.datafusion.CompressionTypeVariant.json).

<a id="op-20f56843feb3466aa1e36b60"></a>
## CompressionTypeVariant

`enum` · `datafusion_proto_models::generated::datafusion::CompressionTypeVariant` · datafusion-proto-models 55.1.0

```rust
enum CompressionTypeVariant
```

Source: `src/generated/datafusion_proto_common.rs:1234`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8527e3cfa8a6179903680c8c"></a>
## Bzip2

`variant` · `datafusion_proto_models::generated::datafusion::CompressionTypeVariant::Bzip2` · datafusion-proto-models 55.1.0

```rust
Bzip2
```

Source: `src/generated/datafusion_proto_common.rs:1236`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1f0377a9d2b5e10699581504"></a>
## Error

`assoc_type` · `datafusion_proto_models::generated::datafusion::CompressionTypeVariant::Error` · datafusion-proto-models 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::CompressionTypeVariant", "path": "CompressionTypeVariant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1232, 68], "end": [1232, 88], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/generated/datafusion_proto_common.rs:1232`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-34d1a77f5177dd081e512209"></a>
## Gzip

`variant` · `datafusion_proto_models::generated::datafusion::CompressionTypeVariant::Gzip` · datafusion-proto-models 55.1.0

```rust
Gzip
```

Source: `src/generated/datafusion_proto_common.rs:1235`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a265be551748c494204e649f"></a>
## Uncompressed

`variant` · `datafusion_proto_models::generated::datafusion::CompressionTypeVariant::Uncompressed` · datafusion-proto-models 55.1.0

```rust
Uncompressed
```

Source: `src/generated/datafusion_proto_common.rs:1239`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b97b092d454f1f0e4bd8d8d5"></a>
## Xz

`variant` · `datafusion_proto_models::generated::datafusion::CompressionTypeVariant::Xz` · datafusion-proto-models 55.1.0

```rust
Xz
```

Source: `src/generated/datafusion_proto_common.rs:1237`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-95b466eaf4995498a6c2e561"></a>
## Zstd

`variant` · `datafusion_proto_models::generated::datafusion::CompressionTypeVariant::Zstd` · datafusion-proto-models 55.1.0

```rust
Zstd
```

Source: `src/generated/datafusion_proto_common.rs:1238`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-012efa44c426a9046db55c25"></a>
## as_str_name

`function` · `datafusion_proto_models::generated::datafusion::CompressionTypeVariant::as_str_name` · datafusion-proto-models 55.1.0

```rust
fn as_str_name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::CompressionTypeVariant", "path": "CompressionTypeVariant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1241, 1], "end": [1266, 2], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/datafusion_proto_common.rs:1246`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

String value of the enum field names used in the ProtoBuf definition.

The values are not transformed in any way and thus are considered stable
(if the ProtoBuf definition does not change) and safe for programmatic use.

<a id="op-3b8510aa6c9c94400f785b86"></a>
## clone

`function` · `datafusion_proto_models::generated::datafusion::CompressionTypeVariant::clone` · datafusion-proto-models 55.1.0

```rust
fn clone(&self) -> CompressionTypeVariant
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::CompressionTypeVariant", "path": "CompressionTypeVariant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1232, 10], "end": [1232, 15], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generated/datafusion_proto_common.rs:1232`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0698188206bb75da47d2717d"></a>
## cmp

`function` · `datafusion_proto_models::generated::datafusion::CompressionTypeVariant::cmp` · datafusion-proto-models 55.1.0

```rust
fn cmp(&self, other: &CompressionTypeVariant) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::CompressionTypeVariant", "path": "CompressionTypeVariant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1232, 63], "end": [1232, 66], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/generated/datafusion_proto_common.rs:1232`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad896cac3e9a2ea8a1066b05"></a>
## default

`function` · `datafusion_proto_models::generated::datafusion::CompressionTypeVariant::default` · datafusion-proto-models 55.1.0

```rust
fn default() -> CompressionTypeVariant
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::CompressionTypeVariant", "path": "CompressionTypeVariant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1232, 68], "end": [1232, 88], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/generated/datafusion_proto_common.rs:1232`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5b627cd43ce2c0e6e45467b8"></a>
## eq

`function` · `datafusion_proto_models::generated::datafusion::CompressionTypeVariant::eq` · datafusion-proto-models 55.1.0

```rust
fn eq(&self, other: &CompressionTypeVariant) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::CompressionTypeVariant", "path": "CompressionTypeVariant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1232, 30], "end": [1232, 39], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/generated/datafusion_proto_common.rs:1232`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b0c3cb7725bdb2e57347cf1"></a>
## fmt

`function` · `datafusion_proto_models::generated::datafusion::CompressionTypeVariant::fmt` · datafusion-proto-models 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::CompressionTypeVariant", "path": "CompressionTypeVariant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1232, 23], "end": [1232, 28], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generated/datafusion_proto_common.rs:1232`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-163e373be3318fa6397d64cd"></a>
## from_i32

`function` · `datafusion_proto_models::generated::datafusion::CompressionTypeVariant::from_i32` · datafusion-proto-models 55.1.0

```rust
fn from_i32(value: i32) -> ::core::option::Option<CompressionTypeVariant>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::CompressionTypeVariant", "path": "CompressionTypeVariant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1232, 68], "end": [1232, 88], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/datafusion_proto_common.rs:1232`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Converts an `i32` to a `CompressionTypeVariant`, or `None` if `value` is not a valid variant.

<a id="op-a363489296732ab6866fc0de"></a>
## from_str_name

`function` · `datafusion_proto_models::generated::datafusion::CompressionTypeVariant::from_str_name` · datafusion-proto-models 55.1.0

```rust
fn from_str_name(value: &str) -> ::core::option::Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::CompressionTypeVariant", "path": "CompressionTypeVariant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1241, 1], "end": [1266, 2], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/datafusion_proto_common.rs:1256`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Creates an enum from field names used in the ProtoBuf definition.

<a id="op-933b15e69e66941f913c7dd1"></a>
## hash

`function` · `datafusion_proto_models::generated::datafusion::CompressionTypeVariant::hash` · datafusion-proto-models 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::CompressionTypeVariant", "path": "CompressionTypeVariant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1232, 45], "end": [1232, 49], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/generated/datafusion_proto_common.rs:1232`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5c37de851aeeff73e6a46caa"></a>
## is_valid

`function` · `datafusion_proto_models::generated::datafusion::CompressionTypeVariant::is_valid` · datafusion-proto-models 55.1.0

```rust
const fn is_valid(value: i32) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::CompressionTypeVariant", "path": "CompressionTypeVariant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1232, 68], "end": [1232, 88], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/datafusion_proto_common.rs:1232`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Returns `true` if `value` is a variant of `CompressionTypeVariant`.

<a id="op-c4a9ede52e1803da2003087f"></a>
## partial_cmp

`function` · `datafusion_proto_models::generated::datafusion::CompressionTypeVariant::partial_cmp` · datafusion-proto-models 55.1.0

```rust
fn partial_cmp(&self, other: &CompressionTypeVariant) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::CompressionTypeVariant", "path": "CompressionTypeVariant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1232, 51], "end": [1232, 61], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/generated/datafusion_proto_common.rs:1232`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e5238feb33bae7aad1979043"></a>
## try_from

`function` · `datafusion_proto_models::generated::datafusion::CompressionTypeVariant::try_from` · datafusion-proto-models 55.1.0

```rust
fn try_from(value: i32) -> ::core::result::Result<CompressionTypeVariant, ::prost::UnknownEnumValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::CompressionTypeVariant", "path": "CompressionTypeVariant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1232, 68], "end": [1232, 88], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/generated/datafusion_proto_common.rs:1232`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
