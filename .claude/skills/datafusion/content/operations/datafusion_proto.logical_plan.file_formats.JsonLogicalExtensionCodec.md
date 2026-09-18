# `datafusion_proto::logical_plan::file_formats::JsonLogicalExtensionCodec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.logical_plan.file_formats.JsonLogicalExtensionCodec.json).

<a id="op-ae9fc3f7eaf50066da34aab3"></a>
## JsonLogicalExtensionCodec

`struct` · `datafusion_proto::logical_plan::file_formats::JsonLogicalExtensionCodec` · datafusion-proto 55.1.0

```rust
struct JsonLogicalExtensionCodec
```

Source: `src/logical_plan/file_formats.rs:110`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0f01a0909c0806f8593aab26"></a>
## fmt

`function` · `datafusion_proto::logical_plan::file_formats::JsonLogicalExtensionCodec::fmt` · datafusion-proto 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::logical_plan::file_formats::JsonLogicalExtensionCodec", "path": "JsonLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [109, 10], "end": [109, 15], "filename": "src/logical_plan/file_formats.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/file_formats.rs:109`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8f159d69c69efac70fb2073e"></a>
## try_decode

`function` · `datafusion_proto::logical_plan::file_formats::JsonLogicalExtensionCodec::try_decode` · datafusion-proto 55.1.0

```rust
fn try_decode(&self, _buf: &[u8], _inputs: &[datafusion_expr::LogicalPlan], _ctx: &TaskContext) -> datafusion_common::Result<datafusion_expr::Extension>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::logical_plan::file_formats::JsonLogicalExtensionCodec", "path": "JsonLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [113, 1], "end": [186, 2], "filename": "src/logical_plan/file_formats.rs"}, "trait": {"args": null, "id": "datafusion_proto::logical_plan::LogicalExtensionCodec", "path": "LogicalExtensionCodec"}, "trait_path": "datafusion_proto::logical_plan::LogicalExtensionCodec"}`

Source: `src/logical_plan/file_formats.rs:114`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2d24fae131fa99cee98cfbed"></a>
## try_decode_file_format

`function` · `datafusion_proto::logical_plan::file_formats::JsonLogicalExtensionCodec::try_decode_file_format` · datafusion-proto 55.1.0

```rust
fn try_decode_file_format(&self, buf: &[u8], _ctx: &TaskContext) -> datafusion_common::Result<Arc<dyn FileFormatFactory>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::logical_plan::file_formats::JsonLogicalExtensionCodec", "path": "JsonLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [113, 1], "end": [186, 2], "filename": "src/logical_plan/file_formats.rs"}, "trait": {"args": null, "id": "datafusion_proto::logical_plan::LogicalExtensionCodec", "path": "LogicalExtensionCodec"}, "trait_path": "datafusion_proto::logical_plan::LogicalExtensionCodec"}`

Source: `src/logical_plan/file_formats.rs:150`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-339e7f165a44c851c6bac110"></a>
## try_decode_table_provider

`function` · `datafusion_proto::logical_plan::file_formats::JsonLogicalExtensionCodec::try_decode_table_provider` · datafusion-proto 55.1.0

```rust
fn try_decode_table_provider(&self, _buf: &[u8], _table_ref: &TableReference, _schema: arrow::datatypes::SchemaRef, _ctx: &TaskContext) -> datafusion_common::Result<Arc<dyn datafusion_catalog::TableProvider>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::logical_plan::file_formats::JsonLogicalExtensionCodec", "path": "JsonLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [113, 1], "end": [186, 2], "filename": "src/logical_plan/file_formats.rs"}, "trait": {"args": null, "id": "datafusion_proto::logical_plan::LogicalExtensionCodec", "path": "LogicalExtensionCodec"}, "trait_path": "datafusion_proto::logical_plan::LogicalExtensionCodec"}`

Source: `src/logical_plan/file_formats.rs:131`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aef6ef3b74d97a1755621f77"></a>
## try_encode

`function` · `datafusion_proto::logical_plan::file_formats::JsonLogicalExtensionCodec::try_encode` · datafusion-proto 55.1.0

```rust
fn try_encode(&self, _node: &datafusion_expr::Extension, _buf: &mut Vec<u8>) -> datafusion_common::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::logical_plan::file_formats::JsonLogicalExtensionCodec", "path": "JsonLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [113, 1], "end": [186, 2], "filename": "src/logical_plan/file_formats.rs"}, "trait": {"args": null, "id": "datafusion_proto::logical_plan::LogicalExtensionCodec", "path": "LogicalExtensionCodec"}, "trait_path": "datafusion_proto::logical_plan::LogicalExtensionCodec"}`

Source: `src/logical_plan/file_formats.rs:123`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a8886f427f2d3ad73108d96a"></a>
## try_encode_file_format

`function` · `datafusion_proto::logical_plan::file_formats::JsonLogicalExtensionCodec::try_encode_file_format` · datafusion-proto 55.1.0

```rust
fn try_encode_file_format(&self, buf: &mut Vec<u8>, node: Arc<dyn FileFormatFactory>) -> datafusion_common::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::logical_plan::file_formats::JsonLogicalExtensionCodec", "path": "JsonLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [113, 1], "end": [186, 2], "filename": "src/logical_plan/file_formats.rs"}, "trait": {"args": null, "id": "datafusion_proto::logical_plan::LogicalExtensionCodec", "path": "LogicalExtensionCodec"}, "trait_path": "datafusion_proto::logical_plan::LogicalExtensionCodec"}`

Source: `src/logical_plan/file_formats.rs:164`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9cb67ce59875e4e938db5114"></a>
## try_encode_table_provider

`function` · `datafusion_proto::logical_plan::file_formats::JsonLogicalExtensionCodec::try_encode_table_provider` · datafusion-proto 55.1.0

```rust
fn try_encode_table_provider(&self, _table_ref: &TableReference, _node: Arc<dyn datafusion_catalog::TableProvider>, _buf: &mut Vec<u8>) -> datafusion_common::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::logical_plan::file_formats::JsonLogicalExtensionCodec", "path": "JsonLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [113, 1], "end": [186, 2], "filename": "src/logical_plan/file_formats.rs"}, "trait": {"args": null, "id": "datafusion_proto::logical_plan::LogicalExtensionCodec", "path": "LogicalExtensionCodec"}, "trait_path": "datafusion_proto::logical_plan::LogicalExtensionCodec"}`

Source: `src/logical_plan/file_formats.rs:141`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
