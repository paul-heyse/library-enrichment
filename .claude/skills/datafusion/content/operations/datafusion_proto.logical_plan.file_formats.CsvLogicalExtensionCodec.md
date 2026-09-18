# `datafusion_proto::logical_plan::file_formats::CsvLogicalExtensionCodec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.logical_plan.file_formats.CsvLogicalExtensionCodec.json).

<a id="op-073611c75fc06e6db4543cd8"></a>
## CsvLogicalExtensionCodec

`struct` · `datafusion_proto::logical_plan::file_formats::CsvLogicalExtensionCodec` · datafusion-proto 55.1.0

```rust
struct CsvLogicalExtensionCodec
```

Source: `src/logical_plan/file_formats.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-63e44b8e70dbc7ebea4dcf0b"></a>
## fmt

`function` · `datafusion_proto::logical_plan::file_formats::CsvLogicalExtensionCodec::fmt` · datafusion-proto 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::logical_plan::file_formats::CsvLogicalExtensionCodec", "path": "CsvLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 10], "end": [31, 15], "filename": "src/logical_plan/file_formats.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/file_formats.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5184e392d84d4192433fb5e9"></a>
## try_decode

`function` · `datafusion_proto::logical_plan::file_formats::CsvLogicalExtensionCodec::try_decode` · datafusion-proto 55.1.0

```rust
fn try_decode(&self, _buf: &[u8], _inputs: &[datafusion_expr::LogicalPlan], _ctx: &TaskContext) -> datafusion_common::Result<datafusion_expr::Extension>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::logical_plan::file_formats::CsvLogicalExtensionCodec", "path": "CsvLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [107, 2], "filename": "src/logical_plan/file_formats.rs"}, "trait": {"args": null, "id": "datafusion_proto::logical_plan::LogicalExtensionCodec", "path": "LogicalExtensionCodec"}, "trait_path": "datafusion_proto::logical_plan::LogicalExtensionCodec"}`

Source: `src/logical_plan/file_formats.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a4f3cb9b54f3eeb516c12f5b"></a>
## try_decode_file_format

`function` · `datafusion_proto::logical_plan::file_formats::CsvLogicalExtensionCodec::try_decode_file_format` · datafusion-proto 55.1.0

```rust
fn try_decode_file_format(&self, buf: &[u8], _ctx: &TaskContext) -> datafusion_common::Result<Arc<dyn FileFormatFactory>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::logical_plan::file_formats::CsvLogicalExtensionCodec", "path": "CsvLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [107, 2], "filename": "src/logical_plan/file_formats.rs"}, "trait": {"args": null, "id": "datafusion_proto::logical_plan::LogicalExtensionCodec", "path": "LogicalExtensionCodec"}, "trait_path": "datafusion_proto::logical_plan::LogicalExtensionCodec"}`

Source: `src/logical_plan/file_formats.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0ff59926dec40fe888f705a1"></a>
## try_decode_table_provider

`function` · `datafusion_proto::logical_plan::file_formats::CsvLogicalExtensionCodec::try_decode_table_provider` · datafusion-proto 55.1.0

```rust
fn try_decode_table_provider(&self, _buf: &[u8], _table_ref: &TableReference, _schema: arrow::datatypes::SchemaRef, _ctx: &TaskContext) -> datafusion_common::Result<Arc<dyn datafusion_catalog::TableProvider>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::logical_plan::file_formats::CsvLogicalExtensionCodec", "path": "CsvLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [107, 2], "filename": "src/logical_plan/file_formats.rs"}, "trait": {"args": null, "id": "datafusion_proto::logical_plan::LogicalExtensionCodec", "path": "LogicalExtensionCodec"}, "trait_path": "datafusion_proto::logical_plan::LogicalExtensionCodec"}`

Source: `src/logical_plan/file_formats.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6b2c645b733f18e782285b98"></a>
## try_encode

`function` · `datafusion_proto::logical_plan::file_formats::CsvLogicalExtensionCodec::try_encode` · datafusion-proto 55.1.0

```rust
fn try_encode(&self, _node: &datafusion_expr::Extension, _buf: &mut Vec<u8>) -> datafusion_common::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::logical_plan::file_formats::CsvLogicalExtensionCodec", "path": "CsvLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [107, 2], "filename": "src/logical_plan/file_formats.rs"}, "trait": {"args": null, "id": "datafusion_proto::logical_plan::LogicalExtensionCodec", "path": "LogicalExtensionCodec"}, "trait_path": "datafusion_proto::logical_plan::LogicalExtensionCodec"}`

Source: `src/logical_plan/file_formats.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a8df0e0130d127da2e04dc09"></a>
## try_encode_file_format

`function` · `datafusion_proto::logical_plan::file_formats::CsvLogicalExtensionCodec::try_encode_file_format` · datafusion-proto 55.1.0

```rust
fn try_encode_file_format(&self, buf: &mut Vec<u8>, node: Arc<dyn FileFormatFactory>) -> datafusion_common::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::logical_plan::file_formats::CsvLogicalExtensionCodec", "path": "CsvLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [107, 2], "filename": "src/logical_plan/file_formats.rs"}, "trait": {"args": null, "id": "datafusion_proto::logical_plan::LogicalExtensionCodec", "path": "LogicalExtensionCodec"}, "trait_path": "datafusion_proto::logical_plan::LogicalExtensionCodec"}`

Source: `src/logical_plan/file_formats.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d1fb2dfb86e2e2249fd7eb8f"></a>
## try_encode_table_provider

`function` · `datafusion_proto::logical_plan::file_formats::CsvLogicalExtensionCodec::try_encode_table_provider` · datafusion-proto 55.1.0

```rust
fn try_encode_table_provider(&self, _table_ref: &TableReference, _node: Arc<dyn datafusion_catalog::TableProvider>, _buf: &mut Vec<u8>) -> datafusion_common::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::logical_plan::file_formats::CsvLogicalExtensionCodec", "path": "CsvLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [107, 2], "filename": "src/logical_plan/file_formats.rs"}, "trait": {"args": null, "id": "datafusion_proto::logical_plan::LogicalExtensionCodec", "path": "LogicalExtensionCodec"}, "trait_path": "datafusion_proto::logical_plan::LogicalExtensionCodec"}`

Source: `src/logical_plan/file_formats.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
