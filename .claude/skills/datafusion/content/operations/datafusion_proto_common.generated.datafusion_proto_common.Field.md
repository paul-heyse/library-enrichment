# `datafusion_proto_common::generated::datafusion_proto_common::Field`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_common.generated.datafusion_proto_common.Field.json).

<a id="op-3396088736f44ddd518182e2"></a>
## Field

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::Field` · datafusion-proto-common 55.1.0

```rust
struct Field
```

Source: `src/generated/prost.rs:95`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a13ca15c69cb83c5c90d3693"></a>
## Error

`assoc_type` · `datafusion_proto_common::generated::datafusion_proto_common::Field::Error` · datafusion-proto-common 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::Field", "path": "protobuf::Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [92, 1], "end": [105, 2], "filename": "src/to_proto/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/to_proto/mod.rs:93`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b69b7f12ca15b3e57c7118f7"></a>
## arrow_type

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::Field::arrow_type` · datafusion-proto-common 55.1.0

```rust
arrow_type: ::core::option::Option<::prost::alloc::boxed::Box<ArrowType>>
```

Source: `src/generated/prost.rs:100`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0e9fd4284ef5228727b8f930"></a>
## children

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::Field::children` · datafusion-proto-common 55.1.0

```rust
children: ::prost::alloc::vec::Vec<Field>
```

Source: `src/generated/prost.rs:105`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

for complex data types like structs, unions

<a id="op-d027ec4988d4343035895ecb"></a>
## clear

`function` · `datafusion_proto_common::generated::datafusion_proto_common::Field::clear` · datafusion-proto-common 55.1.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 28], "end": [94, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-03dfc060dab83ba3f10da20c"></a>
## clone

`function` · `datafusion_proto_common::generated::datafusion_proto_common::Field::clone` · datafusion-proto-common 55.1.0

```rust
fn clone(&self) -> Field
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 10], "end": [94, 15], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generated/prost.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c59f91a899c40abb5bc738e8"></a>
## default

`function` · `datafusion_proto_common::generated::datafusion_proto_common::Field::default` · datafusion-proto-common 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 28], "end": [94, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/generated/prost.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d0b36e64e5c4b594d642dc26"></a>
## deserialize

`function` · `datafusion_proto_common::generated::datafusion_proto_common::Field::deserialize` · datafusion-proto-common 55.1.0

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::Field", "path": "Field"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [4212, 1], "end": [4331, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/generated/pbjson.rs:4214`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-56363d0ef80c966264832405"></a>
## encoded_len

`function` · `datafusion_proto_common::generated::datafusion_proto_common::Field::encoded_len` · datafusion-proto-common 55.1.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 28], "end": [94, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-152c46e76b45fa5e08488f37"></a>
## eq

`function` · `datafusion_proto_common::generated::datafusion_proto_common::Field::eq` · datafusion-proto-common 55.1.0

```rust
fn eq(&self, other: &Field) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 17], "end": [94, 26], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/generated/prost.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-63bc776f137aa13adbbaafd0"></a>
## fmt

`function` · `datafusion_proto_common::generated::datafusion_proto_common::Field::fmt` · datafusion-proto-common 55.1.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 28], "end": [94, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generated/prost.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-792c137758ed5d744389a60c"></a>
## metadata

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::Field::metadata` · datafusion-proto-common 55.1.0

```rust
metadata: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>
```

Source: `src/generated/prost.rs:107`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cf3db26dec9cfe02e7b517dd"></a>
## name

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::Field::name` · datafusion-proto-common 55.1.0

```rust
name: ::prost::alloc::string::String
```

Source: `src/generated/prost.rs:98`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

name of the field

<a id="op-bcc15775dbfe22cbc88238ec"></a>
## nullable

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::Field::nullable` · datafusion-proto-common 55.1.0

```rust
nullable: bool
```

Source: `src/generated/prost.rs:102`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f65b8b75a4832e47143ea65c"></a>
## serialize

`function` · `datafusion_proto_common::generated::datafusion_proto_common::Field::serialize` · datafusion-proto-common 55.1.0

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4170, 1], "end": [4211, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/generated/pbjson.rs:4172`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-63aaea7baa18bee00754d894"></a>
## try_from

`function` · `datafusion_proto_common::generated::datafusion_proto_common::Field::try_from` · datafusion-proto-common 55.1.0

```rust
fn try_from(field: &Field) -> Result<Self, Self::Error>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::Field", "path": "protobuf::Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [92, 1], "end": [105, 2], "filename": "src/to_proto/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/to_proto/mod.rs:95`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
