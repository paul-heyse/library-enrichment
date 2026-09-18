# `datafusion_proto_common::generated::datafusion_proto_common::Schema`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_common.generated.datafusion_proto_common.Schema.json).

<a id="op-72c66437312f43f678b27df1"></a>
## Schema

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::Schema` · datafusion-proto-common 55.1.0

```rust
struct Schema
```

Source: `src/generated/prost.rs:85`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-385712c9ad6ec0f1af3198c8"></a>
## Error

`assoc_type` · `datafusion_proto_common::generated::datafusion_proto_common::Schema::Error` · datafusion-proto-common 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::Schema", "path": "protobuf::Schema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [262, 1], "end": [271, 2], "filename": "src/to_proto/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "arrow_schema::schema::Schema", "path": "Schema"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/to_proto/mod.rs:263`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aa3a9f85bede388096db9965"></a>
## Error

`assoc_type` · `datafusion_proto_common::generated::datafusion_proto_common::Schema::Error` · datafusion-proto-common 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::Schema", "path": "protobuf::Schema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [273, 1], "end": [282, 2], "filename": "src/to_proto/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::schema::Schema", "path": "Schema"}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/to_proto/mod.rs:274`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1867dd1430b5858ef63ba8ed"></a>
## clear

`function` · `datafusion_proto_common::generated::datafusion_proto_common::Schema::clear` · datafusion-proto-common 55.1.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::Schema", "path": "Schema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 28], "end": [84, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1281149ce0b4fc193c6deb60"></a>
## clone

`function` · `datafusion_proto_common::generated::datafusion_proto_common::Schema::clone` · datafusion-proto-common 55.1.0

```rust
fn clone(&self) -> Schema
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::Schema", "path": "Schema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 10], "end": [84, 15], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generated/prost.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-996de40300698f0cb6d07e92"></a>
## columns

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::Schema::columns` · datafusion-proto-common 55.1.0

```rust
columns: ::prost::alloc::vec::Vec<Field>
```

Source: `src/generated/prost.rs:87`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8e68ba077deb1a65f0649bf7"></a>
## default

`function` · `datafusion_proto_common::generated::datafusion_proto_common::Schema::default` · datafusion-proto-common 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::Schema", "path": "Schema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 28], "end": [84, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/generated/prost.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4dfbe8daf529971986fab133"></a>
## deserialize

`function` · `datafusion_proto_common::generated::datafusion_proto_common::Schema::deserialize` · datafusion-proto-common 55.1.0

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::Schema", "path": "Schema"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [9283, 1], "end": [9368, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/generated/pbjson.rs:9285`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-84163acf136251c2cc9380c9"></a>
## encoded_len

`function` · `datafusion_proto_common::generated::datafusion_proto_common::Schema::encoded_len` · datafusion-proto-common 55.1.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::Schema", "path": "Schema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 28], "end": [84, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-37f6c81de9e6678774d52924"></a>
## eq

`function` · `datafusion_proto_common::generated::datafusion_proto_common::Schema::eq` · datafusion-proto-common 55.1.0

```rust
fn eq(&self, other: &Schema) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::Schema", "path": "Schema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 17], "end": [84, 26], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/generated/prost.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b5b1f7b6e948fd1aac6d8de3"></a>
## fmt

`function` · `datafusion_proto_common::generated::datafusion_proto_common::Schema::fmt` · datafusion-proto-common 55.1.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::Schema", "path": "Schema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 28], "end": [84, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generated/prost.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8cce826e04282d93544c64df"></a>
## metadata

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::Schema::metadata` · datafusion-proto-common 55.1.0

```rust
metadata: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>
```

Source: `src/generated/prost.rs:89`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-03b6077e59adf39ff67db444"></a>
## serialize

`function` · `datafusion_proto_common::generated::datafusion_proto_common::Schema::serialize` · datafusion-proto-common 55.1.0

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::Schema", "path": "Schema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9259, 1], "end": [9282, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/generated/pbjson.rs:9261`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-77bcf4cfaa9f50a3c94cd929"></a>
## try_from

`function` · `datafusion_proto_common::generated::datafusion_proto_common::Schema::try_from` · datafusion-proto-common 55.1.0

```rust
fn try_from(schema: SchemaRef) -> Result<Self, Self::Error>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::Schema", "path": "protobuf::Schema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [273, 1], "end": [282, 2], "filename": "src/to_proto/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::schema::Schema", "path": "Schema"}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/to_proto/mod.rs:276`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-97b7f589a64e087c15d0db5c"></a>
## try_from

`function` · `datafusion_proto_common::generated::datafusion_proto_common::Schema::try_from` · datafusion-proto-common 55.1.0

```rust
fn try_from(schema: &Schema) -> Result<Self, Self::Error>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::Schema", "path": "protobuf::Schema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [262, 1], "end": [271, 2], "filename": "src/to_proto/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "arrow_schema::schema::Schema", "path": "Schema"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/to_proto/mod.rs:265`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
