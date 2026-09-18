# `datafusion_proto_models::generated::datafusion::FileScanExecConf`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_models.generated.datafusion.FileScanExecConf.json).

<a id="op-04f424c5f288cef6460ccd58"></a>
## FileScanExecConf

`struct` · `datafusion_proto_models::generated::datafusion::FileScanExecConf` · datafusion-proto-models 55.1.0

```rust
struct FileScanExecConf
```

Source: `src/generated/prost.rs:1916`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0369cd87dcebc43f5373e54f"></a>
## batch_size

`struct_field` · `datafusion_proto_models::generated::datafusion::FileScanExecConf::batch_size` · datafusion-proto-models 55.1.0

```rust
batch_size: ::core::option::Option<u64>
```

Source: `src/generated/prost.rs:1936`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-528b14746eb489b53510bdad"></a>
## batch_size

`function` · `datafusion_proto_models::generated::datafusion::FileScanExecConf::batch_size` · datafusion-proto-models 55.1.0

```rust
fn batch_size(&self) -> u64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileScanExecConf", "path": "FileScanExecConf"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1915, 28], "end": [1915, 44], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:1915`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Returns the value of `batch_size`, or the default value if `batch_size` is unset.

<a id="op-6c90680cf28e009eca200fbf"></a>
## clear

`function` · `datafusion_proto_models::generated::datafusion::FileScanExecConf::clear` · datafusion-proto-models 55.1.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileScanExecConf", "path": "FileScanExecConf"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1915, 28], "end": [1915, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:1915`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-40cc3c44721ea9f5d617c672"></a>
## clone

`function` · `datafusion_proto_models::generated::datafusion::FileScanExecConf::clone` · datafusion-proto-models 55.1.0

```rust
fn clone(&self) -> FileScanExecConf
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileScanExecConf", "path": "FileScanExecConf"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1915, 10], "end": [1915, 15], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generated/prost.rs:1915`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1485d0fdbe4d0fb65a32f2d7"></a>
## constraints

`struct_field` · `datafusion_proto_models::generated::datafusion::FileScanExecConf::constraints` · datafusion-proto-models 55.1.0

```rust
constraints: ::core::option::Option<super::datafusion_common::Constraints>
```

Source: `src/generated/prost.rs:1934`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c3afb1fbca4aed85d4ed6845"></a>
## default

`function` · `datafusion_proto_models::generated::datafusion::FileScanExecConf::default` · datafusion-proto-models 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileScanExecConf", "path": "FileScanExecConf"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1915, 28], "end": [1915, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/generated/prost.rs:1915`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ba7c45de9f7780817b1efeaa"></a>
## deserialize

`function` · `datafusion_proto_models::generated::datafusion::FileScanExecConf::deserialize` · datafusion-proto-models 55.1.0

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileScanExecConf", "path": "FileScanExecConf"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [7064, 1], "end": [7269, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/generated/pbjson.rs:7066`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-af32bedca5497527c34fc7e2"></a>
## encoded_len

`function` · `datafusion_proto_models::generated::datafusion::FileScanExecConf::encoded_len` · datafusion-proto-models 55.1.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileScanExecConf", "path": "FileScanExecConf"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1915, 28], "end": [1915, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:1915`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8d39bf67450242a46fb4d3b7"></a>
## eq

`function` · `datafusion_proto_models::generated::datafusion::FileScanExecConf::eq` · datafusion-proto-models 55.1.0

```rust
fn eq(&self, other: &FileScanExecConf) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileScanExecConf", "path": "FileScanExecConf"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1915, 17], "end": [1915, 26], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/generated/prost.rs:1915`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2a96b09bff819422f6436079"></a>
## file_groups

`struct_field` · `datafusion_proto_models::generated::datafusion::FileScanExecConf::file_groups` · datafusion-proto-models 55.1.0

```rust
file_groups: ::prost::alloc::vec::Vec<FileGroup>
```

Source: `src/generated/prost.rs:1918`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-857006fa136c3042fd8cf407"></a>
## fmt

`function` · `datafusion_proto_models::generated::datafusion::FileScanExecConf::fmt` · datafusion-proto-models 55.1.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileScanExecConf", "path": "FileScanExecConf"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1915, 28], "end": [1915, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generated/prost.rs:1915`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-deaad5b3f81d7c559246821c"></a>
## limit

`struct_field` · `datafusion_proto_models::generated::datafusion::FileScanExecConf::limit` · datafusion-proto-models 55.1.0

```rust
limit: ::core::option::Option<ScanLimit>
```

Source: `src/generated/prost.rs:1924`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2aea2f138bd8c9eccd09e430"></a>
## object_store_url

`struct_field` · `datafusion_proto_models::generated::datafusion::FileScanExecConf::object_store_url` · datafusion-proto-models 55.1.0

```rust
object_store_url: ::prost::alloc::string::String
```

Source: `src/generated/prost.rs:1930`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ee6e649c232a4a24045d9df2"></a>
## output_ordering

`struct_field` · `datafusion_proto_models::generated::datafusion::FileScanExecConf::output_ordering` · datafusion-proto-models 55.1.0

```rust
output_ordering: ::prost::alloc::vec::Vec<PhysicalSortExprNodeCollection>
```

Source: `src/generated/prost.rs:1932`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-92ece0bf67f869aa2a12b307"></a>
## output_partitioning

`struct_field` · `datafusion_proto_models::generated::datafusion::FileScanExecConf::output_partitioning` · datafusion-proto-models 55.1.0

```rust
output_partitioning: ::core::option::Option<Partitioning>
```

Source: `src/generated/prost.rs:1940`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-14f7d1f63a121039e77e5374"></a>
## projection

`struct_field` · `datafusion_proto_models::generated::datafusion::FileScanExecConf::projection` · datafusion-proto-models 55.1.0

```rust
projection: ::prost::alloc::vec::Vec<u32>
```

Source: `src/generated/prost.rs:1922`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e26cc79b3caef146d0855d78"></a>
## projection_exprs

`struct_field` · `datafusion_proto_models::generated::datafusion::FileScanExecConf::projection_exprs` · datafusion-proto-models 55.1.0

```rust
projection_exprs: ::core::option::Option<ProjectionExprs>
```

Source: `src/generated/prost.rs:1938`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1f996b3d9e3707f58758316b"></a>
## schema

`struct_field` · `datafusion_proto_models::generated::datafusion::FileScanExecConf::schema` · datafusion-proto-models 55.1.0

```rust
schema: ::core::option::Option<super::datafusion_common::Schema>
```

Source: `src/generated/prost.rs:1920`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-af6fef869e3ce55aee44ef15"></a>
## serialize

`function` · `datafusion_proto_models::generated::datafusion::FileScanExecConf::serialize` · datafusion-proto-models 55.1.0

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileScanExecConf", "path": "FileScanExecConf"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6978, 1], "end": [7063, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/generated/pbjson.rs:6980`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7362528c204a98a5ae562f67"></a>
## statistics

`struct_field` · `datafusion_proto_models::generated::datafusion::FileScanExecConf::statistics` · datafusion-proto-models 55.1.0

```rust
statistics: ::core::option::Option<super::datafusion_common::Statistics>
```

Source: `src/generated/prost.rs:1926`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fdfbe6eb330afa9d17ffd787"></a>
## table_partition_cols

`struct_field` · `datafusion_proto_models::generated::datafusion::FileScanExecConf::table_partition_cols` · datafusion-proto-models 55.1.0

```rust
table_partition_cols: ::prost::alloc::vec::Vec<::prost::alloc::string::String>
```

Source: `src/generated/prost.rs:1928`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
