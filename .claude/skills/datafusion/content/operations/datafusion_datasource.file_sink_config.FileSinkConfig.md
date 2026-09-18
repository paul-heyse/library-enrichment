# `datafusion_datasource::file_sink_config::FileSinkConfig`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.file_sink_config.FileSinkConfig.json).

<a id="op-6f456460209c39e089a175e2"></a>
## FileSinkConfig

`struct` · `datafusion_datasource::file_sink_config::FileSinkConfig` · datafusion-datasource 55.1.0

```rust
struct FileSinkConfig
```

Source: `src/file_sink_config.rs:142`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

The base configurations to provide when creating a physical plan for
writing to any given file format.

<a id="op-a0ecf560a6864f573c3b08a4"></a>
## Error

`assoc_type` · `datafusion_datasource::file_sink_config::FileSinkConfig::Error` · datafusion-datasource 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_sink_config::FileSinkConfig", "path": "crate::file_sink_config::FileSinkConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [82, 1], "end": [155, 2], "filename": "src/file_sink_config/proto.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileSinkConfig", "path": "FileSinkConfig"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/file_sink_config/proto.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4e02156ffa5a01fe6fed8a4e"></a>
## clone

`function` · `datafusion_datasource::file_sink_config::FileSinkConfig::clone` · datafusion-datasource 55.1.0

```rust
fn clone(&self) -> FileSinkConfig
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_sink_config::FileSinkConfig", "path": "FileSinkConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [141, 17], "end": [141, 22], "filename": "src/file_sink_config.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/file_sink_config.rs:141`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bed2e8e5e5686fc480855a3d"></a>
## file_extension

`struct_field` · `datafusion_datasource::file_sink_config::FileSinkConfig::file_extension` · datafusion-datasource 55.1.0

```rust
file_extension: String
```

Source: `src/file_sink_config.rs:163`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

File extension without a dot(.)

<a id="op-bfd3bc7cedfc7c318badf399"></a>
## file_group

`struct_field` · `datafusion_datasource::file_sink_config::FileSinkConfig::file_group` · datafusion-datasource 55.1.0

```rust
file_group: file_groups::FileGroup
```

Source: `src/file_sink_config.rs:149`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

A collection of files organized into groups.
Each FileGroup contains one or more PartitionedFile objects.

<a id="op-f838b3daa78ae1a6ab5d62b8"></a>
## file_output_mode

`struct_field` · `datafusion_datasource::file_sink_config::FileSinkConfig::file_output_mode` · datafusion-datasource 55.1.0

```rust
file_output_mode: FileOutputMode
```

Source: `src/file_sink_config.rs:165`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Determines how the output path is interpreted.

<a id="op-0b995e780e37fb86d811e09c"></a>
## fmt

`function` · `datafusion_datasource::file_sink_config::FileSinkConfig::fmt` · datafusion-datasource 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_sink_config::FileSinkConfig", "path": "FileSinkConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [141, 10], "end": [141, 15], "filename": "src/file_sink_config.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file_sink_config.rs:141`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-31a3ba5f1c6f2880c07f52e6"></a>
## insert_op

`struct_field` · `datafusion_datasource::file_sink_config::FileSinkConfig::insert_op` · datafusion-datasource 55.1.0

```rust
insert_op: datafusion_expr::dml::InsertOp
```

Source: `src/file_sink_config.rs:159`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Controls how new data should be written to the file, determining whether
to append to, overwrite, or replace records in existing files.

<a id="op-7c0ff67b93e508a44c5f41c0"></a>
## keep_partition_by_columns

`struct_field` · `datafusion_datasource::file_sink_config::FileSinkConfig::keep_partition_by_columns` · datafusion-datasource 55.1.0

```rust
keep_partition_by_columns: bool
```

Source: `src/file_sink_config.rs:161`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Controls whether partition columns are kept for the file

<a id="op-a5358020432fff481c02eb64"></a>
## object_store_url

`struct_field` · `datafusion_datasource::file_sink_config::FileSinkConfig::object_store_url` · datafusion-datasource 55.1.0

```rust
object_store_url: datafusion_execution::object_store::ObjectStoreUrl
```

Source: `src/file_sink_config.rs:146`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Object store URL, used to get an ObjectStore instance

<a id="op-e75edcd1f81a87d83cda3e36"></a>
## original_url

`struct_field` · `datafusion_datasource::file_sink_config::FileSinkConfig::original_url` · datafusion-datasource 55.1.0

```rust
original_url: String
```

Source: `src/file_sink_config.rs:144`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

The unresolved URL specified by the user

<a id="op-296716e91cf7b3531f7c5a6e"></a>
## output_schema

`struct_field` · `datafusion_datasource::file_sink_config::FileSinkConfig::output_schema` · datafusion-datasource 55.1.0

```rust
output_schema: arrow::datatypes::SchemaRef
```

Source: `src/file_sink_config.rs:153`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

The schema of the output file

<a id="op-c3d6982fa0702e28f09c1b32"></a>
## output_schema

`function` · `datafusion_datasource::file_sink_config::FileSinkConfig::output_schema` · datafusion-datasource 55.1.0

```rust
fn output_schema(&self) -> &SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_sink_config::FileSinkConfig", "path": "FileSinkConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [168, 1], "end": [173, 2], "filename": "src/file_sink_config.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_sink_config.rs:170`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Get output schema

<a id="op-87d273949748720351499c27"></a>
## table_partition_cols

`struct_field` · `datafusion_datasource::file_sink_config::FileSinkConfig::table_partition_cols` · datafusion-datasource 55.1.0

```rust
table_partition_cols: Vec<(String, arrow::datatypes::DataType)>
```

Source: `src/file_sink_config.rs:156`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

A vector of column names and their corresponding data types,
representing the partitioning columns for the file

<a id="op-9df31c23360c2420f248b33a"></a>
## table_paths

`struct_field` · `datafusion_datasource::file_sink_config::FileSinkConfig::table_paths` · datafusion-datasource 55.1.0

```rust
table_paths: Vec<ListingTableUrl>
```

Source: `src/file_sink_config.rs:151`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Vector of partition paths

<a id="op-26f73138df4bf21f7539bb9a"></a>
## try_from

`function` · `datafusion_datasource::file_sink_config::FileSinkConfig::try_from` · datafusion-datasource 55.1.0

```rust
fn try_from(conf: &protobuf::FileSinkConfig) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_sink_config::FileSinkConfig", "path": "crate::file_sink_config::FileSinkConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [82, 1], "end": [155, 2], "filename": "src/file_sink_config/proto.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileSinkConfig", "path": "FileSinkConfig"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/file_sink_config/proto.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Reconstruct a shared file-sink configuration from protobuf.
