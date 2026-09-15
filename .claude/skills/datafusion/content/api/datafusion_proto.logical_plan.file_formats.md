# `datafusion_proto::logical_plan::file_formats`

Crate `datafusion-proto` · 4 public items · structured records in [`model/datafusion_proto.logical_plan.file_formats.json`](../model/datafusion_proto.logical_plan.file_formats.json)

## ArrowLogicalExtensionCodec

`struct` · `datafusion_proto::logical_plan::file_formats::ArrowLogicalExtensionCodec`

```rust
struct ArrowLogicalExtensionCodec
```

**Implements**: `datafusion_proto::logical_plan::LogicalExtensionCodec`

**Derives**: Debug

**via `datafusion_proto::logical_plan::LogicalExtensionCodec`**

```rust
fn try_decode(&self, _buf: &[u8], _inputs: &[datafusion_expr::LogicalPlan], _ctx: &TaskContext) -> datafusion_common::Result<datafusion_expr::Extension>
fn try_decode_file_format(&self, __buf: &[u8], __ctx: &TaskContext) -> datafusion_common::Result<Arc<dyn FileFormatFactory>>
fn try_decode_table_provider(&self, _buf: &[u8], _table_ref: &TableReference, _schema: arrow::datatypes::SchemaRef, _ctx: &TaskContext) -> datafusion_common::Result<Arc<dyn datafusion_catalog::TableProvider>>
fn try_encode(&self, _node: &datafusion_expr::Extension, _buf: &mut Vec<u8>) -> datafusion_common::Result<()>
fn try_encode_file_format(&self, __buf: &mut Vec<u8>, __node: Arc<dyn FileFormatFactory>) -> datafusion_common::Result<()>
fn try_encode_table_provider(&self, _table_ref: &TableReference, _node: Arc<dyn datafusion_catalog::TableProvider>, _buf: &mut Vec<u8>) -> datafusion_common::Result<()>
```

---

## AvroLogicalExtensionCodec

`struct` · `datafusion_proto::logical_plan::file_formats::AvroLogicalExtensionCodec`

```rust
struct AvroLogicalExtensionCodec
```

**Implements**: `datafusion_proto::logical_plan::LogicalExtensionCodec`

**Derives**: Debug

**via `datafusion_proto::logical_plan::LogicalExtensionCodec`**

```rust
fn try_decode(&self, _buf: &[u8], _inputs: &[datafusion_expr::LogicalPlan], _ctx: &TaskContext) -> datafusion_common::Result<datafusion_expr::Extension>
fn try_decode_file_format(&self, __buf: &[u8], __ctx: &TaskContext) -> datafusion_common::Result<Arc<dyn FileFormatFactory>>
fn try_decode_table_provider(&self, _buf: &[u8], _table_ref: &TableReference, _schema: arrow::datatypes::SchemaRef, _cts: &TaskContext) -> datafusion_common::Result<Arc<dyn datafusion_catalog::TableProvider>>
fn try_encode(&self, _node: &datafusion_expr::Extension, _buf: &mut Vec<u8>) -> datafusion_common::Result<()>
fn try_encode_file_format(&self, __buf: &mut Vec<u8>, __node: Arc<dyn FileFormatFactory>) -> datafusion_common::Result<()>
fn try_encode_table_provider(&self, _table_ref: &TableReference, _node: Arc<dyn datafusion_catalog::TableProvider>, _buf: &mut Vec<u8>) -> datafusion_common::Result<()>
```

---

## CsvLogicalExtensionCodec

`struct` · `datafusion_proto::logical_plan::file_formats::CsvLogicalExtensionCodec`

```rust
struct CsvLogicalExtensionCodec
```

**Implements**: `datafusion_proto::logical_plan::LogicalExtensionCodec`

**Derives**: Debug

**via `datafusion_proto::logical_plan::LogicalExtensionCodec`**

```rust
fn try_decode(&self, _buf: &[u8], _inputs: &[datafusion_expr::LogicalPlan], _ctx: &TaskContext) -> datafusion_common::Result<datafusion_expr::Extension>
fn try_decode_file_format(&self, buf: &[u8], _ctx: &TaskContext) -> datafusion_common::Result<Arc<dyn FileFormatFactory>>
fn try_decode_table_provider(&self, _buf: &[u8], _table_ref: &TableReference, _schema: arrow::datatypes::SchemaRef, _ctx: &TaskContext) -> datafusion_common::Result<Arc<dyn datafusion_catalog::TableProvider>>
fn try_encode(&self, _node: &datafusion_expr::Extension, _buf: &mut Vec<u8>) -> datafusion_common::Result<()>
fn try_encode_file_format(&self, buf: &mut Vec<u8>, node: Arc<dyn FileFormatFactory>) -> datafusion_common::Result<()>
fn try_encode_table_provider(&self, _table_ref: &TableReference, _node: Arc<dyn datafusion_catalog::TableProvider>, _buf: &mut Vec<u8>) -> datafusion_common::Result<()>
```

---

## JsonLogicalExtensionCodec

`struct` · `datafusion_proto::logical_plan::file_formats::JsonLogicalExtensionCodec`

```rust
struct JsonLogicalExtensionCodec
```

**Implements**: `datafusion_proto::logical_plan::LogicalExtensionCodec`

**Derives**: Debug

**via `datafusion_proto::logical_plan::LogicalExtensionCodec`**

```rust
fn try_decode(&self, _buf: &[u8], _inputs: &[datafusion_expr::LogicalPlan], _ctx: &TaskContext) -> datafusion_common::Result<datafusion_expr::Extension>
fn try_decode_file_format(&self, buf: &[u8], _ctx: &TaskContext) -> datafusion_common::Result<Arc<dyn FileFormatFactory>>
fn try_decode_table_provider(&self, _buf: &[u8], _table_ref: &TableReference, _schema: arrow::datatypes::SchemaRef, _ctx: &TaskContext) -> datafusion_common::Result<Arc<dyn datafusion_catalog::TableProvider>>
fn try_encode(&self, _node: &datafusion_expr::Extension, _buf: &mut Vec<u8>) -> datafusion_common::Result<()>
fn try_encode_file_format(&self, buf: &mut Vec<u8>, node: Arc<dyn FileFormatFactory>) -> datafusion_common::Result<()>
fn try_encode_table_provider(&self, _table_ref: &TableReference, _node: Arc<dyn datafusion_catalog::TableProvider>, _buf: &mut Vec<u8>) -> datafusion_common::Result<()>
```

---
