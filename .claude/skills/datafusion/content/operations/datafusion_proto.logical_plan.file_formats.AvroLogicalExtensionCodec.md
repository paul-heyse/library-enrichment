# `datafusion_proto::logical_plan::file_formats::AvroLogicalExtensionCodec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.logical_plan.file_formats.AvroLogicalExtensionCodec.json).

<a id="op-50db5d89d9029a00e3e545aa"></a>
## AvroLogicalExtensionCodec

`struct` · `datafusion_proto::logical_plan::file_formats::AvroLogicalExtensionCodec` · datafusion-proto 55.1.0

```rust
struct AvroLogicalExtensionCodec
```

Source: `src/logical_plan/file_formats.rs:401`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4521c0f59d8950020eb04f52"></a>
## fmt

`function` · `datafusion_proto::logical_plan::file_formats::AvroLogicalExtensionCodec::fmt` · datafusion-proto 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::logical_plan::file_formats::AvroLogicalExtensionCodec", "path": "AvroLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [400, 10], "end": [400, 15], "filename": "src/logical_plan/file_formats.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/file_formats.rs:400`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a3baef6bb5fcd08305ac4d1c"></a>
## try_decode

`function` · `datafusion_proto::logical_plan::file_formats::AvroLogicalExtensionCodec::try_decode` · datafusion-proto 55.1.0

```rust
fn try_decode(&self, _buf: &[u8], _inputs: &[datafusion_expr::LogicalPlan], _ctx: &TaskContext) -> datafusion_common::Result<datafusion_expr::Extension>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::logical_plan::file_formats::AvroLogicalExtensionCodec", "path": "AvroLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [404, 1], "end": [456, 2], "filename": "src/logical_plan/file_formats.rs"}, "trait": {"args": null, "id": "datafusion_proto::logical_plan::LogicalExtensionCodec", "path": "LogicalExtensionCodec"}, "trait_path": "datafusion_proto::logical_plan::LogicalExtensionCodec"}`

Source: `src/logical_plan/file_formats.rs:405`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a2e8cd1e729ccd67163c4630"></a>
## try_decode_file_format

`function` · `datafusion_proto::logical_plan::file_formats::AvroLogicalExtensionCodec::try_decode_file_format` · datafusion-proto 55.1.0

```rust
fn try_decode_file_format(&self, __buf: &[u8], __ctx: &TaskContext) -> datafusion_common::Result<Arc<dyn FileFormatFactory>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::logical_plan::file_formats::AvroLogicalExtensionCodec", "path": "AvroLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [404, 1], "end": [456, 2], "filename": "src/logical_plan/file_formats.rs"}, "trait": {"args": null, "id": "datafusion_proto::logical_plan::LogicalExtensionCodec", "path": "LogicalExtensionCodec"}, "trait_path": "datafusion_proto::logical_plan::LogicalExtensionCodec"}`

Source: `src/logical_plan/file_formats.rs:441`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-019e4fc72a87f62e4eb116b4"></a>
## try_decode_table_provider

`function` · `datafusion_proto::logical_plan::file_formats::AvroLogicalExtensionCodec::try_decode_table_provider` · datafusion-proto 55.1.0

```rust
fn try_decode_table_provider(&self, _buf: &[u8], _table_ref: &TableReference, _schema: arrow::datatypes::SchemaRef, _cts: &TaskContext) -> datafusion_common::Result<Arc<dyn datafusion_catalog::TableProvider>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::logical_plan::file_formats::AvroLogicalExtensionCodec", "path": "AvroLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [404, 1], "end": [456, 2], "filename": "src/logical_plan/file_formats.rs"}, "trait": {"args": null, "id": "datafusion_proto::logical_plan::LogicalExtensionCodec", "path": "LogicalExtensionCodec"}, "trait_path": "datafusion_proto::logical_plan::LogicalExtensionCodec"}`

Source: `src/logical_plan/file_formats.rs:422`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c80bf3bbabdf8bc146317b6c"></a>
## try_encode

`function` · `datafusion_proto::logical_plan::file_formats::AvroLogicalExtensionCodec::try_encode` · datafusion-proto 55.1.0

```rust
fn try_encode(&self, _node: &datafusion_expr::Extension, _buf: &mut Vec<u8>) -> datafusion_common::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::logical_plan::file_formats::AvroLogicalExtensionCodec", "path": "AvroLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [404, 1], "end": [456, 2], "filename": "src/logical_plan/file_formats.rs"}, "trait": {"args": null, "id": "datafusion_proto::logical_plan::LogicalExtensionCodec", "path": "LogicalExtensionCodec"}, "trait_path": "datafusion_proto::logical_plan::LogicalExtensionCodec"}`

Source: `src/logical_plan/file_formats.rs:414`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e9fcf0f9bba4d0dcfc82398f"></a>
## try_encode_file_format

`function` · `datafusion_proto::logical_plan::file_formats::AvroLogicalExtensionCodec::try_encode_file_format` · datafusion-proto 55.1.0

```rust
fn try_encode_file_format(&self, __buf: &mut Vec<u8>, __node: Arc<dyn FileFormatFactory>) -> datafusion_common::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::logical_plan::file_formats::AvroLogicalExtensionCodec", "path": "AvroLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [404, 1], "end": [456, 2], "filename": "src/logical_plan/file_formats.rs"}, "trait": {"args": null, "id": "datafusion_proto::logical_plan::LogicalExtensionCodec", "path": "LogicalExtensionCodec"}, "trait_path": "datafusion_proto::logical_plan::LogicalExtensionCodec"}`

Source: `src/logical_plan/file_formats.rs:449`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6822e92a12e476438dc4b8c6"></a>
## try_encode_table_provider

`function` · `datafusion_proto::logical_plan::file_formats::AvroLogicalExtensionCodec::try_encode_table_provider` · datafusion-proto 55.1.0

```rust
fn try_encode_table_provider(&self, _table_ref: &TableReference, _node: Arc<dyn datafusion_catalog::TableProvider>, _buf: &mut Vec<u8>) -> datafusion_common::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::logical_plan::file_formats::AvroLogicalExtensionCodec", "path": "AvroLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [404, 1], "end": [456, 2], "filename": "src/logical_plan/file_formats.rs"}, "trait": {"args": null, "id": "datafusion_proto::logical_plan::LogicalExtensionCodec", "path": "LogicalExtensionCodec"}, "trait_path": "datafusion_proto::logical_plan::LogicalExtensionCodec"}`

Source: `src/logical_plan/file_formats.rs:432`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
