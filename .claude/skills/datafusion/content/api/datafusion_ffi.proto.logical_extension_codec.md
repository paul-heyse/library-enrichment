# `datafusion_ffi::proto::logical_extension_codec`

Crate `datafusion-ffi` · 2 public items · structured records in [`model/datafusion_ffi.proto.logical_extension_codec.json`](../model/datafusion_ffi.proto.logical_extension_codec.json)

## FFI_LogicalExtensionCodec

`struct` · `datafusion_ffi::proto::logical_extension_codec::FFI_LogicalExtensionCodec`

```rust
struct FFI_LogicalExtensionCodec
```

**Fields**: `clone`, `release`, `version`, `private_data`, `library_marker_id`

**Implements**: `core::ops::drop::Drop`

**Derives**: Clone, Debug, Send, Sync

**Methods** (2)

```rust
fn new(codec: Arc<dyn LogicalExtensionCodec>, runtime: Option<Handle>, task_ctx_provider: impl Into<FFI_TaskContextProvider>) -> Self
fn new_default(task_ctx_provider: &Arc<dyn TaskContextProvider>) -> Self
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.proto.logical_extension_codec.FFI_LogicalExtensionCodec.md).


A stable struct for sharing [`LogicalExtensionCodec`] across FFI boundaries.

---

## ForeignLogicalExtensionCodec

`struct` · `datafusion_ffi::proto::logical_extension_codec::ForeignLogicalExtensionCodec`

```rust
struct ForeignLogicalExtensionCodec
```

**Implements**: `datafusion_proto::logical_plan::LogicalExtensionCodec`

**Derives**: Debug, Send, Sync

**via `datafusion_proto::logical_plan::LogicalExtensionCodec`**

```rust
fn try_decode(&self, _buf: &[u8], _inputs: &[LogicalPlan], _ctx: &TaskContext) -> Result<Extension>
fn try_decode_file_format(&self, _buf: &[u8], _ctx: &TaskContext) -> Result<Arc<dyn FileFormatFactory>>
fn try_decode_table_provider(&self, buf: &[u8], table_ref: &TableReference, schema: SchemaRef, _ctx: &TaskContext) -> Result<Arc<dyn TableProvider>>
fn try_decode_udaf(&self, name: &str, buf: &[u8]) -> Result<Arc<AggregateUDF>>
fn try_decode_udf(&self, name: &str, buf: &[u8]) -> Result<Arc<ScalarUDF>>
fn try_decode_udwf(&self, name: &str, buf: &[u8]) -> Result<Arc<WindowUDF>>
fn try_encode(&self, _node: &Extension, _buf: &mut Vec<u8>) -> Result<()>
fn try_encode_file_format(&self, _buf: &mut Vec<u8>, _node: Arc<dyn FileFormatFactory>) -> Result<()>
fn try_encode_table_provider(&self, table_ref: &TableReference, node: Arc<dyn TableProvider>, buf: &mut Vec<u8>) -> Result<()>
fn try_encode_udaf(&self, node: &AggregateUDF, buf: &mut Vec<u8>) -> Result<()>
fn try_encode_udf(&self, node: &ScalarUDF, buf: &mut Vec<u8>) -> Result<()>
fn try_encode_udwf(&self, node: &WindowUDF, buf: &mut Vec<u8>) -> Result<()>
```

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.proto.logical_extension_codec.ForeignLogicalExtensionCodec.md).


This wrapper struct exists on the receiver side of the FFI interface, so it has
no guarantees about being able to access the data in `private_data`. Any functions
defined on this struct must only use the stable functions provided in
FFI_LogicalExtensionCodec to interact with the foreign table provider.

---
