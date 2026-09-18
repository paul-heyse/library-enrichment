# `datafusion_proto::bytes::physical_plan_from_bytes_with_extension_codec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.bytes.physical_plan_from_bytes_with_extension_codec.json).

<a id="op-4979c9864453e122ca6f54a1"></a>
## physical_plan_from_bytes_with_extension_codec

`function` · `datafusion_proto::bytes::physical_plan_from_bytes_with_extension_codec` · datafusion-proto 55.1.0

```rust
fn physical_plan_from_bytes_with_extension_codec(bytes: &[u8], ctx: &datafusion_execution::TaskContext, extension_codec: &dyn PhysicalExtensionCodec) -> datafusion_common::Result<std::sync::Arc<dyn ExecutionPlan>>
```

Source: `src/bytes/mod.rs:264`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

Deserialize a PhysicalPlan from bytes
