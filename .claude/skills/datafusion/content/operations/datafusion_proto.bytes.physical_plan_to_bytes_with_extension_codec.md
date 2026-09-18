# `datafusion_proto::bytes::physical_plan_to_bytes_with_extension_codec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.bytes.physical_plan_to_bytes_with_extension_codec.json).

<a id="op-edb53e85ec9297e5dfd8c4a3"></a>
## physical_plan_to_bytes_with_extension_codec

`function` · `datafusion_proto::bytes::physical_plan_to_bytes_with_extension_codec` · datafusion-proto 55.1.0

```rust
fn physical_plan_to_bytes_with_extension_codec(plan: std::sync::Arc<dyn ExecutionPlan>, extension_codec: &dyn PhysicalExtensionCodec) -> datafusion_common::Result<prost::bytes::Bytes>
```

Source: `src/bytes/mod.rs:210`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

Serialize a PhysicalPlan as bytes, using the provided extension codec
