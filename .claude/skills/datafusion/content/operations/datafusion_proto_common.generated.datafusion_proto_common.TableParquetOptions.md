# `datafusion_proto_common::generated::datafusion_proto_common::TableParquetOptions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_common.generated.datafusion_proto_common.TableParquetOptions.json).

<a id="op-87018c86c6010fb6bbec2928"></a>
## TableParquetOptions

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::TableParquetOptions` · datafusion-proto-common 55.1.0

```rust
struct TableParquetOptions
```

Source: `src/generated/prost.rs:719`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9a0aab4371191df32c92bd07"></a>
## Error

`assoc_type` · `datafusion_proto_common::generated::datafusion_proto_common::TableParquetOptions::Error` · datafusion-proto-common 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::TableParquetOptions", "path": "protobuf::TableParquetOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [986, 1], "end": [1015, 2], "filename": "src/to_proto/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_common::config::TableParquetOptions", "path": "TableParquetOptions"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/to_proto/mod.rs:987`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-65a811cdc9fb809deba00618"></a>
## clear

`function` · `datafusion_proto_common::generated::datafusion_proto_common::TableParquetOptions::clear` · datafusion-proto-common 55.1.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::TableParquetOptions", "path": "TableParquetOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [718, 28], "end": [718, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:718`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-da6dfa9bb9fc164a305cbdf2"></a>
## clone

`function` · `datafusion_proto_common::generated::datafusion_proto_common::TableParquetOptions::clone` · datafusion-proto-common 55.1.0

```rust
fn clone(&self) -> TableParquetOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::TableParquetOptions", "path": "TableParquetOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [718, 10], "end": [718, 15], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generated/prost.rs:718`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-524636289bba04c11bd949fc"></a>
## column_specific_options

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::TableParquetOptions::column_specific_options` · datafusion-proto-common 55.1.0

```rust
column_specific_options: ::prost::alloc::vec::Vec<ParquetColumnSpecificOptions>
```

Source: `src/generated/prost.rs:723`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3dc4034f3296b481c5c9c82a"></a>
## default

`function` · `datafusion_proto_common::generated::datafusion_proto_common::TableParquetOptions::default` · datafusion-proto-common 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::TableParquetOptions", "path": "TableParquetOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [718, 28], "end": [718, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/generated/prost.rs:718`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-502f2431eec73bb155393d76"></a>
## deserialize

`function` · `datafusion_proto_common::generated::datafusion_proto_common::TableParquetOptions::deserialize` · datafusion-proto-common 55.1.0

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::TableParquetOptions", "path": "TableParquetOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [9619, 1], "end": [9717, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/generated/pbjson.rs:9621`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2828a3b53f86c6ba4fe3f826"></a>
## encoded_len

`function` · `datafusion_proto_common::generated::datafusion_proto_common::TableParquetOptions::encoded_len` · datafusion-proto-common 55.1.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::TableParquetOptions", "path": "TableParquetOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [718, 28], "end": [718, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:718`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0213132f96c63987d81fe0d5"></a>
## eq

`function` · `datafusion_proto_common::generated::datafusion_proto_common::TableParquetOptions::eq` · datafusion-proto-common 55.1.0

```rust
fn eq(&self, other: &TableParquetOptions) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::TableParquetOptions", "path": "TableParquetOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [718, 17], "end": [718, 26], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/generated/prost.rs:718`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-664448565d09b393777dea43"></a>
## fmt

`function` · `datafusion_proto_common::generated::datafusion_proto_common::TableParquetOptions::fmt` · datafusion-proto-common 55.1.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::TableParquetOptions", "path": "TableParquetOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [718, 28], "end": [718, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generated/prost.rs:718`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4911dd18e418510d551f0786"></a>
## global

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::TableParquetOptions::global` · datafusion-proto-common 55.1.0

```rust
global: ::core::option::Option<ParquetOptions>
```

Source: `src/generated/prost.rs:721`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4f3b5a83056c68cfe57575f3"></a>
## key_value_metadata

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::TableParquetOptions::key_value_metadata` · datafusion-proto-common 55.1.0

```rust
key_value_metadata: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>
```

Source: `src/generated/prost.rs:725`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c850d85e0eff2d812edb6d7a"></a>
## serialize

`function` · `datafusion_proto_common::generated::datafusion_proto_common::TableParquetOptions::serialize` · datafusion-proto-common 55.1.0

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::TableParquetOptions", "path": "TableParquetOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9589, 1], "end": [9618, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/generated/pbjson.rs:9591`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1ee4721e95a4dacc0326aaa9"></a>
## try_from

`function` · `datafusion_proto_common::generated::datafusion_proto_common::TableParquetOptions::try_from` · datafusion-proto-common 55.1.0

```rust
fn try_from(value: &TableParquetOptions) -> datafusion_common::Result<Self, Self::Error>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::TableParquetOptions", "path": "protobuf::TableParquetOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [986, 1], "end": [1015, 2], "filename": "src/to_proto/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_common::config::TableParquetOptions", "path": "TableParquetOptions"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/to_proto/mod.rs:988`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
