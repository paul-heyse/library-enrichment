# `datafusion_proto::bytes::logical_plan_from_bytes_with_extension_codec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.bytes.logical_plan_from_bytes_with_extension_codec.json).

<a id="op-ff3013ece16bfe2c4ec26b63"></a>
## logical_plan_from_bytes_with_extension_codec

`function` · `datafusion_proto::bytes::logical_plan_from_bytes_with_extension_codec` · datafusion-proto 55.1.0

```rust
fn logical_plan_from_bytes_with_extension_codec(bytes: &[u8], ctx: &datafusion_execution::TaskContext, extension_codec: &dyn LogicalExtensionCodec) -> datafusion_common::Result<datafusion_expr::LogicalPlan>
```

Source: `src/bytes/mod.rs:164`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

Deserialize a LogicalPlan from bytes
