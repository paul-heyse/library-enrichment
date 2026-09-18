# `datafusion_proto_models::generated::datafusion::Field`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_models.generated.datafusion.Field.json).

<a id="op-1bd3f0c2910ed034a583956b"></a>
## Field

`struct` · `datafusion_proto_models::generated::datafusion::Field` · datafusion-proto-models 55.1.0

```rust
struct Field
```

Source: `src/generated/datafusion_proto_common.rs:95`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-301ffaf46ab51aabe4eab27c"></a>
## arrow_type

`struct_field` · `datafusion_proto_models::generated::datafusion::Field::arrow_type` · datafusion-proto-models 55.1.0

```rust
arrow_type: ::core::option::Option<::prost::alloc::boxed::Box<ArrowType>>
```

Source: `src/generated/datafusion_proto_common.rs:100`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ece1b733b91de20cd8b96239"></a>
## children

`struct_field` · `datafusion_proto_models::generated::datafusion::Field::children` · datafusion-proto-models 55.1.0

```rust
children: ::prost::alloc::vec::Vec<Field>
```

Source: `src/generated/datafusion_proto_common.rs:105`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

for complex data types like structs, unions

<a id="op-950c84f6f40b97d49173c9a7"></a>
## clear

`function` · `datafusion_proto_models::generated::datafusion::Field::clear` · datafusion-proto-models 55.1.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 28], "end": [94, 44], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/datafusion_proto_common.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d3e03decd4a74315b3f364df"></a>
## clone

`function` · `datafusion_proto_models::generated::datafusion::Field::clone` · datafusion-proto-models 55.1.0

```rust
fn clone(&self) -> Field
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 10], "end": [94, 15], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generated/datafusion_proto_common.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ac1b0e1f04d8d58102f0ea0f"></a>
## default

`function` · `datafusion_proto_models::generated::datafusion::Field::default` · datafusion-proto-models 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 28], "end": [94, 44], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/generated/datafusion_proto_common.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-70140accfe37b8728145b1ed"></a>
## encoded_len

`function` · `datafusion_proto_models::generated::datafusion::Field::encoded_len` · datafusion-proto-models 55.1.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 28], "end": [94, 44], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/datafusion_proto_common.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-be4878a0e601f4e84a97ea61"></a>
## eq

`function` · `datafusion_proto_models::generated::datafusion::Field::eq` · datafusion-proto-models 55.1.0

```rust
fn eq(&self, other: &Field) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 17], "end": [94, 26], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/generated/datafusion_proto_common.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2292fbdb6adbb910b52e2aad"></a>
## fmt

`function` · `datafusion_proto_models::generated::datafusion::Field::fmt` · datafusion-proto-models 55.1.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 28], "end": [94, 44], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generated/datafusion_proto_common.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ada2e3547a5c5d354b91cfef"></a>
## metadata

`struct_field` · `datafusion_proto_models::generated::datafusion::Field::metadata` · datafusion-proto-models 55.1.0

```rust
metadata: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>
```

Source: `src/generated/datafusion_proto_common.rs:107`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-be0e0784d8b04ecbabbcdaaf"></a>
## name

`struct_field` · `datafusion_proto_models::generated::datafusion::Field::name` · datafusion-proto-models 55.1.0

```rust
name: ::prost::alloc::string::String
```

Source: `src/generated/datafusion_proto_common.rs:98`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

name of the field

<a id="op-f4c56eb8f811691b1b3cf644"></a>
## nullable

`struct_field` · `datafusion_proto_models::generated::datafusion::Field::nullable` · datafusion-proto-models 55.1.0

```rust
nullable: bool
```

Source: `src/generated/datafusion_proto_common.rs:102`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
