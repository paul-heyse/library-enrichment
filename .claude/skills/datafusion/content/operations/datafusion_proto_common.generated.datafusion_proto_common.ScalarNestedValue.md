# `datafusion_proto_common::generated::datafusion_proto_common::ScalarNestedValue`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_common.generated.datafusion_proto_common.ScalarNestedValue.json).

<a id="op-a2a5dbb19374d5da4534277a"></a>
## ScalarNestedValue

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::ScalarNestedValue` · datafusion-proto-common 55.1.0

```rust
struct ScalarNestedValue
```

Source: `src/generated/prost.rs:196`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Used for List/FixedSizeList/LargeList/ListView/LargeListView/Struct/Map

<a id="op-302fcf2c54d45bfaff034ace"></a>
## arrow_data

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::ScalarNestedValue::arrow_data` · datafusion-proto-common 55.1.0

```rust
arrow_data: ::prost::alloc::vec::Vec<u8>
```

Source: `src/generated/prost.rs:200`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-37282702770f82e447894ec9"></a>
## clear

`function` · `datafusion_proto_common::generated::datafusion_proto_common::ScalarNestedValue::clear` · datafusion-proto-common 55.1.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::ScalarNestedValue", "path": "ScalarNestedValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [195, 28], "end": [195, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:195`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0d4fcbe01285f21ac10515db"></a>
## clone

`function` · `datafusion_proto_common::generated::datafusion_proto_common::ScalarNestedValue::clone` · datafusion-proto-common 55.1.0

```rust
fn clone(&self) -> ScalarNestedValue
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::ScalarNestedValue", "path": "ScalarNestedValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [195, 10], "end": [195, 15], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generated/prost.rs:195`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1215e62cff8e449ce5c020d6"></a>
## default

`function` · `datafusion_proto_common::generated::datafusion_proto_common::ScalarNestedValue::default` · datafusion-proto-common 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::ScalarNestedValue", "path": "ScalarNestedValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [195, 28], "end": [195, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/generated/prost.rs:195`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-22b01a993b781843b6ef43aa"></a>
## deserialize

`function` · `datafusion_proto_common::generated::datafusion_proto_common::ScalarNestedValue::deserialize` · datafusion-proto-common 55.1.0

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::ScalarNestedValue", "path": "ScalarNestedValue"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [7811, 1], "end": [7922, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/generated/pbjson.rs:7813`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c9a615923e12202b7ef73df"></a>
## dictionaries

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::ScalarNestedValue::dictionaries` · datafusion-proto-common 55.1.0

```rust
dictionaries: ::prost::alloc::vec::Vec<scalar_nested_value::Dictionary>
```

Source: `src/generated/prost.rs:204`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea083e16fea6323458a119de"></a>
## encoded_len

`function` · `datafusion_proto_common::generated::datafusion_proto_common::ScalarNestedValue::encoded_len` · datafusion-proto-common 55.1.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::ScalarNestedValue", "path": "ScalarNestedValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [195, 28], "end": [195, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:195`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3cda359e9174a7db07eb1cab"></a>
## eq

`function` · `datafusion_proto_common::generated::datafusion_proto_common::ScalarNestedValue::eq` · datafusion-proto-common 55.1.0

```rust
fn eq(&self, other: &ScalarNestedValue) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::ScalarNestedValue", "path": "ScalarNestedValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [195, 17], "end": [195, 26], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/generated/prost.rs:195`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e0553ceeba5d9259e4ce4c94"></a>
## fmt

`function` · `datafusion_proto_common::generated::datafusion_proto_common::ScalarNestedValue::fmt` · datafusion-proto-common 55.1.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::ScalarNestedValue", "path": "ScalarNestedValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [195, 28], "end": [195, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generated/prost.rs:195`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8acc61576a9c17473e9bac5c"></a>
## ipc_message

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::ScalarNestedValue::ipc_message` · datafusion-proto-common 55.1.0

```rust
ipc_message: ::prost::alloc::vec::Vec<u8>
```

Source: `src/generated/prost.rs:198`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2a261dbc330de4501da5d0a0"></a>
## schema

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::ScalarNestedValue::schema` · datafusion-proto-common 55.1.0

```rust
schema: ::core::option::Option<Schema>
```

Source: `src/generated/prost.rs:202`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7b35f53e1e88a95f12b4cb1f"></a>
## serialize

`function` · `datafusion_proto_common::generated::datafusion_proto_common::ScalarNestedValue::serialize` · datafusion-proto-common 55.1.0

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::ScalarNestedValue", "path": "ScalarNestedValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7771, 1], "end": [7810, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/generated/pbjson.rs:7773`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
