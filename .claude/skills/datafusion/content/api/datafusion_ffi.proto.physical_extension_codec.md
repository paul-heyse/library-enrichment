# `datafusion_ffi::proto::physical_extension_codec`

Crate `datafusion-ffi` · 2 public items · structured records in [`model/datafusion_ffi.proto.physical_extension_codec.json`](../model/datafusion_ffi.proto.physical_extension_codec.json)

## FFI_PhysicalExtensionCodec

`struct` · `datafusion_ffi::proto::physical_extension_codec::FFI_PhysicalExtensionCodec`

```rust
struct FFI_PhysicalExtensionCodec
```

**Fields**: `clone`, `release`, `version`, `private_data`, `library_marker_id`

**Implements**: `core::ops::drop::Drop`

**Derives**: Clone, Debug, Send, Sync

**Methods** (1)

```rust
fn new(codec: Arc<dyn PhysicalExtensionCodec>, runtime: Option<Handle>, task_ctx_provider: impl Into<FFI_TaskContextProvider>) -> Self
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

A stable struct for sharing [`PhysicalExtensionCodec`] across FFI boundaries.

---

## ForeignPhysicalExtensionCodec

`struct` · `datafusion_ffi::proto::physical_extension_codec::ForeignPhysicalExtensionCodec`

```rust
struct ForeignPhysicalExtensionCodec
```

**Implements**: `datafusion_proto::physical_plan::PhysicalExtensionCodec`

**Derives**: Debug, Send, Sync

**via `datafusion_proto::physical_plan::PhysicalExtensionCodec`**

```rust
fn try_decode(&self, buf: &[u8], inputs: &[Arc<dyn ExecutionPlan>], _ctx: &TaskContext, _proto_converter: &dyn PhysicalProtoConverterExtension) -> Result<Arc<dyn ExecutionPlan>>
fn try_decode_udaf(&self, name: &str, buf: &[u8]) -> Result<Arc<AggregateUDF>>
fn try_decode_udf(&self, name: &str, buf: &[u8]) -> Result<Arc<ScalarUDF>>
fn try_decode_udwf(&self, name: &str, buf: &[u8]) -> Result<Arc<WindowUDF>>
fn try_encode(&self, node: Arc<dyn ExecutionPlan>, buf: &mut Vec<u8>, _proto_converter: &dyn PhysicalProtoConverterExtension) -> Result<()>
fn try_encode_udaf(&self, node: &AggregateUDF, buf: &mut Vec<u8>) -> Result<()>
fn try_encode_udf(&self, node: &ScalarUDF, buf: &mut Vec<u8>) -> Result<()>
fn try_encode_udwf(&self, node: &WindowUDF, buf: &mut Vec<u8>) -> Result<()>
```

This wrapper struct exists on the receiver side of the FFI interface, so it has
no guarantees about being able to access the data in `private_data`. Any functions
defined on this struct must only use the stable functions provided in
FFI_PhysicalExtensionCodec to interact with the foreign table provider.

---
